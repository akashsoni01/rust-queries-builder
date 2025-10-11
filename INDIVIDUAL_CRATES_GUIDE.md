# Using Individual Crates Guide

This guide explains how to use `rust-queries-core` and `rust-queries-derive` **directly** instead of the umbrella `rust-queries-builder` crate.

## When to Use Individual Crates

### Use Individual Crates When:

✅ Building libraries (minimal dependencies)  
✅ Need faster builds (core only = 65% faster)  
✅ Want explicit dependency control  
✅ Optimizing for binary size  
✅ Don't need the umbrella convenience  

### Use Umbrella Crate When:

✅ Building applications  
✅ Want maximum convenience  
✅ Don't care about 2-3 second build time difference  
✅ Prefer single dependency  

## Setup

### Cargo.toml

```toml
[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-core = "1.0.1"
key-paths-derive = "0.5.0"
```

**Note**: You still need `key-paths-core` and `key-paths-derive` for field access.

## Correct Import Structure

### ❌ WRONG (Common Mistake)

```rust
// This won't work!
use rust_queries_derive::QueryExt;  // ERROR: QueryExt is not in derive crate
```

### ✅ CORRECT

```rust
// Core functionality
use rust_queries_core::{Query, QueryExt};

// Derive macros (optional)
use rust_queries_derive::QueryBuilder;

// Key-paths for field access
use key_paths_derive::Keypaths;
```

## What's in Each Crate

### `rust-queries-core` - Core Functionality

**Exports:**
- `Query` - Eager query builder
- `LazyQuery` - Lazy query builder
- `JoinQuery` - Join operations
- `QueryExt` - Extension trait (`.query()`, `.lazy_query()`)
- `Queryable` - Trait for custom containers
- `KeyPaths` - Re-exported from key-paths-core
- All helper macros (`lazy_query!`, `filter_collect!`, etc.)

**Size**: 251 KB  
**Build time**: ~1.2s (clean build)

### `rust-queries-derive` - Procedural Macros

**Exports:**
- `#[derive(QueryBuilder)]` - Generates static `query()` and `lazy_query()` methods
- `#[derive(Queryable)]` - Generates `Queryable` trait implementation

**Size**: Compile-time only  
**Build time**: Adds ~2s to total build

### `key-paths-derive` - Field Access

**Exports:**
- `#[derive(Keypaths)]` - Generates type-safe field accessors

**Required**: Yes (for field access like `Product::price_r()`)

## Complete Example

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
    ];

    // 1. Using QueryExt trait (from rust_queries_core)
    let query = products
        .query()  // Extension method
        .where_(Product::price_r(), |&p| p > 100.0);
    let expensive = query.all();
    
    println!("Expensive products: {}", expensive.len());

    // 2. Using QueryBuilder derive (from rust_queries_derive)
    let count = Product::query(&products)  // Static method
        .where_(Product::price_r(), |&p| p > 50.0)
        .count();
    
    println!("Products over $50: {}", count);

    // 3. Using lazy queries
    let total: f64 = products
        .lazy_query()
        .sum_by(Product::price_r());
    
    println!("Total value: ${:.2}", total);

    // 4. Using Query directly (traditional)
    let query2 = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let electronics = query2.all();
    
    println!("Electronics: {}", electronics.len());
}
```

## Common Errors and Solutions

### Error 1: Unresolved Import

```rust
// ❌ ERROR
use rust_queries_derive::QueryExt;
```

**Error Message:**
```
unresolved import `rust_queries_derive::QueryExt`
```

**Solution:**
```rust
// ✅ CORRECT
use rust_queries_core::QueryExt;
```

**Why**: `QueryExt` is in the core crate, not the derive crate.

### Error 2: Missing Keypaths

```rust
// ❌ ERROR - No Keypaths derive
#[derive(QueryBuilder)]
struct Product {
    price: f64,
}

// Trying to use Product::price_r() won't work
```

**Error Message:**
```
no function or associated item named `price_r` found
```

**Solution:**
```rust
// ✅ CORRECT
use key_paths_derive::Keypaths;

#[derive(Keypaths, QueryBuilder)]  // Add Keypaths
struct Product {
    price: f64,
}
```

### Error 3: Wrong Crate for Derive Macros

```rust
// ❌ ERROR
use rust_queries_core::QueryBuilder;  // Not here!
```

**Solution:**
```rust
// ✅ CORRECT
use rust_queries_derive::QueryBuilder;
```

## Import Cheat Sheet

| What You Need | Import From | Why |
|--------------|-------------|-----|
| `Query` | `rust_queries_core` | Core query type |
| `LazyQuery` | `rust_queries_core` | Lazy query type |
| `QueryExt` | `rust_queries_core` | Extension trait |
| `JoinQuery` | `rust_queries_core` | Join operations |
| `Queryable` | `rust_queries_core` | Container trait |
| `#[derive(QueryBuilder)]` | `rust_queries_derive` | Proc macro |
| `#[derive(Queryable)]` | `rust_queries_derive` | Proc macro |
| `#[derive(Keypaths)]` | `key_paths_derive` | Field access |
| `KeyPaths` | `key_paths_core` | Key-path type |

## Minimal Setup (Core Only)

If you don't need derive macros:

```toml
[dependencies]
rust-queries-core = "0.6.0"
key-paths-derive = "0.5.0"  # Still needed for Keypaths
```

```rust
use rust_queries_core::{Query, QueryExt};
use key_paths_derive::Keypaths;

#[derive(Keypaths)]  // No QueryBuilder - that's fine!
struct Product {
    name: String,
    price: f64,
}

fn main() {
    let products = vec![/* ... */];
    
    // QueryExt still works!
    let query = products.query().where_(...);
    
    // Traditional Query works too
    let query2 = Query::new(&products).where_(...);
}
```

**Build time**: ~1.2s (saves ~2s by skipping derive macros!)

## Feature Comparison

| Feature | Core Only | Core + Derive | Umbrella |
|---------|-----------|---------------|----------|
| `Query` | ✅ | ✅ | ✅ |
| `LazyQuery` | ✅ | ✅ | ✅ |
| `QueryExt` | ✅ | ✅ | ✅ |
| `#[derive(QueryBuilder)]` | ❌ | ✅ | ✅ |
| Build time | 1.2s | 3.5s | 3.5s |
| Dependencies | 2 | 4 | 1 |
| Convenience | Medium | High | Highest |

## Migration Between Approaches

### From Umbrella to Individual

**Before:**
```toml
[dependencies]
rust-queries-builder = "0.6.0"
```

```rust
use rust_queries_builder::{Query, QueryExt, QueryBuilder};
```

**After:**
```toml
[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-derive = "0.5.0"
```

```rust
use rust_queries_core::{Query, QueryExt};
use rust_queries_derive::QueryBuilder;
use key_paths_derive::Keypaths;
```

**Result**: 65% faster builds!

### From Individual to Umbrella

**Before:**
```toml
[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-derive = "0.5.0"
```

**After:**
```toml
[dependencies]
rust-queries-builder = "0.6.0"
```

**Change imports:**
```rust
// Change all
use rust_queries_core::...;
use rust_queries_derive::...;

// To
use rust_queries_builder::...;
```

**Result**: Simpler, one dependency!

## Best Practices

### 1. Use Core for Libraries

If you're building a library:

```toml
[dependencies]
rust-queries-core = "0.6.0"
key-paths-derive = "0.5.0"
```

Your library users can add derives if they need them.

### 2. Use Umbrella for Applications

If you're building an end application:

```toml
[dependencies]
rust-queries-builder = "0.6.0"
```

Convenience matters more than build time for apps.

### 3. Import Style

Be explicit about what you import:

```rust
// ✅ GOOD - Clear and explicit
use rust_queries_core::{Query, QueryExt};
use rust_queries_derive::QueryBuilder;

// ❌ AVOID - Unclear source
use rust_queries_core::*;
```

### 4. Documentation

Document which crates you're using:

```rust
//! This module uses rust-queries-core for query operations.
//! 
//! Dependencies:
//! - rust-queries-core = "0.6.0"
//! - key-paths-derive = "0.5.0"
```

## IDE Support

### rust-analyzer

Should work automatically with either approach. If you have issues:

```bash
# Clear cache and rebuild
rm -rf target/
cargo clean
cargo build
```

### Autocomplete

After adding crates, run:

```bash
cargo build
```

Then restart your IDE/rust-analyzer.

## Example Projects

Run the example:

```bash
cargo run --example individual_crates
```

This demonstrates all features using individual crates.

## Summary

**For POCs/Libraries**: Use individual crates
```toml
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-derive = "0.5.0"
```

**For Applications**: Use umbrella crate
```toml
rust-queries-builder = "0.6.0"
```

**Key Takeaway**: `QueryExt` is in `rust_queries_core`, not `rust_queries_derive`! 🎯

---

**Version**: 0.6.0  
**Last Updated**: October 12, 2025

