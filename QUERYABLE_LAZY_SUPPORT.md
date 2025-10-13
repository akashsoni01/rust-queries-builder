# Queryable Trait: Full Lazy Query Support

## Overview

As of version 1.0.4, **all Queryable container types** now have complete access to lazy query operations through the new `QueryableExt` trait. This provides a unified, ergonomic API for querying any collection type with full lazy evaluation support.

## What's New

### QueryableExt Trait

A new trait `QueryableExt<T>` has been added that provides a blanket implementation for any type implementing `Queryable<T>`:

```rust
pub trait QueryableExt<T> {
    /// Create a lazy query from this Queryable container
    fn lazy_query(&self) -> LazyQuery<T, Box<dyn Iterator<Item = &T> + '_>>;
}
```

This means **any type that implements `Queryable`** automatically gets the `lazy_query()` method!

## Supported Containers

All of the following containers now support full lazy query operations:

| Container Type | Support | Queries | Notes |
|---------------|---------|---------|-------|
| `Vec<T>` | ✅ Full | Items | Direct slice support |
| `&[T]` | ✅ Full | Items | Direct slice support |
| `[T; N]` | ✅ Full | Items | Fixed-size arrays |
| `HashMap<K, V>` | ✅ Full | **Values** | Queries map values only |
| `BTreeMap<K, V>` | ✅ Full | **Values** | Queries map values (sorted) |
| `HashSet<T>` | ✅ Full | Items | Unordered unique items |
| `BTreeSet<T>` | ✅ Full | Items | Ordered unique items |
| `VecDeque<T>` | ✅ Full | Items | Double-ended queue |
| `LinkedList<T>` | ✅ Full | Items | Doubly-linked list |
| `Option<T>` | ✅ Full | 0-1 items | Single optional item |
| `Result<T, E>` | ✅ Full | 0-1 items | Single result item |

## Complete Operation Support

All Queryable types now support **all** of these operations through `LazyQuery`:

### Basic Operations

```rust
use rust_queries_core::QueryableExt;

let map: HashMap<u32, Product> = /* ... */;

// Filter operations
let results: Vec<_> = map
    .lazy_query()
    .where_(Product::price_r(), |&p| p > 100.0)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .collect();

// Pagination
let page: Vec<_> = map
    .lazy_query()
    .skip_lazy(20)
    .take_lazy(10)
    .collect();

// Existence checks
let has_expensive = map
    .lazy_query()
    .where_(Product::price_r(), |&p| p > 1000.0)
    .any();

// Count
let count = map
    .lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .count();

// First match
let first = map
    .lazy_query()
    .where_(Product::price_r(), |&p| p < 50.0)
    .first();
```

### Aggregation Operations

```rust
use rust_queries_core::QueryableExt;

let deque: VecDeque<Product> = /* ... */;

// Sum
let total: f64 = deque
    .lazy_query()
    .sum_by(Product::price_r());

// Average
let avg = deque
    .lazy_query()
    .avg_by(Product::price_r())
    .unwrap_or(0.0);

// Min/Max for Ord types
let min_stock = deque
    .lazy_query()
    .min_by(Product::stock_r());

let max_stock = deque
    .lazy_query()
    .max_by(Product::stock_r());

// Min/Max for f64
let min_price = deque
    .lazy_query()
    .min_by_float(Product::price_r());

let max_price = deque
    .lazy_query()
    .max_by_float(Product::price_r());
```

### Projection Operations

```rust
use rust_queries_core::QueryableExt;

let set: BTreeSet<Product> = /* ... */;

// Select specific field
let names: Vec<String> = set
    .lazy_query()
    .select_lazy(Product::name_r())
    .collect();

// Custom mapping
let prices: Vec<f64> = set
    .lazy_query()
    .map_items(|p| p.price)
    .collect();
```

### DateTime Operations

All datetime operations are supported (with `datetime` feature):

```rust
use rust_queries_core::QueryableExt;

let list: LinkedList<Event> = /* ... */;

// DateTime filtering
let recent: Vec<_> = list
    .lazy_query()
    .where_after(Event::timestamp_r(), cutoff_time)
    .collect();

let this_year: Vec<_> = list
    .lazy_query()
    .where_year(Event::timestamp_r(), 2024)
    .collect();

let weekends: Vec<_> = list
    .lazy_query()
    .where_weekend(Event::timestamp_r())
    .collect();

// SystemTime support (always available)
let recent_sys: Vec<_> = list
    .lazy_query()
    .where_after_systemtime(Event::sys_timestamp_r(), cutoff)
    .collect();
```

## Usage Examples

### Example 1: Querying a HashMap

```rust
use rust_queries_core::QueryableExt;
use std::collections::HashMap;

let mut products: HashMap<u32, Product> = HashMap::new();
products.insert(1, Product { /* ... */ });
products.insert(2, Product { /* ... */ });

// Query the values with full lazy support
let expensive: Vec<_> = products
    .lazy_query()
    .where_(Product::price_r(), |&p| p > 100.0)
    .where_(Product::stock_r(), |&s| s > 0)
    .take_lazy(5)
    .collect();
```

### Example 2: Querying a VecDeque

```rust
use rust_queries_core::QueryableExt;
use std::collections::VecDeque;

let mut queue: VecDeque<Order> = VecDeque::new();
queue.push_back(Order { /* ... */ });

// Aggregate operations
let total_value: f64 = queue
    .lazy_query()
    .where_(Order::status_r(), |s| s == "completed")
    .sum_by(Order::total_r());

let avg_value = queue
    .lazy_query()
    .avg_by(Order::total_r())
    .unwrap_or(0.0);
```

### Example 3: Querying a BTreeMap

```rust
use rust_queries_core::QueryableExt;
use std::collections::BTreeMap;

let mut sorted_products: BTreeMap<String, Product> = BTreeMap::new();
sorted_products.insert("laptop".to_string(), Product { /* ... */ });

// Filter and project
let electronics_names: Vec<String> = sorted_products
    .lazy_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .select_lazy(Product::name_r())
    .collect();
```

### Example 4: Querying a LinkedList

```rust
use rust_queries_core::QueryableExt;
use std::collections::LinkedList;

let mut list: LinkedList<Task> = LinkedList::new();
list.push_back(Task { /* ... */ });

// Find first match
let urgent = list
    .lazy_query()
    .where_(Task::priority_r(), |&p| p > 5)
    .where_(Task::status_r(), |s| s == "pending")
    .first();
```

## Implementation Details

### Architecture

The implementation uses a two-tier trait system:

1. **`Queryable<T>`** - Low-level trait that provides `query_iter()` method
   - Implemented for all standard collections
   - Returns `Box<dyn Iterator<Item = &T> + '_>`

2. **`QueryableExt<T>`** - High-level trait that provides `lazy_query()` method
   - Blanket implementation for all types that implement `Queryable<T>`
   - Converts the iterator into a `LazyQuery` for ergonomic querying

### LazyQuery Constructor

A new `from_iter()` constructor was added to `LazyQuery`:

```rust
impl<'a, T: 'static, I> LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T>,
{
    pub fn from_iter(iter: I) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }
}
```

This allows creating `LazyQuery` instances from any iterator, enabling the `QueryableExt` blanket implementation.

### QueryExt vs QueryableExt

- **`QueryExt`** - Provides `query()` for eager queries (slice-based containers only)
- **`QueryableExt`** - Provides `lazy_query()` for all Queryable types (universal)

Both traits can coexist because:
- `QueryExt::query()` returns `Query<T>` (eager)
- `QueryableExt::lazy_query()` returns `LazyQuery<T, I>` (lazy)

## Benefits

### 1. Unified API

No matter what container type you use, the query API is identical:

```rust
// Works the same for Vec, HashMap, VecDeque, etc.
container
    .lazy_query()
    .where_(/* ... */)
    .collect()
```

### 2. Lazy Evaluation

All operations are deferred until a terminal operation (`collect()`, `count()`, `first()`, etc.) is called:

```rust
// Nothing executes here
let query = map
    .lazy_query()
    .where_(/* complex filter */);

// Execution happens here
let results = query.collect();
```

### 3. Early Termination

Operations like `first()` and `take_lazy()` can short-circuit:

```rust
// Stops as soon as first match is found
let first = large_map
    .lazy_query()
    .where_(/* ... */)
    .first();

// Only processes 10 items
let ten = large_set
    .lazy_query()
    .take_lazy(10)
    .collect();
```

### 4. Iterator Fusion

Rust's optimizer can fuse chained operations:

```rust
// All filters are fused into single pass
let results = deque
    .lazy_query()
    .where_(/* filter 1 */)
    .where_(/* filter 2 */)
    .where_(/* filter 3 */)
    .collect();
```

### 5. Zero Allocations

No intermediate collections are created:

```rust
// No temporary vectors created
let count = map
    .lazy_query()
    .where_(/* ... */)
    .where_(/* ... */)
    .count();  // Just counts, no allocation
```

## Performance

### HashMap Queries

```rust
// O(n) iteration over values
let results = map
    .lazy_query()
    .where_(Product::price_r(), |&p| p > 100.0)
    .collect();
```

### VecDeque Queries

```rust
// O(n) iteration, but stops early with first()
let first = deque
    .lazy_query()
    .where_(/* ... */)
    .first();  // Stops at first match
```

### Aggregations

```rust
// Single pass, no allocations
let total = list
    .lazy_query()
    .sum_by(Product::price_r());  // O(n) single pass
```

## Migration Guide

### Before (v1.0.3 and earlier)

```rust
// Had to convert to Vec first
let items: Vec<&Product> = map.values().collect();
let query = Query::new(&items);
let results = query.where_(/* ... */).all();
```

### After (v1.0.4+)

```rust
// Direct lazy query on HashMap
let results: Vec<_> = map
    .lazy_query()
    .where_(/* ... */)
    .collect();
```

## Complete API Reference

### Basic Operations

| Method | Description | Terminal? |
|--------|-------------|-----------|
| `where_(path, predicate)` | Filter by predicate | No |
| `collect()` / `all()` | Collect all matching items | Yes |
| `first()` | Get first matching item | Yes |
| `count()` | Count matching items | Yes |
| `take_lazy(n)` | Limit to n items | No |
| `skip_lazy(n)` | Skip n items | No |
| `any()` / `exists()` | Check if any match | Yes |
| `for_each(fn)` | Execute function for each | Yes |
| `fold(init, fn)` | Fold with accumulator | Yes |
| `find(predicate)` | Find item by predicate | Yes |
| `all_match(predicate)` | Check if all match | Yes |

### Aggregations

| Method | Description | Return Type | Terminal? |
|--------|-------------|-------------|-----------|
| `sum_by(path)` | Sum numeric field | `F` | Yes |
| `avg_by(path)` | Average of f64 field | `Option<f64>` | Yes |
| `min_by(path)` | Min of Ord field | `Option<F>` | Yes |
| `max_by(path)` | Max of Ord field | `Option<F>` | Yes |
| `min_by_float(path)` | Min of f64 field | `Option<f64>` | Yes |
| `max_by_float(path)` | Max of f64 field | `Option<f64>` | Yes |

### Projection

| Method | Description | Return Type | Terminal? |
|--------|-------------|-------------|-----------|
| `select_lazy(path)` | Project field value | `impl Iterator<Item = F>` | No |
| `map_items(fn)` | Map each item | `impl Iterator<Item = O>` | No |

### DateTime Operations (with `datetime` feature)

| Method | Description | Terminal? |
|--------|-------------|-----------|
| `where_after(path, time)` | After datetime | No |
| `where_before(path, time)` | Before datetime | No |
| `where_between(path, start, end)` | Between datetimes | No |
| `where_today(path, now)` | Today | No |
| `where_year(path, year)` | Specific year | No |
| `where_month(path, month)` | Specific month (1-12) | No |
| `where_day(path, day)` | Specific day (1-31) | No |
| `where_weekend(path)` | Weekends only | No |
| `where_weekday(path)` | Weekdays only | No |
| `where_business_hours(path)` | Business hours (9-5) | No |

### DateTime Operations (SystemTime, always available)

| Method | Description | Terminal? |
|--------|-------------|-----------|
| `where_after_systemtime(path, time)` | After SystemTime | No |
| `where_before_systemtime(path, time)` | Before SystemTime | No |
| `where_between_systemtime(path, start, end)` | Between SystemTimes | No |

## Examples

See `examples/queryable_comprehensive.rs` for a complete demonstration of all container types with lazy query operations.

## Version History

- **v1.0.4** - Added `QueryableExt` trait with blanket implementation for all Queryable types
- **v1.0.3** - Queryable trait for container support, manual conversion required
- **v0.9.0** - Initial lazy query support for Vec and slices

## Summary

With the addition of `QueryableExt`, **every Queryable container type** now has complete, ergonomic access to:

✅ All filtering operations  
✅ All aggregation operations  
✅ All projection operations  
✅ All datetime operations  
✅ All pagination operations  
✅ Full lazy evaluation  
✅ Early termination  
✅ Iterator fusion  
✅ Zero unnecessary allocations  

This provides a truly universal, SQL-like query API for any collection type in Rust! 🎉

