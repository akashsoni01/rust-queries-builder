# Rust Query Builder

A powerful, type-safe query builder library for Rust that leverages **key-paths** for SQL-like operations on in-memory collections. This library brings the expressiveness of SQL to Rust's collections with compile-time type safety.

> 🎯 **v0.5.0 - Extension Trait & Derive Macros!** Call `.query()` and `.lazy_query()` directly on containers - [see extension guide](EXTENSION_TRAIT_GUIDE.md)

> ⚡ **v0.5.0 - Build Optimized!** Split into 3 crates - **65% faster builds**, **6KB umbrella crate** - [see build guide](BUILD_OPTIMIZATION.md)

> 🎨 **v0.4.0 - Helper Macros!** 12 macros to reduce boilerplate - save 20-45 characters per operation - [see macro guide](MACRO_GUIDE.md)

> 📦 **v0.3.0 - Container Support!** Query Vec, HashMap, HashSet, BTreeMap, VecDeque, and more - [see container guide](CONTAINER_SUPPORT.md)

> ⚡ **v0.3.0 - Lazy Evaluation!** New `LazyQuery` with deferred execution and early termination - [see lazy guide](LAZY_EVALUATION.md)

> 🚀 **v0.2.0 - Performance Optimized!** Most operations now work **without `Clone`** - [see optimization guide](OPTIMIZATION.md)

> 🔒 **Memory Safe!** Using `'static` bounds causes **0 memory leaks** - [verified with tests](MEMORY_SAFETY.md) ✅

> 💡 **New!** See how SQL queries map to Rust Query Builder in our [SQL Comparison Example](#example-sql-comparison) - demonstrates 15 SQL patterns side-by-side!

> ✅ **Verified!** All query results are **exact SQL equivalents** - [see verification tests](SQL_FEATURES.md) (17/17 tests passing)

## Features

- 🔒 **Type-safe queries**: Compile-time type checking using key-paths
- 📊 **SQL-like operations**: WHERE, SELECT, ORDER BY, GROUP BY, JOIN
- 🧮 **Rich aggregations**: COUNT, SUM, AVG, MIN, MAX
- 📄 **Pagination**: LIMIT and SKIP operations
- 🔗 **Join operations**: INNER JOIN, LEFT JOIN, RIGHT JOIN, CROSS JOIN
- ⏰ **DateTime operations**: Filter by dates, times, weekdays, business hours - [details](DATETIME_GUIDE.md)
- ⚡ **Zero-cost abstractions**: Leverages Rust's zero-cost abstractions
- 🎯 **Fluent API**: Chain operations naturally
- 🚀 **Clone-free operations**: Most operations work without `Clone` - [details](OPTIMIZATION.md)
- ⚡ **Lazy evaluation**: Deferred execution with early termination - **up to 1000x faster** - [details](LAZY_EVALUATION.md)
- 📦 **Multiple containers**: Vec, HashMap, HashSet, BTreeMap, VecDeque, arrays, and more - [details](CONTAINER_SUPPORT.md)
- 🎨 **Helper macros**: 12 macros to reduce boilerplate - **30% less code** - [details](MACRO_GUIDE.md)
- 🎯 **Extension trait**: Call `.query()` and `.lazy_query()` directly on containers - [details](EXTENSION_TRAIT_GUIDE.md)
- 📝 **Derive macros**: Auto-generate query helpers with `#[derive(QueryBuilder)]` - [details](EXTENSION_TRAIT_GUIDE.md)
- 🔒 **Lock-aware querying**: Query `Arc<RwLock<T>>` and `Arc<Mutex<T>>` without copying - **5x faster!**

## Installation

### Option 1: Umbrella Crate (Recommended for Applications)

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-queries-builder = "0.7.0"
key-paths-derive = "0.5.0"

# Optional: Enable datetime operations with chrono
rust-queries-builder = { version = "0.7.0", features = ["datetime"] }
chrono = "0.4"
```

### Option 2: Individual Crates (Recommended for Libraries/POCs)

For faster builds (65% faster) and minimal dependencies:

```toml
[dependencies]
rust-queries-core = "0.7.0"
rust-queries-derive = "0.7.0"  # Optional, only if using derive macros
key-paths-derive = "0.5.0"

# Optional: Enable datetime operations with chrono
rust-queries-core = { version = "0.7.0", features = ["datetime"] }
chrono = "0.4"
```

**⚠️ Important**: When using individual crates, import from the correct locations:
```rust
use rust_queries_core::{Query, QueryExt};  // ← QueryExt is here!
use rust_queries_derive::QueryBuilder;      // ← Derive macros here
```

See the [Individual Crates Guide](INDIVIDUAL_CRATES_GUIDE.md) for complete details.

## Quick Start

### Extension Trait (Easiest)

```rust
use rust_queries_builder::QueryExt;  // Extension trait
use key_paths_derive::Keypaths;

// Note: Clone not required for most operations!
#[derive(Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

let products = vec![/* ... */];

// Call .query() directly on Vec!
let query = products.query().where_(Product::price_r(), |&p| p > 100.0);
let expensive = query.all();

// Or use lazy queries for better performance
let cheap: Vec<_> = products
    .lazy_query()
    .where_(Product::price_r(), |&p| p < 50.0)
    .collect();
```

### Standard Query (Eager)

```rust
use rust_queries_builder::Query;
use key_paths_derive::Keypaths;

#[derive(Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

fn main() {
    let products = vec![
        Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15 },
        Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50 },
        Product { id: 3, name: "Desk".to_string(), price: 299.99, category: "Furniture".to_string(), stock: 10 },
    ];

    // Filter products by category and price
    let query = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&price| price < 100.0);
    let affordable_electronics = query.all();

    println!("Found {} affordable electronics", affordable_electronics.len());
}
```

### Lazy Query (Deferred Execution - NEW in v0.3.0!)

```rust
use rust_queries_builder::LazyQuery;
use key_paths_derive::Keypaths;

fn main() {
    let products = vec![/* ... */];

    // Build query (nothing executes yet!)
    let query = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&price| price < 100.0)
        .take_lazy(10);  // Will stop after finding 10 items!

    // Execute query (lazy evaluation with early termination)
    let first_10: Vec<_> = query.collect();

    println!("Found {} items (stopped early!)", first_10.len());
    // Up to 100x faster for large datasets with take_lazy!
}
```

## Core Operations

### Filtering with `where_`

Filter collections using type-safe key-paths:

```rust
let query = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics");
let electronics = query.all();

// Multiple conditions
let query2 = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&price| price > 500.0)
    .where_(Product::stock_r(), |&stock| stock > 0);
let premium_electronics = query2.all();
```

### Selecting Fields with `select`

Project specific fields from your data:

```rust
// Get all product names
let names: Vec<String> = Query::new(&products)
    .select(Product::name_r());

// Get prices of electronics
let prices: Vec<f64> = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .select(Product::price_r());
```

### Ordering Results

Sort results by any field:

```rust
// Sort by price (ascending)
let by_price = Query::new(&products)
    .order_by_float(Product::price_r());

// Sort by name (descending)
let by_name_desc = Query::new(&products)
    .order_by_desc(Product::name_r());

// Sort with filtering
let sorted_electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .order_by_float(Product::price_r());
```

### Aggregations

Compute statistics over your data:

```rust
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics");

// Count
let count = electronics.count();

// Sum
let total_value: f64 = electronics.sum(Product::price_r());

// Average
let avg_price = electronics.avg(Product::price_r()).unwrap_or(0.0);

// Min and Max
let cheapest = electronics.min_float(Product::price_r());
let most_expensive = electronics.max_float(Product::price_r());
```

### Grouping with `group_by`

Group data by field values:

```rust
use std::collections::HashMap;

// Group products by category
let by_category: HashMap<String, Vec<Product>> = Query::new(&products)
    .group_by(Product::category_r());

// Calculate statistics per group
for (category, items) in &by_category {
    let cat_query = Query::new(items);
    let avg = cat_query.avg(Product::price_r()).unwrap_or(0.0);
    println!("{}: {} products, avg price ${:.2}", category, items.len(), avg);
}
```

### Pagination

Limit and skip results for pagination:

```rust
// Get first 10 products
let query = Query::new(&products);
let first_page = query.limit(10);

// Get second page (skip 10, take 10)
let query = Query::new(&products);
let second_page = query.skip(10).limit(10);

// Get first matching item
let query = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 1000.0);
let first = query.first();
```

## Join Operations

Combine multiple collections with type-safe joins:

```rust
use rust_queries_builder::JoinQuery;

#[derive(Clone, Keypaths)]
struct User {
    id: u32,
    name: String,
}

#[derive(Clone, Keypaths)]
struct Order {
    id: u32,
    user_id: u32,
    total: f64,
}

let users = vec![
    User { id: 1, name: "Alice".to_string() },
    User { id: 2, name: "Bob".to_string() },
];

let orders = vec![
    Order { id: 101, user_id: 1, total: 99.99 },
    Order { id: 102, user_id: 1, total: 149.99 },
    Order { id: 103, user_id: 2, total: 199.99 },
];

// Inner join: users with their orders
let user_orders = JoinQuery::new(&users, &orders)
    .inner_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| (user.name.clone(), order.total)
    );

// Left join: all users, with or without orders
let all_users_orders = JoinQuery::new(&users, &orders)
    .left_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| match order {
            Some(o) => format!("{} has order totaling ${:.2}", user.name, o.total),
            None => format!("{} has no orders", user.name),
        }
    );

// Join with filter: only high-value orders
let high_value = JoinQuery::new(&users, &orders)
    .inner_join_where(
        User::id_r(),
        Order::user_id_r(),
        |_user, order| order.total > 100.0,
        |user, order| (user.name.clone(), order.total)
    );
```

### Available Join Types

- **Inner Join**: Returns only matching pairs
- **Left Join**: Returns all left items with optional right matches
- **Right Join**: Returns all right items with optional left matches
- **Cross Join**: Returns Cartesian product of both collections
- **Join Where**: Inner join with additional predicates

## DateTime Operations (NEW in v0.7.0!)

Query by dates, times, weekdays, and business hours with optional chrono support:

```rust
use rust_queries_builder::Query;
use chrono::{Utc, Duration};
use key_paths_derive::Keypaths;

#[derive(Keypaths)]
struct Event {
    id: u32,
    title: String,
    scheduled_at: DateTime<Utc>,
    category: String,
}

let events = vec![/* ... */];
let now = Utc::now();

// Events scheduled in the next 7 days
let upcoming = Query::new(&events)
    .where_between(
        Event::scheduled_at_r(), 
        now, 
        now + Duration::days(7)
    );

// Weekend events
let weekend = Query::new(&events)
    .where_weekend(Event::scheduled_at_r());

// Work events during business hours on weekdays
let work_hours = Query::new(&events)
    .where_(Event::category_r(), |c| c == "Work")
    .where_weekday(Event::scheduled_at_r())
    .where_business_hours(Event::scheduled_at_r());

// Events in December 2024
let december = Query::new(&events)
    .where_year(Event::scheduled_at_r(), 2024)
    .where_month(Event::scheduled_at_r(), 12);
```

### Available DateTime Operations

- **Date Comparisons**: `where_after`, `where_before`, `where_between`
- **Date Components**: `where_year`, `where_month`, `where_day`
- **Day Type**: `where_weekend`, `where_weekday`, `where_today`
- **Time Filters**: `where_business_hours`
- **SystemTime Support**: Basic operations without feature flags

See the [DateTime Guide](DATETIME_GUIDE.md) for complete documentation and examples.

## Advanced Examples

### Complex Multi-Stage Query

```rust
// Find top 5 expensive electronics in stock, ordered by rating
let top_electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::stock_r(), |&stock| stock > 0)
    .where_(Product::price_r(), |&price| price > 100.0)
    .order_by_float_desc(Product::rating_r());

for product in top_electronics.iter().take(5) {
    println!("{} - ${:.2} - Rating: {:.1}", 
        product.name, product.price, product.rating);
}
```

### Three-Way Join

```rust
#[derive(Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

// First join: Orders with Users
let orders_users = JoinQuery::new(&orders, &users)
    .inner_join(
        Order::user_id_r(),
        User::id_r(),
        |order, user| (order.clone(), user.clone())
    );

// Second join: Add Products
let mut complete_orders = Vec::new();
for (order, user) in orders_users {
    for product in &products {
        if order.product_id == product.id {
            complete_orders.push((user.name.clone(), product.name.clone(), order.total));
        }
    }
}
```

### Category Sales Analysis

```rust
// Join orders with products, then aggregate by category
let order_products = JoinQuery::new(&orders, &products)
    .inner_join(
        Order::product_id_r(),
        Product::id_r(),
        |order, product| (product.category.clone(), order.total)
    );

let mut category_sales: HashMap<String, f64> = HashMap::new();
for (category, total) in order_products {
    *category_sales.entry(category).or_insert(0.0) += total;
}
```

## API Reference

### Query Methods

**Basic Operations:**
- `new(data: &[T])` - Create a new query
- `where_(path, predicate)` - Filter by predicate
- `all()` - Get all matching items
- `first()` - Get first matching item
- `count()` - Count matching items
- `limit(n)` - Limit results
- `skip(n)` - Skip results for pagination
- `exists()` - Check if any match

**Ordering:**
- `order_by(path)` - Sort ascending
- `order_by_desc(path)` - Sort descending
- `order_by_float(path)` - Sort f64 ascending
- `order_by_float_desc(path)` - Sort f64 descending

**Projection & Grouping:**
- `select(path)` - Project field
- `group_by(path)` - Group by field

**Aggregations:**
- `sum(path)` - Sum numeric field
- `avg(path)` - Average of f64 field
- `min(path)` / `max(path)` - Min/max of Ord field
- `min_float(path)` / `max_float(path)` - Min/max of f64 field

**DateTime Operations (with `datetime` feature):**
- `where_after(path, time)` - Filter after datetime
- `where_before(path, time)` - Filter before datetime
- `where_between(path, start, end)` - Filter within range
- `where_today(path, now)` - Filter for today
- `where_year(path, year)` - Filter by year
- `where_month(path, month)` - Filter by month (1-12)
- `where_day(path, day)` - Filter by day (1-31)
- `where_weekend(path)` - Filter for weekends
- `where_weekday(path)` - Filter for weekdays
- `where_business_hours(path)` - Filter for business hours (9 AM - 5 PM)

**DateTime Operations (SystemTime, always available):**
- `where_after_systemtime(path, time)` - Filter after SystemTime
- `where_before_systemtime(path, time)` - Filter before SystemTime
- `where_between_systemtime(path, start, end)` - Filter within range

### JoinQuery Methods

- `new(left, right)` - Create a new join query
- `inner_join(left_key, right_key, mapper)` - Inner join
- `left_join(left_key, right_key, mapper)` - Left join
- `right_join(left_key, right_key, mapper)` - Right join
- `inner_join_where(left_key, right_key, predicate, mapper)` - Filtered join
- `cross_join(mapper)` - Cartesian product

## Running Examples

```bash
# Advanced query builder example
cargo run --example advanced_query_builder

# Join operations example
cargo run --example join_query_builder

# DateTime operations - filter by dates, times, weekdays (v0.7.0+, requires datetime feature)
cargo run --example datetime_operations --features datetime

# Lazy DateTime operations - efficient datetime queries with early termination (v0.7.0+)
cargo run --example lazy_datetime_operations --features datetime --release

# DateTime helper functions - all datetime helpers with SQL equivalents (v0.7.0+)
cargo run --example datetime_helper_functions --features datetime

# Lazy datetime helpers - all helpers with lazy evaluation and performance benchmarks (v0.7.0+)
cargo run --example lazy_datetime_helpers --features datetime --release

# SQL comparison - see how SQL queries map to Rust Query Builder
cargo run --example sql_comparison

# SQL verification - verify exact SQL equivalence (17 tests)
cargo run --example sql_verification

# Documentation examples - verify all doc examples compile and run (10 tests)
cargo run --example doc_examples

# Clone-free operations - demonstrates performance optimization (v0.2.0+)
cargo run --example without_clone

# Memory safety verification - proves 'static doesn't cause memory leaks
cargo run --example memory_safety_verification

# Lazy evaluation - demonstrates deferred execution and early termination
cargo run --example lazy_evaluation

# Container support - demonstrates querying various container types
cargo run --example container_support

# Custom Queryable - implement Queryable for custom containers (7 examples)
cargo run --example custom_queryable

# Arc<RwLock<T>> HashMap - thread-safe shared data with all 17 lazy operations
cargo run --example arc_rwlock_hashmap

# Lock-aware queries - query Arc<RwLock<T>> WITHOUT copying (v0.8.0+, 5x faster!)
cargo run --example lock_aware_queries --release

# Macro helpers - reduce boilerplate with 12 helper macros (30% less code)
cargo run --example macro_helpers

# Extension trait & derive macros - call .query() directly on containers (v0.5.0+)
cargo run --example derive_and_ext

# Individual crates usage - demonstrates using core + derive separately (v0.6.0+)
cargo run --example individual_crates
```

### Example: SQL Comparison

The `sql_comparison` example demonstrates how traditional SQL queries (like those in HSQLDB) translate to Rust Query Builder:

```rust
// SQL: SELECT * FROM employees WHERE department = 'Engineering';
let engineering = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Engineering")
    .all();

// SQL: SELECT AVG(salary) FROM employees WHERE age < 30;
let avg_salary = Query::new(&employees)
    .where_(Employee::age_r(), |&age| age < 30)
    .avg(Employee::salary_r());

// SQL: SELECT * FROM employees ORDER BY salary DESC LIMIT 5;
let top_5 = Query::new(&employees)
    .order_by_float_desc(Employee::salary_r())
    .into_iter()
    .take(5)
    .collect::<Vec<_>>();
```

The example demonstrates 15 different SQL patterns including SELECT, WHERE, JOIN, GROUP BY, ORDER BY, aggregations, and subqueries.

## Performance

The query builder uses:
- **O(n)** filtering operations
- **O(n log n)** sorting operations  
- **O(n + m)** hash-based joins
- **Zero-cost abstractions** - compiled down to efficient iterators
- **Clone-free by default** - most operations work with references (v0.2.0+)

### Performance Characteristics

| Operation | Complexity | Memory | Clone Required? |
|-----------|-----------|--------|-----------------|
| `where_` / `all` | O(n) | Zero extra | ❌ No |
| `count` | O(n) | Zero extra | ❌ No |
| `select` | O(n) | Only field copies | ❌ No |
| `sum` / `avg` | O(n) | Zero extra | ❌ No |
| `limit` / `skip` | O(n) | Zero extra | ❌ No |
| `order_by*` | O(n log n) | Clones all items | ✅ Yes |
| `group_by` | O(n) | Clones all items | ✅ Yes |
| Joins | O(n + m) | Zero extra | ❌ No |

**Example**: Filtering 10,000 employees (1KB each)
- **v0.1.0**: ~5ms (cloned 10MB)
- **v0.2.0**: ~0.1ms (zero copy) - **50x faster!**

## Key-Paths

This library leverages the `key-paths` crate to provide type-safe field access. The `Keypaths` derive macro automatically generates accessor methods for your structs:

```rust
#[derive(Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

// Generated methods:
// - Product::id_r() -> KeyPaths<Product, u32>
// - Product::name_r() -> KeyPaths<Product, String>
// - Product::price_r() -> KeyPaths<Product, f64>
```

## License

This project is licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built with [rust-key-paths](https://github.com/codefonsi/rust-key-paths) for type-safe field access.
