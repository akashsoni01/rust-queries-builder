//! Lazy query implementation using iterators.
//!
//! This module provides lazy evaluation of queries, deferring execution
//! until results are actually consumed.

use key_paths_core::KeyPaths;
use std::marker::PhantomData;
use std::time::SystemTime;

#[cfg(feature = "datetime")]
use chrono::{DateTime, TimeZone};

/// A lazy query builder that uses iterators for deferred execution.
///
/// Unlike the standard `Query`, `LazyQuery` doesn't execute until you call
/// a terminal operation like `.collect()`, `.count()`, or `.first()`.
///
/// # Benefits
///
/// - **Deferred execution**: No work until results needed
/// - **Iterator fusion**: Rust optimizes chained operations
/// - **Early termination**: `.take()` stops as soon as enough items found
/// - **Composable**: Build complex queries by composition
///
/// # Example
///
/// ```ignore
/// // Nothing executes yet
/// let query = LazyQuery::new(&products)
///     .where_(Product::price_r(), |&p| p < 100.0)
///     .where_(Product::stock_r(), |&s| s > 0);
///
/// // Execution happens here
/// let results: Vec<_> = query.collect();
/// ```
pub struct LazyQuery<'a, T: 'static, I>
where
    I: Iterator<Item = &'a T>,
{
    iter: I,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: 'static> LazyQuery<'a, T, std::slice::Iter<'a, T>> {
    /// Creates a new lazy query from a slice.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LazyQuery::new(&products);
    /// ```
    pub fn new(data: &'a [T]) -> Self {
        Self {
            iter: data.iter(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'static, I> LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T>,
{
    /// Creates a new lazy query from an iterator.
    ///
    /// This is useful for creating LazyQuery instances from custom iterators
    /// or for implementing extension traits.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let iter = vec![1, 2, 3].iter();
    /// let query = LazyQuery::from_iter(iter);
    /// ```
    pub fn from_iter(iter: I) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'static, I> LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T> + 'a,
{
    /// Adds a filter predicate (lazy - not executed yet).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = LazyQuery::new(&products)
    ///     .where_(Product::price_r(), |&p| p < 100.0);
    /// ```
    pub fn where_<F, P>(self, path: KeyPaths<T, F>, predicate: P) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        F: 'static,
        P: Fn(&F) -> bool + 'a,
    {
        LazyQuery {
            iter: self.iter.filter(move |item| {
                path.get(item).map_or(false, |val| predicate(val))
            }),
            _phantom: PhantomData,
        }
    }

    /// Maps each item through a transformation (lazy).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let prices = LazyQuery::new(&products)
    ///     .map_items(|p| p.price)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn map_items<F, O>(self, f: F) -> impl Iterator<Item = O> + 'a
    where
        F: Fn(&'a T) -> O + 'a,
        I: 'a,
    {
        self.iter.map(f)
    }

    /// Selects/projects a field value (lazy).
    ///
    /// Returns iterator over cloned field values.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let names: Vec<String> = LazyQuery::new(&products)
    ///     .select_lazy(Product::name_r())
    ///     .collect();
    /// ```
    pub fn select_lazy<F>(self, path: KeyPaths<T, F>) -> impl Iterator<Item = F> + 'a
    where
        F: Clone + 'static,
        I: 'a,
    {
        self.iter.filter_map(move |item| path.get(item).cloned())
    }

    /// Takes at most `n` items (lazy).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first_10: Vec<_> = LazyQuery::new(&products)
    ///     .take_lazy(10)
    ///     .collect();
    /// ```
    pub fn take_lazy(self, n: usize) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        I: 'a,
    {
        LazyQuery {
            iter: self.iter.take(n),
            _phantom: PhantomData,
        }
    }

    /// Skips `n` items (lazy).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2: Vec<_> = LazyQuery::new(&products)
    ///     .skip_lazy(10)
    ///     .take_lazy(10)
    ///     .collect();
    /// ```
    pub fn skip_lazy(self, n: usize) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        I: 'a,
    {
        LazyQuery {
            iter: self.iter.skip(n),
            _phantom: PhantomData,
        }
    }

    /// Collects all items into a vector (terminal operation - executes query).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<&Product> = query.collect();
    /// ```
    pub fn collect(self) -> Vec<&'a T> {
        self.iter.collect()
    }

    /// Gets the first item (terminal operation - executes until first match).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = query.first();
    /// ```
    pub fn first(mut self) -> Option<&'a T> {
        self.iter.next()
    }

    /// Counts items (terminal operation - executes query).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count();
    /// ```
    pub fn count(self) -> usize {
        self.iter.count()
    }

    /// Checks if any items match (terminal operation - short-circuits).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let exists = query.any();
    /// ```
    pub fn any(mut self) -> bool {
        self.iter.next().is_some()
    }

    /// Executes a function for each item (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// query.for_each(|item| println!("{:?}", item));
    /// ```
    pub fn for_each<F>(self, f: F)
    where
        F: FnMut(&'a T),
    {
        self.iter.for_each(f)
    }

    /// Folds the iterator (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sum = query.fold(0.0, |acc, item| acc + item.price);
    /// ```
    pub fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, &'a T) -> B,
    {
        self.iter.fold(init, f)
    }

    /// Finds an item matching a predicate (terminal - short-circuits).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let found = query.find(|item| item.id == 42);
    /// ```
    pub fn find<P>(mut self, predicate: P) -> Option<&'a T>
    where
        P: FnMut(&&'a T) -> bool,
    {
        self.iter.find(predicate)
    }

    /// Checks if all items match a predicate (terminal - short-circuits).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let all_positive = query.all_match(|item| item.value > 0);
    /// ```
    pub fn all_match<P>(mut self, mut predicate: P) -> bool
    where
        P: FnMut(&'a T) -> bool,
    {
        self.iter.all(move |item| predicate(item))
    }

    /// Collects all items into a vector (terminal operation - executes query).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results: Vec<&Product> = query.all();
    /// ```
    pub fn all(self) -> Vec<&'a T> {
        self.iter.collect()
    }

    /// Converts to a standard iterator for further chaining.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let custom: Vec<_> = query
    ///     .into_iter()
    ///     .map(|item| item.custom_transform())
    ///     .filter(|x| x.is_valid())
    ///     .collect();
    /// ```
    pub fn into_iter(self) -> I {
        self.iter
    }
}

// Aggregation operations
impl<'a, T: 'static, I> LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T> + 'a,
{
    /// Computes sum of a field (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total: f64 = LazyQuery::new(&products)
    ///     .sum_by(Product::price_r());
    /// ```
    pub fn sum_by<F>(self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static,
        I: 'a,
    {
        self.iter
            .filter_map(move |item| path.get(item).cloned())
            .fold(F::default(), |acc, val| acc + val)
    }

    /// Computes average of a float field (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = LazyQuery::new(&products)
    ///     .avg_by(Product::price_r());
    /// ```
    pub fn avg_by(self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        I: 'a,
    {
        let items: Vec<f64> = self
            .iter
            .filter_map(move |item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.iter().sum::<f64>() / items.len() as f64)
        }
    }

    /// Finds minimum value of a field (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min = LazyQuery::new(&products)
    ///     .min_by(Product::price_r());
    /// ```
    pub fn min_by<F>(self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
        I: 'a,
    {
        self.iter.filter_map(move |item| path.get(item).cloned()).min()
    }

    /// Finds maximum value of a field (terminal operation).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max = LazyQuery::new(&products)
    ///     .max_by(Product::price_r());
    /// ```
    pub fn max_by<F>(self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
        I: 'a,
    {
        self.iter.filter_map(move |item| path.get(item).cloned()).max()
    }

    /// Finds minimum float value (terminal operation).
    pub fn min_by_float(self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        I: 'a,
    {
        self.iter
            .filter_map(move |item| path.get(item).cloned())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Finds maximum float value (terminal operation).
    pub fn max_by_float(self, path: KeyPaths<T, f64>) -> Option<f64>
    where
        I: 'a,
    {
        self.iter
            .filter_map(move |item| path.get(item).cloned())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    // DateTime operations for SystemTime (lazy)
    /// Filter by SystemTime being after a reference time (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyQuery::new(&events)
    ///     .where_after_systemtime(Event::timestamp_r(), cutoff_time)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_after_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a> {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by SystemTime being before a reference time (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the SystemTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = LazyQuery::new(&events)
    ///     .where_before_systemtime(Event::timestamp_r(), cutoff_time)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_before_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a> {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by SystemTime being between two times (inclusive, lazy).
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
    /// let range = LazyQuery::new(&events)
    ///     .where_between_systemtime(Event::timestamp_r(), start, end)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_between_systemtime(
        self,
        path: KeyPaths<T, SystemTime>,
        start: SystemTime,
        end: SystemTime,
    ) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a> {
        self.where_(path, move |time| time >= &start && time <= &end)
    }
}

// DateTime operations with chrono (only available with datetime feature, lazy)
#[cfg(feature = "datetime")]
impl<'a, T: 'static, I> LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T> + 'a,
{
    /// Filter by DateTime being after a reference time (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = LazyQuery::new(&events)
    ///     .where_after(Event::timestamp_r(), cutoff_time)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_after<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time > &reference)
    }

    /// Filter by DateTime being before a reference time (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `reference` - The reference time to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = LazyQuery::new(&events)
    ///     .where_before(Event::timestamp_r(), cutoff_time)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_before<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time < &reference)
    }

    /// Filter by DateTime being between two times (inclusive, lazy).
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
    /// let range = LazyQuery::new(&events)
    ///     .where_between(Event::timestamp_r(), start, end)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_between<Tz>(
        self,
        path: KeyPaths<T, DateTime<Tz>>,
        start: DateTime<Tz>,
        end: DateTime<Tz>,
    ) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| time >= &start && time <= &end)
    }

    /// Filter by DateTime being today (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `now` - The current DateTime to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let today = LazyQuery::new(&events)
    ///     .where_today(Event::timestamp_r(), Utc::now())
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_today<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, now: DateTime<Tz>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        self.where_(path, move |time| {
            time.date_naive() == now.date_naive()
        })
    }

    /// Filter by DateTime year (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `year` - The year to filter by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let this_year = LazyQuery::new(&events)
    ///     .where_year(Event::timestamp_r(), 2024)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_year<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, year: i32) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.year() == year)
    }

    /// Filter by DateTime month (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `month` - The month to filter by (1-12)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let december = LazyQuery::new(&events)
    ///     .where_month(Event::timestamp_r(), 12)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_month<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, month: u32) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.month() == month)
    }

    /// Filter by DateTime day (lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    /// * `day` - The day to filter by (1-31)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = LazyQuery::new(&events)
    ///     .where_day(Event::timestamp_r(), 1)
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_day<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, day: u32) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
    where
        Tz: TimeZone + 'static,
        Tz::Offset: std::fmt::Display,
    {
        use chrono::Datelike;
        self.where_(path, move |time| time.day() == day)
    }

    /// Filter by weekend dates (Saturday and Sunday, lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekend_events = LazyQuery::new(&events)
    ///     .where_weekend(Event::timestamp_r())
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_weekend<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
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

    /// Filter by weekday dates (Monday through Friday, lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let weekday_events = LazyQuery::new(&events)
    ///     .where_weekday(Event::timestamp_r())
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_weekday<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
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

    /// Filter by business hours (9 AM - 5 PM, lazy).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the DateTime field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let business_hours = LazyQuery::new(&events)
    ///     .where_business_hours(Event::timestamp_r())
    ///     .collect::<Vec<_>>();
    /// ```
    pub fn where_business_hours<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> LazyQuery<'a, T, impl Iterator<Item = &'a T> + 'a>
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

// Enable using LazyQuery in for loops
impl<'a, T: 'static, I> IntoIterator for LazyQuery<'a, T, I>
where
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}

