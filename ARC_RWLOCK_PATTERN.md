# Arc<RwLock<T>> HashMap Pattern

## Overview

This guide demonstrates how to query `HashMap<K, Arc<RwLock<V>>>` - a common pattern for thread-safe shared data in Rust applications. The example shows all 17 lazy query operations.

## Pattern

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

type ProductId = String;
type SharedProduct = Arc<RwLock<Product>>;
type ProductMap = HashMap<ProductId, SharedProduct>;
```

## Why This Pattern?

### Thread Safety

- **Arc** (Atomic Reference Counting): Share ownership across threads
- **RwLock** (Read-Write Lock): Multiple readers OR single writer
- **HashMap**: Fast O(1) key-based lookup

### Use Cases

- 🌐 Web server shared state
- 📦 Product catalogs
- 👤 User session stores
- ⚙️ Configuration caches
- 📊 Real-time inventory
- 🔄 Multi-threaded data processing

## Basic Usage

### Step 1: Extract Data for Querying

```rust
fn extract_products(map: &HashMap<String, Arc<RwLock<Product>>>) -> Vec<Product> {
    map.values()
        .filter_map(|arc_lock| {
            // Try to acquire read lock
            arc_lock.read().ok().map(|guard| guard.clone())
        })
        .collect()
}

// Usage
let product_map: HashMap<String, Arc<RwLock<Product>>> = /* ... */;
let products = extract_products(&product_map);
```

### Step 2: Query with LazyQuery

```rust
// Now you can use all lazy query operations!
let electronics: Vec<_> = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&p| p < 500.0)
    .collect();
```

## All 17 Lazy Operations

### 1. where_ - Lazy Filtering

```rust
let query = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::stock_r(), |&s| s > 20);

let results: Vec<_> = query.collect();  // Executes here
```

**Benefit**: Deferred execution - nothing happens until `.collect()`

### 2. select_lazy - Lazy Projection

```rust
let names: Vec<String> = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Furniture")
    .select_lazy(Product::name_r())
    .collect();
```

**Benefit**: Only extracts fields you need

### 3. take_lazy - Early Termination

```rust
let first_3: Vec<_> = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .take_lazy(3)
    .collect();
```

**Benefit**: Stops after finding 3 items - doesn't process rest

### 4. skip_lazy - Pagination

```rust
let page_2: Vec<_> = LazyQuery::new(&products)
    .skip_lazy(10)
    .take_lazy(10)
    .collect();
```

**Benefit**: Efficient pagination - only processes needed items

### 5. first - Short-Circuit Search

```rust
let expensive = LazyQuery::new(&products)
    .where_(Product::price_r(), |&p| p > 1000.0)
    .first();  // Stops at first match!
```

**Benefit**: Immediate termination on first match

### 6. any - Existence Check

```rust
let has_furniture = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Furniture")
    .any();  // Returns true/false, stops at first match
```

**Benefit**: Short-circuits - doesn't check all items

### 7. count - Count Items

```rust
let count = LazyQuery::new(&products)
    .where_(Product::active_r(), |&active| active)
    .count();
```

**Benefit**: Lazy iteration through items

### 8. sum_by - Sum Aggregation

```rust
let total: f64 = LazyQuery::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .sum_by(Product::price_r());
```

**Benefit**: Single-pass aggregation

### 9. avg_by - Average

```rust
let avg = LazyQuery::new(&products)
    .avg_by(Product::price_r())
    .unwrap_or(0.0);
```

**Benefit**: Computes average in single pass

### 10. min_by_float - Minimum

```rust
let min = LazyQuery::new(&products)
    .min_by_float(Product::price_r())
    .unwrap_or(0.0);
```

**Benefit**: Finds minimum efficiently

### 11. max_by_float - Maximum

```rust
let max = LazyQuery::new(&products)
    .max_by_float(Product::price_r())
    .unwrap_or(0.0);
```

**Benefit**: Finds maximum efficiently

### 12. find - Find with Predicate

```rust
let high_rated = LazyQuery::new(&products)
    .find(|item| item.rating > 4.7);  // Short-circuits!
```

**Benefit**: Stops at first match

### 13. for_each - Iteration

```rust
LazyQuery::new(&products)
    .where_(Product::stock_r(), |&s| s < 20)
    .for_each(|product| {
        println!("Low stock: {}", product.name);
    });
```

**Benefit**: Process items one at a time

### 14. fold - Custom Aggregation

```rust
let inventory_value = LazyQuery::new(&products)
    .fold(0.0, |acc, p| acc + (p.price * p.stock as f64));
```

**Benefit**: Custom aggregation logic

### 15. all_match - Validation

```rust
let all_valid = LazyQuery::new(&products)
    .all_match(|item| item.price > 0.0);  // Short-circuits on first false
```

**Benefit**: Early exit on validation failure

### 16. into_iter - For Loop Support

```rust
for product in LazyQuery::new(&products)
    .where_(Product::price_r(), |&p| p > 300.0)
    .take_lazy(5)
{
    println!("{}", product.name);
}
```

**Benefit**: Natural Rust iteration

### 17. map_items - Transformation

```rust
let price_tags: Vec<String> = LazyQuery::new(&products)
    .map_items(|p| format!("{}: ${:.2}", p.name, p.price))
    .take(5)
    .collect();
```

**Benefit**: Transform items lazily

## Updating Arc<RwLock<T>> Data

### Read Access

```rust
if let Some(product_arc) = product_map.get("PROD-001") {
    // Acquire read lock
    if let Ok(guard) = product_arc.read() {
        println!("Price: ${}", guard.price);
    }
}
```

### Write Access (Mutation)

```rust
if let Some(product_arc) = product_map.get("PROD-001") {
    // Acquire write lock
    if let Ok(mut guard) = product_arc.write() {
        guard.stock += 10;  // Mutate through write guard
        println!("Updated stock: {}", guard.stock);
    }
}

// Re-extract for querying updated data
let updated_products = extract_products(&product_map);
let query = LazyQuery::new(&updated_products);
```

## Multi-Threaded Example

```rust
use std::thread;

let product_map = Arc::new(product_map);

// Spawn multiple reader threads
let handles: Vec<_> = (0..4).map(|i| {
    let map_clone = Arc::clone(&product_map);
    
    thread::spawn(move || {
        // Extract and query in each thread
        let products = extract_products(&map_clone);
        
        let count = LazyQuery::new(&products)
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .count();
        
        println!("Thread {}: Found {} electronics", i, count);
    })
}).collect();

// Wait for all threads
for handle in handles {
    handle.join().unwrap();
}
```

## Performance Considerations

### Extraction Cost

```rust
// ⚠️ Clones all values (necessary to escape locks)
let products = extract_products(&product_map);

// For large datasets, consider:
// 1. Extract once, query multiple times
// 2. Extract only needed items
// 3. Use lazy extraction if possible
```

### Lock Contention

```rust
// ✅ Good: Short lock duration
let products = extract_products(&map);  // Locks acquired briefly
drop(map);  // Can release map ref

// Query without holding locks
let results = LazyQuery::new(&products).collect();

// ❌ Bad: Hold locks during query
// (Don't do this - prevents other threads from accessing)
```

## Complete Example

```bash
cargo run --example arc_rwlock_hashmap
```

**Demonstrates:**
- ✅ All 17 lazy operations
- ✅ HashMap<String, Arc<RwLock<Product>>>
- ✅ Thread-safe data extraction
- ✅ Updates through RwLock
- ✅ Category-based statistics
- ✅ Performance comparison

## Key Takeaways

1. **Extract First, Then Query**
   ```rust
   let products = extract_products(&map);  // Extract once
   let q1 = LazyQuery::new(&products);     // Query many times
   let q2 = LazyQuery::new(&products);
   ```

2. **Short Lock Duration**
   - Extract data quickly
   - Release locks
   - Query without holding locks

3. **All Lazy Operations Work**
   - 17 operations demonstrated
   - Early termination available
   - Deferred execution
   - Iterator fusion

4. **Thread-Safe Updates**
   - Use `.write()` for mutations
   - Re-extract after updates
   - Query reflects latest state

## See Also

- [examples/arc_rwlock_hashmap.rs](examples/arc_rwlock_hashmap.rs) - Complete example
- [LAZY_EVALUATION.md](LAZY_EVALUATION.md) - Lazy query guide
- [CONTAINER_SUPPORT.md](CONTAINER_SUPPORT.md) - Container support

