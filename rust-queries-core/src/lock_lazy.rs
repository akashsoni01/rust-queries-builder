//! Lazy query support for locked data structures.
//!
//! Provides lazy evaluation with early termination for locked collections.

use crate::locks::LockValue;
use key_paths_core::KeyPaths;
use std::marker::PhantomData;

/// Lazy query for locked data with early termination.
pub struct LockLazyQuery<'a, T: 'static, L, I>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = &'a L>,
{
    iter: I,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'static, L, I> LockLazyQuery<'a, T, L, I>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = &'a L> + 'a,
{
    /// Create a new lazy query from an iterator of locks.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }

    /// Filter using a key-path predicate (lazy).
    pub fn where_<F, P>(self, path: KeyPaths<T, F>, predicate: P) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
    {
        LockLazyQuery {
            iter: self.iter.filter(move |lock| {
                lock.with_value(|item| {
                    path.get(item).map_or(false, |val| predicate(val))
                })
                .unwrap_or(false)
            }),
            _phantom: PhantomData,
        }
    }

    /// Map to a field value (lazy).
    /// 
    /// This allows you to select only specific fields from locked data without
    /// cloning the entire object. Perfect for projecting data efficiently.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Select only product names (not full objects)
    /// let names: Vec<String> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::price_r(), |&p| p > 100.0)
    ///     .select_lazy(Product::name_r())
    ///     .collect();
    /// 
    /// // Select only IDs  
    /// let ids: Vec<u32> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .select_lazy(Product::id_r())
    ///     .take(10)
    ///     .collect();
    /// 
    /// // Select prices and compute sum
    /// let total: f64 = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .select_lazy(Product::price_r())
    ///     .sum();
    /// ```
    /// 
    /// **Performance Note**: This is much more efficient than collecting full objects
    /// and then extracting fields, as it only clones the specific field value.
    pub fn select_lazy<F>(self, path: KeyPaths<T, F>) -> impl Iterator<Item = F> + 'a
    where
        F: Clone + 'static,
    {
        self.iter.filter_map(move |lock| {
            lock.with_value(|item| path.get(item).cloned()).flatten()
        })
    }

    /// Take first N items (lazy).
    pub fn take_lazy(self, n: usize) -> impl Iterator<Item = T> + 'a
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .take(n)
    }

    /// Skip first N items (lazy).
    pub fn skip_lazy(self, n: usize) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        LockLazyQuery {
            iter: self.iter.skip(n),
            _phantom: PhantomData,
        }
    }

    /// Count matching items (terminal).
    pub fn count(self) -> usize {
        self.iter.count()
    }

    /// Get first matching item (terminal).
    pub fn first(mut self) -> Option<T>
    where
        T: Clone,
    {
        self.iter
            .find_map(|lock| lock.with_value(|item| item.clone()))
    }

    /// Check if any items match (terminal).
    pub fn any(mut self) -> bool {
        self.iter.next().is_some()
    }

    /// Collect into Vec (terminal).
    pub fn collect(self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect()
    }

    /// Get all matching items (alias for collect, similar to LockQuery::all).
    /// 
    /// This provides a familiar API for users coming from LockQuery.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let all_items: Vec<Product> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::price_r(), |&p| p > 100.0)
    ///     .all();
    /// ```
    pub fn all(self) -> Vec<T>
    where
        T: Clone,
    {
        self.collect()
    }
}

