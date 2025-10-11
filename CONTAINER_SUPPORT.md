# Container Support

## Overview

The Rust Query Builder supports querying various container types beyond just `Vec<T>` and slices. Version 0.3.0 includes the `Queryable` trait that enables querying:

- ✅ Vec<T>
- ✅ Slices &[T]
- ✅ Arrays [T; N]
- ✅ VecDeque<T>
- ✅ LinkedList<T>
- ✅ HashSet<T>
- ✅ BTreeSet<T>
- ✅ HashMap<K, V> (queries values)
- ✅ BTreeMap<K, V> (queries values)
- ✅ Option<T>
- ✅ Result<T, E>

## Quick Reference

| Container | Direct Support | Pattern |
|-----------|---------------|---------|
| `Vec<T>` | ✅ Yes | `Query::new(&vec)` |
| `&[T]` | ✅ Yes | `Query::new(slice)` |
| `[T; N]` | ✅ Yes | `Query::new(&array)` |
| `VecDeque<T>` | ⚠️ Convert | `vec.make_contiguous()` or `.iter().cloned().collect()` |
| `LinkedList<T>` | ⚠️ Convert | `.iter().cloned().collect::<Vec<_>>()` |
| `HashSet<T>` | ⚠️ Convert | `.iter().cloned().collect::<Vec<_>>()` |
| `BTreeSet<T>` | ⚠️ Convert | `.iter().cloned().collect::<Vec<_>>()` |
| `HashMap<K,V>` | ⚠️ Convert | `.values().cloned().collect::<Vec<_>>()` |
| `BTreeMap<K,V>` | ⚠️ Convert | `.values().cloned().collect::<Vec<_>>()` |
| `Option<T>` | ⚠️ Convert | `std::slice::from_ref(&some)` |
| `Result<T,E>` | ⚠️ Convert | `result.iter().collect()` |

## Usage Examples

### Vec<T> - Direct Support

```rust
let products = vec![
    Product { id: 1, name: "Laptop".to_string(), price: 999.0, category: "Electronics".to_string() },
    Product { id: 2, name: "Mouse".to_string(), price: 29.0, category: "Electronics".to_string() },
];

// Direct support - most efficient
let query = Query::new(&products)
    .where_(Product::category_r(), |cat| cat == "Electronics");
let results = query.all();
```

### Slices &[T] - Direct Support

```rust
let array = [product1, product2, product3];
let slice = &array[1..3];

// Direct support
let query = Query::new(slice);
let count = query.count();
```

### Arrays [T; N] - Direct Support

```rust
let products = [
    Product { /* ... */ },
    Product { /* ... */ },
    Product { /* ... */ },
];

// Direct support
let query = Query::new(&products);
let total = query.sum(Product::price_r());
```

### VecDeque<T> - Zero-Copy Option

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(product1);
deque.push_back(product2);

// Option 1: Zero-copy (most efficient)
let slice = deque.make_contiguous();
let query = Query::new(slice);

// Option 2: Clone to Vec
let vec: Vec<Product> = deque.iter().cloned().collect();
let query = Query::new(&vec);
```

### HashSet<T> - Clone to Vec

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(product1);
set.insert(product2);

// Convert to Vec for querying
let vec: Vec<Product> = set.iter().cloned().collect();
let query = Query::new(&vec)
    .where_(Product::price_r(), |&p| p < 100.0);
let results = query.all();
```

### BTreeSet<T> - Clone to Vec (Maintains Order)

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(product1);
set.insert(product2);

// Convert to Vec (items will be in sorted order)
let vec: Vec<Product> = set.iter().cloned().collect();
let query = Query::new(&vec);

// Items are already sorted by Product's Ord implementation
```

### HashMap<K, V> - Query Values

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("prod1", product1);
map.insert("prod2", product2);

// Query just the values
let vec: Vec<Product> = map.values().cloned().collect();
let query = Query::new(&vec)
    .where_(Product::category_r(), |cat| cat == "Electronics");
let electronics = query.all();
```

### HashMap<K, V> - Query Key-Value Pairs

```rust
// If you need both keys and values
let pairs: Vec<(&str, &Product)> = map.iter()
    .map(|(k, v)| (*k, v))
    .collect();

// Now you can filter by key or value
let filtered: Vec<_> = pairs.iter()
    .filter(|(key, _)| key.starts_with("prod"))
    .filter(|(_, product)| product.price < 100.0)
    .collect();
```

### BTreeMap<K, V> - Query Values (Sorted)

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(1, product1);
map.insert(2, product2);

// Query values (iteration order is by sorted keys)
let vec: Vec<Product> = map.values().cloned().collect();
let query = Query::new(&vec);
let avg = query.avg(Product::price_r());
```

### LinkedList<T> - Clone to Vec

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(product1);
list.push_back(product2);

// Convert to Vec for querying
let vec: Vec<Product> = list.iter().cloned().collect();
let query = Query::new(&vec);
let count = query.count();
```

### Option<T> - Query Single Item

```rust
let maybe_product: Option<Product> = Some(product);

// Convert to slice
if let Some(ref product) = maybe_product {
    let query = Query::new(std::slice::from_ref(product));
    let count = query.count();  // Will be 1
}

// Or just use Option's methods
if let Some(product) = &maybe_product {
    if product.price < 100.0 {
        // ... process ...
    }
}
```

### Result<T, E> - Query Single Item

```rust
let result: Result<Product, Error> = Ok(product);

// Use Result's iterator
let count = result.iter().count();

// Or pattern match
if let Ok(product) = &result {
    // ... query product ...
}
```

## Performance Considerations

### Best Performance (Zero-Copy)

```rust
// ✅ Best: Direct slice access
let vec = vec![/* ... */];
Query::new(&vec)  // No cloning!

// ✅ Best: Array direct access
let array = [/* ... */];
Query::new(&array)  // No cloning!

// ✅ Good: VecDeque zero-copy
let mut deque = VecDeque::new();
let slice = deque.make_contiguous();
Query::new(slice)  // No cloning!
```

### Requires Cloning

```rust
// ⚠️ Requires Clone: Sets
let set: HashSet<Product> = /* ... */;
let vec: Vec<Product> = set.iter().cloned().collect();  // Clones all items
Query::new(&vec)

// ⚠️ Requires Clone: Maps
let map: HashMap<K, Product> = /* ... */;
let vec: Vec<Product> = map.values().cloned().collect();  // Clones all values
Query::new(&vec)

// ⚠️ Requires Clone: LinkedList
let list: LinkedList<Product> = /* ... */;
let vec: Vec<Product> = list.iter().cloned().collect();  // Clones all items
Query::new(&vec)
```

### Memory Trade-offs

| Container | Conversion | Memory Cost | Performance |
|-----------|------------|-------------|-------------|
| Vec | None | 0 | ✅ Best |
| Slice | None | 0 | ✅ Best |
| Array | None | 0 | ✅ Best |
| VecDeque | `.make_contiguous()` | 0 (in-place) | ✅ Excellent |
| HashSet | `.iter().cloned()` | 100% (full copy) | ⚠️ OK |
| BTreeSet | `.iter().cloned()` | 100% (full copy) | ⚠️ OK |
| HashMap | `.values().cloned()` | 100% (values) | ⚠️ OK |
| BTreeMap | `.values().cloned()` | 100% (values) | ⚠️ OK |
| LinkedList | `.iter().cloned()` | 100% (full copy) | ⚠️ OK |

## Queryable Trait

For custom container types, implement the `Queryable` trait:

```rust
use rust_queries_builder::Queryable;

struct MyCustomContainer<T> {
    items: Vec<T>,
    // ... other fields ...
}

impl<T> Queryable<T> for MyCustomContainer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// Now you can query it!
let container = MyCustomContainer { items: vec![/* ... */] };
let items: Vec<&T> = container.query_iter().collect();
let query = Query::new(&items);
```

## Best Practices

### 1. Use Direct Support When Possible

```rust
// ✅ Best: Use Vec or slice directly
let vec = vec![/* ... */];
Query::new(&vec)

// ❌ Avoid: Unnecessary conversions
let vec = vec![/* ... */];
let set: HashSet<_> = vec.into_iter().collect();
let back_to_vec: Vec<_> = set.iter().cloned().collect();
Query::new(&back_to_vec)  // Wasted work!
```

### 2. For Sets, Clone Once and Reuse

```rust
// ✅ Good: Clone once, use many times
let set: HashSet<Product> = /* ... */;
let vec: Vec<Product> = set.iter().cloned().collect();

let query1 = Query::new(&vec).where_(/* ... */);
let query2 = Query::new(&vec).where_(/* ... */);
let query3 = Query::new(&vec).where_(/* ... */);

// ❌ Bad: Clone multiple times
let query1 = Query::new(&set.iter().cloned().collect::<Vec<_>>());
let query2 = Query::new(&set.iter().cloned().collect::<Vec<_>>());
let query3 = Query::new(&set.iter().cloned().collect::<Vec<_>>());
```

### 3. Use VecDeque's make_contiguous() for Zero-Copy

```rust
//✅ Best: Zero-copy access
let mut deque = VecDeque::from(vec![/* ... */]);
let slice = deque.make_contiguous();
Query::new(slice)

// ❌ Less efficient: Cloning
let deque = VecDeque::from(vec![/* ... */]);
let vec: Vec<_> = deque.iter().cloned().collect();
Query::new(&vec)
```

### 4. Choose Right Container for Your Use Case

```rust
// If you need frequent querying:
// ✅ Use Vec<T> - best query performance
let products = vec![/* ... */];

// If you need uniqueness AND querying:
// ✅ Keep both Vec and HashSet
let products = vec![/* ... */];
let unique: HashSet<&Product> = products.iter().collect();
// Query the Vec, use HashSet for lookups
```

## Examples

Run the comprehensive demo:

```bash
cargo run --example container_support
```

This demonstrates querying all supported container types with practical examples.

## Summary

✅ **9+ container types supported**  
✅ **Zero-copy for Vec, slices, arrays, VecDeque**  
✅ **Simple conversion pattern for Sets and Maps**  
✅ **Queryable trait for custom containers**  
✅ **Type-safe across all containers**  

Choose the right container for your needs, and the query builder adapts!

