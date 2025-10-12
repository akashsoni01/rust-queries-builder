//! Extended lock support for parking_lot and tokio.
//!
//! This module provides wrappers and extension traits for third-party lock types,
//! enabling them to work with the lock-aware query system.
//!
//! ## Features
//!
//! - **parking_lot Support**: High-performance RwLock and Mutex wrappers
//! - **tokio Support**: Async RwLock support for async applications
//! - **Extension Traits**: Direct `.lock_query()` and `.lock_join()` support
//!
//! ## Example (parking_lot)
//!
//! ```ignore
//! use rust_queries_core::lock_ext::{ParkingLotRwLockWrapper, ParkingLotQueryExt};
//! use std::collections::HashMap;
//! use parking_lot::RwLock;
//!
//! let mut products: HashMap<String, ParkingLotRwLockWrapper<Product>> = HashMap::new();
//! products.insert("p1".to_string(), ParkingLotRwLockWrapper::new(Product {
//!     id: 1,
//!     price: 999.99,
//! }));
//!
//! // Direct method call!
//! let expensive = products
//!     .lock_query()
//!     .where_(Product::price_r(), |&p| p > 500.0)
//!     .all();
//! ```
//!
//! ## Example (tokio)
//!
//! ```ignore
//! use rust_queries_core::lock_ext::{TokioRwLockWrapper, TokioLockQueryExt};
//! use std::collections::HashMap;
//!
//! async fn query_products(products: &HashMap<String, TokioRwLockWrapper<Product>>) {
//!     let expensive = products
//!         .lock_query()  // Direct method call!
//!         .where_(Product::price_r(), |&p| p > 500.0)
//!         .all();
//! }
//! ```

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use crate::locks::LockValue;

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use crate::lock_query::LockQuery;

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use crate::lock_lazy::LockLazyQuery;

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use crate::lock_join::LockJoinQuery;

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use std::collections::HashMap;

#[cfg(any(feature = "parking_lot", feature = "tokio"))]
use std::sync::Arc;

// ============================================================================
// parking_lot Support
// ============================================================================

/// Wrapper around Arc<parking_lot::RwLock<T>>.
///
/// This newtype is needed because of Rust's orphan rules - we can't implement
/// foreign traits (LockValue) on foreign types.
///
/// # Example
///
/// ```ignore
/// use rust_queries_core::lock_ext::ParkingLotRwLockWrapper;
/// use parking_lot::RwLock;
///
/// let wrapper = ParkingLotRwLockWrapper::new(Product {
///     id: 1,
///     name: "Laptop".to_string(),
///     price: 999.99,
/// });
/// ```
#[cfg(feature = "parking_lot")]
#[derive(Clone, Debug)]
pub struct ParkingLotRwLockWrapper<T>(Arc<parking_lot::RwLock<T>>);

#[cfg(feature = "parking_lot")]
impl<T> ParkingLotRwLockWrapper<T> {
    /// Create a new ParkingLotRwLockWrapper.
    pub fn new(value: T) -> Self {
        Self(Arc::new(parking_lot::RwLock::new(value)))
    }

    /// Get a reference to the inner Arc<RwLock<T>>.
    pub fn inner(&self) -> &Arc<parking_lot::RwLock<T>> {
        &self.0
    }
}

#[cfg(feature = "parking_lot")]
impl<T> LockValue<T> for ParkingLotRwLockWrapper<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // parking_lot RwLock is synchronous and doesn't panic on poisoning
        let guard = self.0.read();
        Some(f(&*guard))
    }
}

/// Wrapper around Arc<parking_lot::Mutex<T>>.
#[cfg(feature = "parking_lot")]
#[derive(Clone, Debug)]
pub struct ParkingLotMutexWrapper<T>(Arc<parking_lot::Mutex<T>>);

#[cfg(feature = "parking_lot")]
impl<T> ParkingLotMutexWrapper<T> {
    /// Create a new ParkingLotMutexWrapper.
    pub fn new(value: T) -> Self {
        Self(Arc::new(parking_lot::Mutex::new(value)))
    }

    /// Get a reference to the inner Arc<Mutex<T>>.
    pub fn inner(&self) -> &Arc<parking_lot::Mutex<T>> {
        &self.0
    }
}

#[cfg(feature = "parking_lot")]
impl<T> LockValue<T> for ParkingLotMutexWrapper<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // parking_lot Mutex is synchronous and doesn't panic on poisoning
        let guard = self.0.lock();
        Some(f(&*guard))
    }
}

// Extension traits for parking_lot

/// Extension trait to enable direct .lock_query() and .lock_lazy_query() calls
/// on HashMap with parking_lot RwLock.
#[cfg(feature = "parking_lot")]
pub trait ParkingLotQueryExt<V> {
    /// Create a LockQuery for SQL-like operations.
    fn lock_query(&self) -> LockQuery<'_, V, ParkingLotRwLockWrapper<V>>;
    
    /// Create a lazy lock query.
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, ParkingLotRwLockWrapper<V>, impl Iterator<Item = &ParkingLotRwLockWrapper<V>>>;
}

#[cfg(feature = "parking_lot")]
impl<K, V: 'static> ParkingLotQueryExt<V> for HashMap<K, ParkingLotRwLockWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_query(&self) -> LockQuery<'_, V, ParkingLotRwLockWrapper<V>> {
        let locks: Vec<_> = self.values().collect();
        LockQuery::from_locks(locks)
    }
    
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, ParkingLotRwLockWrapper<V>, impl Iterator<Item = &ParkingLotRwLockWrapper<V>>> {
        LockLazyQuery::new(self.values())
    }
}

/// Extension trait for Mutex queries.
#[cfg(feature = "parking_lot")]
pub trait ParkingLotMutexQueryExt<V> {
    /// Create a LockQuery for SQL-like operations.
    fn lock_query(&self) -> LockQuery<'_, V, ParkingLotMutexWrapper<V>>;
    
    /// Create a lazy lock query.
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, ParkingLotMutexWrapper<V>, impl Iterator<Item = &ParkingLotMutexWrapper<V>>>;
}

#[cfg(feature = "parking_lot")]
impl<K, V: 'static> ParkingLotMutexQueryExt<V> for HashMap<K, ParkingLotMutexWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_query(&self) -> LockQuery<'_, V, ParkingLotMutexWrapper<V>> {
        let locks: Vec<_> = self.values().collect();
        LockQuery::from_locks(locks)
    }
    
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, ParkingLotMutexWrapper<V>, impl Iterator<Item = &ParkingLotMutexWrapper<V>>> {
        LockLazyQuery::new(self.values())
    }
}

// Extension trait for JOIN operations

/// Extension trait for JOIN operations with parking_lot RwLock.
#[cfg(feature = "parking_lot")]
pub trait ParkingLotJoinExt<V> {
    /// Create a join query with another collection.
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, ParkingLotRwLockWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, ParkingLotRwLockWrapper<V>, ParkingLotRwLockWrapper<R>>
    where
        R: 'static;
}

#[cfg(feature = "parking_lot")]
impl<K, V: 'static> ParkingLotJoinExt<V> for HashMap<K, ParkingLotRwLockWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, ParkingLotRwLockWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, ParkingLotRwLockWrapper<V>, ParkingLotRwLockWrapper<R>>
    where
        R: 'static,
    {
        let left_locks: Vec<_> = self.values().collect();
        let right_locks: Vec<_> = right.values().collect();
        LockJoinQuery::new(left_locks, right_locks)
    }
}

/// Extension trait for JOIN operations with parking_lot Mutex.
#[cfg(feature = "parking_lot")]
pub trait ParkingLotMutexJoinExt<V> {
    /// Create a join query with another collection.
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, ParkingLotMutexWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, ParkingLotMutexWrapper<V>, ParkingLotMutexWrapper<R>>
    where
        R: 'static;
}

#[cfg(feature = "parking_lot")]
impl<K, V: 'static> ParkingLotMutexJoinExt<V> for HashMap<K, ParkingLotMutexWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, ParkingLotMutexWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, ParkingLotMutexWrapper<V>, ParkingLotMutexWrapper<R>>
    where
        R: 'static,
    {
        let left_locks: Vec<_> = self.values().collect();
        let right_locks: Vec<_> = right.values().collect();
        LockJoinQuery::new(left_locks, right_locks)
    }
}

// ============================================================================
// tokio Support
// ============================================================================

/// Wrapper around Arc<tokio::sync::RwLock<T>>.
///
/// This newtype enables tokio RwLock to work with the lock-aware query system.
///
/// # Example
///
/// ```ignore
/// use rust_queries_core::lock_ext::TokioRwLockWrapper;
///
/// let wrapper = TokioRwLockWrapper::new(Product {
///     id: 1,
///     name: "Laptop".to_string(),
///     price: 999.99,
/// });
/// ```
#[cfg(feature = "tokio")]
#[derive(Clone, Debug)]
pub struct TokioRwLockWrapper<T>(Arc<tokio::sync::RwLock<T>>);

#[cfg(feature = "tokio")]
impl<T> TokioRwLockWrapper<T> {
    /// Create a new TokioRwLockWrapper.
    pub fn new(value: T) -> Self {
        Self(Arc::new(tokio::sync::RwLock::new(value)))
    }

    /// Get a reference to the inner Arc<RwLock<T>>.
    pub fn inner(&self) -> &Arc<tokio::sync::RwLock<T>> {
        &self.0
    }
}

#[cfg(feature = "tokio")]
impl<T> LockValue<T> for TokioRwLockWrapper<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // Note: This blocks! For fully async code, consider using async queries.
        // We use blocking here because LockValue trait is synchronous.
        let guard = self.0.blocking_read();
        Some(f(&*guard))
    }
}

/// Wrapper around Arc<tokio::sync::Mutex<T>>.
#[cfg(feature = "tokio")]
#[derive(Clone, Debug)]
pub struct TokioMutexWrapper<T>(Arc<tokio::sync::Mutex<T>>);

#[cfg(feature = "tokio")]
impl<T> TokioMutexWrapper<T> {
    /// Create a new TokioMutexWrapper.
    pub fn new(value: T) -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(value)))
    }

    /// Get a reference to the inner Arc<Mutex<T>>.
    pub fn inner(&self) -> &Arc<tokio::sync::Mutex<T>> {
        &self.0
    }
}

#[cfg(feature = "tokio")]
impl<T> LockValue<T> for TokioMutexWrapper<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // Note: This blocks! For fully async code, consider using async queries.
        let guard = self.0.blocking_lock();
        Some(f(&*guard))
    }
}

// Extension traits for tokio

/// Extension trait to enable direct .lock_query() and .lock_lazy_query() calls
/// on HashMap with tokio RwLock.
#[cfg(feature = "tokio")]
pub trait TokioLockQueryExt<V> {
    /// Create a LockQuery for SQL-like operations.
    fn lock_query(&self) -> LockQuery<'_, V, TokioRwLockWrapper<V>>;
    
    /// Create a lazy lock query.
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, TokioRwLockWrapper<V>, impl Iterator<Item = &TokioRwLockWrapper<V>>>;
}

#[cfg(feature = "tokio")]
impl<K, V: 'static> TokioLockQueryExt<V> for HashMap<K, TokioRwLockWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_query(&self) -> LockQuery<'_, V, TokioRwLockWrapper<V>> {
        let locks: Vec<_> = self.values().collect();
        LockQuery::from_locks(locks)
    }
    
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, TokioRwLockWrapper<V>, impl Iterator<Item = &TokioRwLockWrapper<V>>> {
        LockLazyQuery::new(self.values())
    }
}

/// Extension trait for Mutex queries.
#[cfg(feature = "tokio")]
pub trait TokioMutexQueryExt<V> {
    /// Create a LockQuery for SQL-like operations.
    fn lock_query(&self) -> LockQuery<'_, V, TokioMutexWrapper<V>>;
    
    /// Create a lazy lock query.
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, TokioMutexWrapper<V>, impl Iterator<Item = &TokioMutexWrapper<V>>>;
}

#[cfg(feature = "tokio")]
impl<K, V: 'static> TokioMutexQueryExt<V> for HashMap<K, TokioMutexWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_query(&self) -> LockQuery<'_, V, TokioMutexWrapper<V>> {
        let locks: Vec<_> = self.values().collect();
        LockQuery::from_locks(locks)
    }
    
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, V, TokioMutexWrapper<V>, impl Iterator<Item = &TokioMutexWrapper<V>>> {
        LockLazyQuery::new(self.values())
    }
}

// Extension trait for JOIN operations

/// Extension trait for JOIN operations with tokio RwLock.
#[cfg(feature = "tokio")]
pub trait TokioLockJoinExt<V> {
    /// Create a join query with another collection.
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, TokioRwLockWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, TokioRwLockWrapper<V>, TokioRwLockWrapper<R>>
    where
        R: 'static;
}

#[cfg(feature = "tokio")]
impl<K, V: 'static> TokioLockJoinExt<V> for HashMap<K, TokioRwLockWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, TokioRwLockWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, TokioRwLockWrapper<V>, TokioRwLockWrapper<R>>
    where
        R: 'static,
    {
        let left_locks: Vec<_> = self.values().collect();
        let right_locks: Vec<_> = right.values().collect();
        LockJoinQuery::new(left_locks, right_locks)
    }
}

/// Extension trait for JOIN operations with tokio Mutex.
#[cfg(feature = "tokio")]
pub trait TokioMutexJoinExt<V> {
    /// Create a join query with another collection.
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, TokioMutexWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, TokioMutexWrapper<V>, TokioMutexWrapper<R>>
    where
        R: 'static;
}

#[cfg(feature = "tokio")]
impl<K, V: 'static> TokioMutexJoinExt<V> for HashMap<K, TokioMutexWrapper<V>>
where
    K: std::hash::Hash + Eq,
{
    fn lock_join<'a, R>(&'a self, right: &'a HashMap<impl std::hash::Hash + Eq, TokioMutexWrapper<R>>) 
        -> LockJoinQuery<'a, V, R, TokioMutexWrapper<V>, TokioMutexWrapper<R>>
    where
        R: 'static,
    {
        let left_locks: Vec<_> = self.values().collect();
        let right_locks: Vec<_> = right.values().collect();
        LockJoinQuery::new(left_locks, right_locks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(feature = "parking_lot")]
    #[test]
    fn test_parking_lot_wrapper() {
        let wrapper = ParkingLotRwLockWrapper::new(42);
        let result = wrapper.with_value(|v| *v * 2);
        assert_eq!(result, Some(84));
    }
    
    #[cfg(feature = "tokio")]
    #[test]
    fn test_tokio_wrapper() {
        let wrapper = TokioRwLockWrapper::new(42);
        let result = wrapper.with_value(|v| *v * 2);
        assert_eq!(result, Some(84));
    }
}

