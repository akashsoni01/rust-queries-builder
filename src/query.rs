//! Query builder implementation for filtering, selecting, ordering, grouping, and aggregating data.
//!
//! This module provides the `Query` struct which enables SQL-like operations on collections
//! using type-safe key-paths.

use key_paths_core::KeyPaths;
use std::collections::HashMap;

/// A query builder for filtering, selecting, ordering, grouping, and aggregating data.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the data being queried
/// * `T` - The type of items in the collection
///
/// # Example
///
/// ```ignore
/// let products = vec![/* ... */];
/// let query = Query::new(&products)
///     .where_(Product::price_r(), |&price| price < 100.0)
///     .order_by_float(Product::price_r());
/// ```
pub struct Query<'a, T: 'static> {
    data: &'a [T],
    filters: Vec<Box<dyn Fn(&T) -> bool>>,
}

impl<'a, T: 'static + Clone> Query<'a, T> {
    /// Creates a new query from a slice of data.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of items to query
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = Query::new(&products);
    /// ```
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            filters: Vec::new(),
        }
    }

    /// Adds a filter predicate using a key-path.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to filter on
    /// * `predicate` - A function that returns true for items to keep
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = Query::new(&products)
    ///     .where_(Product::category_r(), |cat| cat == "Electronics");
    /// ```
    pub fn where_<F>(mut self, path: KeyPaths<T, F>, predicate: impl Fn(&F) -> bool + 'static) -> Self
    where
        F: 'static,
    {
        self.filters.push(Box::new(move |item| {
            path.get(item).map_or(false, |val| predicate(val))
        }));
        self
    }

    /// Returns all items matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = query.all();
    /// ```
    pub fn all(&self) -> Vec<&T> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect()
    }

    /// Returns the first item matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = query.first();
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.data
            .iter()
            .find(|item| self.filters.iter().all(|f| f(item)))
    }

    /// Returns the count of items matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count();
    /// ```
    pub fn count(&self) -> usize {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .count()
    }

    /// Returns the first `n` items matching the query filters.
    ///
    /// # Arguments
    ///
    /// * `n` - The maximum number of items to return
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first_10 = query.limit(10);
    /// ```
    pub fn limit(&self, n: usize) -> Vec<&T> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .take(n)
            .collect()
    }

    /// Skips the first `offset` items for pagination.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of items to skip
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2 = query.skip(20).limit(10);
    /// ```
    pub fn skip<'b>(&'b self, offset: usize) -> QueryWithSkip<'a, 'b, T> {
        QueryWithSkip {
            query: self,
            offset,
        }
    }

    /// Orders results by a field in ascending order.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by(Product::name_r());
    /// ```
    pub fn order_by<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
    {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by_key(|item| path.get(item).cloned());
        results
    }

    /// Orders results by a field in descending order.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_desc(Product::stock_r());
    /// ```
    pub fn order_by_desc<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
    {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned();
            let b_val = path.get(b).cloned();
            b_val.cmp(&a_val)
        });
        results
    }

    /// Orders results by a float field in ascending order.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_float(Product::price_r());
    /// ```
    pub fn order_by_float(&self, path: KeyPaths<T, f64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Orders results by a float field in descending order.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_float_desc(Product::rating_r());
    /// ```
    pub fn order_by_float_desc(&self, path: KeyPaths<T, f64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Projects/selects a single field from results.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to select
    ///
    /// # Example
    ///
    /// ```ignore
    /// let names = query.select(Product::name_r());
    /// ```
    pub fn select<F>(&self, path: KeyPaths<T, F>) -> Vec<F>
    where
        F: Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect()
    }

    /// Groups results by a field value.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to group by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let by_category = query.group_by(Product::category_r());
    /// ```
    pub fn group_by<F>(&self, path: KeyPaths<T, F>) -> HashMap<F, Vec<T>>
    where
        F: Eq + std::hash::Hash + Clone + 'static,
    {
        let mut groups: HashMap<F, Vec<T>> = HashMap::new();

        for item in self.data.iter() {
            if self.filters.iter().all(|f| f(item)) {
                if let Some(key) = path.get(item).cloned() {
                    groups.entry(key).or_insert_with(Vec::new).push(item.clone());
                }
            }
        }

        groups
    }

    /// Computes the sum of a numeric field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the numeric field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total_price = query.sum(Product::price_r());
    /// ```
    pub fn sum<F>(&self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .fold(F::default(), |acc, val| acc + val)
    }

    /// Computes the average of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg_price = query.avg(Product::price_r()).unwrap_or(0.0);
    /// ```
    pub fn avg(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        let items: Vec<f64> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.iter().sum::<f64>() / items.len() as f64)
        }
    }

    /// Finds the minimum value of a field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min_stock = query.min(Product::stock_r());
    /// ```
    pub fn min<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Finds the maximum value of a field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max_stock = query.max(Product::stock_r());
    /// ```
    pub fn max<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Finds the minimum value of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min_price = query.min_float(Product::price_r());
    /// ```
    pub fn min_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Finds the maximum value of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max_price = query.max_float(Product::price_r());
    /// ```
    pub fn max_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Checks if any items match the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let has_results = query.exists();
    /// ```
    pub fn exists(&self) -> bool {
        self.data
            .iter()
            .any(|item| self.filters.iter().all(|f| f(item)))
    }
}

/// Helper struct for pagination after a skip operation.
///
/// Created by calling `skip()` on a `Query`.
pub struct QueryWithSkip<'a, 'b, T: 'static> {
    query: &'b Query<'a, T>,
    offset: usize,
}

impl<'a, 'b, T: 'static> QueryWithSkip<'a, 'b, T> {
    /// Returns up to `n` items after skipping the offset.
    ///
    /// # Arguments
    ///
    /// * `n` - The maximum number of items to return
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2 = query.skip(20).limit(10);
    /// ```
    pub fn limit(&self, n: usize) -> Vec<&'a T> {
        self.query
            .data
            .iter()
            .filter(|item| self.query.filters.iter().all(|f| f(item)))
            .skip(self.offset)
            .take(n)
            .collect()
    }
}

