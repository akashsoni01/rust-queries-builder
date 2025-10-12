//! JOIN operations for locked data structures.
//!
//! This module provides join operations between collections of locked values,
//! enabling INNER JOIN, LEFT JOIN, RIGHT JOIN without unnecessary copying.
//!
//! # Example
//!
//! ```ignore
//! use rust_queries_core::lock_join::LockJoinQuery;
//!
//! let users: HashMap<String, Arc<RwLock<User>>> = /* ... */;
//! let orders: HashMap<String, Arc<RwLock<Order>>> = /* ... */;
//!
//! let user_orders = LockJoinQuery::new(&users, &orders)
//!     .inner_join(
//!         User::id_r(),
//!         Order::user_id_r(),
//!         |user, order| (user.name.clone(), order.total)
//!     );
//! ```

use crate::locks::LockValue;
use key_paths_core::KeyPaths;

/// A join query builder for locked data structures.
///
/// Enables joining two collections of locked values.
pub struct LockJoinQuery<'a, L, R, LL, LR>
where
    LL: LockValue<L> + 'a,
    LR: LockValue<R> + 'a,
{
    left: Vec<&'a LL>,
    right: Vec<&'a LR>,
    _phantom: std::marker::PhantomData<(L, R)>,
}

impl<'a, L: 'static, R: 'static, LL, LR> LockJoinQuery<'a, L, R, LL, LR>
where
    LL: LockValue<L> + 'a,
    LR: LockValue<R> + 'a,
{
    /// Create a new join query from two collections of locks.
    pub fn new(left: Vec<&'a LL>, right: Vec<&'a LR>) -> Self {
        Self {
            left,
            right,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Perform an INNER JOIN.
    ///
    /// Returns only pairs where keys match.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = LockJoinQuery::new(&users, &orders)
    ///     .inner_join(
    ///         User::id_r(),
    ///         Order::user_id_r(),
    ///         |user, order| (user.name.clone(), order.total)
    ///     );
    /// ```
    pub fn inner_join<LK, RK, M, Out>(&self, left_key: KeyPaths<L, LK>, right_key: KeyPaths<R, RK>, mapper: M) -> Vec<Out>
    where
        LK: Eq + Clone + 'static,
        RK: Eq + Clone + 'static,
        LK: PartialEq<RK>,
        M: Fn(&L, &R) -> Out,
        L: Clone,
        R: Clone,
    {
        let mut results = Vec::new();

        for left_lock in &self.left {
            let left_data = left_lock.with_value(|l| (left_key.get(l).cloned(), l.clone()));
            if let Some((Some(left_k), left_item)) = left_data {
                for right_lock in &self.right {
                    let right_data = right_lock.with_value(|r| (right_key.get(r).cloned(), r.clone()));
                    if let Some((Some(right_k), right_item)) = right_data {
                        if left_k == right_k {
                            results.push(mapper(&left_item, &right_item));
                        }
                    }
                }
            }
        }

        results
    }

    /// Perform a LEFT JOIN.
    ///
    /// Returns all left items with optional right matches.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = LockJoinQuery::new(&users, &orders)
    ///     .left_join(
    ///         User::id_r(),
    ///         Order::user_id_r(),
    ///         |user, order_opt| match order_opt {
    ///             Some(order) => format!("{} has order {}", user.name, order.id),
    ///             None => format!("{} has no orders", user.name),
    ///         }
    ///     );
    /// ```
    pub fn left_join<LK, RK, M, Out>(&self, left_key: KeyPaths<L, LK>, right_key: KeyPaths<R, RK>, mapper: M) -> Vec<Out>
    where
        LK: Eq + Clone + 'static,
        RK: Eq + Clone + 'static,
        LK: PartialEq<RK>,
        M: Fn(&L, Option<&R>) -> Out,
        L: Clone,
        R: Clone,
    {
        let mut results = Vec::new();

        for left_lock in &self.left {
            let left_data = left_lock.with_value(|l| (left_key.get(l).cloned(), l.clone()));
            if let Some((Some(left_key_val), left_item)) = left_data {
                let mut found_match = false;

                for right_lock in &self.right {
                    let right_data = right_lock.with_value(|r| (right_key.get(r).cloned(), r.clone()));
                    if let Some((Some(right_key_val), right_item)) = right_data {
                        if left_key_val == right_key_val {
                            results.push(mapper(&left_item, Some(&right_item)));
                            found_match = true;
                        }
                    }
                }

                if !found_match {
                    results.push(mapper(&left_item, None));
                }
            }
        }

        results
    }

    /// Perform a RIGHT JOIN.
    ///
    /// Returns all right items with optional left matches.
    pub fn right_join<LK, RK, M, Out>(&self, left_key: KeyPaths<L, LK>, right_key: KeyPaths<R, RK>, mapper: M) -> Vec<Out>
    where
        LK: Eq + Clone + 'static,
        RK: Eq + Clone + 'static,
        LK: PartialEq<RK>,
        M: Fn(Option<&L>, &R) -> Out,
        L: Clone,
        R: Clone,
    {
        let mut results = Vec::new();

        for right_lock in &self.right {
            let right_data = right_lock.with_value(|r| (right_key.get(r).cloned(), r.clone()));
            if let Some((Some(right_key_val), right_item)) = right_data {
                let mut found_match = false;

                for left_lock in &self.left {
                    let left_data = left_lock.with_value(|l| (left_key.get(l).cloned(), l.clone()));
                    if let Some((Some(left_key_val), left_item)) = left_data {
                        if left_key_val == right_key_val {
                            results.push(mapper(Some(&left_item), &right_item));
                            found_match = true;
                        }
                    }
                }

                if !found_match {
                    results.push(mapper(None, &right_item));
                }
            }
        }

        results
    }

    /// Perform a CROSS JOIN (Cartesian product).
    ///
    /// Returns all combinations of left and right items.
    pub fn cross_join<M, Out>(&self, mapper: M) -> Vec<Out>
    where
        M: Fn(&L, &R) -> Out,
        L: Clone,
        R: Clone,
    {
        let mut results = Vec::new();

        for left_lock in &self.left {
            if let Some(left_item) = left_lock.with_value(|l| l.clone()) {
                for right_lock in &self.right {
                    if let Some(right_item) = right_lock.with_value(|r| r.clone()) {
                        results.push(mapper(&left_item, &right_item));
                    }
                }
            }
        }

        results
    }
}

/// Helper trait for creating join queries from locked collections.
pub trait LockJoinable<T, L>
where
    L: LockValue<T>,
{
    /// Create a join query with another locked collection.
    fn lock_join<'a, R, LR>(&'a self, right: &'a impl LockJoinableCollection<R, LR>) -> LockJoinQuery<'a, T, R, L, LR>
    where
        LR: LockValue<R> + 'a;
}

/// Helper trait for collections that can participate in joins.
pub trait LockJoinableCollection<T, L>
where
    L: LockValue<T>,
{
    /// Get locks for join operations.
    fn get_locks(&self) -> Vec<&L>;
}

// Implementations for HashMap
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};

impl<K, V> LockJoinableCollection<V, Arc<RwLock<V>>> for HashMap<K, Arc<RwLock<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn get_locks(&self) -> Vec<&Arc<RwLock<V>>> {
        self.values().collect()
    }
}

impl<K, V> LockJoinableCollection<V, Arc<Mutex<V>>> for HashMap<K, Arc<Mutex<V>>>
where
    K: Eq + std::hash::Hash,
{
    fn get_locks(&self) -> Vec<&Arc<Mutex<V>>> {
        self.values().collect()
    }
}

// Implementations for Vec
impl<T> LockJoinableCollection<T, Arc<RwLock<T>>> for Vec<Arc<RwLock<T>>> {
    fn get_locks(&self) -> Vec<&Arc<RwLock<T>>> {
        self.iter().collect()
    }
}

impl<T> LockJoinableCollection<T, Arc<Mutex<T>>> for Vec<Arc<Mutex<T>>> {
    fn get_locks(&self) -> Vec<&Arc<Mutex<T>>> {
        self.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;
    use key_paths_derive::Keypaths;

    #[derive(Clone, Keypaths)]
    struct User {
        id: u32,
        name: String,
    }

    #[derive(Clone, Keypaths)]
    struct Order {
        id: u32,
        user_id: u32,
        total: f64,
    }

    fn create_test_data() -> (HashMap<String, Arc<RwLock<User>>>, HashMap<String, Arc<RwLock<Order>>>) {
        let mut users = HashMap::new();
        users.insert("u1".to_string(), Arc::new(RwLock::new(User { id: 1, name: "Alice".to_string() })));
        users.insert("u2".to_string(), Arc::new(RwLock::new(User { id: 2, name: "Bob".to_string() })));

        let mut orders = HashMap::new();
        orders.insert("o1".to_string(), Arc::new(RwLock::new(Order { id: 101, user_id: 1, total: 99.99 })));
        orders.insert("o2".to_string(), Arc::new(RwLock::new(Order { id: 102, user_id: 1, total: 149.99 })));
        orders.insert("o3".to_string(), Arc::new(RwLock::new(Order { id: 103, user_id: 3, total: 199.99 })));

        (users, orders)
    }

    #[test]
    fn test_inner_join() {
        let (users, orders) = create_test_data();
        
        let user_locks: Vec<_> = users.values().collect();
        let order_locks: Vec<_> = orders.values().collect();
        
        let results = LockJoinQuery::new(user_locks, order_locks)
            .inner_join(
                User::id_r(),
                Order::user_id_r(),
                |user, order| (user.name.clone(), order.total)
            );

        assert_eq!(results.len(), 2); // Only Alice's orders match
    }

    #[test]
    fn test_left_join() {
        let (users, orders) = create_test_data();
        
        let user_locks: Vec<_> = users.values().collect();
        let order_locks: Vec<_> = orders.values().collect();
        
        let results = LockJoinQuery::new(user_locks, order_locks)
            .left_join(
                User::id_r(),
                Order::user_id_r(),
                |user, order_opt| match order_opt {
                    Some(_) => format!("{} has order", user.name),
                    None => format!("{} no orders", user.name),
                }
            );

        assert_eq!(results.len(), 3); // Alice (2 orders) + Bob (no orders)
    }
}

