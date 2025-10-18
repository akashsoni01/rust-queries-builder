//! Lock-aware querying for thread-safe data structures.
//!
//! This module provides support for querying data behind locks (RwLock, Mutex)
//! without copying. It works with both std and tokio locks (when enabled).
//!
//! # Features
//!
//! - Query `Arc<RwLock<T>>` and `Arc<Mutex<T>>` directly
//! - No data copying - works with lock guards
//! - Extensible for tokio::sync locks
//! - Works with HashMap, Vec, and other collections of locks
//!
//! # Example
//!
//! ```ignore
//! use std::sync::{Arc, RwLock};
//! use std::collections::HashMap;
//! use rust_queries_core::{locks::LockQueryExt, LazyQuery};
//!
//! type ProductMap = HashMap<String, Arc<RwLock<Product>>>;
//!
//! let products: ProductMap = /* ... */;
//!
//! // Query without copying!
//! let electronics: Vec<_> = products
//!     .query_locks()
//!     .where_(Product::category(), |cat| cat == "Electronics")
//!     .collect();
//! ```

use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;

/// Helper trait for lock-aware value extraction.
///
/// This trait enables querying locked data without cloning.
pub trait LockValue<T> {
    /// Execute a function with access to the inner value.
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R;
}

// Implementation for Arc<RwLock<T>>
impl<T> LockValue<T> for Arc<RwLock<T>> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.read().ok().map(|guard| f(&*guard))
    }
}

// Implementation for Arc<Mutex<T>>
impl<T> LockValue<T> for Arc<Mutex<T>> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.lock().ok().map(|guard| f(&*guard))
    }
}

// Implementation for RwLock<T> (non-Arc)
impl<T> LockValue<T> for RwLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.read().ok().map(|guard| f(&*guard))
    }
}

// Implementation for Mutex<T> (non-Arc)
impl<T> LockValue<T> for Mutex<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.lock().ok().map(|guard| f(&*guard))
    }
}


/// Extension trait for querying collections of locks.
///
/// Provides convenient methods for querying HashMaps and Vecs
/// containing locked values without data copying.
pub trait LockQueryExt<T, L>
where
    L: LockValue<T>,
{
    /// Create an iterator over locked values for querying.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let map: HashMap<String, Arc<RwLock<Product>>> = /* ... */;
    /// let iter = map.lock_iter();
    /// ```
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, T, L>> + '_>;
}

/// A reference to a value behind a lock.
///
/// This struct allows querying locked data without copying.
/// The lock is acquired when needed and released when dropped.
pub struct LockedValueRef<'a, T, L>
where
    L: LockValue<T>,
{
    lock: &'a L,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T, L> LockedValueRef<'a, T, L>
where
    L: LockValue<T>,
{
    pub fn new(lock: &'a L) -> Self {
        Self {
            lock,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Execute a function with access to the locked value.
    pub fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.lock.with_value(f)
    }

    /// Try to get a value from the lock by applying a function.
    pub fn map<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.lock.with_value(f)
    }

    /// Check if the locked value matches a predicate.
    pub fn matches<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&T) -> bool,
    {
        self.lock.with_value(predicate).unwrap_or(false)
    }

}

// Implementation for HashMap<K, Arc<RwLock<V>>>
impl<K, V> LockQueryExt<V, Arc<RwLock<V>>> for HashMap<K, Arc<RwLock<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, V, Arc<RwLock<V>>>> + '_> {
        Box::new(self.values().map(|lock| LockedValueRef::new(lock)))
    }
}

// Implementation for HashMap<K, Arc<Mutex<V>>>
impl<K, V> LockQueryExt<V, Arc<Mutex<V>>> for HashMap<K, Arc<Mutex<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, V, Arc<Mutex<V>>>> + '_> {
        Box::new(self.values().map(|lock| LockedValueRef::new(lock)))
    }
}

// Implementation for Vec<Arc<RwLock<T>>>
impl<T> LockQueryExt<T, Arc<RwLock<T>>> for Vec<Arc<RwLock<T>>> {
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, T, Arc<RwLock<T>>>> + '_> {
        Box::new(self.iter().map(|lock| LockedValueRef::new(lock)))
    }
}

// Implementation for Vec<Arc<Mutex<T>>>
impl<T> LockQueryExt<T, Arc<Mutex<T>>> for Vec<Arc<Mutex<T>>> {
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, T, Arc<Mutex<T>>>> + '_> {
        Box::new(self.iter().map(|lock| LockedValueRef::new(lock)))
    }
}

// Implementation for slice of Arc<RwLock<T>>
impl<T> LockQueryExt<T, Arc<RwLock<T>>> for [Arc<RwLock<T>>] {
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, T, Arc<RwLock<T>>>> + '_> {
        Box::new(self.iter().map(|lock| LockedValueRef::new(lock)))
    }
}

// Implementation for slice of Arc<Mutex<T>>
impl<T> LockQueryExt<T, Arc<Mutex<T>>> for [Arc<Mutex<T>>] {
    fn lock_iter(&self) -> Box<dyn Iterator<Item = LockedValueRef<'_, T, Arc<Mutex<T>>>> + '_> {
        Box::new(self.iter().map(|lock| LockedValueRef::new(lock)))
    }
}

/// Iterator adapter for filtering locked values.
pub struct LockFilterIter<'a, T, L, I, F>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = LockedValueRef<'a, T, L>>,
    F: Fn(&T) -> bool,
{
    iter: I,
    predicate: F,
    _phantom: std::marker::PhantomData<(&'a T, L)>,
}

impl<'a, T, L, I, F> Iterator for LockFilterIter<'a, T, L, I, F>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = LockedValueRef<'a, T, L>>,
    F: Fn(&T) -> bool,
{
    type Item = LockedValueRef<'a, T, L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|locked_ref| locked_ref.matches(&self.predicate))
    }
}

/// Extension methods for lock iterators.
pub trait LockIterExt<'a, T: 'a, L>: Iterator<Item = LockedValueRef<'a, T, L>> + Sized
where
    L: LockValue<T> + 'a,
{
    /// Filter locked values by a predicate.
    fn filter_locked<F>(self, predicate: F) -> LockFilterIter<'a, T, L, Self, F>
    where
        F: Fn(&T) -> bool,
    {
        LockFilterIter {
            iter: self,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Map locked values to a new type.
    fn map_locked<F, R>(self, f: F) -> impl Iterator<Item = R> + 'a
    where
        F: Fn(&T) -> R + 'a,
        Self: 'a,
    {
        self.filter_map(move |locked_ref| locked_ref.map(&f))
    }

    /// Count locked values matching a predicate.
    fn count_locked<F>(self, predicate: F) -> usize
    where
        F: Fn(&T) -> bool,
        Self: 'a,
    {
        self.filter(|locked_ref| locked_ref.matches(&predicate))
            .count()
    }

    /// Find first locked value matching a predicate.
    fn find_locked<F>(mut self, predicate: F) -> Option<LockedValueRef<'a, T, L>>
    where
        F: Fn(&T) -> bool,
    {
        self.find(|locked_ref| locked_ref.matches(&predicate))
    }

    /// Check if any locked value matches a predicate.
    fn any_locked<F>(mut self, predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        self.any(|locked_ref| locked_ref.matches(&predicate))
    }

    /// Collect locked values into a Vec by cloning (when needed).
    fn collect_cloned(self) -> Vec<T>
    where
        T: Clone,
        Self: 'a,
    {
        self.filter_map(|locked_ref| {
            locked_ref.with_value(|v| v.clone())
        })
        .collect()
    }

}

// Blanket implementation for all iterators over LockedValueRef
impl<'a, T: 'a, L, I> LockIterExt<'a, T, L> for I
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = LockedValueRef<'a, T, L>>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};

    #[test]
    fn test_rwlock_lock_value() {
        let data = Arc::new(RwLock::new(42));
        let result = data.with_value(|v| *v * 2);
        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_mutex_lock_value() {
        let data = Arc::new(Mutex::new("hello"));
        let result = data.with_value(|v| v.len());
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_hashmap_lock_query() {
        let mut map: HashMap<String, Arc<RwLock<i32>>> = HashMap::new();
        map.insert("a".to_string(), Arc::new(RwLock::new(10)));
        map.insert("b".to_string(), Arc::new(RwLock::new(20)));
        map.insert("c".to_string(), Arc::new(RwLock::new(30)));

        let sum: i32 = map
            .lock_iter()
            .map_locked(|v| *v)
            .sum();

        assert_eq!(sum, 60);
    }

    #[test]
    fn test_lock_filter() {
        let mut map: HashMap<String, Arc<RwLock<i32>>> = HashMap::new();
        map.insert("a".to_string(), Arc::new(RwLock::new(10)));
        map.insert("b".to_string(), Arc::new(RwLock::new(20)));
        map.insert("c".to_string(), Arc::new(RwLock::new(30)));

        let count = map
            .lock_iter()
            .count_locked(|v| *v > 15);

        assert_eq!(count, 2);
    }

    #[test]
    fn test_lock_any() {
        let mut map: HashMap<String, Arc<RwLock<i32>>> = HashMap::new();
        map.insert("a".to_string(), Arc::new(RwLock::new(10)));
        map.insert("b".to_string(), Arc::new(RwLock::new(20)));

        let has_large = map
            .lock_iter()
            .any_locked(|v| *v > 15);

        assert!(has_large);
    }
}

