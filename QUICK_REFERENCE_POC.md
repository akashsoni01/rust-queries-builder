# Quick Reference for POC Development

## Your POC Setup

```toml
[dependencies]
rust-queries-core = "0.6.0"
rust-queries-derive = "0.6.0"
key-paths-core = "1.0.1"
key-paths-derive = "0.5.0"
```

## Correct Imports

```rust
// ✅ CORRECT IMPORTS
use rust_queries_core::{Query, QueryExt};  // QueryExt is HERE!
use rust_queries_derive::QueryBuilder;      // Derive macros
use key_paths_derive::Keypaths;             // Field access
```

## Quick Start Template

```rust
use rust_queries_core::{Query, QueryExt};
use rust_queries_derive::QueryBuilder;
use key_paths_derive::Keypaths;

#[derive(Debug, Clone, Keypaths, QueryBuilder)]
struct YourStruct {
    id: u32,
    name: String,
    value: f64,
}

fn main() {
    let items = vec![/* your data */];
    
    // Method 1: Extension trait
    let results = items
        .query()
        .where_(YourStruct::value_r(), |&v| v > 100.0)
        .all();
    
    // Method 2: QueryBuilder derive
    let count = YourStruct::query(&items)
        .where_(YourStruct::value_r(), |&v| v > 50.0)
        .count();
    
    // Method 3: Lazy queries
    let total: f64 = items
        .lazy_query()
        .sum_by(YourStruct::value_r());
}
```

## Common Operations

### Filtering
```rust
items.query().where_(YourStruct::field_r(), |&x| x > 10).all()
```

### Selecting
```rust
items.query().select(YourStruct::field_r())
```

### Aggregations
```rust
items.lazy_query().sum_by(YourStruct::field_r())
items.lazy_query().count()
items.lazy_query().avg_by(YourStruct::field_r())
```

### Early Termination
```rust
items.lazy_query().where_(...).first()
items.lazy_query().where_(...).any()
```

## Error Solutions

### Error: unresolved import `rust_queries_derive::QueryExt`

**Solution**: Change to `rust_queries_core::QueryExt`

### Error: no function `field_r` found

**Solution**: Add `#[derive(Keypaths)]` to your struct

### Error: trait `QueryExt` is not in scope

**Solution**: Add `use rust_queries_core::QueryExt;`

## Performance Tips

- Use `.lazy_query()` for large datasets
- Use `.first()` instead of `.all()[0]` for early termination
- Use `.any()` instead of `.count() > 0` for existence checks

## Full Documentation

- [Individual Crates Guide](INDIVIDUAL_CRATES_GUIDE.md) - Complete guide
- [Example Code](examples/individual_crates.rs) - Working example
- [README](README.md) - Full documentation

## Run Example

```bash
cargo run --example individual_crates
```

---

**Key Takeaway**: `QueryExt` is in `rust_queries_core`, not `rust_queries_derive`! 🎯

