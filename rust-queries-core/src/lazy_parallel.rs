//! Parallel lazy query implementation using rayon for parallel processing.
//!
//! This module provides parallel lazy evaluation of queries, deferring execution
//! until results are actually consumed, but using multiple threads for better performance.
//!
//! # Benefits
//!
//! - **Deferred execution**: No work until results needed
//! - **Parallel processing**: Utilizes multiple CPU cores
//! - **Iterator fusion**: Rust optimizes chained operations
//! - **Early termination**: `.take()` stops as soon as enough items found
//! - **Composable**: Build complex queries by composition
//! - **Thread-safe**: All operations are Send + Sync
//!
//! # Example
//!
//! ```ignore
//! // Nothing executes yet
//! let query = LazyParallelQuery::new(&products)
//!     .where_(Product::price(), |&p| p < 100.0)
//!     .where_(Product::stock(), |&s| s > 0);
//!
//! // Parallel execution happens here
//! let results: Vec<_> = query.collect_parallel();
//! ```

#[cfg(feature = "parallel")]
use {
    rayon::prelude::*,
    key_paths_core::KeyPaths,
    std::marker::PhantomData,
    std::time::SystemTime,
};

#[cfg(feature = "datetime")]
use chrono::{DateTime, TimeZone};

/// A parallel lazy query builder that uses rayon for parallel processing.
///
/// Unlike the standard `LazyQuery`, `LazyParallelQuery` uses parallel iterators
/// for better performance on large datasets while maintaining lazy evaluation.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the data being queried
/// * `T` - The type of items in the collection
///
/// # Example
///
/// ```ignore
/// let query = LazyParallelQuery::new(&products)
///     .where_(Product::price(), |&p| p < 100.0)
///     .collect_parallel();
/// ```
#[cfg(feature = "parallel")]
pub struct LazyParallelQuery<'a, T: 'static + Send + Sync> {
    data: &'a [T],
    filters: Vec<Box<dyn Fn(&T) -> bool + Send + Sync>>,
    _phantom: PhantomData<&'a T>,
}

#[cfg(feature = "parallel")]
impl<'a, T: 'static + Send + Sync> LazyParallelQuery<'a, T> {
    /// Creates a new parallel lazy query from a slice.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LazyParallelQuery::new(&products);
    /// ```
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            filters: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Adds a filter predicate (lazy - not executed yet).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LazyParallelQuery::new(&products)
    ///     .where_(Product::price(), |&p| p < 100.0);
    /// ```
    pub fn where_<F>(mut self, path: KeyPaths<T, F>, predicate: impl Fn(&F) -> bool + Send + Sync + 'static) -> Self
    where
        F: 'static,
    {
        self.filters.push(Box::new(move |item| {
            path.get(item).map_or(false, |val| predicate(val))
        }));
        self
    }

    /// Collects all items into a vector (terminal operation - executes query in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<&Product> = query.collect_parallel();
    /// ```
    pub fn collect_parallel(&self) -> Vec<&'a T> {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect()
    }

    /// Gets the first item (terminal operation - executes until first match in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = query.first_parallel();
    /// ```
    pub fn first_parallel(&self) -> Option<&'a T> {
        self.data
            .par_iter()
            .find_any(|item| self.filters.iter().all(|f| f(item)))
    }

    /// Counts items (terminal operation - executes query in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count_parallel();
    /// ```
    pub fn count_parallel(&self) -> usize {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .count()
    }

    /// Checks if any items match (terminal operation - short-circuits in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let exists = query.any_parallel();
    /// ```
    pub fn any_parallel(&self) -> bool {
        self.data
            .par_iter()
            .any(|item| self.filters.iter().all(|f| f(item)))
    }

    /// Checks if all items match a predicate (terminal operation - short-circuits in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let all_positive = query.all_match_parallel(|item| item.value > 0);
    /// ```
    pub fn all_match_parallel<P>(&self, predicate: P) -> bool
    where
        P: Fn(&'a T) -> bool + Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .all(predicate)
    }

    /// Executes a function for each item (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// query.for_each_parallel(|item| println!("{:?}", item));
    /// ```
    pub fn for_each_parallel<F>(&self, f: F)
    where
        F: Fn(&'a T) + Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .for_each(f)
    }

    /// Folds the iterator (terminal operation in parallel).
    ///
    /// Note: This is a simplified implementation that collects all items first.
    /// For true parallel folding with custom reduction, consider using the sequential version
    /// or implementing a custom parallel reduction strategy.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sum = query.fold_parallel(0.0, |acc, item| acc + item.price);
    /// ```
    pub fn fold_parallel<B, F>(&self, init: B, f: F) -> B
    where
        B: Send + Sync,
        F: Fn(B, &'a T) -> B + Send + Sync,
    {
        let items: Vec<&'a T> = self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect();
        
        items.into_iter().fold(init, f)
    }

    /// Finds an item matching a predicate (terminal - short-circuits in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let found = query.find_parallel(|item| item.id == 42);
    /// ```
    pub fn find_parallel<P>(&self, predicate: P) -> Option<&'a T>
    where
        P: Fn(&&'a T) -> bool + Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .find_any(predicate)
    }

    /// Projects/selects a single field from results (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to select
    ///
    /// # Example
    ///
    /// ```ignore
    /// let names = query.select_parallel(Product::name());
    /// ```
    pub fn select_parallel<F>(&self, path: KeyPaths<T, F>) -> Vec<F>
    where
        F: Clone + Send + Sync + 'static,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect()
    }

    /// Maps each item through a transformation (parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let prices = query.map_items_parallel(|p| p.price);
    /// ```
    pub fn map_items_parallel<F, O>(&self, f: F) -> Vec<O>
    where
        F: Fn(&'a T) -> O + Send + Sync,
        O: Send,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .map(f)
            .collect()
    }

    /// Takes at most `n` items (parallel).
    ///
    /// Note: This collects all filtered items and then takes the first n.
    /// For true lazy evaluation with take, use the sequential LazyQuery.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first_10: Vec<_> = query.take_parallel(10);
    /// ```
    pub fn take_parallel(&self, n: usize) -> Vec<&'a T> {
        let mut results: Vec<&'a T> = self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect();
        results.truncate(n);
        results
    }

    /// Skips `n` items (parallel).
    ///
    /// Note: This collects all filtered items and then skips the first n.
    /// For true lazy evaluation with skip, use the sequential LazyQuery.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2: Vec<_> = query.skip_parallel(10);
    /// ```
    pub fn skip_parallel(&self, n: usize) -> Vec<&'a T> {
        let results: Vec<&'a T> = self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect();
        results.into_iter().skip(n).collect()
    }
}

// Aggregation operations (parallel)
#[cfg(feature = "parallel")]
impl<'a, T: 'static + Send + Sync> LazyParallelQuery<'a, T> {
    /// Computes sum of a field (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total: f64 = LazyParallelQuery::new(&products)
    ///     .sum_by_parallel(Product::price());
    /// ```
    pub fn sum_by_parallel<F>(&self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + Send + Sync + std::iter::Sum + 'static,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .sum()
    }

    /// Computes average of a float field (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = LazyParallelQuery::new(&products)
    ///     .avg_by_parallel(Product::price());
    /// ```
    pub fn avg_by_parallel(&self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        T: Send + Sync,
    {
        let items: Vec<f64> = self
            .data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.par_iter().sum::<f64>() / items.len() as f64)
        }
    }

    /// Finds minimum value of a field (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min = LazyParallelQuery::new(&products)
    ///     .min_by_parallel(Product::price());
    /// ```
    pub fn min_by_parallel<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + Send + Sync + 'static,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Finds maximum value of a field (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max = LazyParallelQuery::new(&products)
    ///     .max_by_parallel(Product::price());
    /// ```
    pub fn max_by_parallel<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + Send + Sync + 'static,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Finds minimum float value (terminal operation in parallel).
    pub fn min_by_float_parallel(&self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Finds maximum float value (terminal operation in parallel).
    pub fn max_by_float_parallel(&self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
}

// DateTime operations for SystemTime (parallel)
#[cfg(feature = "parallel")]
impl<'a, T: 'static + Send + Sync> LazyParallelQuery<'a, T> {
    /// Filter by SystemTime being after a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_after_systemtime_parallel(Event::timestamp(), cutoff_time);
    /// ```
    pub fn where_after_systemtime_parallel(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> Self {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by SystemTime being before a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = LazyParallelQuery::new(&events)
    ///     .where_before_systemtime_parallel(Event::timestamp(), cutoff_time);
    /// ```
    pub fn where_before_systemtime_parallel(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> Self {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by SystemTime being between two times (inclusive, parallel).
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
    /// let range = LazyParallelQuery::new(&events)
    ///     .where_between_systemtime_parallel(Event::timestamp(), start, end);
    /// ```
    pub fn where_between_systemtime_parallel(
        self,
        path: KeyPaths<T, SystemTime>,
        start: SystemTime,
        end: SystemTime,
    ) -> Self {
        self.where_(path, move |time| time >= &start && time <= &end)
    }
}

// DateTime operations with chrono (only available with datetime feature, parallel)
#[cfg(all(feature = "parallel", feature = "datetime"))]
impl<'a, T: 'static + Send + Sync> LazyParallelQuery<'a, T> {
    /// Filter by DateTime being after a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_after_parallel(Event::timestamp(), cutoff_time);
    /// ```
    pub fn where_after_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by DateTime being before a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = LazyParallelQuery::new(&events)
    ///     .where_before_parallel(Event::timestamp(), cutoff_time);
    /// ```
    pub fn where_before_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by DateTime being between two times (inclusive, parallel).
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
    /// let range = LazyParallelQuery::new(&events)
    ///     .where_between_parallel(Event::timestamp(), start, end);
    /// ```
    pub fn where_between_parallel<Tz>(
        self,
        path: KeyPaths<T, DateTime<Tz>>,
        start: DateTime<Tz>,
        end: DateTime<Tz>,
    ) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        self.where_(path, move |time| time >= &start && time <= &end)
    }

    /// Filter by DateTime being today (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `now` - The current DateTime to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let today = LazyParallelQuery::new(&events)
    ///     .where_today_parallel(Event::timestamp(), Utc::now());
    /// ```
    pub fn where_today_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, now: DateTime<Tz>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        self.where_(path, move |time| {
            time.date_naive() == now.date_naive()
        })
    }

    /// Filter by DateTime year (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `year` - The year to filter by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let this_year = LazyParallelQuery::new(&events)
    ///     .where_year_parallel(Event::timestamp(), 2024);
    /// ```
    pub fn where_year_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, year: i32) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.year() == year)
    }

    /// Filter by DateTime month (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `month` - The month to filter by (1-12)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let december = LazyParallelQuery::new(&events)
    ///     .where_month_parallel(Event::timestamp(), 12);
    /// ```
    pub fn where_month_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, month: u32) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.month() == month)
    }

    /// Filter by DateTime day (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `day` - The day to filter by (1-31)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = LazyParallelQuery::new(&events)
    ///     .where_day_parallel(Event::timestamp(), 1);
    /// ```
    pub fn where_day_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, day: u32) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.day() == day)
    }

    /// Filter by weekend dates (Saturday and Sunday, parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekend_events = LazyParallelQuery::new(&events)
    ///     .where_weekend_parallel(Event::timestamp());
    /// ```
    pub fn where_weekend_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Datelike;
        self.where_(path, |time| {
            let weekday = time.weekday().num_days_from_monday();
            weekday >= 5
        })
    }

    /// Filter by weekday dates (Monday through Friday, parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekday_events = LazyParallelQuery::new(&events)
    ///     .where_weekday_parallel(Event::timestamp());
    /// ```
    pub fn where_weekday_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Datelike;
        self.where_(path, |time| {
            let weekday = time.weekday().num_days_from_monday();
            weekday < 5
        })
    }

    /// Filter by business hours (9 AM - 5 PM, parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let business_hours = LazyParallelQuery::new(&events)
    ///     .where_business_hours_parallel(Event::timestamp());
    /// ```
    pub fn where_business_hours_parallel<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display + Send + Sync,
    {
        use chrono::Timelike;
        self.where_(path, |time| {
            let hour = time.hour();
            hour >= 9 && hour < 17
        })
    }
}

// i64 DateTime Aggregators (Unix timestamps in milliseconds, parallel)
#[cfg(feature = "parallel")]
impl<'a, T: 'static + Send + Sync> LazyParallelQuery<'a, T> {
    /// Finds minimum i64 timestamp value (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let earliest = LazyParallelQuery::new(&events)
    ///     .min_timestamp_parallel(Event::created_at_r());
    /// ```
    pub fn min_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64>
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Finds maximum i64 timestamp value (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let latest = LazyParallelQuery::new(&events)
    ///     .max_timestamp_parallel(Event::created_at_r());
    /// ```
    pub fn max_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64>
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Computes average of i64 timestamp values (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = LazyParallelQuery::new(&events)
    ///     .avg_timestamp_parallel(Event::created_at_r());
    /// ```
    pub fn avg_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64>
    where
        T: Send + Sync,
    {
        let items: Vec<i64> = self
            .data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.par_iter().sum::<i64>() / items.len() as i64)
        }
    }

    /// Computes sum of i64 timestamp values (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total = LazyParallelQuery::new(&events)
    ///     .sum_timestamp_parallel(Event::created_at_r());
    /// ```
    pub fn sum_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> i64
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .sum()
    }

    /// Counts i64 timestamp values (terminal operation in parallel).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = LazyParallelQuery::new(&events)
    ///     .count_timestamp_parallel(Event::created_at_r());
    /// ```
    pub fn count_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> usize
    where
        T: Send + Sync,
    {
        self.data
            .par_iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter(|item| path.get(item).is_some())
            .count()
    }

    /// Filter by i64 timestamp being after a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `reference` - The reference timestamp to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_after_timestamp_parallel(Event::created_at_r(), cutoff_time);
    /// ```
    pub fn where_after_timestamp_parallel(self, path: KeyPaths<T, i64>, reference: i64) -> Self {
        self.where_(path, move |timestamp| timestamp > &reference)
    }

    /// Filter by i64 timestamp being before a reference time (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `reference` - The reference timestamp to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = LazyParallelQuery::new(&events)
    ///     .where_before_timestamp_parallel(Event::created_at_r(), cutoff_time);
    /// ```
    pub fn where_before_timestamp_parallel(self, path: KeyPaths<T, i64>, reference: i64) -> Self {
        self.where_(path, move |timestamp| timestamp < &reference)
    }

    /// Filter by i64 timestamp being between two times (inclusive, parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `start` - The start timestamp
    /// * `end` - The end timestamp
    ///
    /// # Example
    ///
    /// ```ignore
    /// let range = LazyParallelQuery::new(&events)
    ///     .where_between_timestamp_parallel(Event::created_at_r(), start, end);
    /// ```
    pub fn where_between_timestamp_parallel(
        self,
        path: KeyPaths<T, i64>,
        start: i64,
        end: i64,
    ) -> Self {
        self.where_(path, move |timestamp| timestamp >= &start && timestamp <= &end)
    }

    /// Filter by i64 timestamp being within the last N days (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `days` - Number of days to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_last_days_timestamp_parallel(Event::created_at_r(), 30);
    /// ```
    pub fn where_last_days_timestamp_parallel(self, path: KeyPaths<T, i64>, days: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (days * 24 * 60 * 60 * 1000); // Convert days to milliseconds
        self.where_after_timestamp_parallel(path, cutoff)
    }

    /// Filter by i64 timestamp being within the next N days (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `days` - Number of days to look forward
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = LazyParallelQuery::new(&events)
    ///     .where_next_days_timestamp_parallel(Event::scheduled_at_r(), 7);
    /// ```
    pub fn where_next_days_timestamp_parallel(self, path: KeyPaths<T, i64>, days: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (days * 24 * 60 * 60 * 1000); // Convert days to milliseconds
        self.where_before_timestamp_parallel(path, cutoff)
    }

    /// Filter by i64 timestamp being within the last N hours (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `hours` - Number of hours to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_last_hours_timestamp_parallel(Event::created_at_r(), 24);
    /// ```
    pub fn where_last_hours_timestamp_parallel(self, path: KeyPaths<T, i64>, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (hours * 60 * 60 * 1000); // Convert hours to milliseconds
        self.where_after_timestamp_parallel(path, cutoff)
    }

    /// Filter by i64 timestamp being within the next N hours (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `hours` - Number of hours to look forward
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = LazyParallelQuery::new(&events)
    ///     .where_next_hours_timestamp_parallel(Event::scheduled_at_r(), 2);
    /// ```
    pub fn where_next_hours_timestamp_parallel(self, path: KeyPaths<T, i64>, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (hours * 60 * 60 * 1000); // Convert hours to milliseconds
        self.where_before_timestamp_parallel(path, cutoff)
    }

    /// Filter by i64 timestamp being within the last N minutes (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `minutes` - Number of minutes to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyParallelQuery::new(&events)
    ///     .where_last_minutes_timestamp_parallel(Event::created_at_r(), 60);
    /// ```
    pub fn where_last_minutes_timestamp_parallel(self, path: KeyPaths<T, i64>, minutes: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (minutes * 60 * 1000); // Convert minutes to milliseconds
        self.where_after_timestamp_parallel(path, cutoff)
    }

    /// Filter by i64 timestamp being within the next N minutes (parallel).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `minutes` - Number of minutes to look forward
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = LazyParallelQuery::new(&events)
    ///     .where_next_minutes_timestamp_parallel(Event::scheduled_at_r(), 30);
    /// ```
    pub fn where_next_minutes_timestamp_parallel(self, path: KeyPaths<T, i64>, minutes: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (minutes * 60 * 1000); // Convert minutes to milliseconds
        self.where_before_timestamp_parallel(path, cutoff)
    }
}

/// Extension trait for easy access to parallel lazy queries.
///
/// This trait provides a convenient method to create parallel lazy queries
/// from collections that implement the required traits.
///
/// # Example
///
/// ```ignore
/// use rust_queries_core::LazyParallelQueryExt;
///
/// let products = vec![/* ... */];
/// let query = products.lazy_parallel_query()
///     .where_(Product::price(), |&p| p < 100.0)
///     .collect_parallel();
/// ```
#[cfg(feature = "parallel")]
pub trait LazyParallelQueryExt<T: 'static + Send + Sync> {
    /// Creates a new parallel lazy query from the collection.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = products.lazy_parallel_query();
    /// ```
    fn lazy_parallel_query(&self) -> LazyParallelQuery<T>;
}

#[cfg(feature = "parallel")]
impl<T: 'static + Send + Sync> LazyParallelQueryExt<T> for [T] {
    fn lazy_parallel_query(&self) -> LazyParallelQuery<T> {
        LazyParallelQuery::new(self)
    }
}

#[cfg(feature = "parallel")]
impl<T: 'static + Send + Sync> LazyParallelQueryExt<T> for Vec<T> {
    fn lazy_parallel_query(&self) -> LazyParallelQuery<T> {
        LazyParallelQuery::new(self)
    }
}
