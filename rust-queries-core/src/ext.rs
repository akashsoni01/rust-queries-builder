use crate::{Query, LazyQuery, Queryable};

/// Extension trait that adds eager Query methods directly to slice-like containers
/// 
/// This trait allows you to call eager query methods directly on containers.
/// For lazy queries, use the `QueryableExt::lazy_query()` method instead.
/// 
/// ```ignore
/// use rust_queries_builder::{QueryExt, QueryableExt};
/// 
/// let products = vec![...];
/// 
/// // Eager query (collects immediately)
/// let results = products.query().where_(...).all();
/// 
/// // Lazy query (deferred execution)
/// let results = products.lazy_query().where_(...).collect();
/// ```
pub trait QueryExt<T> {
    /// Create an eager Query from this container
    /// 
    /// This creates a Query that executes operations immediately when terminal
    /// methods like `all()`, `first()`, or `count()` are called.
    fn query(&self) -> Query<T>;
}

// Implementations for Vec
impl<T: 'static> QueryExt<T> for Vec<T> {
    fn query(&self) -> Query<T> {
        Query::new(self)
    }
}

// Implementations for slices
impl<T: 'static> QueryExt<T> for [T] {
    fn query(&self) -> Query<T> {
        Query::new(self)
    }
}

// Array implementations
impl<T: 'static, const N: usize> QueryExt<T> for [T; N] {
    fn query(&self) -> Query<T> {
        Query::new(&self[..])
    }
}

/// Extension trait for Queryable types that provides query building capabilities.
/// 
/// This trait extends any type implementing `Queryable<T>` with methods to create
/// `LazyQuery` instances directly from the container's iterator.
/// 
/// # Example
/// 
/// ```ignore
/// use rust_queries_builder::QueryableExt;
/// 
/// let map: HashMap<String, Product> = ...;
/// let results: Vec<_> = map.lazy_query()
///     .where_(Product::price_r(), |&p| p > 100.0)
///     .collect();
/// ```
pub trait QueryableExt<T> {
    /// Create a lazy query from this Queryable container
    /// 
    /// This returns a `LazyQuery` that can be used with all lazy query operations.
    /// The operations are executed lazily when a terminal operation is called.
    fn lazy_query(&self) -> LazyQuery<T, Box<dyn Iterator<Item = &T> + '_>>;
}

// Blanket implementation for all Queryable types
impl<T: 'static, Q> QueryableExt<T> for Q
where
    Q: Queryable<T>,
{
    fn lazy_query(&self) -> LazyQuery<T, Box<dyn Iterator<Item = &T> + '_>> {
        LazyQuery::from_iter(self.query_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use key_paths_derive::Keypaths;

    #[derive(Debug, Clone, PartialEq, Keypaths)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
        category: String,
    }

    #[test]
    fn test_vec_query_ext() {
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

        let query = products
            .query()
            .where_(Product::price_r(), |&p| p > 50.0);
        let results = query.all();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_vec_lazy_query_ext() {
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

        let results: Vec<_> = products
            .lazy_query()
            .where_(Product::price_r(), |&p| p > 50.0)
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_array_query_ext() {
        let products = [
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

        let results: Vec<_> = products
            .lazy_query()
            .where_(Product::price_r(), |&p| p < 100.0)
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mouse");
    }

    #[test]
    fn test_slice_query_ext() {
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

        let slice = &products[..];
        let query = slice
            .query()
            .where_(Product::category_r(), |cat| cat == "Electronics");
        let results = query.count();

        assert_eq!(results, 2);
    }

    #[test]
    fn test_hashmap_queryable_ext() {
        use std::collections::HashMap;

        let mut map: HashMap<u32, Product> = HashMap::new();
        map.insert(
            1,
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
            },
        );
        map.insert(
            2,
            Product {
                id: 2,
                name: "Mouse".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            },
        );

        // Using QueryableExt for HashMap
        let results: Vec<_> = map
            .lazy_query()
            .where_(Product::price_r(), |&p| p > 50.0)
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Laptop");
    }

    #[test]
    fn test_btreemap_queryable_ext() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<u32, Product> = BTreeMap::new();
        map.insert(
            1,
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
            },
        );
        map.insert(
            2,
            Product {
                id: 2,
                name: "Mouse".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            },
        );

        // Using QueryableExt for BTreeMap
        let count = map
            .lazy_query()
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .count();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_vecdeque_queryable_ext() {
        use std::collections::VecDeque;

        let mut deque: VecDeque<Product> = VecDeque::new();
        deque.push_back(Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
        });
        deque.push_back(Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
        });

        // Using QueryableExt for VecDeque
        let results: Vec<_> = deque
            .lazy_query()
            .where_(Product::price_r(), |&p| p < 100.0)
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mouse");
    }

    #[test]
    fn test_linkedlist_queryable_ext() {
        use std::collections::LinkedList;

        let mut list: LinkedList<Product> = LinkedList::new();
        list.push_back(Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
        });
        list.push_back(Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
        });

        // Using QueryableExt for LinkedList
        let first = list
            .lazy_query()
            .where_(Product::price_r(), |&p| p > 900.0)
            .first();

        assert!(first.is_some());
        assert_eq!(first.unwrap().name, "Laptop");
    }

    #[test]
    fn test_queryable_ext_aggregations() {
        use std::collections::VecDeque;

        let mut deque: VecDeque<Product> = VecDeque::new();
        deque.push_back(Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
        });
        deque.push_back(Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
        });
        deque.push_back(Product {
            id: 3,
            name: "Keyboard".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
        });

        // Test aggregation operations
        let total = deque.lazy_query().sum_by(Product::price_r());
        assert!((total - 1109.97).abs() < 0.01);

        let avg = deque.lazy_query().avg_by(Product::price_r()).unwrap();
        assert!((avg - 369.99).abs() < 0.01);

        let min = deque.lazy_query().min_by_float(Product::price_r()).unwrap();
        assert!((min - 29.99).abs() < 0.01);

        let max = deque.lazy_query().max_by_float(Product::price_r()).unwrap();
        assert!((max - 999.99).abs() < 0.01);
    }

    #[test]
    fn test_queryable_ext_chaining() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<u32, Product> = BTreeMap::new();
        map.insert(
            1,
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
            },
        );
        map.insert(
            2,
            Product {
                id: 2,
                name: "Mouse".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            },
        );
        map.insert(
            3,
            Product {
                id: 3,
                name: "Desk".to_string(),
                price: 299.99,
                category: "Furniture".to_string(),
            },
        );

        // Test complex chaining with multiple where clauses
        let results: Vec<_> = map
            .lazy_query()
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .where_(Product::price_r(), |&p| p < 500.0)
            .skip_lazy(0)
            .take_lazy(10)
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mouse");
    }
}
