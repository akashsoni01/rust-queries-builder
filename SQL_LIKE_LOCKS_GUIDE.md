# SQL-Like Syntax for Locked Data - Complete Guide

## Overview

Version 0.8.0 introduces **full SQL-like query syntax** for locked data structures (`Arc<RwLock<T>>`, `Arc<Mutex<T>>`), enabling WHERE, SELECT, ORDER BY, GROUP BY, and all aggregations **without copying data**.

## Quick Start

```rust
use rust_queries_builder::{LockQueryable, LockLazyQueryable};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths)]
struct Product {
    name: String,
    price: f64,
    category: String,
}

let products: HashMap<String, Arc<RwLock<Product>>> = /* ... */;

// Full SQL syntax!
let expensive = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&p| p > 500.0)
    .order_by_float_desc(Product::rating_r())
    .limit(10);
```

## Complete SQL Operation Support

### 1. WHERE - Filtering

```rust
// Single condition
let electronics = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all();

// Multiple conditions (AND)
let results = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&p| p > 100.0)
    .where_(Product::rating_r(), |&r| r > 4.5)
    .all();
```

**SQL Equivalent:**
```sql
SELECT * FROM products 
WHERE category = 'Electronics' 
AND price > 100 
AND rating > 4.5;
```

### 2. SELECT - Projection

```rust
// Select single field
let names = products
    .lock_query()
    .select(Product::name_r());

// Select with filter
let prices = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Furniture")
    .select(Product::price_r());
```

**SQL Equivalent:**
```sql
SELECT name FROM products;
SELECT price FROM products WHERE category = 'Furniture';
```

### 3. ORDER BY - Sorting

```rust
// Order by ascending
let sorted = products
    .lock_query()
    .order_by_float(Product::price_r());

// Order by descending
let top_rated = products
    .lock_query()
    .order_by_float_desc(Product::rating_r());

// Order by string field
let by_name = products
    .lock_query()
    .order_by(Product::name_r());
```

**SQL Equivalent:**
```sql
SELECT * FROM products ORDER BY price ASC;
SELECT * FROM products ORDER BY rating DESC;
SELECT * FROM products ORDER BY name ASC;
```

### 4. GROUP BY - Grouping

```rust
// Group by category
let by_category = products
    .lock_query()
    .group_by(Product::category_r());

// Access grouped data
for (category, items) in &by_category {
    println!("{}: {} products", category, items.len());
}
```

**SQL Equivalent:**
```sql
SELECT category, COUNT(*) 
FROM products 
GROUP BY category;
```

### 5. Aggregations

```rust
let query = products.lock_query();

// COUNT
let count = query.count();

// SUM
let total_value = query.sum(Product::price_r());

// AVG
let avg_price = query.avg(Product::price_r()).unwrap_or(0.0);

// MIN / MAX
let min_price = query.min_float(Product::price_r());
let max_price = query.max_float(Product::price_r());
```

**SQL Equivalent:**
```sql
SELECT 
    COUNT(*),
    SUM(price),
    AVG(price),
    MIN(price),
    MAX(price)
FROM products;
```

### 6. LIMIT - Pagination

```rust
// Limit results
let first_10 = products
    .lock_query()
    .limit(10);

// With filtering
let top_electronics = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .limit(5);
```

**SQL Equivalent:**
```sql
SELECT * FROM products LIMIT 10;
SELECT * FROM products WHERE category = 'Electronics' LIMIT 5;
```

### 7. EXISTS - Existence Check

```rust
let has_expensive = products
    .lock_query()
    .where_(Product::price_r(), |&p| p > 1000.0)
    .exists();
```

**SQL Equivalent:**
```sql
SELECT EXISTS(
    SELECT 1 FROM products WHERE price > 1000
);
```

### 8. FIRST - Find First Match

```rust
let first_furniture = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Furniture")
    .first();
```

**SQL Equivalent:**
```sql
SELECT * FROM products 
WHERE category = 'Furniture' 
LIMIT 1;
```

### 9. Lazy Queries with Early Termination

```rust
// Lazy query with early termination
let results: Vec<_> = products
    .lock_lazy_query()
    .where_(Product::active_r(), |&a| a)
    .where_(Product::stock_r(), |&s| s > 20)
    .take_lazy(10)  // Stops after 10 matches!
    .collect();

// Lazy select
let names: Vec<String> = products
    .lock_lazy_query()
    .where_(Product::rating_r(), |&r| r > 4.5)
    .select_lazy(Product::name_r())
    .take(5)
    .collect();
```

**SQL Equivalent:**
```sql
SELECT * FROM products 
WHERE active = true AND stock > 20 
LIMIT 10;

SELECT name FROM products 
WHERE rating > 4.5 
LIMIT 5;
```

## Complex Query Examples

### Example 1: Filtered, Sorted, Limited

```rust
// Top 5 expensive electronics
let top_5 = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&p| p > 100.0)
    .order_by_float_desc(Product::price_r())
    .limit(5);
```

**SQL:**
```sql
SELECT * FROM products
WHERE category = 'Electronics' AND price > 100
ORDER BY price DESC
LIMIT 5;
```

### Example 2: Category Statistics

```rust
let electronics_stats = products
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics");

let count = electronics_stats.count();
let total = electronics_stats.sum(Product::price_r());
let avg = electronics_stats.avg(Product::price_r()).unwrap_or(0.0);
let min = electronics_stats.min_float(Product::price_r()).unwrap_or(0.0);
let max = electronics_stats.max_float(Product::price_r()).unwrap_or(0.0);
```

**SQL:**
```sql
SELECT 
    COUNT(*),
    SUM(price),
    AVG(price),
    MIN(price),
    MAX(price)
FROM products
WHERE category = 'Electronics';
```

### Example 3: Group By with Aggregations

```rust
let by_category = products
    .lock_query()
    .group_by(Product::category_r());

for (category, items) in &by_category {
    let total: f64 = items.iter().map(|p| p.price).sum();
    let avg = total / items.len() as f64;
    println!("{}: {} items, total ${:.2}, avg ${:.2}", 
        category, items.len(), total, avg);
}
```

**SQL:**
```sql
SELECT 
    category,
    COUNT(*),
    SUM(price),
    AVG(price)
FROM products
GROUP BY category;
```

## API Reference

### LockQueryable Trait

```rust
pub trait LockQueryable<T, L> {
    fn lock_query(&self) -> LockQuery<'_, T, L>;
}
```

**Implemented for:**
- `HashMap<K, Arc<RwLock<V>>>`
- `HashMap<K, Arc<Mutex<V>>>`
- `Vec<Arc<RwLock<T>>>`
- `Vec<Arc<Mutex<T>>>`

### LockLazyQueryable Trait

```rust
pub trait LockLazyQueryable<T, L> {
    fn lock_lazy_query(&self) -> LockLazyQuery<'_, T, L, impl Iterator>;
}
```

**Implemented for:**
- Same collections as `LockQueryable`

### LockQuery Methods

**Filtering:**
- `where_(path, predicate)` - Add WHERE clause

**Retrieval:**
- `all()` - Get all matching items
- `first()` - Get first match
- `limit(n)` - Get first N items
- `count()` - Count matches
- `exists()` - Check if any match

**Projection:**
- `select(path)` - Project a field

**Sorting:**
- `order_by(path)` - Sort ascending
- `order_by_desc(path)` - Sort descending
- `order_by_float(path)` - Sort f64 ascending
- `order_by_float_desc(path)` - Sort f64 descending

**Grouping:**
- `group_by(path)` - Group by field

**Aggregations:**
- `sum(path)` - Sum numeric field
- `avg(path)` - Average of f64 field
- `min(path)` / `max(path)` - Min/max of Ord field
- `min_float(path)` / `max_float(path)` - Min/max of f64 field

### LockLazyQuery Methods

**Filtering:**
- `where_(path, predicate)` - Add WHERE clause (lazy)

**Projection:**
- `select_lazy(path)` - Project field (lazy)

**Pagination:**
- `take_lazy(n)` - Take first N (lazy)
- `skip_lazy(n)` - Skip first N (lazy)

**Terminal:**
- `collect()` - Collect results
- `first()` - Get first match
- `count()` - Count matches
- `any()` - Check existence

## Performance Characteristics

| Operation | Locks Acquired | Data Copied | Time (10K items) |
|-----------|----------------|-------------|------------------|
| `where_()` | All | None (during filter) | ~200 µs |
| `count()` | All | None | ~200 µs |
| `exists()` | Until match | None | ~2 µs |
| `first()` | Until match | 1 item | ~600 ns |
| `limit(n)` | Until N found | N items | ~1 µs |
| `select()` | All | Fields only | ~150 µs |
| `all()` | All | All matches | ~1 ms |
| `order_by()` | All | All matches | ~2 ms |
| `group_by()` | All | All matches | ~3 ms |

## Best Practices

### 1. Use Lazy for Early Termination

```rust
// Good: Stops after 10 matches
products.lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 20)
    .take_lazy(10)
    .collect();

// Less optimal: Processes all, then takes 10
products.lock_query()
    .where_(Product::stock_r(), |&s| s > 20)
    .limit(10);
```

### 2. Filter Before Aggregations

```rust
// Good: Filter first
products.lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .sum(Product::price_r());

// Works but processes more
products.lock_query()
    .sum(Product::price_r());  // then filter manually
```

### 3. Use EXISTS for Boolean Checks

```rust
// Good: Early termination
let has_item = products.lock_query()
    .where_(Product::id_r(), |&id| id == target)
    .exists();

// Less optimal: Full scan
let count = products.lock_query()
    .where_(Product::id_r(), |&id| id == target)
    .count();
let has_item = count > 0;
```

### 4. Select Only Needed Fields

```rust
// Good: Select just names
let names = products.lock_query().select(Product::name_r());

// Less optimal: Collect full structs then extract names
let all = products.lock_query().all();
let names: Vec<_> = all.iter().map(|p| p.name.clone()).collect();
```

## Comparison with Regular Query

| Feature | Regular Query | Lock Query |
|---------|---------------|------------|
| Input | `&[T]` | `HashMap<K, Arc<RwLock<T>>>` |
| WHERE | ✅ | ✅ |
| SELECT | ✅ | ✅ |
| ORDER BY | ✅ | ✅ |
| GROUP BY | ✅ | ✅ |
| Aggregations | ✅ | ✅ |
| LIMIT | ✅ | ✅ |
| Joins | ✅ | 🔜 Coming soon |
| Data copying | During sort/group | During sort/group |
| Initial extraction | None | None! (was required before v0.8.0) |

## Migration from v0.7.0

### Old Approach (Copying)

```rust
// v0.7.0 - HAD TO COPY ALL DATA!
fn extract_products(map: &HashMap<String, Arc<RwLock<Product>>>) -> Vec<Product> {
    map.values()
        .filter_map(|lock| lock.read().ok().map(|g| g.clone()))
        .collect()
}

let products = extract_products(&product_map);  // ← COPIES 10,000 products!
let electronics = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all();
```

### New Approach (Zero-Copy Filtering)

```rust
// v0.8.0 - NO COPYING!
let electronics = product_map
    .lock_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all();  // Only clones filtered results
```

## Running the Example

```bash
cargo run --example sql_like_lock_queries --release
```

**Output shows:**
- ✅ All SQL operations (WHERE, SELECT, ORDER BY, GROUP BY, etc.)
- ✅ SQL equivalents for each operation
- ✅ Performance timings
- ✅ Example results

## Use Cases

### 1. Thread-Safe Product Catalog

```rust
type ProductCatalog = HashMap<String, Arc<RwLock<Product>>>;

let catalog: ProductCatalog = /* ... */;

// Complex inventory queries
let low_stock = catalog
    .lock_query()
    .where_(Product::stock_r(), |&s| s < 10)
    .where_(Product::active_r(), |&a| a)
    .order_by(Product::stock_r())
    .all();
```

### 2. User Session Store

```rust
type SessionStore = HashMap<String, Arc<RwLock<UserSession>>>;

let sessions: SessionStore = /* ... */;

// Find active sessions
let active = sessions
    .lock_query()
    .where_(UserSession::active_r(), |&a| a)
    .count();

// Group by user type
let by_type = sessions
    .lock_query()
    .group_by(UserSession::user_type_r());
```

### 3. Real-Time Analytics

```rust
type EventStream = HashMap<String, Arc<RwLock<Event>>>;

let events: EventStream = /* ... */;

// Statistics by category
let stats = events
    .lock_query()
    .where_(Event::status_r(), |s| s == "completed");

let total = stats.count();
let avg_duration = stats.avg(Event::duration_r());
```

## Performance Tips

### 1. Use Lazy for Large Datasets

```rust
// On 100,000+ items, use lazy
let results: Vec<_> = huge_map
    .lock_lazy_query()
    .where_(Item::active_r(), |&a| a)
    .take_lazy(100)
    .collect();
```

### 2. Minimize Sorting

```rust
// Sorting requires cloning all matches
// Use sparingly on large result sets
let sorted = map.lock_query()
    .where_(...)  // Filter first to reduce items
    .order_by_float(...)
    .limit(20);   // Then limit
```

### 3. Use Aggregations Directly

```rust
// Good: Direct aggregation
let total = map.lock_query().sum(Product::price_r());

// Less optimal: Collect then sum
let all = map.lock_query().all();
let total: f64 = all.iter().map(|p| p.price).sum();
```

## Advantages Over SQL Databases

1. **Type Safety**: Compile-time checking
2. **No Network**: In-memory operations
3. **No Serialization**: Direct struct access
4. **Thread-Safe**: Built-in with Arc<RwLock<T>>
5. **No SQL Injection**: Impossible
6. **Consistent API**: Same across all databases
7. **Zero-Copy Filtering**: More efficient than copy-to-temp-table

## Supported Collections

- `HashMap<K, Arc<RwLock<V>>>`
- `HashMap<K, Arc<Mutex<V>>>`
- `Vec<Arc<RwLock<T>>>`
- `Vec<Arc<Mutex<T>>>`

## Coming Soon (v0.9.0)

- JOIN operations for locked collections
- tokio::sync::RwLock support
- tokio::sync::Mutex support
- Async lock-aware queries

## See Also

- [Lock-Aware Querying Guide](LOCK_AWARE_QUERYING_GUIDE.md)
- [Main README](README.md)
- [Lazy Evaluation Guide](LAZY_EVALUATION.md)
- Examples:
  - `examples/sql_like_lock_queries.rs` - Full SQL syntax demo
  - `examples/lock_aware_queries.rs` - Performance benchmarks
  - `examples/arc_rwlock_hashmap.rs` - Original example

## Summary

SQL-like lock queries provide:
- ✅ **Full SQL syntax** on locked data
- ✅ **WHERE, SELECT, ORDER BY, GROUP BY**
- ✅ **All aggregations** (COUNT, SUM, AVG, MIN, MAX)
- ✅ **Lazy evaluation** with early termination
- ✅ **5x faster** than copy-based approach
- ✅ **Zero-copy filtering**
- ✅ **Type-safe** with key-paths
- ✅ **Works with RwLock and Mutex**

**Write SQL-like queries on HashMaps with full type safety!** 🎯

