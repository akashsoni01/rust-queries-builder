use crate::{Query, LazyQuery};

/// Extension trait that adds query methods directly to containers
/// 
/// This trait allows you to call query methods directly on containers:
/// 
/// ```ignore
/// use rust_queries_builder::QueryExt;
/// 
/// let products = vec![...];
/// let results = products.query().where_(...).all();
/// let results = products.lazy_query().where_(...).collect();
/// ```
pub trait QueryExt<T> {
    /// Create an eager Query from this container
    fn query(&self) -> Query<T>;
    
    /// Create a lazy Query from this container
    /// 
    /// This method returns an opaque iterator type that implements all lazy query methods.
    /// The iterator borrows from the container.
    fn lazy_query(&self) -> LazyQuery<T, std::slice::Iter<'_, T>>;
}

// Implementations for Vec
impl<T: 'static> QueryExt<T> for Vec<T> {
    fn query(&self) -> Query<T> {
        Query::new(self)
    }
    
    fn lazy_query(&self) -> LazyQuery<T, std::slice::Iter<'_, T>> {
        LazyQuery::new(self)
    }
}

// Implementations for slices
impl<T: 'static> QueryExt<T> for [T] {
    fn query(&self) -> Query<T> {
        Query::new(self)
    }
    
    fn lazy_query(&self) -> LazyQuery<T, std::slice::Iter<'_, T>> {
        LazyQuery::new(self)
    }
}

// Array implementations
impl<T: 'static, const N: usize> QueryExt<T> for [T; N] {
    fn query(&self) -> Query<T> {
        Query::new(&self[..])
    }
    
    fn lazy_query(&self) -> LazyQuery<T, std::slice::Iter<'_, T>> {
        LazyQuery::new(&self[..])
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
}
