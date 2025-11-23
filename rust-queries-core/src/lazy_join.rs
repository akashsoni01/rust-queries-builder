//! Lazy join query implementation for combining multiple collections with deferred execution.
//!
//! This module provides lazy join operations that return iterators instead of eagerly
//! collecting results, enabling early termination and memory efficiency.

use key_paths_core::KeyPaths;
use std::collections::HashMap;

/// A lazy join query builder that returns iterators instead of collecting results.
///
/// Supports inner joins and left joins with deferred execution for better performance
/// on large datasets.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the data being joined
/// * `L` - The type of items in the left collection
/// * `R` - The type of items in the right collection
///
/// # Example
///
/// ```ignore
/// let user_orders = LazyJoinQuery::new(&users, &orders)
///     .inner_join_lazy(
///         User::id(),
///         Order::user_id(),
///         |user, order| (user.name.clone(), order.total)
///     );
///
/// // Nothing executed yet - just an iterator
/// let first_5: Vec<_> = user_orders.take(5).collect();
/// ```
pub struct LazyJoinQuery<'a, L: 'static, R: 'static> {
    left: &'a [L],
    right: &'a [R],
}

impl<'a, L: 'static, R: 'static> LazyJoinQuery<'a, L, R> {
    /// Creates a new lazy join query from two collections.
    ///
    /// # Arguments
    ///
    /// * `left` - The left collection to join
    /// * `right` - The right collection to join
    ///
    /// # Example
    ///
    /// ```ignore
    /// let join = LazyJoinQuery::new(&users, &orders);
    /// ```
    pub fn new(left: &'a [L], right: &'a [R]) -> Self {
        Self { left, right }
    }

    /// Performs a lazy inner join between two collections.
    ///
    /// Returns an iterator over matching pairs. The join is evaluated lazily,
    /// allowing for early termination and memory efficiency.
    ///
    /// # Arguments
    ///
    /// * `left_key` - Key-path to the join field in the left collection
    /// * `right_key` - Key-path to the join field in the right collection
    /// * `mapper` - Function to transform matching pairs into the result type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<_> = LazyJoinQuery::new(&users, &orders)
    ///     .inner_join_lazy(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| (user.name.clone(), order.total)
    ///     )
    ///     .take(10)  // Early termination - only process first 10 matches
    ///     .collect();
    /// ```
    pub fn inner_join_lazy<K, O, F>(
        &self,
        left_key: KeyPaths<L, K>,
        right_key: KeyPaths<R, K>,
        mapper: F,
    ) -> impl Iterator<Item = O> + 'a
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, &R) -> O + 'a,
    {
        // Build index for right side for O(n) lookup
        let mut right_index: HashMap<K, Vec<&'a R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Return iterator that lazily evaluates the join
        self.left.iter().flat_map(move |left_item| {
            let key_opt = left_key.get(left_item).cloned();
            let mapper_ref = &mapper;
            
            key_opt
                .and_then(|key| right_index.get(&key).cloned())
                .into_iter()
                .flatten()
                .map(move |right_item| mapper_ref(left_item, right_item))
        })
    }

    /// Performs a lazy left join between two collections.
    ///
    /// Returns an iterator over all left items with optional right matches.
    /// The join is evaluated lazily, allowing for early termination.
    ///
    /// # Arguments
    ///
    /// * `left_key` - Key-path to the join field in the left collection
    /// * `right_key` - Key-path to the join field in the right collection
    /// * `mapper` - Function to transform pairs into the result type (right item may be None)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<_> = LazyJoinQuery::new(&users, &orders)
    ///     .left_join_lazy(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| match order {
    ///             Some(o) => format!("{} has order {}", user.name, o.id),
    ///             None => format!("{} has no orders", user.name),
    ///         }
    ///     )
    ///     .take(5)  // Early termination
    ///     .collect();
    /// ```
    pub fn left_join_lazy<K, O, F>(
        &self,
        left_key: KeyPaths<L, K>,
        right_key: KeyPaths<R, K>,
        mapper: F,
    ) -> impl Iterator<Item = O> + 'a
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, Option<&R>) -> O + 'a,
    {
        // Build index for right side
        let mut right_index: HashMap<K, Vec<&'a R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Return iterator that lazily evaluates the join
        self.left.iter().flat_map(move |left_item| {
            let key_opt = left_key.get(left_item).cloned();
            let mapper_ref = &mapper;
            
            if let Some(key) = key_opt {
                if let Some(right_items) = right_index.get(&key) {
                    // Has matches - yield all matches
                    let matches: Vec<_> = right_items.iter()
                        .map(|right_item| mapper_ref(left_item, Some(right_item)))
                        .collect();
                    matches.into_iter()
                } else {
                    // No matches - yield None
                    std::iter::once(mapper_ref(left_item, None))
                }
            } else {
                // No key - yield None
                std::iter::once(mapper_ref(left_item, None))
            }
        })
    }

    /// Performs a lazy inner join with an additional filter predicate.
    ///
    /// Like `inner_join_lazy`, but only includes pairs that satisfy both the join
    /// condition and the additional predicate.
    ///
    /// # Arguments
    ///
    /// * `left_key` - Key-path to the join field in the left collection
    /// * `right_key` - Key-path to the join field in the right collection
    /// * `predicate` - Additional condition that must be true for pairs to be included
    /// * `mapper` - Function to transform matching pairs into the result type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<_> = LazyJoinQuery::new(&orders, &products)
    ///     .inner_join_where_lazy(
    ///         Order::product_id(),
    ///         Product::id(),
    ///         |order, _product| order.total > 100.0,
    ///         |order, product| (product.name.clone(), order.total)
    ///     )
    ///     .take(10)
    ///     .collect();
    /// ```
    pub fn inner_join_where_lazy<K, O, F, P>(
        &self,
        left_key: KeyPaths<L, K>,
        right_key: KeyPaths<R, K>,
        predicate: P,
        mapper: F,
    ) -> impl Iterator<Item = O> + 'a
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, &R) -> O + 'a,
        P: Fn(&L, &R) -> bool + 'a,
    {
        // Build index for right side
        let mut right_index: HashMap<K, Vec<&'a R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Return iterator that lazily evaluates the join with predicate
        self.left.iter().flat_map(move |left_item| {
            let key_opt = left_key.get(left_item).cloned();
            let mapper_ref = &mapper;
            let predicate_ref = &predicate;
            
            key_opt
                .and_then(|key| right_index.get(&key).cloned())
                .into_iter()
                .flatten()
                .filter(move |right_item| predicate_ref(left_item, right_item))
                .map(move |right_item| mapper_ref(left_item, right_item))
        })
    }
}

