//! Queryable trait for supporting multiple container types.
//!
//! This module provides the `Queryable` trait which enables querying
//! various container types: Vec, HashMap, HashSet, BTreeMap, VecDeque, etc.

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList};

/// Trait for types that can be queried.
///
/// Implemented for standard collections like Vec, HashMap, HashSet, etc.
pub trait Queryable<T> {
    /// Returns an iterator over references to items.
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

// Implementation for slice
impl<T> Queryable<T> for [T] {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for Vec
impl<T> Queryable<T> for Vec<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for VecDeque
impl<T> Queryable<T> for VecDeque<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for LinkedList
impl<T> Queryable<T> for LinkedList<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for HashSet
impl<T> Queryable<T> for HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for BTreeSet
impl<T> Queryable<T> for BTreeSet<T>
where
    T: Ord,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for HashMap (queries values)
impl<K, V> Queryable<V> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.values())
    }
}

// Implementation for BTreeMap (queries values)
impl<K, V> Queryable<V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.values())
    }
}

// Implementation for arrays of any size
impl<T, const N: usize> Queryable<T> for [T; N] {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for Option (0 or 1 item)
impl<T> Queryable<T> for Option<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

// Implementation for Result (0 or 1 item)
impl<T, E> Queryable<T> for Result<T, E> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

