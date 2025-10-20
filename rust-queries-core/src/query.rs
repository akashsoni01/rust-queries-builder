//! Query builder implementation for filtering, selecting, ordering, grouping, and aggregating data.
//!
//! This module provides the `Query` struct which enables SQL-like operations on collections
//! using type-safe key-paths.

use key_paths_core::KeyPaths;
use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(feature = "datetime")]
use chrono::{DateTime, TimeZone};

/// A query builder for filtering, selecting, ordering, grouping, and aggregating data.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the data being queried
/// * `T` - The type of items in the collection
///
/// # Example
///
/// ```ignore
/// let products = vec![/* ... */];
/// let query = Query::new(&products)
///     .where_(Product::price(), |&price| price < 100.0)
///     .order_by_float(Product::price());
/// ```
pub struct Query<'a, T: 'static> {
    data: &'a [T],
    filters: Vec<Box<dyn Fn(&T) -> bool>>,
}

// Core implementation without Clone requirement
impl<'a, T: 'static> Query<'a, T> {
    /// Creates a new query from a slice of data.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of items to query
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = Query::new(&products);
    /// ```
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            filters: Vec::new(),
        }
    }

    /// Adds a filter predicate using a key-path.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to filter on
    /// * `predicate` - A function that returns true for items to keep
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query = Query::new(&products)
    ///     .where_(Product::category(), |cat| cat == "Electronics");
    /// ```
    pub fn where_<F>(mut self, path: KeyPaths<T, F>, predicate: impl Fn(&F) -> bool + 'static) -> Self
    where
        F: 'static,
    {
        self.filters.push(Box::new(move |item| {
            path.get(item).map_or(false, |val| predicate(val))
        }));
        self
    }

    /// Returns all items matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = query.all();
    /// ```
    pub fn all(&self) -> Vec<&T> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .collect()
    }

    /// Returns the first item matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first = query.first();
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.data
            .iter()
            .find(|item| self.filters.iter().all(|f| f(item)))
    }

    /// Returns the count of items matching the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count();
    /// ```
    pub fn count(&self) -> usize {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .count()
    }

    /// Returns the first `n` items matching the query filters.
    ///
    /// # Arguments
    ///
    /// * `n` - The maximum number of items to return
    ///
    /// # Example
    ///
    /// ```ignore
    /// let first_10 = query.limit(10);
    /// ```
    pub fn limit(&self, n: usize) -> Vec<&T> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .take(n)
            .collect()
    }

    /// Skips the first `offset` items for pagination.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of items to skip
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2 = query.skip(20).limit(10);
    /// ```
    pub fn skip<'b>(&'b self, offset: usize) -> QueryWithSkip<'a, 'b, T> {
        QueryWithSkip {
            query: self,
            offset,
        }
    }

    /// Projects/selects a single field from results.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to select
    ///
    /// # Example
    ///
    /// ```ignore
    /// let names = query.select(Product::name());
    /// ```
    pub fn select<F>(&self, path: KeyPaths<T, F>) -> Vec<F>
    where
        F: Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect()
    }

    /// Computes the sum of a numeric field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the numeric field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total_price = query.sum(Product::price());
    /// ```
    pub fn sum<F>(&self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .fold(F::default(), |acc, val| acc + val)
    }

    /// Computes the average of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg_price = query.avg(Product::price()).unwrap_or(0.0);
    /// ```
    pub fn avg(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        let items: Vec<f64> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.iter().sum::<f64>() / items.len() as f64)
        }
    }

    /// Finds the minimum value of a field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min_stock = query.min(Product::stock());
    /// ```
    pub fn min<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Finds the maximum value of a field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max_stock = query.max(Product::stock());
    /// ```
    pub fn max<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static,
    {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Finds the minimum value of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min_price = query.min_float(Product::price());
    /// ```
    pub fn min_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Finds the maximum value of a float field.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max_price = query.max_float(Product::price());
    /// ```
    pub fn max_float(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Checks if any items match the query filters.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let has_results = query.exists();
    /// ```
    pub fn exists(&self) -> bool {
        self.data
            .iter()
            .any(|item| self.filters.iter().all(|f| f(item)))
    }

    // DateTime operations for SystemTime
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
    /// let recent = query.where_after_systemtime(Event::timestamp(), &cutoff_time);
    /// ```
    pub fn where_after_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> Self {
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
    /// let old = query.where_before_systemtime(Event::timestamp(), &cutoff_time);
    /// ```
    pub fn where_before_systemtime(self, path: KeyPaths<T, SystemTime>, reference: SystemTime) -> Self {
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
    /// let range = query.where_between_systemtime(Event::timestamp(), &start, &end);
    /// ```
    pub fn where_between_systemtime(
        self,
        path: KeyPaths<T, SystemTime>,
        start: SystemTime,
        end: SystemTime,
    ) -> Self {
        self.where_(path, move |time| time >= &start && time <= &end)
    }
}

// DateTime operations with chrono (only available with datetime feature)
#[cfg(feature = "datetime")]
impl<'a, T: 'static> Query<'a, T> {
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
    /// let recent = query.where_after(Event::timestamp(), &cutoff_time);
    /// ```
    pub fn where_after<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> Self
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
    /// let old = query.where_before(Event::timestamp(), &cutoff_time);
    /// ```
    pub fn where_before<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, reference: DateTime<Tz>) -> Self
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
    /// let range = query.where_between(Event::timestamp(), &start, &end);
    /// ```
    pub fn where_between<Tz>(
        self,
        path: KeyPaths<T, DateTime<Tz>>,
        start: DateTime<Tz>,
        end: DateTime<Tz>,
    ) -> Self
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
    /// let today = query.where_today(Event::timestamp(), &Utc::now());
    /// ```
    pub fn where_today<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, now: DateTime<Tz>) -> Self
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
    /// let this_year = query.where_year(Event::timestamp(), 2024);
    /// ```
    pub fn where_year<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, year: i32) -> Self
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
    /// let december = query.where_month(Event::timestamp(), 12);
    /// ```
    pub fn where_month<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, month: u32) -> Self
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
    /// let first = query.where_day(Event::timestamp(), 1);
    /// ```
    pub fn where_day<Tz>(self, path: KeyPaths<T, DateTime<Tz>>, day: u32) -> Self
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
    /// let weekend_events = query.where_weekend(Event::timestamp());
    /// ```
    pub fn where_weekend<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
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
    /// let weekday_events = query.where_weekday(Event::timestamp());
    /// ```
    pub fn where_weekday<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
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
    /// let business_hours = query.where_business_hours(Event::timestamp());
    /// ```
    pub fn where_business_hours<Tz>(self, path: KeyPaths<T, DateTime<Tz>>) -> Self
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

// Operations that require Clone - separated for flexibility
impl<'a, T: 'static + Clone> Query<'a, T> {
    /// Orders results by a field in ascending order.
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by(Product::name());
    /// ```
    pub fn order_by<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
    {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by_key(|item| path.get(item).cloned());
        results
    }

    /// Orders results by a field in descending order.
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_desc(Product::stock());
    /// ```
    pub fn order_by_desc<F>(&self, path: KeyPaths<T, F>) -> Vec<T>
    where
        F: Ord + Clone + 'static,
    {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned();
            let b_val = path.get(b).cloned();
            b_val.cmp(&a_val)
        });
        results
    }

    /// Orders results by a float field in ascending order.
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_float(Product::price());
    /// ```
    pub fn order_by_float(&self, path: KeyPaths<T, f64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Orders results by a float field in descending order.
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned sorted copies.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the f64 field to order by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_float_desc(Product::rating());
    /// ```
    pub fn order_by_float_desc(&self, path: KeyPaths<T, f64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0.0);
            let b_val = path.get(b).cloned().unwrap_or(0.0);
            b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Groups results by a field value.
    /// 
    /// **Note**: This method requires `T: Clone` as it creates owned copies in groups.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the field to group by
    ///
    /// # Example
    ///
    /// ```ignore
    /// let by_category = query.group_by(Product::category());
    /// ```
    pub fn group_by<F>(&self, path: KeyPaths<T, F>) -> HashMap<F, Vec<T>>
    where
        F: Eq + std::hash::Hash + Clone + 'static,
    {
        let mut groups: HashMap<F, Vec<T>> = HashMap::new();

        for item in self.data.iter() {
            if self.filters.iter().all(|f| f(item)) {
                if let Some(key) = path.get(item).cloned() {
                    groups.entry(key).or_insert_with(Vec::new).push(item.clone());
                }
            }
        }

        groups
    }

    // ============================================================================
    // i64 DateTime Aggregators (Unix timestamps in milliseconds)
    // ============================================================================

    /// Finds the minimum i64 timestamp value.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let earliest = query.min_timestamp(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn min_timestamp(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Finds the maximum i64 timestamp value.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let latest = query.max_timestamp(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn max_timestamp(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Calculates the average of i64 timestamp values.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg_timestamp = query.avg_timestamp(Event::created_at()).unwrap_or(0);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn avg_timestamp(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        let items: Vec<i64> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.iter().sum::<i64>() / items.len() as i64)
        }
    }

    /// Calculates the sum of i64 timestamp values.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total_timestamp = query.sum_timestamp(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn sum_timestamp(&self, path: KeyPaths<T, i64>) -> i64 {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter_map(|item| path.get(item).cloned())
            .sum()
    }

    /// Counts the number of non-null i64 timestamp values.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let timestamp_count = query.count_timestamp(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn count_timestamp(&self, path: KeyPaths<T, i64>) -> usize {
        self.data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .filter(|item| path.get(item).is_some())
            .count()
    }

    /// Filters by i64 timestamp being after a reference timestamp.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `reference` - The reference timestamp to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = query.where_after_timestamp(Event::created_at(), cutoff_timestamp);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_after_timestamp(self, path: KeyPaths<T, i64>, reference: i64) -> Self {
        self.where_(path, move |timestamp| timestamp > &reference)
    }

    /// Filters by i64 timestamp being before a reference timestamp.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `reference` - The reference timestamp to compare against
    ///
    /// # Example
    ///
    /// ```ignore
    /// let old = query.where_before_timestamp(Event::created_at(), cutoff_timestamp);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_before_timestamp(self, path: KeyPaths<T, i64>, reference: i64) -> Self {
        self.where_(path, move |timestamp| timestamp < &reference)
    }

    /// Filters by i64 timestamp being between two timestamps (inclusive).
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
    /// let range = query.where_between_timestamp(Event::created_at(), start_ts, end_ts);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_between_timestamp(self, path: KeyPaths<T, i64>, start: i64, end: i64) -> Self {
        self.where_(path, move |timestamp| timestamp >= &start && timestamp <= &end)
    }

    /// Filters by i64 timestamp being within the last N days from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `days` - Number of days to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = query.where_last_days_timestamp(Event::created_at(), 30);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_last_days_timestamp(self, path: KeyPaths<T, i64>, days: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (days * 24 * 60 * 60 * 1000); // Convert days to milliseconds
        self.where_after_timestamp(path, cutoff)
    }

    /// Filters by i64 timestamp being within the next N days from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `days` - Number of days to look ahead
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = query.where_next_days_timestamp(Event::scheduled_at(), 7);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_next_days_timestamp(self, path: KeyPaths<T, i64>, days: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (days * 24 * 60 * 60 * 1000); // Convert days to milliseconds
        self.where_before_timestamp(path, cutoff)
    }

    /// Filters by i64 timestamp being within the last N hours from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `hours` - Number of hours to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = query.where_last_hours_timestamp(Event::created_at(), 24);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_last_hours_timestamp(self, path: KeyPaths<T, i64>, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (hours * 60 * 60 * 1000); // Convert hours to milliseconds
        self.where_after_timestamp(path, cutoff)
    }

    /// Filters by i64 timestamp being within the next N hours from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `hours` - Number of hours to look ahead
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = query.where_next_hours_timestamp(Event::scheduled_at(), 2);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_next_hours_timestamp(self, path: KeyPaths<T, i64>, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (hours * 60 * 60 * 1000); // Convert hours to milliseconds
        self.where_before_timestamp(path, cutoff)
    }

    /// Filters by i64 timestamp being within the last N minutes from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `minutes` - Number of minutes to look back
    ///
    /// # Example
    ///
    /// ```ignore
    /// let recent = query.where_last_minutes_timestamp(Event::created_at(), 60);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_last_minutes_timestamp(self, path: KeyPaths<T, i64>, minutes: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now - (minutes * 60 * 1000); // Convert minutes to milliseconds
        self.where_after_timestamp(path, cutoff)
    }

    /// Filters by i64 timestamp being within the next N minutes from now.
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    /// * `minutes` - Number of minutes to look ahead
    ///
    /// # Example
    ///
    /// ```ignore
    /// let upcoming = query.where_next_minutes_timestamp(Event::scheduled_at(), 30);
    /// ```
    #[cfg(feature = "datetime")]
    pub fn where_next_minutes_timestamp(self, path: KeyPaths<T, i64>, minutes: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let cutoff = now + (minutes * 60 * 1000); // Convert minutes to milliseconds
        self.where_before_timestamp(path, cutoff)
    }

    /// Orders results by i64 timestamp in ascending order (oldest first).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_timestamp(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn order_by_timestamp(&self, path: KeyPaths<T, i64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0);
            let b_val = path.get(b).cloned().unwrap_or(0);
            a_val.cmp(&b_val)
        });
        results
    }

    /// Orders results by i64 timestamp in descending order (newest first).
    ///
    /// # Arguments
    ///
    /// * `path` - The key-path to the i64 timestamp field
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sorted = query.order_by_timestamp_desc(Event::created_at());
    /// ```
    #[cfg(feature = "datetime")]
    pub fn order_by_timestamp_desc(&self, path: KeyPaths<T, i64>) -> Vec<T> {
        let mut results: Vec<T> = self
            .data
            .iter()
            .filter(|item| self.filters.iter().all(|f| f(item)))
            .cloned()
            .collect();

        results.sort_by(|a, b| {
            let a_val = path.get(a).cloned().unwrap_or(0);
            let b_val = path.get(b).cloned().unwrap_or(0);
            b_val.cmp(&a_val)
        });
        results
    }
}

/// Helper struct for pagination after a skip operation.
///
/// Created by calling `skip()` on a `Query`.
pub struct QueryWithSkip<'a, 'b, T: 'static> {
    query: &'b Query<'a, T>,
    offset: usize,
}

impl<'a, 'b, T: 'static> QueryWithSkip<'a, 'b, T> {
    /// Returns up to `n` items after skipping the offset.
    ///
    /// # Arguments
    ///
    /// * `n` - The maximum number of items to return
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page_2 = query.skip(20).limit(10);
    /// ```
    pub fn limit(&self, n: usize) -> Vec<&'a T> {
        self.query
            .data
            .iter()
            .filter(|item| self.query.filters.iter().all(|f| f(item)))
            .skip(self.offset)
            .take(n)
            .collect()
    }
}

    // Parallel operations (only available with parallel feature)
    #[cfg(feature = "parallel")]
    impl<'a, T: 'static + Send + Sync> Query<'a, T> {
    /// Get all items using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = query.all_parallel();
    /// ```
    pub fn all_parallel(&self) -> Vec<&'a T> {
        use rayon::prelude::*;
        self.data.par_iter().collect()
    }

    /// Count all items using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count_parallel();
    /// ```
    pub fn count_parallel(&self) -> usize {
        use rayon::prelude::*;
        self.data.par_iter().count()
    }

    /// Check if any items exist using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let exists = query.exists_parallel();
    /// ```
    pub fn exists_parallel(&self) -> bool {
        use rayon::prelude::*;
        self.data.par_iter().any(|_| true)
    }

    /// Find minimum value using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let min = query.min_parallel(Product::price());
    /// ```
    pub fn min_parallel<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static + Send + Sync,
    {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Find maximum value using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let max = query.max_parallel(Product::price());
    /// ```
    pub fn max_parallel<F>(&self, path: KeyPaths<T, F>) -> Option<F>
    where
        F: Ord + Clone + 'static + Send + Sync,
    {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Compute sum using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sum = query.sum_parallel(Product::price());
    /// ```
    pub fn sum_parallel<F>(&self, path: KeyPaths<T, F>) -> F
    where
        F: Clone + std::ops::Add<Output = F> + Default + 'static + Send + Sync + std::iter::Sum,
    {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .sum()
    }

    /// Compute average using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = query.avg_parallel(Product::price());
    /// ```
    pub fn avg_parallel(&self, path: KeyPaths<T, f64>) -> Option<f64> {
        use rayon::prelude::*;
        let items: Vec<f64> = self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.par_iter().sum::<f64>() / items.len() as f64)
        }
    }

    /// Find minimum i64 timestamp using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let earliest = query.min_timestamp_parallel(Event::created_at());
    /// ```
    pub fn min_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .min()
    }

    /// Find maximum i64 timestamp using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let latest = query.max_timestamp_parallel(Event::created_at());
    /// ```
    pub fn max_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .max()
    }

    /// Compute average i64 timestamp using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let avg = query.avg_timestamp_parallel(Event::created_at());
    /// ```
    pub fn avg_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> Option<i64> {
        use rayon::prelude::*;
        let items: Vec<i64> = self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items.par_iter().sum::<i64>() / items.len() as i64)
        }
    }

    /// Compute sum of i64 timestamps using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let total = query.sum_timestamp_parallel(Event::created_at());
    /// ```
    pub fn sum_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> i64 {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter_map(|item| path.get(item).cloned())
            .sum()
    }

    /// Count i64 timestamps using parallel processing.
    /// Note: This method ignores filters for thread safety.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let count = query.count_timestamp_parallel(Event::created_at());
    /// ```
    pub fn count_timestamp_parallel(&self, path: KeyPaths<T, i64>) -> usize {
        use rayon::prelude::*;
        self.data
            .par_iter()
            .filter(|item| path.get(item).is_some())
            .count()
    }
}

