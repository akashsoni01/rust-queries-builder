# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-11

### Added

#### Query Operations
- `Query::new()` - Create new query from data slice
- `where_()` - Filter data using type-safe predicates
- `all()` - Get all matching items
- `first()` - Get first matching item
- `count()` - Count matching items
- `limit()` - Limit number of results
- `skip()` - Skip items for pagination
- `exists()` - Check if any items match

#### Ordering
- `order_by()` - Sort by Ord field (ascending)
- `order_by_desc()` - Sort by Ord field (descending)
- `order_by_float()` - Sort by f64 field (ascending)
- `order_by_float_desc()` - Sort by f64 field (descending)

#### Projection
- `select()` - Project/select specific fields from results

#### Grouping
- `group_by()` - Group items by field value

#### Aggregations
- `sum()` - Sum numeric field
- `avg()` - Average of f64 field
- `min()` / `max()` - Min/max of Ord field
- `min_float()` / `max_float()` - Min/max of f64 field

#### Join Operations
- `JoinQuery::new()` - Create new join query
- `inner_join()` - Inner join between collections
- `left_join()` - Left join with optional right matches
- `right_join()` - Right join with optional left matches
- `inner_join_where()` - Inner join with additional predicate
- `cross_join()` - Cartesian product

#### Examples
- `advanced_query_builder` - Comprehensive query operations example
- `join_query_builder` - Join operations example
- `sql_comparison` - SQL vs Rust Query Builder comparison with 15 examples

#### Documentation
- Comprehensive README.md
- Detailed USAGE.md guide
- Inline API documentation

### Performance
- O(n) filtering operations
- O(n log n) sorting operations
- O(n + m) hash-based joins
- Zero-cost abstractions

### Type Safety
- Compile-time type checking via key-paths
- Type-safe field access
- No runtime type errors

## [Unreleased]

### Planned Features
- Async query support
- Query optimization
- Index-based operations
- More join types (FULL OUTER JOIN)
- Query builder macros
- Database backend adapters
- Query caching
- Batch operations

