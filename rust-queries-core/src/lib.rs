//! # Rust Query Builder Core
//!
//! Core functionality for rust-queries-builder - a powerful, type-safe query builder 
//! library for Rust that leverages key-paths for SQL-like operations on in-memory collections.
//!
//! This crate contains the core query building logic, without proc-macros or derive functionality.
//!
//! ## Features
//!
//! - Type-safe queries with compile-time checking
//! - SQL-like operations: WHERE, SELECT, ORDER BY, GROUP BY, JOIN
//! - Rich aggregations: COUNT, SUM, AVG, MIN, MAX
//! - Pagination: LIMIT and SKIP
//! - Join operations: INNER, LEFT, RIGHT, CROSS
//! - Zero-cost abstractions
//! - Clone-free operations
//! - Lazy evaluation with early termination
//! - Extension traits for ergonomic API
//! - Helper macros to reduce boilerplate
//!
//! ## Example
//!
//! ```rust
//! use rust_queries_core::{Query, QueryExt};
//! use key_paths_derive::Keypath;
//!
//! #[derive(Keypath)]
//! struct Product {
//!     id: u32,
//!     name: String,
//!     price: f64,
//! }
//!
//! let products = vec![
//!     Product { id: 1, name: "Laptop".to_string(), price: 999.99 },
//!     Product { id: 2, name: "Mouse".to_string(), price: 29.99 },
//! ];
//!
//! // Using extension trait
//! let query = products.query().where_(Product::price(), |&p| p > 100.0);
//! let expensive = query.all();
//! ```

pub mod query;
pub mod join;
pub mod lazy;
pub mod lazy_parallel;
pub mod queryable;
pub mod ext;
pub mod datetime;
pub mod locks;
pub mod lock_query;
pub mod lock_lazy;
pub mod lock_join;
pub mod lock_view;
pub mod lock_ext;

#[macro_use]
pub mod macros;

pub use query::{Query, QueryWithSkip};
pub use join::JoinQuery;
pub use lazy::LazyQuery;
pub use lazy_parallel::{LazyParallelQuery, LazyParallelQueryExt};
pub use queryable::Queryable;
pub use ext::{QueryExt, QueryableExt};
pub use locks::{LockValue, LockQueryExt, LockIterExt, LockedValueRef};
pub use lock_query::{LockQuery, LockQueryable, LockLazyQueryable};
pub use lock_lazy::LockLazyQuery;
pub use lock_join::{LockJoinQuery, LockJoinable, LockJoinableCollection};
pub use lock_view::{LockView, MaterializedLockView};

// Re-export lock extensions for parking_lot and tokio
#[cfg(feature = "parking_lot")]
pub use lock_ext::{
    ParkingLotRwLockWrapper, ParkingLotMutexWrapper,
    ParkingLotQueryExt, ParkingLotMutexQueryExt,
    ParkingLotJoinExt, ParkingLotMutexJoinExt,
};

#[cfg(feature = "tokio")]
pub use lock_ext::{
    TokioRwLockWrapper, TokioMutexWrapper,
    TokioLockQueryExt, TokioMutexQueryExt,
    TokioLockJoinExt, TokioMutexJoinExt,
};

// Re-export key-paths for convenience
pub use key_paths_core::KeyPaths;

