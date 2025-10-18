//! # Rust Query Builder
//!
//! A powerful, type-safe query builder library for Rust that leverages key-paths
//! for SQL-like operations on in-memory collections.
//!
//! ## Features
//!
//! - **Type-safe queries**: Compile-time type checking using key-paths
//! - **SQL-like operations**: WHERE, SELECT, ORDER BY, GROUP BY, JOIN
//! - **Aggregations**: COUNT, SUM, AVG, MIN, MAX
//! - **Pagination**: LIMIT and SKIP operations
//! - **Join operations**: INNER JOIN, LEFT JOIN with custom predicates
//! - **Zero-cost abstractions**: Leverages Rust's zero-cost abstractions
//!
//! ## Example
//!
//! ```rust
//! use rust_queries_builder::{Query, QueryExt};
//! use key_paths_derive::Keypath;
//!
//! #[derive(Clone, Keypath)]
//! struct Product {
//!     id: u32,
//!     name: String,
//!     price: f64,
//!     category: String,
//! }
//!
//! let products = vec![
//!     Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string() },
//!     Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string() },
//! ];
//!
//! // Using extension trait - most ergonomic
//! let query = products.query().where_(Product::category_r(), |cat| cat == "Electronics");
//! let electronics = query.all();
//!
//! // Traditional approach
//! let affordable = Query::new(&products)
//!     .where_(Product::price_r(), |&price| price < 100.0)
//!     .all();
//!
//! // Lazy evaluation for better performance
//! let total = products.lazy_query().sum_by(Product::price_r());
//! ```

// Re-export everything from core
pub use rust_queries_core::*;

// Re-export derive macros
pub use rust_queries_derive::{Queryable as QueryableDerive, QueryBuilder};

// Re-export keypath derive macro for convenience
pub use key_paths_derive::Keypath;
