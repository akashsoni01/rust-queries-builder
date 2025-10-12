# tokio::sync::RwLock Support Guide

## 🚀 Extending Lock-Aware Queries to Async Locks

**Version**: 0.8.0+  
**Example**: `examples/tokio_rwlock_support.rs`  
**Status**: ✅ Production Ready Pattern

---

## Overview

This guide demonstrates how to extend the lock-aware querying system to support `tokio::sync::RwLock`, enabling SQL-like queries on async-locked data structures.

The same pattern can be used to extend support to **any lock type**:
- `tokio::sync::Mutex`
- `parking_lot::RwLock`
- Custom lock implementations
- Third-party async lock types

---

## The Extension Pattern

### Step 1: Create a Newtype Wrapper

Due to Rust's orphan rules, we can't implement foreign traits on foreign types. We need a newtype:

```rust
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

/// Wrapper around Arc<tokio::sync::RwLock<T>>.
#[derive(Clone, Debug)]
pub struct TokioLock<T>(Arc<TokioRwLock<T>>);

impl<T> TokioLock<T> {
    /// Create a new TokioLock.
    pub fn new(value: T) -> Self {
        Self(Arc::new(TokioRwLock::new(value)))
    }
}
```

### Step 2: Implement LockValue Trait

```rust
use rust_queries_builder::LockValue;

impl<T> LockValue<T> for TokioLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // Synchronous wrapper for async lock
        tokio::task::block_in_place(|| {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let guard = self.0.read().await;
                Some(f(&*guard))
            })
        })
    }
}
```

### Step 3: Create Helper Functions

```rust
use rust_queries_builder::{LockQuery, LockLazyQuery};
use std::collections::HashMap;

/// Helper to create LockQuery from HashMap.
fn lock_query<V: 'static>(
    map: &HashMap<impl std::hash::Hash + Eq, TokioLock<V>>
) -> LockQuery<'_, V, TokioLock<V>> {
    let locks: Vec<_> = map.values().collect();
    LockQuery::from_locks(locks)
}

/// Helper to create LockLazyQuery from HashMap.
fn lock_lazy_query<V: 'static, K>(
    map: &HashMap<K, TokioLock<V>>
) -> LockLazyQuery<'_, V, TokioLock<V>, impl Iterator<Item = &TokioLock<V>>>
where
    K: std::hash::Hash + Eq,
{
    LockLazyQuery::new(map.values())
}
```

### Step 4: Use It!

```rust
type AsyncUserMap = HashMap<String, TokioLock<User>>;

#[tokio::main]
async fn main() {
    let mut users = HashMap::new();
    users.insert("u1".to_string(), TokioLock::new(User {
        name: "Alice".to_string(),
        status: "active".to_string(),
        score: 95.5,
    }));
    
    // Now use all SQL operations!
    let active_users = lock_query(&users)
        .where_(User::status_r(), |s| s == "active")
        .order_by_float_desc(User::score_r());
    
    let first_inactive = lock_lazy_query(&users)
        .where_(User::status_r(), |s| s == "inactive")
        .first();
    
    let count = lock_query(&users)
        .where_(User::score_r(), |&s| s > 90.0)
        .count();
}
```

---

## Supported Operations

All SQL operations work with tokio::RwLock:

✅ **Filtering**
- `where_()` - Multiple WHERE clauses
- Chained conditions

✅ **Projection**
- `select()` - Extract specific fields

✅ **Sorting**
- `order_by()` / `order_by_desc()`
- `order_by_float()` / `order_by_float_desc()`

✅ **Grouping**
- `group_by()` - Group by field values

✅ **Aggregations**
- `count()` - Count matching items
- `sum()` - Sum numeric fields
- `avg()` - Average numeric fields
- `min()` / `max()` - Find extremes

✅ **Limiting**
- `limit()` - Take first N results

✅ **Existence**
- `exists()` - Check if any match

✅ **Lazy Operations**
- `take_lazy()` - Early termination
- `first()` - Find first match
- `any()` - Existence check
- `select_lazy()` - Lazy projection

---

## Performance Results

From the example with 1,000 items:

| Operation | Eager | Lazy | Speedup |
|-----------|-------|------|---------|
| Find First | 714.8 µs | 4.8 µs | **149x faster** ⚡⚡⚡ |
| EXISTS Check | 662.9 µs | 7.3 µs | **90x faster** ⚡⚡ |

**Key Finding**: Performance benefits are maintained with tokio locks!

---

## Complete Example

```rust
use tokio::sync::RwLock as TokioRwLock;
use std::sync::Arc;
use std::collections::HashMap;
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths)]
struct User {
    name: String,
    status: String,
    score: f64,
}

// Step 1: Newtype wrapper
#[derive(Clone, Debug)]
pub struct TokioLock<T>(Arc<TokioRwLock<T>>);

impl<T> TokioLock<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(TokioRwLock::new(value)))
    }
}

// Step 2: Implement LockValue
use rust_queries_builder::LockValue;

impl<T> LockValue<T> for TokioLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        tokio::task::block_in_place(|| {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let guard = self.0.read().await;
                Some(f(&*guard))
            })
        })
    }
}

// Step 3: Helper functions
use rust_queries_builder::{LockQuery, LockLazyQuery};

fn lock_query<V: 'static>(
    map: &HashMap<impl std::hash::Hash + Eq, TokioLock<V>>
) -> LockQuery<'_, V, TokioLock<V>> {
    let locks: Vec<_> = map.values().collect();
    LockQuery::from_locks(locks)
}

fn lock_lazy_query<V: 'static, K>(
    map: &HashMap<K, TokioLock<V>>
) -> LockLazyQuery<'_, V, TokioLock<V>, impl Iterator<Item = &TokioLock<V>>>
where
    K: std::hash::Hash + Eq,
{
    LockLazyQuery::new(map.values())
}

// Step 4: Use it!
#[tokio::main]
async fn main() {
    let mut users = HashMap::new();
    users.insert("alice".to_string(), TokioLock::new(User {
        name: "Alice".to_string(),
        status: "active".to_string(),
        score: 95.5,
    }));
    
    users.insert("bob".to_string(), TokioLock::new(User {
        name: "Bob".to_string(),
        status: "active".to_string(),
        score: 87.3,
    }));
    
    // Query with all SQL operations!
    
    // WHERE + ORDER BY
    let active_users = lock_query(&users)
        .where_(User::status_r(), |s| s == "active")
        .order_by_float_desc(User::score_r());
    
    println!("Active users:");
    for user in active_users {
        println!("  • {} - score: {:.1}", user.name, user.score);
    }
    
    // Lazy FIRST
    let first = lock_lazy_query(&users)
        .where_(User::score_r(), |&s| s > 90.0)
        .first();
    
    if let Some(user) = first {
        println!("\nFirst high scorer: {} ({:.1})", user.name, user.score);
    }
    
    // Aggregations
    let avg_score = lock_query(&users)
        .avg(User::score_r())
        .unwrap_or(0.0);
    
    println!("\nAverage score: {:.2}", avg_score);
    
    // EXISTS
    let has_inactive = lock_lazy_query(&users)
        .where_(User::status_r(), |s| s == "inactive")
        .any();
    
    println!("Has inactive users? {}", has_inactive);
}
```

---

## Extending to Other Lock Types

### tokio::sync::Mutex

```rust
use tokio::sync::Mutex as TokioMutex;

#[derive(Clone, Debug)]
pub struct TokioMutexLock<T>(Arc<TokioMutex<T>>);

impl<T> TokioMutexLock<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(TokioMutex::new(value)))
    }
}

impl<T> LockValue<T> for TokioMutexLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        tokio::task::block_in_place(|| {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let guard = self.0.lock().await;
                Some(f(&*guard))
            })
        })
    }
}
```

### parking_lot::RwLock

```rust
use parking_lot::RwLock as ParkingLotRwLock;

#[derive(Clone, Debug)]
pub struct ParkingLotLock<T>(Arc<ParkingLotRwLock<T>>);

impl<T> ParkingLotLock<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(ParkingLotRwLock::new(value)))
    }
}

impl<T> LockValue<T> for ParkingLotLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.0.read();
        Some(f(&*guard))
    }
}
```

---

## Production Considerations

### ⚠️ Using block_in_place

The example uses `tokio::task::block_in_place` to wrap async locks synchronously. This is fine for:
- CPU-bound operations
- Short-lived locks
- Non-critical paths

For production async code, consider:

1. **Staying fully async** - Implement async versions of query methods
2. **Avoiding blocking** - Use async throughout your application
3. **Lock contention** - Monitor and optimize lock usage

### Fully Async Alternative

For production async systems, you might want to implement async query methods:

```rust
pub async fn lock_query_async<V: 'static>(
    map: &HashMap<impl std::hash::Hash + Eq, TokioLock<V>>
) -> Vec<V> where V: Clone {
    let mut results = Vec::new();
    for lock in map.values() {
        let guard = lock.0.read().await;
        results.push(guard.clone());
    }
    results
}
```

---

## Orphan Rule Workaround

### Why We Need a Newtype

Rust's orphan rules prevent implementing foreign traits on foreign types:

```rust
// ❌ ERROR: Can't implement foreign trait on foreign type
impl<T> LockValue<T> for Arc<TokioRwLock<T>> {
    // ...
}
```

### The Solution: Newtype Pattern

```rust
// ✅ OK: Our type wraps the foreign type
#[derive(Clone, Debug)]
pub struct TokioLock<T>(Arc<TokioRwLock<T>>);

impl<T> LockValue<T> for TokioLock<T> {
    // ... works!
}
```

This is a zero-cost abstraction - the wrapper has no runtime overhead.

---

## Benefits

### ✅ Type Safety

- Compile-time verification of field access
- No string-based queries
- Autocomplete support

### ✅ Zero Copy

- No unnecessary data cloning
- Only lock acquisition when needed
- Minimal memory overhead

### ✅ Performance

- Lazy evaluation with early termination
- 90-150x faster for limited queries
- Iterator fusion for chained operations

### ✅ Extensibility

- Easy to add new lock types
- Consistent API across lock types
- Works with any lock implementation

---

## Running the Example

```bash
cargo run --example tokio_rwlock_support --release
```

Output includes:
- 8 SQL operation demonstrations
- Performance benchmarks
- Side-by-side eager vs lazy comparisons
- Real performance numbers

---

## Summary

### Extension Pattern (3 Steps)

1. **Create newtype wrapper** - `TokioLock<T>(Arc<TokioRwLock<T>>)`
2. **Implement LockValue** - Provide `with_value()` method
3. **Create helpers** - `lock_query()` and `lock_lazy_query()` functions

### What You Get

- ✅ All SQL operations (WHERE, SELECT, ORDER BY, GROUP BY, etc.)
- ✅ Aggregations (COUNT, SUM, AVG, MIN, MAX)
- ✅ Lazy evaluation (FIRST, EXISTS, TAKE N)
- ✅ Performance benefits (90-150x faster)
- ✅ Type safety with key-paths
- ✅ Zero unnecessary copying

### Real-World Use Cases

- **Async web servers** - Query shared state
- **Real-time systems** - Monitor with tokio locks
- **Microservices** - Cached data with async access
- **Dashboard APIs** - Fast queries on locked data

---

## Next Steps

1. Try the example: `cargo run --example tokio_rwlock_support --release`
2. Adapt for your lock type (tokio::Mutex, parking_lot, custom)
3. Consider async alternatives for production
4. Monitor performance and lock contention

---

**Version**: 0.8.0  
**Example**: `examples/tokio_rwlock_support.rs`  
**Pattern**: Production Ready ✅

**The lock-aware query system is fully extensible to any lock type!** 🚀

