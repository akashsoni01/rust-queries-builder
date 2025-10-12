//! View-like functionality for locked data.
//!
//! This module provides saved query patterns (views) that can be reused,
//! similar to SQL VIEWs.
//!
//! # Example
//!
//! ```ignore
//! use rust_queries_core::lock_view::LockView;
//!
//! // Define a reusable view
//! let active_electronics = LockView::new(|map: &ProductMap| {
//!     map.lock_query()
//!         .where_(Product::active_r(), |&a| a)
//!         .where_(Product::category_r(), |cat| cat == "Electronics")
//! });
//!
//! // Use the view multiple times
//! let count = active_electronics.query(&products).count();
//! let items = active_electronics.query(&products).all();
//! ```

use crate::lock_query::LockQuery;
use crate::locks::LockValue;
use std::marker::PhantomData;

/// A reusable query pattern (like a SQL VIEW).
///
/// Views encapsulate query logic that can be reused across multiple queries.
pub struct LockView<'a, T: 'static, L, F>
where
    L: LockValue<T> + 'a,
    F: Fn(&LockQuery<'a, T, L>) -> LockQuery<'a, T, L>,
{
    builder: F,
    _phantom: PhantomData<(&'a T, L)>,
}

impl<'a, T: 'static, L, F> LockView<'a, T, L, F>
where
    L: LockValue<T> + 'a,
    F: Fn(&LockQuery<'a, T, L>) -> LockQuery<'a, T, L>,
{
    /// Create a new view with a query builder function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let expensive_view = LockView::new(|query| {
    ///     query.where_(Product::price_r(), |&p| p > 500.0)
    /// });
    /// ```
    pub fn new(builder: F) -> Self {
        Self {
            builder,
            _phantom: PhantomData,
        }
    }

    /// Apply the view to a base query.
    pub fn apply(&self, base: &LockQuery<'a, T, L>) -> LockQuery<'a, T, L>
    where
        T: Clone,
        L: Clone,
    {
        (self.builder)(base)
    }
}

/// Materialized view - a cached query result.
///
/// Like SQL materialized views, stores query results for fast access.
pub struct MaterializedLockView<T>
where
    T: Clone,
{
    data: Vec<T>,
    refresh_fn: Box<dyn Fn() -> Vec<T>>,
}

impl<T> MaterializedLockView<T>
where
    T: Clone,
{
    /// Create a new materialized view with a refresh function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mat_view = MaterializedLockView::new(|| {
    ///     product_map
    ///         .lock_query()
    ///         .where_(Product::active_r(), |&a| a)
    ///         .all()
    /// });
    /// ```
    pub fn new<F>(refresh_fn: F) -> Self
    where
        F: Fn() -> Vec<T> + 'static,
    {
        let data = refresh_fn();
        Self {
            data,
            refresh_fn: Box::new(refresh_fn),
        }
    }

    /// Get the cached data.
    pub fn get(&self) -> &[T] {
        &self.data
    }

    /// Refresh the view with latest data.
    pub fn refresh(&mut self) {
        self.data = (self.refresh_fn)();
    }

    /// Get count without refreshing.
    pub fn count(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;
    use key_paths_derive::Keypaths;

    #[derive(Clone, Keypaths)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
        active: bool,
    }

    #[test]
    fn test_materialized_view() {
        let mut map = HashMap::new();
        map.insert("p1".to_string(), Arc::new(RwLock::new(Product {
            id: 1,
            name: "A".to_string(),
            price: 100.0,
            active: true,
        })));

        let mat_view = MaterializedLockView::new(|| {
            vec![Product {
                id: 1,
                name: "A".to_string(),
                price: 100.0,
                active: true,
            }]
        });

        assert_eq!(mat_view.count(), 1);
        assert_eq!(mat_view.get()[0].name, "A");
    }
}

