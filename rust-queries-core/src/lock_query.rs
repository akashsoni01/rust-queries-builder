//! Full SQL-like query support for locked data structures.
//!
//! This module provides a complete Query API for collections of locked values,
//! enabling WHERE, SELECT, ORDER BY, GROUP BY, aggregations, and JOIN operations
//! without copying data unnecessarily.
//!
//! # Example
//!
//! ```ignore
//! use rust_queries_core::{LockQuery};
//! use std::sync::{Arc, RwLock};
//! use std::collections::HashMap;
//!
//! let products: HashMap<String, Arc<RwLock<Product>>> = /* ... */;
//!
//! // Full SQL-like syntax on locked data!
//! let expensive = LockQuery::new(&products)
//!     .where_(Product::category_r(), |cat| cat == "Electronics")
//!     .where_(Product::price_r(), |&p| p > 500.0)
//!     .order_by_float(Product::rating_r())
//!     .limit(10);
//! ```

use crate::locks::LockValue;
use key_paths_core::KeyPaths;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};

/// A query builder for locked data structures.
///
/// Provides full SQL-like query operations (WHERE, SELECT, ORDER BY, GROUP BY)
/// on collections of locked values without unnecessary copying.
pub struct LockQuery<'a, T: 'static, L>
where
    L: LockValue<T> + 'a,
{
    locks: Vec<&'a L>,
    filters: Vec<Box<dyn Fn(&T) -> bool + 'a>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: 'static, L> LockQuery<'a, T, L>
where
    L: LockValue<T> + 'a,
{
    /// Create a new lock query from a collection of locks.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LockQuery::from_locks(product_map.values().collect());
    /// ```
    pub fn from_locks(locks: Vec<&'a L>) -> Self {
        Self {
            locks,
            filters: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a WHERE clause using a key-path.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LockQuery::new(&products)
    ///     .where_(Product::category_r(), |cat| cat == "Electronics");
    /// ```
    pub fn where_<F>(mut self, path: KeyPaths<T, F>, predicate: impl Fn(&F) -> bool + 'a) -> Self
    where
        F: 'static,
    {
        self.filters.push(Box::new(move |item| {
            path.get(item).map_or(false, |val| predicate(val))
        }));
        self
    }

    /// Get all matching items (collects by cloning).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<Product> = query.all();
    /// ```
    pub fn all(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.locks
            .iter()
            .filter_map(|lock| {
                lock.with_value(|item| {
                    if self.filters.iter().all(|f| f(item)) {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .flatten()
            })
            .collect()
    }

    /// Get the first matching item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = query.first();
    /// ```
    pub fn first(&self) -> Option<T>
    where
        T: Clone,
    {
        self.locks
            .iter()
            .find_map(|lock| {
                lock.with_value(|item| {
                    if self.filters.iter().all(|f| f(item)) {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .flatten()
            })
    }

    /// Count matching items.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count();
    /// ```
    pub fn count(&self) -> usize {
        self.locks
            .iter()
            .filter(|lock| {
                lock.with_value(|item| self.filters.iter().all(|f| f(item)))
                    .unwrap_or(false)
            })
            .count()
    }

    /// Check if any items match.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let exists = query.exists();
    /// ```
    pub fn exists(&self) -> bool {
        self.locks
            .iter()
            .any(|lock| {
                lock.with_value(|item| self.filters.iter().all(|f| f(item)))
                    .unwrap_or(false)
            })
    }

    /// Limit results to first N items.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first_10 = query.limit(10);
    /// ```
    pub fn limit(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.locks
            .iter()
            .filter_map(|lock| {
                lock.with_value(|item| {
                    if self.filters.iter().all(|f| f(item)) {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .flatten()
            })
            .take(n)
            .collect()
    }

    /// Select/project a field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let names: Vec<String> = query.select(Product::name_r());
    /// ```
    pub fn select<F>(&self, path: KeyPaths<T, F>) -> Vec<F>
    where
        F: Clone + 'static,
    {
        self.locks
            .iter()
            .filter_map(|lock| {
                lock.with_value(|item| {
                    if self.filters.iter().all(|f| f(item)) {
                        path.get(item).cloned()
                    } else {
                        None
                    }
                })
                .flatten()
            })
            .collect()
    }

    /// Sum a numeric field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total = query.sum(Product::price_r());
    /// ```
    pub fn sum<F>(&self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static,
    {
        self.locks
            .iter()
            .filter_map(|lock| {
                lock.with_value(|item| {
                    if self.filters.iter().all(|f| f(item)) {
                        path.get(item).cloned()
                    } else {
                        None
                    }
                })
                .flatten()
            })
            .fold(F::default(), |acc, val| acc + val)
    }

    /// Calculate average of f64 field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = query.avg(Product::price_r());
    /// ```
    pub fn avg(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        let values: Vec<f64> = self.select(path);
        if values.is_empty() {
            None
        } else {
            Some(values.iter().sum::<f64>() / values.len() as f64)
        }
    }

    /// Find minimum value.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min = query.min(Product::stock_r());
    /// ```
    pub fn min<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.select(path).into_iter().min()
    }

    /// Find maximum value.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max = query.max(Product::stock_r());
    /// ```
    pub fn max<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.select(path).into_iter().max()
    }

    /// Find minimum float value.
    pub fn min_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.select(path)
            .into_iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find maximum float value.
    pub fn max_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.select(path)
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Order by a field (requires collecting data).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by(Product::name_r());
    /// ```
    pub fn order_by<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
        T: Clone,
    {
        let mut results = self.all();
        results.sort_by_key(|item| path.get(item).cloned());
        results
    }

    /// Order by a field descending.
    pub fn order_by_desc<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
        T: Clone,
    {
        let mut results = self.all();
        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned();
            let b_val = path.get(b).cloned();
            b_val.cmp(&a_val)
        });
        results
    }

    /// Order by float field.
    pub fn order_by_float(&self, path: KeyPaths<T, f64>) -> Vec<T>
    where
        T: Clone,
    {
        let mut results = self.all();
        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Order by float field descending.
    pub fn order_by_float_desc(&self, path: KeyPaths<T, f64>) -> Vec<T>
    where
        T: Clone,
    {
        let mut results = self.all();
        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Group by a field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let groups = query.group_by(Product::category_r());
    /// ```
    pub fn group_by<F>(&self, path: KeyPaths<T, F>) -> HashMap<F, Vec<T>>
    where
        F: Eq + std::hash::Hash + Clone + 'static,
        T: Clone,
    {
        let mut groups: HashMap<F, Vec<T>> = HashMap::new();

        for lock in &self.locks {
            if let Some(item) = lock.with_value(|item| {
                if self.filters.iter().all(|f| f(item)) {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .flatten()
            {
                if let Some(key) = path.get(&item).cloned() {
                    groups.entry(key).or_insert_with(Vec::new).push(item);
                }
            }
        }

        groups
    }
}

/// Helper to create LockQuery from HashMap.
pub trait LockQueryable<T, L>
where
    L: LockValue<T>,
{
    /// Create a LockQuery for SQL-like operations.
    fn lock_query(&self) -> LockQuery<'_, T, L>;
}

// Implementation for HashMap<K, Arc<RwLock<V>>>
impl<K, V> LockQueryable<V, Arc<RwLock<V>>> for HashMap<K, Arc<RwLock<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_query(&self) -> LockQuery<'_, V, Arc<RwLock<V>>> {
        LockQuery::from_locks(self.values().collect())
    }
}

// Implementation for HashMap<K, Arc<Mutex<V>>>
impl<K, V> LockQueryable<V, Arc<Mutex<V>>> for HashMap<K, Arc<Mutex<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_query(&self) -> LockQuery<'_, V, Arc<Mutex<V>>> {
        LockQuery::from_locks(self.values().collect())
    }
}

// Implementation for Vec<Arc<RwLock<T>>>
impl<T> LockQueryable<T, Arc<RwLock<T>>> for Vec<Arc<RwLock<T>>> {
    fn lock_query(&self) -> LockQuery<'_, T, Arc<RwLock<T>>> {
        LockQuery::from_locks(self.iter().collect())
    }
}

// Implementation for Vec<Arc<Mutex<T>>>
impl<T> LockQueryable<T, Arc<Mutex<T>>> for Vec<Arc<Mutex<T>>> {
    fn lock_query(&self) -> LockQuery<'_, T, Arc<Mutex<T>>> {
        LockQuery::from_locks(self.iter().collect())
    }
}

// Extension trait for creating lazy lock queries
use crate::lock_lazy::LockLazyQuery;

/// Extension trait for creating lazy lock queries.
pub trait LockLazyQueryable<T, L>
where
    L: LockValue<T>,
{
    /// Create a lazy lock query.
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, T, L, impl Iterator<Item = &L>>;
}

// Implementation for HashMap<K, Arc<RwLock<V>>>
impl<K, V> LockLazyQueryable<V, Arc<RwLock<V>>> for HashMap<K, Arc<RwLock<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, Arc<RwLock<V>>, impl Iterator<Item = &Arc<RwLock<V>>>> {
        LockLazyQuery::new(self.values())
    }
}

// Implementation for HashMap<K, Arc<Mutex<V>>>
impl<K, V> LockLazyQueryable<V, Arc<Mutex<V>>> for HashMap<K, Arc<Mutex<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, Arc<Mutex<V>>, impl Iterator<Item = &Arc<Mutex<V>>>> {
        LockLazyQuery::new(self.values())
    }
}

// Implementation for Vec<Arc<RwLock<T>>>
impl<T> LockLazyQueryable<T, Arc<RwLock<T>>> for Vec<Arc<RwLock<T>>> {
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, T, Arc<RwLock<T>>, impl Iterator<Item = &Arc<RwLock<T>>>> {
        LockLazyQuery::new(self.iter())
    }
}

// Implementation for Vec<Arc<Mutex<T>>>
impl<T> LockLazyQueryable<T, Arc<Mutex<T>>> for Vec<Arc<Mutex<T>>> {
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, T, Arc<Mutex<T>>, impl Iterator<Item = &Arc<Mutex<T>>>> {
        LockLazyQuery::new(self.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use key_paths_derive::Keypaths;

    #[derive(Clone, Keypaths)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
        category: String,
    }

    fn create_test_map() -> HashMap<String, Arc<RwLock<Product>>> {
        let mut map = HashMap::new();
        map.insert(
            "p1".to_string(),
            Arc::new(RwLock::new(Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
            })),
        );
        map.insert(
            "p2".to_string(),
            Arc::new(RwLock::new(Product {
                id: 2,
                name: "Chair".to_string(),
                price: 299.99,
                category: "Furniture".to_string(),
            })),
        );
        map.insert(
            "p3".to_string(),
            Arc::new(RwLock::new(Product {
                id: 3,
                name: "Mouse".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            })),
        );
        map
    }

    #[test]
    fn test_lock_query_where() {
        let map = create_test_map();
        let query = map.lock_query();
        let count = query
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_lock_query_select() {
        let map = create_test_map();
        let names = map
            .lock_query()
            .select(Product::name_r());
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn test_lock_query_sum() {
        let map = create_test_map();
        let total = map
            .lock_query()
            .sum(Product::price_r());
        assert!((total - 1329.97).abs() < 0.01);
    }

    #[test]
    fn test_lock_query_group_by() {
        let map = create_test_map();
        let groups = map
            .lock_query()
            .group_by(Product::category_r());
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get("Electronics").unwrap().len(), 2);
    }

    #[test]
    fn test_lock_query_order_by() {
        let map = create_test_map();
        let sorted = map
            .lock_query()
            .order_by_float(Product::price_r());
        assert_eq!(sorted[0].price, 29.99);
        assert_eq!(sorted[2].price, 999.99);
    }
}

