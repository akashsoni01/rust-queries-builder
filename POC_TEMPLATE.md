# POC Template - Using Individual Crates

## Setup Your POC Project

### 1. Create New Project

```bash
cargo new my_poc
cd my_poc
```

### 2. Update Cargo.toml

```toml
[package]
name = "my_poc"
version = "0.1.0"
edition = "2021"

[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-derive = "0.5.0"
```

### 3. Create src/main.rs

```rust
use rust_queries_core::{Query, QueryExt};
use rust_queries_derive::QueryBuilder;
use key_paths_derive::Keypaths;

#[derive(Debug, Clone, Keypaths, QueryBuilder)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
}

fn main() {
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
        Product {
            id: 3,
            name: "Keyboard".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
        },
    ];

    println!("=== Query Builder POC ===\n");

    // Method 1: Using QueryExt trait (extension methods)
    println!("1. Extension trait - products.query()");
    let query = products
        .query()
        .where_(Product::price_r(), |&p| p > 50.0);
    let results = query.all();
    println!("   Found {} products over $50", results.len());
    for product in results {
        println!("   - {} (${:.2})", product.name, product.price);
    }
    println!();

    // Method 2: Using QueryBuilder derive (static methods)
    println!("2. QueryBuilder derive - Product::query()");
    let query2 = Product::query(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let electronics = query2.all();
    println!("   Found {} electronics", electronics.len());
    for product in electronics {
        println!("   - {}", product.name);
    }
    println!();

    // Method 3: Using lazy queries (best performance)
    println!("3. Lazy query - products.lazy_query()");
    let total: f64 = products
        .lazy_query()
        .sum_by(Product::price_r());
    println!("   Total inventory value: ${:.2}", total);
    println!();

    // Method 4: Early termination
    println!("4. Early termination - first()");
    if let Some(first) = products
        .lazy_query()
        .where_(Product::price_r(), |&p| p < 100.0)
        .first()
    {
        println!("   First affordable product: {} (${:.2})", first.name, first.price);
    }
    println!();

    // Method 5: Aggregations
    println!("5. Aggregations");
    let count = products.lazy_query().count();
    let avg_price = products.lazy_query().avg_by(Product::price_r());
    println!("   Total products: {}", count);
    println!("   Average price: ${:.2}", avg_price);
}
```

### 4. Run

```bash
cargo run
```

## Expected Output

```
=== Query Builder POC ===

1. Extension trait - products.query()
   Found 2 products over $50
   - Laptop ($999.99)
   - Keyboard ($79.99)

2. QueryBuilder derive - Product::query()
   Found 3 electronics
   - Laptop
   - Mouse
   - Keyboard

3. Lazy query - products.lazy_query()
   Total inventory value: $1109.97

4. Early termination - first()
   First affordable product: Mouse ($29.99)

5. Aggregations
   Total products: 3
   Average price: $369.99
```

## Troubleshooting

### Error: unresolved import `rust_queries_derive::QueryExt`

**Problem**: You're importing from the wrong crate.

**Solution**:
```rust
// ❌ WRONG
use rust_queries_derive::QueryExt;

// ✅ CORRECT
use rust_queries_core::QueryExt;
```

### Error: use of undeclared crate `rust_queries_builder`

**Problem**: You're using an old version of `rust-queries-derive` (< 0.6.0).

**Solution**: Update to version 0.6.0 or later:
```toml
rust-queries-derive = "0.6.0"  # Must be 0.6.0 or later
```

### Error: no function `field_r` found

**Problem**: Missing `#[derive(Keypaths)]`.

**Solution**: Add it to your struct:
```rust
#[derive(Debug, Clone, Keypaths)]  // Add Keypaths
struct MyStruct {
    field: f64,
}
```

### Error: trait `QueryExt` is not in scope

**Problem**: Missing import.

**Solution**:
```rust
use rust_queries_core::QueryExt;
```

## Common Patterns

### Filtering

```rust
// Single condition
products.query().where_(Product::price_r(), |&p| p > 100.0).all()

// Multiple conditions
products.query()
    .where_(Product::price_r(), |&p| p > 50.0)
    .where_(Product::category_r(), |cat| cat == "Electronics")
    .all()
```

### Aggregations

```rust
// Count
products.lazy_query().count()

// Sum
products.lazy_query().sum_by(Product::price_r())

// Average
products.lazy_query().avg_by(Product::price_r())

// Min/Max
products.lazy_query().min_by_float(Product::price_r())
products.lazy_query().max_by_float(Product::price_r())
```

### Early Termination

```rust
// First match
products.lazy_query().where_(...).first()

// Any match
products.lazy_query().where_(...).any()

// Find
products.lazy_query().find(|p| p.name.contains("Laptop"))
```

### Selecting Fields

```rust
// Get all names
let names: Vec<String> = products.query().select(Product::name_r());

// Lazy selection
let names: Vec<String> = products.lazy_query().select_lazy(Product::name_r()).collect();
```

### Pagination

```rust
// Skip and take
products.query().skip(10).limit(5)

// Lazy pagination
products.lazy_query().skip_lazy(10).take_lazy(5).collect()
```

## Performance Tips

1. **Use lazy queries for large datasets**
   ```rust
   items.lazy_query()  // Deferred execution
   ```

2. **Use early termination when possible**
   ```rust
   items.lazy_query().first()  // Stops at first match
   ```

3. **Use `.any()` instead of `.count() > 0`**
   ```rust
   items.lazy_query().any()  // Stops at first match
   ```

4. **Chain filters efficiently**
   ```rust
   items.lazy_query()
       .where_(...)  // Most selective filter first
       .where_(...)
       .collect()
   ```

## Next Steps

- Read the [Individual Crates Guide](INDIVIDUAL_CRATES_GUIDE.md) for complete documentation
- Check out [examples/individual_crates.rs](examples/individual_crates.rs) for more patterns
- See the [Quick Reference](QUICK_REFERENCE_POC.md) for a cheat sheet

## Need Help?

Common issues and solutions:
- **Import errors**: Check you're using `rust_queries_core` not `rust_queries_derive`
- **Version issues**: Make sure you're using `rust-queries-derive >= 0.6.0`
- **Build errors**: Run `cargo clean && cargo build`

---

**Note**: Version 0.6.0+ of `rust-queries-derive` now correctly references `rust_queries_core` instead of `rust_queries_builder`, so the derive macros work with individual crates!

