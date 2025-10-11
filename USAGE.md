# Usage Guide

This guide provides detailed examples and patterns for using the Rust Query Builder library.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Query Operations](#query-operations)
3. [Join Operations](#join-operations)
4. [Common Patterns](#common-patterns)
5. [Performance Tips](#performance-tips)

## Getting Started

### Setup

First, add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
rust-queries-builder = "0.1.0"
key-paths-core = "1.0.1"
key-paths-derive = "0.5.0"
```

### Define Your Data Models

Use the `Keypaths` derive macro to enable type-safe field access:

```rust
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}
```

## Query Operations

### Basic Filtering

```rust
use rust_queries_builder::Query;

// Single filter
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all();

// Multiple filters (AND logic)
let in_stock_electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::stock_r(), |&stock| stock > 0)
    .all();

// Complex predicates
let premium = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 500.0 && price < 2000.0)
    .where_(Product::name_r(), |name| name.contains("Pro"))
    .all();
```

### Projection (Select)

```rust
// Select single field
let product_names: Vec<String> = Query::new(&products)
    .select(Product::name_r());

// Select with filter
let electronics_prices: Vec<f64> = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .select(Product::price_r());
```

### Ordering

```rust
// Sort by string field (ascending)
let by_name = Query::new(&products)
    .order_by(Product::name_r());

// Sort by string field (descending)
let by_name_desc = Query::new(&products)
    .order_by_desc(Product::name_r());

// Sort by float field (ascending)
let by_price = Query::new(&products)
    .order_by_float(Product::price_r());

// Sort by float field (descending)
let by_price_desc = Query::new(&products)
    .order_by_float_desc(Product::price_r());

// Sort with filtering
let cheap_to_expensive = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .order_by_float(Product::price_r());
```

### Aggregations

```rust
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics");

// Count items
let count = electronics.count();

// Sum numeric fields
let total_value: f64 = electronics.sum(Product::price_r());
let total_stock: u32 = electronics.sum(Product::stock_r());

// Average (returns Option<f64>)
let avg_price = electronics.avg(Product::price_r()).unwrap_or(0.0);

// Min/Max for Ord types
let min_stock = electronics.min(Product::stock_r());
let max_stock = electronics.max(Product::stock_r());

// Min/Max for f64
let cheapest = electronics.min_float(Product::price_r());
let most_expensive = electronics.max_float(Product::price_r());
```

### Grouping

```rust
use std::collections::HashMap;

// Group by category
let by_category: HashMap<String, Vec<Product>> = Query::new(&products)
    .group_by(Product::category_r());

// Process each group
for (category, items) in &by_category {
    let cat_query = Query::new(items);
    println!("{}: {} items, avg price ${:.2}", 
        category,
        items.len(),
        cat_query.avg(Product::price_r()).unwrap_or(0.0)
    );
}

// Group with pre-filtering
let high_value_by_category = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 100.0)
    .group_by(Product::category_r());
```

### Pagination

```rust
// Limit results
let first_10 = Query::new(&products).limit(10);

// Skip and limit (pagination)
let page_1 = Query::new(&products).skip(0).limit(10);
let page_2 = Query::new(&products).skip(10).limit(10);
let page_3 = Query::new(&products).skip(20).limit(10);

// Get first matching item
let first_expensive = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 1000.0)
    .first();
```

### Existence Checks

```rust
// Check if any items match
let has_electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .exists();

let has_expensive = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 10000.0)
    .exists();
```

## Join Operations

### Inner Join

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

// Join users with orders
let user_orders = JoinQuery::new(&users, &orders)
    .inner_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| (user.name.clone(), order.id, order.total)
    );

// Result: Vec<(String, u32, f64)>
for (name, order_id, total) in user_orders {
    println!("{} placed order #{} for ${:.2}", name, order_id, total);
}
```

### Left Join

```rust
// Get all users with optional order information
let all_users = JoinQuery::new(&users, &orders)
    .left_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| match order {
            Some(o) => format!("{} has order #{}", user.name, o.id),
            None => format!("{} has no orders", user.name),
        }
    );
```

### Right Join

```rust
// Get all orders with optional user information
let all_orders = JoinQuery::new(&users, &orders)
    .right_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| match user {
            Some(u) => format!("Order #{} by {}", order.id, u.name),
            None => format!("Order #{} by unknown user", order.id),
        }
    );
```

### Join with Filter

```rust
// Join with additional predicate
let high_value_orders = JoinQuery::new(&users, &orders)
    .inner_join_where(
        User::id_r(),
        Order::user_id_r(),
        |_user, order| order.total > 100.0,
        |user, order| (user.name.clone(), order.total)
    );
```

### Three-Way Join

```rust
#[derive(Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
}

// First join: Orders with Users
let orders_users = JoinQuery::new(&orders, &users)
    .inner_join(
        Order::user_id_r(),
        User::id_r(),
        |order, user| (order.clone(), user.clone())
    );

// Second join: Add Products
let complete_orders: Vec<(String, String, f64)> = orders_users
    .iter()
    .flat_map(|(order, user)| {
        products.iter()
            .filter(|p| p.id == order.product_id)
            .map(move |product| {
                (
                    user.name.clone(),
                    product.name.clone(),
                    order.total,
                )
            })
    })
    .collect();
```

### Cross Join

```rust
#[derive(Clone, Keypaths)]
struct Color {
    name: String,
}

#[derive(Clone, Keypaths)]
struct Size {
    name: String,
}

// Generate all combinations
let variants = JoinQuery::new(&colors, &sizes)
    .cross_join(|color, size| {
        format!("{} {}", color.name, size.name)
    });
```

## Common Patterns

### Find Top N Items

```rust
// Top 5 most expensive products
let top_5_expensive = Query::new(&products)
    .order_by_float_desc(Product::price_r())
    .into_iter()
    .take(5)
    .collect::<Vec<_>>();
```

### Complex Filtering

```rust
// Products that meet multiple criteria
let results = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&price| price >= 100.0 && price <= 500.0)
    .where_(Product::stock_r(), |&stock| stock > 10)
    .order_by_float(Product::price_r());
```

### Category Statistics

```rust
// Calculate statistics for each category
let by_category = Query::new(&products).group_by(Product::category_r());

for (category, items) in &by_category {
    let query = Query::new(items);
    
    println!("{} Statistics:", category);
    println!("  Count: {}", query.count());
    println!("  Total Value: ${:.2}", query.sum(Product::price_r()));
    println!("  Avg Price: ${:.2}", query.avg(Product::price_r()).unwrap_or(0.0));
    println!("  Min Price: ${:.2}", query.min_float(Product::price_r()).unwrap_or(0.0));
    println!("  Max Price: ${:.2}", query.max_float(Product::price_r()).unwrap_or(0.0));
}
```

### User Activity Report

```rust
// Join users with orders and calculate statistics
let user_orders = JoinQuery::new(&users, &orders)
    .left_join(
        User::id_r(),
        Order::user_id_r(),
        |user, order| (user.clone(), order.cloned())
    );

// Aggregate by user
use std::collections::HashMap;

let mut user_stats: HashMap<u32, (String, usize, f64)> = HashMap::new();
for (user, order) in user_orders {
    let entry = user_stats
        .entry(user.id)
        .or_insert((user.name.clone(), 0, 0.0));
    
    if let Some(order) = order {
        entry.1 += 1; // order count
        entry.2 += order.total; // total spent
    }
}

// Print report
for (_, (name, count, total)) in user_stats {
    println!("{}: {} orders, ${:.2} total", name, count, total);
}
```

### Revenue Analysis

```rust
// Calculate potential revenue by category
let by_category = Query::new(&products).group_by(Product::category_r());

for (category, items) in &by_category {
    let revenue: f64 = items.iter()
        .map(|p| p.price * p.stock as f64)
        .sum();
    
    println!("{}: ${:.2} potential revenue", category, revenue);
}
```

### Low Stock Alert

```rust
// Find products that need restocking
let low_stock = Query::new(&products)
    .where_(Product::stock_r(), |&stock| stock < 20)
    .order_by(Product::stock_r());

println!("Low Stock Alert:");
for product in &low_stock {
    println!("  ⚠️  {} - Only {} in stock", product.name, product.stock);
}
```

## Performance Tips

### 1. Filter Early

Apply filters before ordering or other operations:

```rust
// Good: Filter first, then order
let result = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .order_by_float(Product::price_r());

// Less efficient: Order everything, then filter
```

### 2. Use Appropriate Join Types

- Use `inner_join` when you only need matching pairs
- Use `left_join` when you need all left items
- Avoid `cross_join` unless necessary (O(n×m) complexity)

### 3. Limit Results When Possible

```rust
// If you only need the first 10 results
let results = Query::new(&products)
    .where_(Product::price_r(), |&price| price > 100.0)
    .limit(10);
```

### 4. Use `exists()` for Boolean Checks

```rust
// More efficient than counting
let has_items = Query::new(&products)
    .where_(Product::stock_r(), |&stock| stock > 0)
    .exists();

// Less efficient
let has_items = query.count() > 0;
```

### 5. Reuse Queries

```rust
// Create query once
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics");

// Reuse for multiple operations
let count = electronics.count();
let total = electronics.sum(Product::price_r());
let avg = electronics.avg(Product::price_r());
```

### 6. Group Once, Query Multiple Times

```rust
let by_category = Query::new(&products).group_by(Product::category_r());

// Perform multiple analyses on the same grouping
for (category, items) in &by_category {
    let query = Query::new(items);
    // ... multiple operations on the same group
}
```

## Error Handling

The library uses `Option` types for operations that might not return results:

```rust
// avg() returns Option<f64>
let avg = Query::new(&products)
    .avg(Product::price_r())
    .unwrap_or(0.0);

// first() returns Option<&T>
if let Some(product) = Query::new(&products).first() {
    println!("Found: {}", product.name);
}

// min/max return Option
let min = Query::new(&products)
    .min_float(Product::price_r())
    .unwrap_or(0.0);
```

## Type Safety

The library leverages Rust's type system for compile-time safety:

```rust
// ✅ Correct: comparing String field with &str
query.where_(Product::category_r(), |cat| cat == "Electronics")

// ✅ Correct: comparing f64 field (note the &price)
query.where_(Product::price_r(), |&price| price > 100.0)

// ❌ Compile error: wrong type
query.where_(Product::price_r(), |price| price == "100") // won't compile

// ❌ Compile error: wrong field
query.where_(Product::name_r(), |&name| name > 100.0) // won't compile
```

## Conclusion

This guide covers the main patterns and best practices for using the Rust Query Builder library. For more examples, see the `examples/` directory in the repository.

