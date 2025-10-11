# Extension Trait and Derive Macros Guide

This guide explains how to use the extension trait and derive macros to simplify query syntax.

## Table of Contents

1. [Extension Trait (`QueryExt`)](#extension-trait)
2. [Derive Macros](#derive-macros)
3. [Comparison](#comparison)
4. [Examples](#examples)

## Extension Trait

The `QueryExt` trait adds `.query()` and `.lazy_query()` methods directly to containers.

### Supported Containers

- `Vec<T>`
- `&[T]` (slices)
- `[T; N]` (arrays)

### Usage

```rust
use rust_queries_builder::QueryExt;
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths)]
struct Product {
    name: String,
    price: f64,
}

let products = vec![
    Product { name: "Laptop".to_string(), price: 999.99 },
    Product { name: "Mouse".to_string(), price: 29.99 },
];

// Old way
let query = Query::new(&products).where_(...);

// New way with extension trait
let query = products.query().where_(...);
```

### Methods

#### `.query()` - Eager Query

Creates an eager `Query` that processes immediately:

```rust
let expensive = products
    .query()
    .where_(Product::price_r(), |&p| p > 100.0)
    .all();
```

**Note**: Store the query in a `let` binding before calling terminal operations to avoid lifetime issues:

```rust
let query = products.query().where_(Product::price_r(), |&p| p > 100.0);
let results = query.all();
```

#### `.lazy_query()` - Lazy Query

Creates a lazy `LazyQuery` with deferred execution:

```rust
let results: Vec<_> = products
    .lazy_query()
    .where_(Product::price_r(), |&p| p < 100.0)
    .collect();
```

## Derive Macros

### `#[derive(QueryBuilder)]`

Generates static helper methods on your struct:

```rust
use rust_queries_builder::QueryBuilder;
use key_paths_derive::Keypaths;

#[derive(Clone, Keypaths, QueryBuilder)]
struct Product {
    name: String,
    price: f64,
}

// Static methods are now available:
let query = Product::query(&products);
let lazy = Product::lazy_query(&products);
```

### Generated Methods

```rust
impl Product {
    /// Creates a new eager Query from a slice of items
    pub fn query(items: &[Self]) -> Query<Self> {
        Query::new(items)
    }

    /// Creates a new lazy Query from a slice of items
    pub fn lazy_query(items: &[Self]) -> LazyQuery<Self, impl Iterator<Item = &Self>> {
        LazyQuery::new(items)
    }
}
```

## Comparison

### Traditional Approach

```rust
use rust_queries_builder::Query;

let query = Query::new(&products)
    .where_(Product::price_r(), |&p| p > 100.0);
let results = query.all();
```

### Extension Trait Approach

```rust
use rust_queries_builder::QueryExt;

let query = products
    .query()
    .where_(Product::price_r(), |&p| p > 100.0);
let results = query.all();
```

### Derive Macro Approach

```rust
use rust_queries_builder::QueryBuilder;

#[derive(QueryBuilder)]
struct Product { /* ... */ }

let query = Product::query(&products)
    .where_(Product::price_r(), |&p| p > 100.0);
let results = query.all();
```

## Examples

### Example 1: Eager Query with Extension Trait

```rust
use rust_queries_builder::QueryExt;
use key_paths_derive::Keypaths;

#[derive(Clone, Debug, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
}

let products = vec![
    Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 999.99,
        category: "Electronics".to_string(),
    },
    Product {
        id: 2,
        name: "Mouse".to_string(),
        price: 29.99,
        category: "Electronics".to_string(),
    },
];

// Filter expensive products
let query = products
    .query()
    .where_(Product::price_r(), |&p| p > 100.0);
let expensive = query.all();

println!("Expensive products: {}", expensive.len());
```

### Example 2: Lazy Query with Extension Trait

```rust
use rust_queries_builder::QueryExt;

// Early termination - stops as soon as condition is met
let first_cheap = products
    .lazy_query()
    .where_(Product::price_r(), |&p| p < 50.0)
    .first();

// Pagination
let page: Vec<_> = products
    .lazy_query()
    .skip_lazy(10)
    .take_lazy(5)
    .collect();

// Aggregation
let total_stock = products
    .lazy_query()
    .sum_by(Product::stock_r());
```

### Example 3: Chained Operations

```rust
use rust_queries_builder::QueryExt;

let names: Vec<String> = products
    .lazy_query()
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .where_(Product::price_r(), |&p| p < 500.0)
    .take_lazy(3)
    .select_lazy(Product::name_r())
    .collect();

println!("Affordable electronics: {:?}", names);
```

### Example 4: QueryBuilder Derive

```rust
use rust_queries_builder::QueryBuilder;
use key_paths_derive::Keypaths;

#[derive(Clone, Debug, Keypaths, QueryBuilder)]
struct Product {
    name: String,
    price: f64,
}

let products = vec![/* ... */];

// Use static methods
let count = Product::query(&products)
    .where_(Product::price_r(), |&p| p > 50.0)
    .count();

let first = Product::lazy_query(&products).first();
```

### Example 5: Slice Extension

```rust
use rust_queries_builder::QueryExt;

let products = vec![/* ... */];
let slice = &products[1..4];

// Works on slices too!
let results: Vec<_> = slice
    .lazy_query()
    .where_(Product::price_r(), |&p| p < 100.0)
    .collect();
```

### Example 6: Array Extension

```rust
use rust_queries_builder::QueryExt;

let products = [
    Product { /* ... */ },
    Product { /* ... */ },
];

// Works on arrays!
let query = products.query();
let results = query.all();
```

## Lifetime Considerations

When using `.query()` (eager), store the query in a `let` binding before calling terminal operations:

```rust
// ❌ Lifetime error - temporary value dropped
let results = products.query().where_(...).all();

// ✅ Correct - query lives long enough
let query = products.query().where_(...);
let results = query.all();
```

For `.lazy_query()`, this isn't usually necessary since most operations consume the query:

```rust
// ✅ Works fine - collect() consumes the query
let results: Vec<_> = products.lazy_query().where_(...).collect();
```

## Performance

The extension trait has **zero overhead**:

- `.query()` directly calls `Query::new()`
- `.lazy_query()` directly calls `LazyQuery::new()`
- All compiler optimizations (inlining, iterator fusion) still apply

## When to Use

### Use Extension Trait When:

- You want more ergonomic syntax
- You're working with `Vec`, slices, or arrays
- You prefer method chaining style

### Use Traditional `Query::new()` When:

- You need explicit control
- Working with custom `Queryable` types
- You prefer functional style

### Use `QueryBuilder` Derive When:

- You want namespaced query methods on your types
- You prefer static methods over extensions
- You want to encapsulate query logic with your type

## See Also

- [Main README](README.md) for general query builder usage
- [Lazy Evaluation Guide](LAZY_EVALUATION.md) for lazy query details
- [Container Support Guide](CONTAINER_SUPPORT.md) for custom containers
- [Examples](examples/) for more usage patterns

