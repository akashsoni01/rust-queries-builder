//! Lazy query support for locked data structures.
//!
//! Provides lazy evaluation with early termination for locked collections.

use crate::locks::LockValue;
use key_paths_core::KeyPaths;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(feature = "datetime")]
use chrono::{DateTime, TimeZone};

/// Lazy query for locked data with early termination.
pub struct LockLazyQuery<'a, T: 'static, L, I>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = &'a L>,
{
    iter: I,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'static, L, I> LockLazyQuery<'a, T, L, I>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = &'a L> + 'a,
{
    /// Create a new lazy query from an iterator of locks.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }

    /// Filter using a key-path predicate (lazy).
    pub fn where_<F, P>(self, path: KeyPaths<T, F>, predicate: P) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
    {
        LockLazyQuery {
            iter: self.iter.filter(move |lock| {
                lock.with_value(|item| {
                    path.get(item).map_or(false, |val| predicate(val))
                })
                .unwrap_or(false)
            }),
            _phantom: PhantomData,
        }
    }

    /// Map to a field value (lazy).
    /// 
    /// This allows you to select only specific fields from locked data without
    /// cloning the entire object. Perfect for projecting data efficiently.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Select only product names (not full objects)
    /// let names: Vec<String> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::price_r(), |&p| p > 100.0)
    ///     .select_lazy(Product::name_r())
    ///     .collect();
    /// 
    /// // Select only IDs  
    /// let ids: Vec<u32> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .select_lazy(Product::id_r())
    ///     .take(10)
    ///     .collect();
    /// 
    /// // Select prices and compute sum
    /// let total: f64 = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .select_lazy(Product::price_r())
    ///     .sum();
    /// ```
    /// 
    /// **Performance Note**: This is much more efficient than collecting full objects
    /// and then extracting fields, as it only clones the specific field value.
    pub fn select_lazy<F>(self, path: KeyPaths<T, F>) -> impl Iterator<Item = F> + 'a
    where
        F: Clone + 'static,
    {
        self.iter.filter_map(move |lock| {
            lock.with_value(|item| path.get(item).cloned()).flatten()
        })
    }

    /// Take first N items (lazy).
    pub fn take_lazy(self, n: usize) -> impl Iterator<Item = T> + 'a
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .take(n)
    }

    /// Skip first N items (lazy).
    pub fn skip_lazy(self, n: usize) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        LockLazyQuery {
            iter: self.iter.skip(n),
            _phantom: PhantomData,
        }
    }

    /// Count matching items (terminal).
    pub fn count(self) -> usize {
        self.iter.count()
    }

    /// Get first matching item (terminal).
    pub fn first(mut self) -> Option<T>
    where
        T: Clone,
    {
        self.iter
            .find_map(|lock| lock.with_value(|item| item.clone()))
    }

    /// Check if any items match (terminal).
    pub fn any(mut self) -> bool {
        self.iter.next().is_some()
    }

    /// Collect into Vec (terminal).
    pub fn collect(self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect()
    }

    /// Get all matching items (alias for collect, similar to LockQuery::all).
    /// 
    /// This provides a familiar API for users coming from LockQuery.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let all_items: Vec<Product> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::price_r(), |&p| p > 100.0)
    ///     .all();
    /// ```
    pub fn all(self) -> Vec<T>
    where
        T: Clone,
    {
        self.collect()
    }

    // ========================================================================
    // AGGREGATION FUNCTIONS
    // ========================================================================

    /// Sum a numeric field (terminal).
    /// 
    /// Efficiently computes the sum without collecting all items into a Vec first.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Sum all prices
    /// let total_value: f64 = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .sum(Product::price_r());
    /// 
    /// // Sum stock quantities
    /// let total_stock: u32 = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .sum(Product::stock_r());
    /// ```
    pub fn sum<F>(self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static,
    {
        self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .fold(F::default(), |acc, val| acc + val)
    }

    /// Calculate average of f64 field (terminal).
    /// 
    /// Returns None if no items match.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let avg_price = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .avg(Product::price_r());
    /// 
    /// match avg_price {
    ///     Some(avg) => println!("Average price: ${:.2}", avg),
    ///     None => println!("No items found"),
    /// }
    /// ```
    pub fn avg(self, path: KeyPaths<T, f64>) -> Option<f64> {
        let values: Vec<f64> = self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .collect();
        
        if values.is_empty() {
            None
        } else {
            Some(values.iter().sum::<f64>() / values.len() as f64)
        }
    }

    /// Find minimum value (terminal).
    /// 
    /// Returns None if no items match.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let min_stock = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .min(Product::stock_r());
    /// 
    /// println!("Minimum stock level: {:?}", min_stock);
    /// ```
    pub fn min<F>(self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .min()
    }

    /// Find maximum value (terminal).
    /// 
    /// Returns None if no items match.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let max_price = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .max(Product::price_r());
    /// 
    /// println!("Most expensive: ${:.2}", max_price.unwrap_or(0.0));
    /// ```
    pub fn max<F>(self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .max()
    }

    /// Find minimum float value (terminal).
    /// 
    /// Handles f64 values correctly with partial ordering.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let cheapest = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .min_float(Product::price_r());
    /// ```
    pub fn min_float(self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Find maximum float value (terminal).
    /// 
    /// Handles f64 values correctly with partial ordering.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let most_expensive = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .max_float(Product::price_r());
    /// ```
    pub fn max_float(self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    // ========================================================================
    // SQL-LIKE FUNCTIONS
    // ========================================================================

    /// Check if any items exist matching the criteria (terminal).
    /// 
    /// Alias for `any()`, provides SQL-like EXISTS semantics.
    /// Stops at the first match - very efficient!
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let has_expensive = products
    ///     .lock_lazy_query()
    ///     .where_(Product::price_r(), |&p| p > 1000.0)
    ///     .exists();
    /// 
    /// if has_expensive {
    ///     println!("We have luxury items!");
    /// }
    /// 
    /// // SQL equivalent: SELECT EXISTS(SELECT 1 FROM products WHERE price > 1000)
    /// ```
    pub fn exists(self) -> bool {
        self.any()
    }

    /// Limit results to first N items (lazy).
    /// 
    /// Alias for creating a limited iterator. Use with `.collect()` or `.all()`.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let top_5: Vec<Product> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 10)
    ///     .limit(5)
    ///     .collect();
    /// 
    /// // SQL equivalent: SELECT * FROM products WHERE stock > 10 LIMIT 5
    /// ```
    pub fn limit(self, n: usize) -> impl Iterator<Item = T> + 'a
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .take(n)
    }

    /// Skip first N items (lazy).
    /// 
    /// Alias for `skip_lazy()` with better SQL-like naming.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Get second page (skip 10, take 10)
    /// let page_2 = products
    ///     .lock_lazy_query()
    ///     .skip(10)
    ///     .limit(10)
    ///     .collect();
    /// 
    /// // SQL equivalent: SELECT * FROM products LIMIT 10 OFFSET 10
    /// ```
    pub fn skip(self, n: usize) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        self.skip_lazy(n)
    }

    /// Get distinct values for a field (terminal).
    /// 
    /// Returns a Vec of unique field values. Uses HashSet internally.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let categories: Vec<String> = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .distinct(Product::category_r());
    /// 
    /// println!("Available categories: {:?}", categories);
    /// 
    /// // SQL equivalent: SELECT DISTINCT category FROM products WHERE stock > 0
    /// ```
    pub fn distinct<F>(self, path: KeyPaths<T, F>) -> Vec<F>
    where
        F: Eq + std::hash::Hash + Clone + 'static,
    {
        use std::collections::HashSet;
        
        let set: HashSet<F> = self.iter
            .filter_map(|lock| {
                lock.with_value(|item| path.get(item).cloned()).flatten()
            })
            .collect();
        
        set.into_iter().collect()
    }

    /// Get last matching item (terminal).
    /// 
    /// **Note**: This consumes the entire iterator to find the last item.
    /// Less efficient than `first()` for lazy evaluation.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let last_product = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .last();
    /// ```
    pub fn last(self) -> Option<T>
    where
        T: Clone,
    {
        self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .last()
    }

    /// Find item at specific index (terminal).
    /// 
    /// Returns None if index is out of bounds.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let third_item = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .nth(2);  // 0-indexed, so this is the 3rd item
    /// ```
    pub fn nth(mut self, n: usize) -> Option<T>
    where
        T: Clone,
    {
        self.iter
            .nth(n)
            .and_then(|lock| lock.with_value(|item| item.clone()))
    }

    /// Check if all items match a predicate (terminal).
    /// 
    /// Returns true if all items match, false otherwise.
    /// Short-circuits on first non-match.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let all_in_stock = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .all_match(Product::stock_r(), |&s| s > 0);
    /// 
    /// if all_in_stock {
    ///     println!("All electronics are in stock!");
    /// }
    /// ```
    pub fn all_match<F, P>(mut self, path: KeyPaths<T, F>, predicate: P) -> bool
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
    {
        self.iter.all(|lock| {
            lock.with_value(|item| {
                path.get(item).map_or(false, |val| predicate(val))
            })
            .unwrap_or(false)
        })
    }

    /// Find first item matching an additional predicate (terminal).
    /// 
    /// Like `first()` but with an extra condition.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let expensive_laptop = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .find(Product::price_r(), |&p| p > 500.0);
    /// ```
    pub fn find<F, P>(mut self, path: KeyPaths<T, F>, predicate: P) -> Option<T>
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
        T: Clone,
    {
        self.iter.find_map(|lock| {
            lock.with_value(|item| {
                if path.get(item).map_or(false, |val| predicate(val)) {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .flatten()
        })
    }

    /// Count items matching an additional condition (terminal).
    /// 
    /// Like `count()` but with a field-specific predicate.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let expensive_count = products
    ///     .lock_lazy_query()
    ///     .where_(Product::category_r(), |c| c == "Electronics")
    ///     .count_where(Product::price_r(), |&p| p > 500.0);
    /// ```
    pub fn count_where<F, P>(self, path: KeyPaths<T, F>, predicate: P) -> usize
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
    {
        self.iter.filter(|lock| {
            lock.with_value(|item| {
                path.get(item).map_or(false, |val| predicate(val))
            })
            .unwrap_or(false)
        }).count()
    }

    // ========================================================================
    // DATETIME OPERATIONS - SystemTime
    // ========================================================================

    /// Filter by SystemTime being after a reference time.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = events
    ///     .lock_lazy_query()
    ///     .where_after_systemtime(Event::timestamp_r(), cutoff_time);
    /// ```
    pub fn where_after_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by SystemTime being before a reference time.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = events
    ///     .lock_lazy_query()
    ///     .where_before_systemtime(Event::timestamp_r(), cutoff_time);
    /// ```
    pub fn where_before_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by SystemTime being between two times (inclusive).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `start` - The start time
    /// * `end` - The end time
    ///
    /// # Example
    ///
    /// ```ignore
    /// let range = events
    ///     .lock_lazy_query()
    ///     .where_between_systemtime(Event::timestamp_r(), start, end);
    /// ```
    pub fn where_between_systemtime(
        self,
        path: KeyPaths<T, SystemTime>,
        start: SystemTime,
        end: SystemTime,
    ) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a> {
        self.where_(path, move |time| time >= &start && time <= &end)
    }

    // ========================================================================
    // ORDERING OPERATIONS (require T: Clone)
    // ========================================================================

    /// Orders results by a field in ascending order (terminal).
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    /// This is a terminal operation that collects and sorts all matching items.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .order_by(Product::name_r());
    /// ```
    pub fn order_by<F>(self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
        T: Clone,
    {
        let mut results: Vec<T> = self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect();

        results.sort_by_key(|item| path.get(item).cloned());
        results
    }

    /// Orders results by a field in descending order (terminal).
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    /// This is a terminal operation that collects and sorts all matching items.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .order_by_desc(Product::stock_r());
    /// ```
    pub fn order_by_desc<F>(self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
        T: Clone,
    {
        let mut results: Vec<T> = self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned();
            let b_val = path.get(b).cloned();
            b_val.cmp(&a_val)
        });
        results
    }

    /// Orders results by a float field in ascending order (terminal).
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    /// This is a terminal operation that collects and sorts all matching items.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .order_by_float(Product::price_r());
    /// ```
    pub fn order_by_float(self, path: KeyPaths<T, f64>) -> Vec<T>
    where
        T: Clone,
    {
        let mut results: Vec<T> = self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Orders results by a float field in descending order (terminal).
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    /// This is a terminal operation that collects and sorts all matching items.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .order_by_float_desc(Product::rating_r());
    /// ```
    pub fn order_by_float_desc(self, path: KeyPaths<T, f64>) -> Vec<T>
    where
        T: Clone,
    {
        let mut results: Vec<T> = self.iter
            .filter_map(|lock| lock.with_value(|item| item.clone()))
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    // ========================================================================
    // GROUPING OPERATIONS (require T: Clone)
    // ========================================================================

    /// Groups results by a field value (terminal).
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned copies in groups.
    /// This is a terminal operation that collects all matching items into groups.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to group by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let by_category = products
    ///     .lock_lazy_query()
    ///     .where_(Product::stock_r(), |&s| s > 0)
    ///     .group_by(Product::category_r());
    /// 
    /// for (category, products) in by_category {
    ///     println!("{}: {} products", category, products.len());
    /// }
    /// 
    /// // SQL equivalent: SELECT * FROM products WHERE stock > 0 GROUP BY category
    /// ```
    pub fn group_by<F>(self, path: KeyPaths<T, F>) -> HashMap<F, Vec<T>>
    where
        F: Eq + std::hash::Hash + Clone + 'static,
        T: Clone,
    {
        let mut groups: HashMap<F, Vec<T>> = HashMap::new();

        for lock in self.iter {
            if let Some(item) = lock.with_value(|item| item.clone()) {
                if let Some(key) = path.get(&item).cloned() {
                    groups.entry(key).or_insert_with(Vec::new).push(item);
                }
            }
        }

        groups
    }
}

// ========================================================================
// DATETIME OPERATIONS - Chrono (only available with datetime feature)
// ========================================================================

#[cfg(feature = "datetime")]
impl<'a, T: 'static, L, I> LockLazyQuery<'a, T, L, I>
where
    L: LockValue<T> + 'a,
    I: Iterator<Item = &'a L> + 'a,
{
    /// Filter by DateTime being after a reference time.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = events
    ///     .lock_lazy_query()
    ///     .where_after(Event::timestamp_r(), cutoff_time);
    /// ```
    pub fn where_after<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by DateTime being before a reference time.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = events
    ///     .lock_lazy_query()
    ///     .where_before(Event::timestamp_r(), cutoff_time);
    /// ```
    pub fn where_before<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by DateTime being between two times (inclusive).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `start` - The start time
    /// * `end` - The end time
    ///
    /// # Example
    ///
    /// ```ignore
    /// let range = events
    ///     .lock_lazy_query()
    ///     .where_between(Event::timestamp_r(), start, end);
    /// ```
    pub fn where_between<Tz>(
        self,
        path: KeyPaths<T, DateTime<Tz>>,
        start: DateTime<Tz>,
        end: DateTime<Tz>,
    ) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time >= &start && time <= &end)
    }

    /// Filter by DateTime being today.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `now` - The current DateTime to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let today = events
    ///     .lock_lazy_query()
    ///     .where_today(Event::timestamp_r(), Utc::now());
    /// ```
    pub fn where_today<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, now: DateTime<Tz>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| {
            time.date_naive() == now.date_naive()
        })
    }

    /// Filter by DateTime year.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `year` - The year to filter by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let this_year = events
    ///     .lock_lazy_query()
    ///     .where_year(Event::timestamp_r(), 2024);
    /// ```
    pub fn where_year<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, year: i32) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.year() == year)
    }

    /// Filter by DateTime month.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `month` - The month to filter by (1-12)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let december = events
    ///     .lock_lazy_query()
    ///     .where_month(Event::timestamp_r(), 12);
    /// ```
    pub fn where_month<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, month: u32) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.month() == month)
    }

    /// Filter by DateTime day.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `day` - The day to filter by (1-31)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = events
    ///     .lock_lazy_query()
    ///     .where_day(Event::timestamp_r(), 1);
    /// ```
    pub fn where_day<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, day: u32) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.day() == day)
    }

    /// Filter by weekend dates (Saturday and Sunday).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekend_events = events
    ///     .lock_lazy_query()
    ///     .where_weekend(Event::timestamp_r());
    /// ```
    pub fn where_weekend<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, |time| {
            let weekday = time.weekday().num_days_from_monday();
            weekday >= 5
        })
    }

    /// Filter by weekday dates (Monday through Friday).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekday_events = events
    ///     .lock_lazy_query()
    ///     .where_weekday(Event::timestamp_r());
    /// ```
    pub fn where_weekday<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, |time| {
            let weekday = time.weekday().num_days_from_monday();
            weekday < 5
        })
    }

    /// Filter by business hours (9 AM - 5 PM).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let business_hours = events
    ///     .lock_lazy_query()
    ///     .where_business_hours(Event::timestamp_r());
    /// ```
    pub fn where_business_hours<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LockLazyQuery<'a, T, L, impl Iterator<Item = &'a L> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Timelike;
        self.where_(path, |time| {
            let hour = time.hour();
            hour >= 9 && hour < 17
        })
    }
}


