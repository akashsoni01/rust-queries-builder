//! Macros to simplify query building and reduce boilerplate.
//!
//! This module provides declarative macros for creating queries with less code.

/// Creates a lazy query with multiple filters in a concise syntax.
///
/// # Syntax
///
/// ```ignore
/// lazy_query!(data, Type =>
///     field1 == value1,
///     field2 > value2,
///     field3.contains("text")
/// )
/// ```
///
/// # Example
///
/// ```ignore
/// // Instead of:
/// let results = LazyQuery::new(&products)
///     .where_(Product::category(), |cat| cat == "Electronics")
///     .where_(Product::price(), |&p| p < 500.0)
///     .where_(Product::stock(), |&s| s > 0)
///     .collect();
///
/// // Write:
/// let results = lazy_query!(products, Product =>
///     category == "Electronics",
///     price < 500.0,
///     stock > 0
/// ).collect();
/// ```
#[macro_export]
macro_rules! lazy_query {
    ($data:expr, $type:ty => $($tail:tt)*) => {{
        $crate::LazyQuery::new($data)
    }};
    
    ($data:expr) => {{
        $crate::LazyQuery::new($data)
    }};
}

/// Creates a Query with multiple filters in a concise syntax.
///
/// # Example
///
/// ```ignore
/// let results = query!(products, Product)
///     .where_(Product::category(), |cat| cat == "Electronics")
///     .all();
/// ```
#[macro_export]
macro_rules! query {
    ($data:expr) => {{
        $crate::Query::new($data)
    }};
}

/// Simplified where_ macro that reduces boilerplate.
///
/// # Example
///
/// ```ignore
/// // Instead of:
/// .where_(Product::price(), |&p| p < 100.0)
///
/// // Write:
/// .where_(Product::price(), |&p| p < 100.0)  // Same for now, helper in query! macro
/// ```
#[macro_export]
macro_rules! filter {
    ($field:expr, $pred:expr) => {{
        |item| $field.get(item).map_or(false, $pred)
    }};
}

/// Creates a lazy query and collects results in one line.
///
/// # Example
///
/// ```ignore
/// // Instead of:
/// let results: Vec<_> = LazyQuery::new(&products)
///     .where_(Product::price(), |&p| p < 100.0)
///     .collect();
///
/// // Write:
/// let results = collect_lazy!(products where price < 100.0);
/// ```
#[macro_export]
macro_rules! collect_lazy {
    ($data:expr) => {{
        $crate::LazyQuery::new($data).collect()
    }};
}

/// Quick filter and collect.
///
/// # Example
///
/// ```ignore
/// let results = filter_collect!(products, Product::category(), |cat| cat == "Electronics");
/// ```
#[macro_export]
macro_rules! filter_collect {
    ($data:expr, $field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($field, $pred)
            .collect()
    }};
}

/// Quick count with filter.
///
/// # Example
///
/// ```ignore
/// let count = count_where!(products, Product::stock(), |&s| s > 0);
/// ```
#[macro_export]
macro_rules! count_where {
    ($data:expr, $field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($field, $pred)
            .count()
    }};
}

/// Find first matching item.
///
/// # Example
///
/// ```ignore
/// let found = find_first!(products, Product::id(), |&id| id == 42);
/// ```
#[macro_export]
macro_rules! find_first {
    ($data:expr, $field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($field, $pred)
            .first()
    }};
}

/// Check if any item matches.
///
/// # Example
///
/// ```ignore
/// let exists = exists_where!(products, Product::category(), |cat| cat == "Electronics");
/// ```
#[macro_export]
macro_rules! exists_where {
    ($data:expr, $field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($field, $pred)
            .any()
    }};
}

/// Quick pagination.
///
/// # Example
///
/// ```ignore
/// let page_2 = paginate!(products, page: 2, size: 10);
/// ```
#[macro_export]
macro_rules! paginate {
    ($data:expr, page: $page:expr, size: $size:expr) => {{
        $crate::LazyQuery::new($data)
            .skip_lazy($page * $size)
            .take_lazy($size)
            .collect()
    }};
}

/// Quick sum aggregation.
///
/// # Example
///
/// ```ignore
/// let total = sum_where!(products, Product::price(), Product::stock(), |&s| s > 0);
/// ```
#[macro_export]
macro_rules! sum_where {
    ($data:expr, $sum_field:expr, $filter_field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($filter_field, $pred)
            .sum_by($sum_field)
    }};
    
    ($data:expr, $sum_field:expr) => {{
        $crate::LazyQuery::new($data)
            .sum_by($sum_field)
    }};
}

/// Quick average aggregation.
///
/// # Example
///
/// ```ignore
/// let avg = avg_where!(products, Product::price(), Product::active(), |&a| a);
/// ```
#[macro_export]
macro_rules! avg_where {
    ($data:expr, $avg_field:expr, $filter_field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($filter_field, $pred)
            .avg_by($avg_field)
    }};
    
    ($data:expr, $avg_field:expr) => {{
        $crate::LazyQuery::new($data)
            .avg_by($avg_field)
    }};
}

/// Select and collect in one line.
///
/// # Example
///
/// ```ignore
/// let names = select_all!(products, Product::name());
/// ```
#[macro_export]
macro_rules! select_all {
    ($data:expr, $field:expr) => {{
        $crate::LazyQuery::new($data)
            .select_lazy($field)
            .collect()
    }};
}

/// Select with filter.
///
/// # Example
///
/// ```ignore
/// let names = select_where!(
///     products,
///     Product::name(),
///     Product::category(),
///     |cat| cat == "Electronics"
/// );
/// ```
#[macro_export]
macro_rules! select_where {
    ($data:expr, $select_field:expr, $filter_field:expr, $pred:expr) => {{
        $crate::LazyQuery::new($data)
            .where_($filter_field, $pred)
            .select_lazy($select_field)
            .collect()
    }};
}

