# Lazy Lock Query Aggregators & SQL Functions

## Overview

Added comprehensive aggregation functions and SQL-like operations to `LockLazyQuery` in version 1.0.1, providing full parity with `LockQuery` while maintaining lazy evaluation and early termination benefits.

## New Features Summary

### 📊 Aggregation Functions (6 new)
- `sum()` - Sum numeric fields
- `avg()` - Calculate averages (f64)
- `min()` - Find minimum (Ord types)
- `max()` - Find maximum (Ord types)
- `min_float()` - Find minimum (f64)
- `max_float()` - Find maximum (f64)

### 🔍 SQL-like Functions (4 new)
- `exists()` - Check existence with early termination
- `limit()` - Limit results (improved naming)
- `skip()` - Skip results (improved naming from skip_lazy)
- `distinct()` - Get unique values

### ⚡ Advanced Functions (6 new)
- `last()` - Get last matching item
- `nth()` - Get item at specific index
- `all_match()` - Verify all items match a condition
- `find()` - Find first item matching additional predicate
- `count_where()` - Count with conditional predicate

---

## Detailed Documentation

### Aggregation Functions

#### 1. `sum<F>(path: KeyPaths<T, F>) -> F`

**Purpose**: Calculate the sum of a numeric field across all matching items.

**Performance**: O(n) - processes each item once, no intermediate collections.

**Example**:
```rust
// Sum all product prices
let total_value: f64 = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .sum(Product::price_r());

// SQL: SELECT SUM(price) FROM products WHERE stock > 0
```

**Use Cases**:
- Calculate total inventory value
- Sum quantities across categories
- Aggregate financial data

---

#### 2. `avg(path: KeyPaths<T, f64>) -> Option<f64>`

**Purpose**: Calculate the average of f64 values. Returns None if no items match.

**Performance**: O(n) - single pass through items.

**Example**:
```rust
let avg_price = products
    .lock_lazy_query()
    .where_(Product::category_r(), |c| c == "Electronics")
    .avg(Product::price_r());

match avg_price {
    Some(avg) => println!("Average: ${:.2}", avg),
    None => println!("No items found"),
}

// SQL: SELECT AVG(price) FROM products WHERE category = 'Electronics'
```

**Use Cases**:
- Calculate average ratings
- Compute mean prices
- Statistical analysis

---

#### 3. `min<F>(path: KeyPaths<T, F>) -> Option<F>`

**Purpose**: Find the minimum value for Ord types. Returns None if no items match.

**Performance**: O(n) - single pass to find minimum.

**Example**:
```rust
let min_stock = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .min(Product::stock_r());

// SQL: SELECT MIN(stock) FROM products WHERE stock > 0
```

**Use Cases**:
- Find lowest stock levels
- Determine minimum thresholds
- Range analysis

---

#### 4. `max<F>(path: KeyPaths<T, F>) -> Option<F>`

**Purpose**: Find the maximum value for Ord types. Returns None if no items match.

**Performance**: O(n) - single pass to find maximum.

**Example**:
```rust
let max_price = products
    .lock_lazy_query()
    .where_(Product::category_r(), |c| c == "Electronics")
    .max(Product::price_r());

// SQL: SELECT MAX(price) FROM products WHERE category = 'Electronics'
```

**Use Cases**:
- Find highest prices
- Determine peak values
- Capacity planning

---

#### 5. `min_float(path: KeyPaths<T, f64>) -> Option<f64>`

**Purpose**: Find minimum f64 value with proper partial ordering.

**Performance**: O(n) - handles NaN values correctly.

**Example**:
```rust
let cheapest = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .min_float(Product::price_r());

// SQL: SELECT MIN(price) FROM products WHERE stock > 0
```

**Use Cases**:
- Financial calculations
- Scientific data analysis
- Price comparisons

---

#### 6. `max_float(path: KeyPaths<T, f64>) -> Option<f64>`

**Purpose**: Find maximum f64 value with proper partial ordering.

**Performance**: O(n) - handles NaN values correctly.

**Example**:
```rust
let most_expensive = products
    .lock_lazy_query()
    .max_float(Product::price_r());

// SQL: SELECT MAX(price) FROM products
```

**Use Cases**:
- Financial calculations
- Scientific data analysis
- Price comparisons

---

### SQL-like Functions

#### 7. `exists() -> bool`

**Purpose**: Check if any items exist matching the criteria. **Stops at first match!**

**Performance**: O(1) to O(n) - early termination makes it very efficient.

**Example**:
```rust
let has_expensive = products
    .lock_lazy_query()
    .where_(Product::price_r(), |&p| p > 1000.0)
    .exists();

if has_expensive {
    println!("We have luxury items!");
}

// SQL: SELECT EXISTS(SELECT 1 FROM products WHERE price > 1000)
```

**Use Cases**:
- Validation checks
- Existence queries
- Permission verification
- Conditional logic

**Benefits**:
- ⚡ Early termination - stops at first match
- 🚀 Much faster than `.count() > 0`
- 💡 SQL-like semantics

---

#### 8. `limit(n: usize) -> impl Iterator<Item = T>`

**Purpose**: Limit results to first N items. Returns an iterator for further processing.

**Performance**: O(n) where n is the limit - stops after N items.

**Example**:
```rust
let top_5: Vec<Product> = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 10)
    .limit(5)
    .collect();

// SQL: SELECT * FROM products WHERE stock > 10 LIMIT 5
```

**Use Cases**:
- Pagination
- Top-N queries
- Preview results
- Performance optimization

---

#### 9. `skip(n: usize) -> LockLazyQuery<...>`

**Purpose**: Skip first N items. Alias for `skip_lazy()` with better SQL-like naming.

**Performance**: O(n) where n is items to skip.

**Example**:
```rust
// Get second page (skip 10, take 10)
let page_2 = products
    .lock_lazy_query()
    .skip(10)
    .limit(10)
    .collect();

// SQL: SELECT * FROM products LIMIT 10 OFFSET 10
```

**Use Cases**:
- Pagination
- Result windowing
- Batch processing

**Improved API**: Better naming than `skip_lazy()` for SQL familiarity.

---

#### 10. `distinct<F>(path: KeyPaths<T, F>) -> Vec<F>`

**Purpose**: Get unique values for a field. Uses HashSet internally for deduplication.

**Performance**: O(n) - single pass with hash-based deduplication.

**Example**:
```rust
let categories: Vec<String> = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .distinct(Product::category_r());

println!("Available categories: {:?}", categories);

// SQL: SELECT DISTINCT category FROM products WHERE stock > 0
```

**Use Cases**:
- Get unique categories
- Find distinct values
- Data analysis
- Filter creation

---

### Advanced Functions

#### 11. `last() -> Option<T>`

**Purpose**: Get the last matching item.

**Performance**: O(n) - must consume entire iterator. Less efficient than `first()`.

**Example**:
```rust
let last_product = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .last();

// Note: Processes all items to find last one
```

**Use Cases**:
- Get most recent item
- Find end of sequence
- Chronological queries

**⚠️ Note**: Not as efficient as `first()` due to need to consume entire iterator.

---

#### 12. `nth(n: usize) -> Option<T>`

**Purpose**: Get item at specific index (0-based). Returns None if out of bounds.

**Performance**: O(n) - must iterate to reach position.

**Example**:
```rust
let third_item = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .nth(2);  // 0-indexed, so this is the 3rd item
```

**Use Cases**:
- Random access
- Sampling
- Testing

---

#### 13. `all_match<F, P>(path, predicate) -> bool`

**Purpose**: Check if ALL items match a predicate. Short-circuits on first non-match.

**Performance**: O(1) to O(n) - stops at first non-matching item.

**Example**:
```rust
let all_in_stock = products
    .lock_lazy_query()
    .where_(Product::category_r(), |c| c == "Electronics")
    .all_match(Product::stock_r(), |&s| s > 0);

if all_in_stock {
    println!("All electronics are in stock!");
}
```

**Use Cases**:
- Validation
- Quality checks
- Invariant verification
- Compliance checks

**Benefits**:
- ⚡ Early termination on first failure
- 🎯 Perfect for validation logic

---

#### 14. `find<F, P>(path, predicate) -> Option<T>`

**Purpose**: Find first item matching an additional predicate. Like `first()` but with extra condition.

**Performance**: O(1) to O(n) - stops at first match.

**Example**:
```rust
let expensive_laptop = products
    .lock_lazy_query()
    .where_(Product::category_r(), |c| c == "Electronics")
    .find(Product::price_r(), |&p| p > 500.0);
```

**Use Cases**:
- Conditional search
- First match queries
- Filtering with multiple conditions

**Benefits**:
- ⚡ Early termination
- 🎯 More expressive than chaining `where_`

---

#### 15. `count_where<F, P>(path, predicate) -> usize`

**Purpose**: Count items matching an additional condition. Like `count()` but with field-specific predicate.

**Performance**: O(n) - must check all items.

**Example**:
```rust
let expensive_count = products
    .lock_lazy_query()
    .where_(Product::category_r(), |c| c == "Electronics")
    .count_where(Product::price_r(), |&p| p > 500.0);

println!("Expensive electronics: {}", expensive_count);
```

**Use Cases**:
- Conditional counting
- Metric calculation
- Analytics queries

---

## Complete Function Reference

### Function Categories

| Category | Function | Return Type | Terminal? | Early Term? |
|----------|----------|-------------|-----------|-------------|
| **Aggregators** | `sum()` | F | ✅ Yes | ❌ No |
| | `avg()` | Option<f64> | ✅ Yes | ❌ No |
| | `min()` | Option<F> | ✅ Yes | ❌ No |
| | `max()` | Option<F> | ✅ Yes | ❌ No |
| | `min_float()` | Option<f64> | ✅ Yes | ❌ No |
| | `max_float()` | Option<f64> | ✅ Yes | ❌ No |
| **SQL Functions** | `exists()` | bool | ✅ Yes | ✅ Yes |
| | `limit()` | Iterator<T> | ❌ No | ✅ Yes |
| | `skip()` | LockLazyQuery | ❌ No | ❌ No |
| | `distinct()` | Vec<F> | ✅ Yes | ❌ No |
| **Advanced** | `first()` | Option<T> | ✅ Yes | ✅ Yes |
| | `last()` | Option<T> | ✅ Yes | ❌ No |
| | `nth()` | Option<T> | ✅ Yes | ✅ Yes |
| | `all_match()` | bool | ✅ Yes | ✅ Yes |
| | `find()` | Option<T> | ✅ Yes | ✅ Yes |
| | `count_where()` | usize | ✅ Yes | ❌ No |

**Legend**:
- **Terminal**: Consumes the query and returns final result
- **Early Term**: Can stop before processing all items

---

## Performance Characteristics

### Early Termination Functions (Most Efficient)

These stop processing as soon as the answer is known:

```rust
// ⚡ FASTEST - stops at first match
exists()      // O(1) to O(n), typically O(1)
find()        // O(1) to O(n), typically O(k) where k << n
first()       // O(1)
all_match()   // O(1) to O(n), stops at first false

// ⚡ EFFICIENT - stops at limit
limit(n)      // O(n) where n is limit
nth(n)        // O(n) where n is position
```

### Full Scan Functions (Still Efficient)

These must process all items but do so efficiently:

```rust
// 🚀 EFFICIENT - single pass, no intermediate collections
sum()         // O(n)
avg()         // O(n)
min()         // O(n)
max()         // O(n)
count()       // O(n)
count_where() // O(n)

// 💾 MEMORY EFFICIENT - uses HashSet for deduplication
distinct()    // O(n)

// ⚠️  LESS EFFICIENT - must consume entire iterator
last()        // O(n) - must process all to find last
```

---

## Usage Examples

### Example 1: Business Analytics

```rust
// Complex analytics query combining multiple aggregators
let electronics_stats = {
    let count = products.lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .count();
    
    let avg_price = products.lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .avg(Product::price_r())
        .unwrap_or(0.0);
    
    let total_value: f64 = products.lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .where_(Product::stock_r(), |&s| s > 0)
        .sum(Product::price_r());
    
    (count, avg_price, total_value)
};

println!("Electronics: {} items, avg ${:.2}, total ${:.2}", 
         electronics_stats.0, electronics_stats.1, electronics_stats.2);
```

### Example 2: Validation Checks

```rust
// Verify all products meet quality standards
let all_quality_pass = products
    .lock_lazy_query()
    .all_match(Product::rating_r(), |&r| r >= 4.0);

// Check if any high-priority items need attention
let needs_attention = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s < 10)
    .exists();

if needs_attention && all_quality_pass {
    println!("Quality good, but restock needed!");
}
```

### Example 3: Efficient Search

```rust
// Find first premium product
let premium = products
    .lock_lazy_query()
    .where_(Product::rating_r(), |&r| r > 4.5)
    .find(Product::price_r(), |&p| p > 500.0);

// Much more efficient than:
// let premium = products.lock_query()
//     .where_(...)
//     .all()
//     .into_iter()
//     .find(...);
```

### Example 4: Pagination

```rust
fn get_page(products: &HashMap<String, Arc<RwLock<Product>>>, page: usize, page_size: usize) -> Vec<Product> {
    products
        .lock_lazy_query()
        .skip(page * page_size)
        .limit(page_size)
        .collect()
}

let page_1 = get_page(&products, 0, 10); // First 10 items
let page_2 = get_page(&products, 1, 10); // Next 10 items
```

---

## Comparison with LockQuery

### Feature Parity

| Feature | LockQuery | LockLazyQuery | Notes |
|---------|-----------|---------------|-------|
| **Aggregations** | ✅ | ✅ | Full parity |
| **SQL Functions** | ✅ | ✅ | Enhanced with distinct |
| **WHERE** | ✅ | ✅ | Chainable |
| **SELECT** | select() | select_lazy() | Different naming |
| **ORDER BY** | ✅ | ❌ | Requires collection |
| **GROUP BY** | ✅ | ❌ | Requires collection |
| **Early Termination** | ❌ | ✅ | Lazy advantage |
| **Memory Usage** | Higher | Lower | Lazy advantage |

### When to Use Each

**Use LockLazyQuery when**:
- ✅ You only need first N results
- ✅ Existence checks (exists, any)
- ✅ Large datasets
- ✅ Memory constrained
- ✅ Early termination beneficial

**Use LockQuery when**:
- ✅ You need ORDER BY
- ✅ You need GROUP BY
- ✅ You need all results anyway
- ✅ Simpler mental model preferred

---

## Migration Guide

### From LockQuery to LockLazyQuery

```rust
// Before (LockQuery - eager)
let total: f64 = products
    .lock_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .sum(Product::price_r());

// After (LockLazyQuery - lazy, same result!)
let total: f64 = products
    .lock_lazy_query()
    .where_(Product::stock_r(), |&s| s > 0)
    .sum(Product::price_r());

// Benefits: Same API, lazy evaluation, potential early termination
```

### Performance Tips

1. **Use `exists()` instead of `count() > 0`**:
   ```rust
   // ❌ Slower - processes all items
   let has_items = products.lock_lazy_query().count() > 0;
   
   // ✅ Faster - stops at first item
   let has_items = products.lock_lazy_query().exists();
   ```

2. **Use `find()` instead of collecting then searching**:
   ```rust
   // ❌ Slower - collects everything first
   let found = products.lock_query().all()
       .into_iter()
       .find(|p| p.price > 1000.0);
   
   // ✅ Faster - stops at first match
   let found = products.lock_lazy_query()
       .find(Product::price_r(), |&p| p > 1000.0);
   ```

3. **Use `limit()` for top-N queries**:
   ```rust
   // ❌ Slower - processes all items
   let top_5: Vec<_> = products.lock_query().all()
       .into_iter()
       .take(5)
       .collect();
   
   // ✅ Faster - stops after 5 items
   let top_5: Vec<_> = products.lock_lazy_query()
       .limit(5)
       .collect();
   ```

---

## Running the Demo

```bash
# Run the comprehensive demo
cargo run --example lazy_aggregators_demo --release

# Features demonstrated:
# ✅ All 15 new functions
# ✅ Performance comparisons
# ✅ Business intelligence queries
# ✅ Real-world use cases
```

---

## Summary

**Added**: 15 new functions to LockLazyQuery
- 📊 6 Aggregators (sum, avg, min, max, min_float, max_float)
- 🔍 4 SQL Functions (exists, limit, skip, distinct)
- ⚡ 5 Advanced Functions (last, nth, all_match, find, count_where)

**Benefits**:
- ✅ Full feature parity with LockQuery for aggregations
- ⚡ Lazy evaluation with early termination where possible
- 💾 Memory efficient - no intermediate collections
- 🎯 Type-safe - compile-time checking
- 📝 SQL-like API - familiar semantics
- 🚀 Performance optimized - O(n) or better for most operations

**Perfect for**:
- Real-time analytics on locked data
- Large datasets with filtering
- Efficient existence checks
- Statistical analysis
- Business intelligence queries
- High-performance applications

The library now offers the most comprehensive lazy query system for locked Rust data structures! 🎉

