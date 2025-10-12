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
//! use key_paths_derive::Keypaths;
//!
//! #[derive(Keypaths)]
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
//! let query = products.query().where_(Product::price_r(), |&p| p > 100.0);
//! let expensive = query.all();
//! ```

pub mod query;
pub mod join;
pub mod lazy;
pub mod queryable;
pub mod ext;
pub mod datetime;
pub mod locks;

#[macro_use]
pub mod macros;

pub use query::{Query, QueryWithSkip};
pub use join::JoinQuery;
pub use lazy::LazyQuery;
pub use queryable::Queryable;
pub use ext::QueryExt;
pub use locks::{LockValue, LockQueryExt, LockIterExt, LockedValueRef};

// Re-export key-paths for convenience
pub use key_paths_core::KeyPaths;
