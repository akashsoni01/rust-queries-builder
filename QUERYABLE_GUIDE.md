# Queryable Trait Guide

## Overview

The `Queryable` trait enables **any container type** to work with the Rust Query Builder. Implement it once, and your custom container gains all query capabilities!

## Quick Start

### Implementing Queryable

```rust
use rust_queries_builder::Queryable;

struct MyContainer<T> {
    items: Vec<T>,
}

impl<T> Queryable<T> for MyContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// Now you can query it!
let container = MyContainer { items: vec![/* ... */] };
let items: Vec<Product> = container.query_iter().cloned().collect();
let query = Query::new(&items);
```

## Built-in Implementations

The following types already implement `Queryable`:

| Type | Iterates Over | Notes |
|------|--------------|-------|
| `Vec<T>` | Items | Direct support |
| `&[T]` | Items | Direct support |
| `[T; N]` | Items | Fixed-size arrays |
| `VecDeque<T>` | Items | Double-ended queue |
| `LinkedList<T>` | Items | Doubly-linked list |
| `HashSet<T>` | Items | Unordered, unique |
| `BTreeSet<T>` | Items | Ordered, unique |
| `HashMap<K, V>` | **Values** | Queries map values |
| `BTreeMap<K, V>` | **Values** | Queries map values (sorted) |
| `Option<T>` | 0 or 1 item | Single item |
| `Result<T, E>` | 0 or 1 item | Single item (Ok variant) |

## Custom Container Examples

### Example 1: Paginated Collection

Store items in fixed-size pages for efficient memory management:

```rust
struct PaginatedCollection<T> {
    pages: Vec<Vec<T>>,
    page_size: usize,
}

impl<T> Queryable<T> for PaginatedCollection<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // Flatten all pages into single iterator
        Box::new(self.pages.iter().flat_map(|page| page.iter()))
    }
}

// Usage
let mut paginated = PaginatedCollection::new(100);
// ... add items ...

let items: Vec<Product> = paginated.query_iter().cloned().collect();
let query = Query::new(&items)
    .where_(Product::price_r(), |&p| p < 100.0);
```

### Example 2: Circular Buffer

Fixed-capacity buffer that overwrites oldest items:

```rust
struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> Queryable<T> for CircularBuffer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.buffer.iter())
    }
}

// Usage
let mut circular = CircularBuffer::new(100);
// ... push items (auto-removes old ones) ...

let items: Vec<Product> = circular.query_iter().cloned().collect();
let avg = Query::new(&items).avg(Product::price_r());
```

### Example 3: Filtered Storage

Container that only accepts items matching a predicate:

```rust
struct FilteredStorage<T, F>
where
    F: Fn(&T) -> bool,
{
    items: Vec<T>,
    filter: F,
}

impl<T, F> Queryable<T> for FilteredStorage<T, F>
where
    F: Fn(&T) -> bool,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// Usage
let mut filtered = FilteredStorage::new(|p: &Product| p.price < 200.0);
filtered.add(product);  // Only stored if price < 200

let items: Vec<Product> = filtered.query_iter().cloned().collect();
let count = Query::new(&items).count();
```

### Example 4: Category Index

Items indexed by category for efficient category-based access:

```rust
use std::collections::HashMap;

struct CategoryIndex<T> {
    by_category: HashMap<String, Vec<T>>,
}

impl<T> Queryable<T> for CategoryIndex<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // Iterate over all items across all categories
        Box::new(self.by_category.values().flat_map(|vec| vec.iter()))
    }
}

// Usage
let mut index = CategoryIndex::new();
index.add("Electronics".to_string(), product1);
index.add("Furniture".to_string(), product2);

// Query across ALL categories
let items: Vec<Product> = index.query_iter().cloned().collect();
let expensive = Query::new(&items)
    .where_(Product::price_r(), |&p| p > 100.0)
    .all();
```

### Example 5: Cache with TTL

Time-based cache that only queries valid (non-expired) items:

```rust
use std::time::{Duration, SystemTime};

struct CachedItem<T> {
    item: T,
    inserted_at: SystemTime,
    ttl: Duration,
}

struct Cache<T> {
    items: Vec<CachedItem<T>>,
}

impl<T> Queryable<T> for Cache<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        let now = SystemTime::now();
        Box::new(
            self.items
                .iter()
                .filter(move |cached| {
                    now.duration_since(cached.inserted_at)
                        .map_or(false, |elapsed| elapsed < cached.ttl)
                })
                .map(|cached| &cached.item)
        )
    }
}

// Usage
let cache = Cache::new();
// ... insert items with TTL ...

// Automatically queries only non-expired items!
let items: Vec<Product> = cache.query_iter().cloned().collect();
let valid_count = Query::new(&items).count();
```

### Example 6: Versioned Collection

Wrapper that adds metadata to a collection:

```rust
struct VersionedCollection<T> {
    items: Vec<T>,
    version: u32,
    last_modified: String,
}

impl<T> Queryable<T> for VersionedCollection<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// Usage
let versioned = VersionedCollection {
    items: vec![product1, product2],
    version: 2,
    last_modified: "2025-10-11".to_string(),
};

let items: Vec<Product> = versioned.query_iter().cloned().collect();
let query = Query::new(&items);
```

### Example 7: Lazy Loader

Simulates lazy loading from database or file:

```rust
struct LazyLoader<T> {
    loaded_items: Vec<T>,
    total_count: usize,
}

impl<T> Queryable<T> for LazyLoader<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.loaded_items.iter())
    }
}

// Usage
let loader = LazyLoader::new(/* loaded items */);

// Query currently loaded items
let items: Vec<Product> = loader.query_iter().cloned().collect();
let total = Query::new(&items).sum(Product::price_r());
```

## Real-World Use Cases

### Database Result Wrapper

```rust
struct QueryResult<T> {
    rows: Vec<T>,
    total_count: usize,
    page: usize,
}

impl<T> Queryable<T> for QueryResult<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.rows.iter())
    }
}
```

### Event Stream Buffer

```rust
struct EventBuffer<T> {
    events: VecDeque<T>,
    max_events: usize,
}

impl<T> Queryable<T> for EventBuffer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.events.iter())
    }
}
```

### Multi-Source Collection

```rust
struct MultiSource<T> {
    primary: Vec<T>,
    secondary: Vec<T>,
    tertiary: Vec<T>,
}

impl<T> Queryable<T> for MultiSource<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.primary.iter()
                .chain(self.secondary.iter())
                .chain(self.tertiary.iter())
        )
    }
}
```

## Implementation Patterns

### Pattern 1: Simple Wrapper

For containers that wrap a single collection:

```rust
impl<T> Queryable<T> for Wrapper<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.inner.iter())
    }
}
```

### Pattern 2: Multiple Collections

For containers with multiple internal collections:

```rust
impl<T> Queryable<T> for MultiCollection<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.collection1.iter()
                .chain(self.collection2.iter())
                .chain(self.collection3.iter())
        )
    }
}
```

### Pattern 3: Filtered Iteration

For containers that should filter items when querying:

```rust
impl<T> Queryable<T> for FilteredContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.items.iter()
                .filter(|item| self.is_valid(item))
        )
    }
}
```

### Pattern 4: Nested Structures

For containers with nested collections:

```rust
impl<T> Queryable<T> for NestedContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.groups.values()
                .flat_map(|group| group.iter())
        )
    }
}
```

## Best Practices

### 1. Return All Queryable Items

```rust
// ✅ Good: Return all items that should be queryable
impl<T> Queryable<T> for Container<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.all_items.iter())
    }
}

// ❌ Avoid: Returning subset without clear reason
impl<T> Queryable<T> for Container<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter().take(10))  // Why only 10?
    }
}
```

### 2. Use Appropriate Iterator Combinators

```rust
// ✅ Good: Use combinators for complex logic
impl<T> Queryable<T> for ComplexContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.primary.iter()
                .chain(self.secondary.iter())
                .filter(|item| !self.is_excluded(item))
        )
    }
}
```

### 3. Document Iteration Behavior

```rust
/// Queries all valid items across all pages.
/// 
/// Items are returned in page order (page 0, page 1, etc.)
/// Expired or invalidated items are automatically filtered out.
impl<T> Queryable<T> for MyContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // Implementation
    }
}
```

## Performance Considerations

### Boxed Iterator Overhead

The `Box<dyn Iterator>` has a small overhead (~1 pointer indirection). For performance-critical code with simple containers, use direct `Query::new(&vec)` instead.

```rust
// ✅ Best performance: Direct slice access
let vec: Vec<Product> = /* ... */;
Query::new(&vec)  // No boxing

// ⚠️ Slight overhead: Through Queryable
let items: Vec<Product> = container.query_iter().cloned().collect();
Query::new(&items)  // Involves boxing
```

### When to Use Queryable

**Use Queryable when:**
- ✅ You have a custom container type
- ✅ You want consistent query API across different containers
- ✅ Container has complex iteration logic
- ✅ Need abstraction over multiple container types

**Skip Queryable when:**
- ❌ You're just using Vec or slice (use `Query::new()` directly)
- ❌ Performance is absolutely critical
- ❌ Container is just a simple Vec wrapper

## Complete Example

See `examples/custom_queryable.rs` for 7 complete implementations:

```bash
cargo run --example custom_queryable
```

**Output shows:**
```
Demo 1: PaginatedCollection (custom container)
  Querying paginated collection:
    Electronics found: 3 ✅

Demo 2: CircularBuffer (fixed capacity)
  Querying circular buffer:
    Average price: $300.00 ✅

Demo 3: FilteredStorage (auto-filtering container)
  Querying filtered storage:
    In stock items: 2 ✅

[... 7 total demonstrations ...]
```

## API Reference

### Trait Definition

```rust
pub trait Queryable<T> {
    /// Returns an iterator over references to items in the container.
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}
```

### Using Queryable Containers

```rust
// Step 1: Get iterator from Queryable container
let items: Vec<T> = container.query_iter().cloned().collect();

// Step 2: Create Query
let query = Query::new(&items);

// Step 3: Use all query operations!
let results = query.where_(/* ... */).all();
let count = query.count();
let total = query.sum(field);
let groups = query.group_by(field);
```

### With LazyQuery

```rust
// For lazy evaluation with custom containers
let items: Vec<T> = container.query_iter().cloned().collect();

let first_match = LazyQuery::new(&items)
    .where_(field, predicate)
    .first();  // Early termination!
```

## Advanced Patterns

### Conditional Iteration

```rust
struct ConditionalContainer<T> {
    items: Vec<T>,
    include_hidden: bool,
}

impl<T> Queryable<T> for ConditionalContainer<T>
where
    T: HasHiddenFlag,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        if self.include_hidden {
            Box::new(self.items.iter())
        } else {
            Box::new(self.items.iter().filter(|item| !item.is_hidden()))
        }
    }
}
```

### Chained Containers

```rust
struct ChainedContainers<T> {
    containers: Vec<Box<dyn Queryable<T>>>,
}

impl<T> Queryable<T> for ChainedContainers<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // This is complex - collect all iterators
        let items: Vec<&T> = self.containers
            .iter()
            .flat_map(|c| c.query_iter())
            .collect();
        Box::new(items.into_iter())
    }
}
```

### Sorted Container

```rust
struct SortedContainer<T>
where
    T: Ord,
{
    items: BTreeSet<T>,
}

impl<T> Queryable<T> for SortedContainer<T>
where
    T: Ord,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())  // Already sorted!
    }
}
```

## Testing Your Implementation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queryable() {
        let mut container = MyContainer::new();
        container.add(item1);
        container.add(item2);

        // Test that iteration works
        let count = container.query_iter().count();
        assert_eq!(count, 2);

        // Test querying
        let items: Vec<Product> = container.query_iter().cloned().collect();
        let query = Query::new(&items);
        assert_eq!(query.count(), 2);
    }
}
```

## Migration Guide

### Making Existing Container Queryable

**Before:**
```rust
struct MyContainer<T> {
    items: Vec<T>,
}

// Had to expose items for querying
impl<T> MyContainer<T> {
    pub fn items(&self) -> &[T] {
        &self.items
    }
}

// Usage
let query = Query::new(container.items());
```

**After:**
```rust
struct MyContainer<T> {
    items: Vec<T>,
}

// Implement Queryable
impl<T> Queryable<T> for MyContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// Usage
let items: Vec<T> = container.query_iter().cloned().collect();
let query = Query::new(&items);
```

## Summary

✅ **Queryable trait enables:**
- Any container type to be queryable
- Consistent API across different containers
- Complex iteration logic encapsulation
- Filter expired/invalid items automatically
- Chain multiple containers

✅ **11+ built-in implementations**  
✅ **Easy to implement for custom types**  
✅ **7 complete examples provided**  
✅ **Works with Query and LazyQuery**  

Make any container queryable in just a few lines of code!

## Examples

Run comprehensive demonstrations:

```bash
# Standard containers (Vec, HashMap, HashSet, etc.)
cargo run --example container_support

# Custom containers (7 implementations)
cargo run --example custom_queryable
```

## See Also

- [CONTAINER_SUPPORT.md](CONTAINER_SUPPORT.md) - Standard container guide
- [examples/custom_queryable.rs](examples/custom_queryable.rs) - 7 custom implementations
- [examples/container_support.rs](examples/container_support.rs) - 11 standard containers

