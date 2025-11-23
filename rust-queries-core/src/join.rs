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

#[cfg(feature = "parallel")]
mod parallel_join {
    use super::JoinQuery;
    use key_paths_core::KeyPaths;
    use rayon::prelude::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Extension trait for parallel join operations.
    ///
    /// Provides parallel versions of join operations using rayon for better
    /// performance on large datasets.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rust_queries_core::join::ParallelJoinExt;
    ///
    /// let results: Vec<_> = JoinQuery::new(&users, &orders)
    ///     .inner_join_parallel(
    ///         User::id(),
    ///         Order::user_id(),
    ///         |user, order| (user.name.clone(), order.total)
    ///     );
    /// ```
    pub trait ParallelJoinExt<'a, L: 'static + Send + Sync, R: 'static + Send + Sync> {
        /// Performs a parallel inner join between two collections.
        ///
        /// Uses rayon to process the join in parallel across multiple CPU cores.
        /// Best performance on large datasets (10,000+ items).
        ///
        /// # Arguments
        ///
        /// * `left_key` - Key-path to the join field in the left collection
        /// * `right_key` - Key-path to the join field in the right collection
        /// * `mapper` - Function to transform matching pairs into the result type (must be Send + Sync)
        ///
        /// # Example
        ///
        /// ```ignore
        /// let results: Vec<_> = JoinQuery::new(&users, &orders)
        ///     .inner_join_parallel(
        ///         User::id(),
        ///         Order::user_id(),
        ///         |user, order| (user.name.clone(), order.total)
        ///     );
        /// ```
        fn inner_join_parallel<K, O, F>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, &R) -> O + Send + Sync;

        /// Performs a parallel left join between two collections.
        ///
        /// Uses rayon to process the join in parallel across multiple CPU cores.
        ///
        /// # Arguments
        ///
        /// * `left_key` - Key-path to the join field in the left collection
        /// * `right_key` - Key-path to the join field in the right collection
        /// * `mapper` - Function to transform pairs into the result type (right item may be None, must be Send + Sync)
        ///
        /// # Example
        ///
        /// ```ignore
        /// let results: Vec<_> = JoinQuery::new(&users, &orders)
        ///     .left_join_parallel(
        ///         User::id(),
        ///         Order::user_id(),
        ///         |user, order| match order {
        ///             Some(o) => format!("{} has order {}", user.name, o.id),
        ///             None => format!("{} has no orders", user.name),
        ///         }
        ///     );
        /// ```
        fn left_join_parallel<K, O, F>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, Option<&R>) -> O + Send + Sync;

        /// Performs a parallel inner join with an additional filter predicate.
        ///
        /// Like `inner_join_parallel`, but only includes pairs that satisfy both the join
        /// condition and the additional predicate.
        ///
        /// # Arguments
        ///
        /// * `left_key` - Key-path to the join field in the left collection
        /// * `right_key` - Key-path to the join field in the right collection
        /// * `predicate` - Additional condition that must be true for pairs to be included (must be Send + Sync)
        /// * `mapper` - Function to transform matching pairs into the result type (must be Send + Sync)
        ///
        /// # Example
        ///
        /// ```ignore
        /// let results: Vec<_> = JoinQuery::new(&orders, &products)
        ///     .inner_join_where_parallel(
        ///         Order::product_id(),
        ///         Product::id(),
        ///         |order, _product| order.total > 100.0,
        ///         |order, product| (product.name.clone(), order.total)
        ///     );
        /// ```
        fn inner_join_where_parallel<K, O, F, P>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            predicate: P,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, &R) -> O + Send + Sync,
            P: Fn(&L, &R) -> bool + Send + Sync;
    }

    #[cfg(feature = "parallel")]
    impl<'a, L: 'static + Send + Sync, R: 'static + Send + Sync> ParallelJoinExt<'a, L, R> for JoinQuery<'a, L, R> {
        fn inner_join_parallel<K, O, F>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, &R) -> O + Send + Sync,
        {
            // Extract keys first to avoid thread safety issues with keypath (which uses Rc internally)
            let left_with_keys: Vec<(usize, K)> = self.left
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    left_key.get(item).cloned().map(|key| (idx, key))
                })
                .collect();

            let right_with_keys: Vec<(usize, K)> = self.right
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    right_key.get(item).cloned().map(|key| (idx, key))
                })
                .collect();

            // Build index for right side
            let mut right_index: HashMap<K, Vec<usize>> = HashMap::new();
            for (idx, key) in right_with_keys {
                right_index.entry(key).or_insert_with(Vec::new).push(idx);
            }

            // Join left with indexed right in parallel
            // Share the index and slices across threads using Arc
            let right_index_arc = Arc::new(right_index);
            let left_slice = self.left;
            let right_slice = self.right;
            
            // Process in parallel and collect results
            let results: Vec<Vec<O>> = left_with_keys
                .into_par_iter()
                .map(|(left_idx, key)| {
                    let left_item = &left_slice[left_idx];
                    let index = right_index_arc.clone();
                    index.get(&key)
                        .map(|right_indices| {
                            right_indices.iter().map(|right_idx| {
                                mapper(left_item, &right_slice[*right_idx])
                            }).collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                })
                .collect();
            
            results.into_iter().flatten().collect()
        }

        fn left_join_parallel<K, O, F>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, Option<&R>) -> O + Send + Sync,
        {
            // Extract keys first to avoid thread safety issues with keypath (which uses Rc internally)
            let left_with_keys: Vec<(usize, Option<K>)> = self.left
                .iter()
                .enumerate()
                .map(|(idx, item)| (idx, left_key.get(item).cloned()))
                .collect();

            let right_with_keys: Vec<(usize, K)> = self.right
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    right_key.get(item).cloned().map(|key| (idx, key))
                })
                .collect();

            // Build index for right side
            let mut right_index: HashMap<K, Vec<usize>> = HashMap::new();
            for (idx, key) in right_with_keys {
                right_index.entry(key).or_insert_with(Vec::new).push(idx);
            }

            // Join left with indexed right in parallel
            // Share the index and slices across threads using Arc
            let right_index_arc = Arc::new(right_index);
            let left_slice = self.left;
            let right_slice = self.right;
            
            // Process in parallel and collect results
            let results: Vec<Vec<O>> = left_with_keys
                .into_par_iter()
                .map(|(left_idx, key_opt)| {
                    let left_item = &left_slice[left_idx];
                    let index = right_index_arc.clone();
                    if let Some(key) = key_opt {
                        if let Some(right_indices) = index.get(&key) {
                            // Has matches - yield all matches
                            right_indices.iter().map(|right_idx| {
                                mapper(left_item, Some(&right_slice[*right_idx]))
                            }).collect::<Vec<_>>()
                        } else {
                            // No matches - yield None
                            vec![mapper(left_item, None)]
                        }
                    } else {
                        // No key - yield None
                        vec![mapper(left_item, None)]
                    }
                })
                .collect();
            
            results.into_iter().flatten().collect()
        }

        fn inner_join_where_parallel<K, O, F, P>(
            &self,
            left_key: KeyPaths<L, K>,
            right_key: KeyPaths<R, K>,
            predicate: P,
            mapper: F,
        ) -> Vec<O>
        where
            K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
            F: Fn(&L, &R) -> O + Send + Sync,
            P: Fn(&L, &R) -> bool + Send + Sync,
        {
            // Extract keys first to avoid thread safety issues with keypath (which uses Rc internally)
            let left_with_keys: Vec<(usize, K)> = self.left
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    left_key.get(item).cloned().map(|key| (idx, key))
                })
                .collect();

            let right_with_keys: Vec<(usize, K)> = self.right
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    right_key.get(item).cloned().map(|key| (idx, key))
                })
                .collect();

            // Build index for right side
            let mut right_index: HashMap<K, Vec<usize>> = HashMap::new();
            for (idx, key) in right_with_keys {
                right_index.entry(key).or_insert_with(Vec::new).push(idx);
            }

            // Join left with indexed right in parallel, applying predicate
            // Share the index and slices across threads using Arc
            let right_index_arc = Arc::new(right_index);
            let left_slice = self.left;
            let right_slice = self.right;
            
            // Process in parallel and collect results
            let results: Vec<Vec<O>> = left_with_keys
                .into_par_iter()
                .map(|(left_idx, key)| {
                    let left_item = &left_slice[left_idx];
                    let index = right_index_arc.clone();
                    index.get(&key)
                        .map(|right_indices| {
                            right_indices.iter()
                                .filter_map(|right_idx| {
                                    let right_item = &right_slice[*right_idx];
                                    if predicate(left_item, right_item) {
                                        Some(mapper(left_item, right_item))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                })
                .collect();
            
            results.into_iter().flatten().collect()
        }
    }
}

#[cfg(feature = "parallel")]
pub use parallel_join::ParallelJoinExt;

