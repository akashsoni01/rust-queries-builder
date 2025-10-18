//! Join query implementation for combining multiple collections.
//!
//! This module provides the `JoinQuery` struct which enables SQL-like JOIN operations
//! between collections using type-safe key-paths.

use key_paths_core::KeyPaths;
use std::collections::HashMap;

/// A query builder for joining two collections.
///
/// Supports inner joins, left joins, and filtered joins using key-paths for type-safe
/// join conditions.
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
/// let user_orders = JoinQuery::new(&users, &orders)
///     .inner_join(
///         User::id(),
///         Order::user_id(),
///         |user, order| (user.name.clone(), order.total)
///     );
/// ```
pub struct JoinQuery<'a, L: 'static, R: 'static> {
    left: &'a [L],
    right: &'a [R],
}

impl<'a, L: 'static, R: 'static> JoinQuery<'a, L, R> {
    /// Creates a new join query from two collections.
    ///
    /// **Note**: No `Clone` required on `L` or `R`. The mapper function 
    /// handles any cloning needed for the result type.
    ///
    /// # Arguments
    ///
    /// * `left` - The left collection to join
    /// * `right` - The right collection to join
    ///
    /// # Example
    ///
    /// ```ignore
    /// let join = JoinQuery::new(&users, &orders);
    /// ```
    pub fn new(left: &'a [L], right: &'a [R]) -> Self {
        Self { left, right }
    }

    /// Performs an inner join between two collections.
    ///
    /// Returns only the pairs where the join keys match. Uses a hash-based
    /// algorithm for O(n + m) performance.
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
    /// let results = JoinQuery::new(&users, &orders)
    ///     .inner_join(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| UserOrder {
    ///             user_name: user.name.clone(),
    ///             order_total: order.total,
    ///         }
    ///     );
    /// ```
    pub fn inner_join<K, O, F>(&self, left_key: KeyPaths<L, K>, right_key: KeyPaths<R, K>, mapper: F) -> Vec<O>
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, &R) -> O,
    {
        // Build index for right side for O(n) lookup
        let mut right_index: HashMap<K, Vec<&R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Join left with indexed right
        let mut results = Vec::new();
        for left_item in self.left.iter() {
            if let Some(key) = left_key.get(left_item).cloned() {
                if let Some(right_items) = right_index.get(&key) {
                    for right_item in right_items {
                        results.push(mapper(left_item, right_item));
                    }
                }
            }
        }

        results
    }

    /// Performs a left join between two collections.
    ///
    /// Returns all items from the left collection with optional matching items
    /// from the right collection. If no match is found, the right item is `None`.
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
    /// let results = JoinQuery::new(&users, &orders)
    ///     .left_join(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| match order {
    ///             Some(o) => format!("{} has order {}", user.name, o.id),
    ///             None => format!("{} has no orders", user.name),
    ///         }
    ///     );
    /// ```
    pub fn left_join<K, O, F>(&self, left_key: KeyPaths<L, K>, right_key: KeyPaths<R, K>, mapper: F) -> Vec<O>
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, Option<&R>) -> O,
    {
        // Build index for right side
        let mut right_index: HashMap<K, Vec<&R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Join left with indexed right
        let mut results = Vec::new();
        for left_item in self.left.iter() {
            if let Some(key) = left_key.get(left_item).cloned() {
                if let Some(right_items) = right_index.get(&key) {
                    for right_item in right_items {
                        results.push(mapper(left_item, Some(right_item)));
                    }
                } else {
                    results.push(mapper(left_item, None));
                }
            } else {
                results.push(mapper(left_item, None));
            }
        }

        results
    }

    /// Performs an inner join with an additional filter predicate.
    ///
    /// Like `inner_join`, but only includes pairs that satisfy both the join
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
    /// // Join orders with products, but only high-value orders
    /// let results = JoinQuery::new(&orders, &products)
    ///     .inner_join_where(
    ///         Order::product_id(),
    ///         Product::id(),
    ///         |order, _product| order.total > 100.0,
    ///         |order, product| (product.name.clone(), order.total)
    ///     );
    /// ```
    pub fn inner_join_where<K, O, F, P>(
        &self,
        left_key: KeyPaths<L, K>,
        right_key: KeyPaths<R, K>,
        predicate: P,
        mapper: F,
    ) -> Vec<O>
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(&L, &R) -> O,
        P: Fn(&L, &R) -> bool,
    {
        // Build index for right side
        let mut right_index: HashMap<K, Vec<&R>> = HashMap::new();
        for item in self.right.iter() {
            if let Some(key) = right_key.get(item).cloned() {
                right_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Join left with indexed right, applying predicate
        let mut results = Vec::new();
        for left_item in self.left.iter() {
            if let Some(key) = left_key.get(left_item).cloned() {
                if let Some(right_items) = right_index.get(&key) {
                    for right_item in right_items {
                        if predicate(left_item, right_item) {
                            results.push(mapper(left_item, right_item));
                        }
                    }
                }
            }
        }

        results
    }

    /// Performs a right join between two collections.
    ///
    /// Returns all items from the right collection with optional matching items
    /// from the left collection. If no match is found, the left item is `None`.
    ///
    /// # Arguments
    ///
    /// * `left_key` - Key-path to the join field in the left collection
    /// * `right_key` - Key-path to the join field in the right collection
    /// * `mapper` - Function to transform pairs into the result type (left item may be None)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = JoinQuery::new(&users, &orders)
    ///     .right_join(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| match user {
    ///             Some(u) => format!("Order {} by {}", order.id, u.name),
    ///             None => format!("Order {} by unknown user", order.id),
    ///         }
    ///     );
    /// ```
    pub fn right_join<K, O, F>(&self, left_key: KeyPaths<L, K>, right_key: KeyPaths<R, K>, mapper: F) -> Vec<O>
    where
        K: Eq + std::hash::Hash + Clone + 'static,
        F: Fn(Option<&L>, &R) -> O,
    {
        // Build index for left side
        let mut left_index: HashMap<K, Vec<&L>> = HashMap::new();
        for item in self.left.iter() {
            if let Some(key) = left_key.get(item).cloned() {
                left_index.entry(key).or_insert_with(Vec::new).push(item);
            }
        }

        // Join right with indexed left
        let mut results = Vec::new();
        for right_item in self.right.iter() {
            if let Some(key) = right_key.get(right_item).cloned() {
                if let Some(left_items) = left_index.get(&key) {
                    for left_item in left_items {
                        results.push(mapper(Some(left_item), right_item));
                    }
                } else {
                    results.push(mapper(None, right_item));
                }
            } else {
                results.push(mapper(None, right_item));
            }
        }

        results
    }

    /// Performs a cross join (Cartesian product) between two collections.
    ///
    /// Returns all possible pairs of items from both collections.
    /// **Warning**: This can produce very large result sets (size = left.len() * right.len()).
    ///
    /// # Arguments
    ///
    /// * `mapper` - Function to transform pairs into the result type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let all_combinations = JoinQuery::new(&colors, &sizes)
    ///     .cross_join(|color, size| ProductVariant {
    ///         color: color.clone(),
    ///         size: size.clone(),
    ///     });
    /// ```
    pub fn cross_join<O, F>(&self, mapper: F) -> Vec<O>
    where
        F: Fn(&L, &R) -> O,
    {
        let mut results = Vec::new();
        for left_item in self.left.iter() {
            for right_item in self.right.iter() {
                results.push(mapper(left_item, right_item));
            }
        }
        results
    }

}

