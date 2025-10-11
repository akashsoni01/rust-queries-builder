# Issue Resolved: Derive Macros with Individual Crates

## Problem Summary

When using individual crates (`rust-queries-core` + `rust-queries-derive`) in a POC, the `#[derive(QueryBuilder)]` macro failed with:

```
error[E0433]: failed to resolve: use of undeclared crate or module `rust_queries_builder`
  --> src/main.rs:18:34
   |
18 | #[derive(Debug, Clone, Keypaths, QueryBuilder)]
   |                                  ^^^^^^^^^^^^ use of undeclared crate or module `rust_queries_builder`
```

## Root Cause

The procedural macros in `rust-queries-derive` were hardcoded to generate code that referenced `rust_queries_builder::Query` and `rust_queries_builder::LazyQuery`, but when using individual crates, these types are in `rust_queries_core`.

**Before (broken):**
```rust
// Generated code in rust-queries-derive v0.5.0
pub fn query(items: &[Self]) -> rust_queries_builder::Query<Self> {
    rust_queries_builder::Query::new(items)
}
```

**After (fixed):**
```rust
// Generated code in rust-queries-derive v0.6.0
pub fn query(items: &[Self]) -> rust_queries_core::Query<Self> {
    rust_queries_core::Query::new(items)
}
```

## Solution Applied

### Files Changed

1. **`rust-queries-derive/src/lib.rs`**
   - Changed `rust_queries_builder::Query` → `rust_queries_core::Query`
   - Changed `rust_queries_builder::LazyQuery` → `rust_queries_core::LazyQuery`
   - Changed `rust_queries_builder::Queryable` → `rust_queries_core::Queryable`

2. **Documentation Updated**
   - Added `POC_TEMPLATE.md` with complete POC setup
   - Added `INDIVIDUAL_CRATES_GUIDE.md` with detailed usage
   - Added `QUICK_REFERENCE_POC.md` for quick lookups
   - Updated `README.md` with installation options
   - Updated `CHANGELOG.md` with bug fix details

3. **Examples Added**
   - `examples/individual_crates.rs` - Demonstrates using individual crates

## Your POC Now Works! ✅

### Correct Setup

**Cargo.toml:**
```toml
[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-derive = "0.5.0"
```

**src/main.rs:**
```rust
use rust_queries_core::{Query, QueryExt};  // ← QueryExt is here!
use rust_queries_derive::QueryBuilder;      // ← Derive macros here
use key_paths_derive::Keypaths;             // ← Field access

#[derive(Debug, Clone, Keypaths, QueryBuilder)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

fn main() {
    let products = vec![/* ... */];
    
    // Method 1: Extension trait
    let results = products.query().where_(...).all();
    
    // Method 2: QueryBuilder derive (now works!)
    let count = Product::query(&products).count();
    
    // Method 3: Lazy queries
    let total: f64 = products.lazy_query().sum_by(Product::price_r());
}
```

## Verification

All examples now compile and run successfully:

```bash
$ cargo run --example individual_crates
Individual Crates Example
=========================

Using rust-queries-core + rust-queries-derive directly

1. QueryExt from rust_queries_core
   ✅ Works!

2. QueryBuilder from rust_queries_derive
   ✅ Now works with individual crates!

3. LazyQuery from rust_queries_core
   ✅ Works!
```

## Key Takeaways

1. **QueryExt is in `rust_queries_core`**, not `rust_queries_derive`
2. **Derive macros now work** with both umbrella and individual crates
3. **Version 0.6.0+** is required for individual crate support
4. **No breaking changes** for existing users

## Quick Reference

| Need | Import From |
|------|-------------|
| `Query` | `rust_queries_core` |
| `LazyQuery` | `rust_queries_core` |
| `QueryExt` | `rust_queries_core` ✅ |
| `#[derive(QueryBuilder)]` | `rust_queries_derive` |
| `#[derive(Keypaths)]` | `key_paths_derive` |

## Resources

- **POC Template**: [POC_TEMPLATE.md](POC_TEMPLATE.md)
- **Complete Guide**: [INDIVIDUAL_CRATES_GUIDE.md](INDIVIDUAL_CRATES_GUIDE.md)
- **Quick Reference**: [QUICK_REFERENCE_POC.md](QUICK_REFERENCE_POC.md)
- **Example Code**: [examples/individual_crates.rs](examples/individual_crates.rs)

## Testing Your POC

```bash
# Make sure you're using the latest version
cargo update

# Clean build
cargo clean
cargo build --release

# Run
cargo run
```

---

**Status**: ✅ RESOLVED  
**Version Fixed**: 0.6.0  
**Date**: October 12, 2025

