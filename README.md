# Rust Query Builder

A powerful, type-safe query builder library for Rust that leverages **key-paths** for SQL-like operations on in-memory collections. This library brings the expressiveness of SQL to Rust's collections with compile-time type safety.

## Features

- 🔒 **Type-safe queries**: Compile-time type checking using key-paths
- 📊 **SQL-like operations**: WHERE, SELECT, ORDER BY, GROUP BY, JOIN
- 🧮 **Rich aggregations**: COUNT, SUM, AVG, MIN, MAX
- 📄 **Pagination**: LIMIT and SKIP operations
- 🔗 **Join operations**: INNER JOIN, LEFT JOIN, RIGHT JOIN, CROSS JOIN
- ⚡ **Zero-cost abstractions**: Leverages Rust's zero-cost abstractions
- 🎯 **Fluent API**: Chain operations naturally

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-queries-builder = "0.1.0"
key-paths-core = "1.0.1"
key-paths-derive = "0.5.0"
```

## Quick Start

```rust
use rust_queries_builder::Query;
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths)]
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
    let affordable_electronics = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&price| price < 100.0)
        .all();

    println!("Found {} affordable electronics", affordable_electronics.len());
}
```

## Core Operations

### Filtering with `where_`

Filter collections using type-safe key-paths:

```rust
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all();

// Multiple conditions
let premium_electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&price| price > 500.0)
    .where_(Product::stock_r(), |&stock| stock > 0)
    .all();
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
let first_page = Query::new(&products).limit(10);

// Get second page (skip 10, take 10)
let second_page = Query::new(&products).skip(10).limit(10);

// Get first matching item
let first = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 1000.0)
    .first();
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

- `new(data: &[T])` - Create a new query
- `where_(path, predicate)` - Filter by predicate
- `all()` - Get all matching items
- `first()` - Get first matching item
- `count()` - Count matching items
- `limit(n)` - Limit results
- `skip(n)` - Skip results for pagination
- `order_by(path)` - Sort ascending
- `order_by_desc(path)` - Sort descending
- `order_by_float(path)` - Sort f64 ascending
- `order_by_float_desc(path)` - Sort f64 descending
- `select(path)` - Project field
- `group_by(path)` - Group by field
- `sum(path)` - Sum numeric field
- `avg(path)` - Average of f64 field
- `min(path)` / `max(path)` - Min/max of Ord field
- `min_float(path)` / `max_float(path)` - Min/max of f64 field
- `exists()` - Check if any match

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
```

## Performance

The query builder uses:
- **O(n)** filtering operations
- **O(n log n)** sorting operations
- **O(n + m)** hash-based joins
- **Zero-cost abstractions** - compiled down to efficient iterators

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

Built with [rust-key-paths](https://github.com/yourusername/rust-key-paths) for type-safe field access.
