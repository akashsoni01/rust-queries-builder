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
//! use rust_queries_builder::Query;
//! use key_paths_derive::Keypaths;
//!
//! #[derive(Clone, Keypaths)]
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
//! // Filter products by category and price
//! let affordable = Query::new(&products)
//!     .where_(Product::category_r(), |cat| cat == "Electronics")
//!     .where_(Product::price_r(), |&price| price < 100.0)
//!     .all();
//!
//! // Order by price
//! let sorted = Query::new(&products)
//!     .order_by_float(Product::price_r());
//!
//! // Aggregate
//! let total = Query::new(&products)
//!     .sum(Product::price_r());
//! ```

pub mod query;
pub mod join;
pub mod lazy;
pub mod queryable;
pub mod ext;

#[macro_use]
pub mod macros;

pub use query::{Query, QueryWithSkip};
pub use join::JoinQuery;
pub use lazy::LazyQuery;
pub use queryable::Queryable;
pub use ext::QueryExt;

// Re-export key-paths for convenience
pub use key_paths_core::KeyPaths;

// Re-export derive macros
pub use rust_queries_derive::{Queryable as QueryableDerive, QueryBuilder};
