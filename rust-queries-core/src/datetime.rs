//! DateTime operations for query builder.
//!
//! This module provides datetime comparison and manipulation operations that can be used
//! with the query builder. It supports both `std::time::SystemTime` and optionally
//! `chrono` types when the `datetime` feature is enabled.
//!
//! # Features
//!
//! - Date comparisons (before, after, between)
//! - Time range queries
//! - Date arithmetic (add/subtract days, hours, etc.)
//! - Date extraction (year, month, day, hour, etc.)
//! - Timezone-aware operations (with chrono)
//!
//! # Example
//!
//! ```ignore
//! use rust_queries_core::{Query, datetime::*};
//! use chrono::{DateTime, Utc};
//!
//! #[derive(Keypath)]
//! struct Event {
//!     name: String,
//!     timestamp: DateTime<Utc>,
//! }
//!
//! let events = vec![/* ... */];
//! let recent = Query::new(&events)
//!     .where_(Event::timestamp(), |ts| {
//!         is_after(ts, &Utc::now() - chrono::Duration::days(7))
//!     });
//! ```

use std::time::{SystemTime, Duration, UNIX_EPOCH};

#[cfg(feature = "datetime")]
pub use chrono;

/// Check if a SystemTime is after another SystemTime
pub fn is_after_systemtime(time: &SystemTime, reference: &SystemTime) -> bool {
    time > reference
}

/// Check if a SystemTime is before another SystemTime
pub fn is_before_systemtime(time: &SystemTime, reference: &SystemTime) -> bool {
    time < reference
}

/// Check if a SystemTime is between two SystemTimes (inclusive)
pub fn is_between_systemtime(time: &SystemTime, start: &SystemTime, end: &SystemTime) -> bool {
    time >= start && time <= end
}

/// Check if a SystemTime is within a duration from now
pub fn is_within_duration_systemtime(time: &SystemTime, duration: Duration) -> bool {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => {
            if let Ok(time_dur) = time.duration_since(UNIX_EPOCH) {
                let diff = if now > time_dur {
                    now - time_dur
                } else {
                    time_dur - now
                };
                diff <= duration
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Add duration to SystemTime
pub fn add_duration_systemtime(time: &SystemTime, duration: Duration) -> SystemTime {
    *time + duration
}

/// Subtract duration from SystemTime
pub fn subtract_duration_systemtime(time: &SystemTime, duration: Duration) -> SystemTime {
    *time - duration
}

// Chrono-specific operations (only available with datetime feature)
#[cfg(feature = "datetime")]
pub mod chrono_ops {
    use chrono::{DateTime, TimeZone, Datelike, Timelike, Duration};

    /// Check if a DateTime is after another DateTime
    pub fn is_after<Tz: TimeZone>(time: &DateTime<Tz>, reference: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time > reference
    }

    /// Check if a DateTime is before another DateTime
    pub fn is_before<Tz: TimeZone>(time: &DateTime<Tz>, reference: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time < reference
    }

    /// Check if a DateTime is between two DateTimes (inclusive)
    pub fn is_between<Tz: TimeZone>(
        time: &DateTime<Tz>,
        start: &DateTime<Tz>,
        end: &DateTime<Tz>,
    ) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time >= start && time <= end
    }

    /// Check if a DateTime is today
    pub fn is_today<Tz: TimeZone>(time: &DateTime<Tz>, now: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time.date_naive() == now.date_naive()
    }

    /// Check if a DateTime is within a duration from now
    pub fn is_within_duration<Tz: TimeZone>(
        time: &DateTime<Tz>,
        now: &DateTime<Tz>,
        duration: Duration,
    ) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        let diff = if time > now {
            time.clone() - now.clone()
        } else {
            now.clone() - time.clone()
        };
        diff <= duration
    }

    /// Check if two DateTimes are on the same day
    pub fn is_same_day<Tz: TimeZone>(time1: &DateTime<Tz>, time2: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time1.date_naive() == time2.date_naive()
    }

    /// Check if a DateTime is in the past
    pub fn is_past<Tz: TimeZone>(time: &DateTime<Tz>, now: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time < now
    }

    /// Check if a DateTime is in the future
    pub fn is_future<Tz: TimeZone>(time: &DateTime<Tz>, now: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        time > now
    }

    /// Extract year from DateTime
    pub fn extract_year<Tz: TimeZone>(time: &DateTime<Tz>) -> i32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.year()
    }

    /// Extract month from DateTime (1-12)
    pub fn extract_month<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.month()
    }

    /// Extract day from DateTime (1-31)
    pub fn extract_day<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.day()
    }

    /// Extract hour from DateTime (0-23)
    pub fn extract_hour<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.hour()
    }

    /// Extract minute from DateTime (0-59)
    pub fn extract_minute<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.minute()
    }

    /// Extract second from DateTime (0-59)
    pub fn extract_second<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.second()
    }

    /// Get day of week (Monday = 0, Sunday = 6)
    pub fn day_of_week<Tz: TimeZone>(time: &DateTime<Tz>) -> u32
    where
        Tz::Offset: std::fmt::Display,
    {
        time.weekday().num_days_from_monday()
    }

    /// Check if a DateTime is on a weekend (Saturday or Sunday)
    pub fn is_weekend<Tz: TimeZone>(time: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        let weekday = time.weekday().num_days_from_monday();
        weekday >= 5 // Saturday = 5, Sunday = 6
    }

    /// Check if a DateTime is on a weekday (Monday-Friday)
    pub fn is_weekday<Tz: TimeZone>(time: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        !is_weekend(time)
    }

    /// Add days to a DateTime
    pub fn add_days<Tz: TimeZone>(time: &DateTime<Tz>, days: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() + Duration::days(days)
    }

    /// Add hours to a DateTime
    pub fn add_hours<Tz: TimeZone>(time: &DateTime<Tz>, hours: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() + Duration::hours(hours)
    }

    /// Add minutes to a DateTime
    pub fn add_minutes<Tz: TimeZone>(time: &DateTime<Tz>, minutes: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() + Duration::minutes(minutes)
    }

    /// Subtract days from a DateTime
    pub fn subtract_days<Tz: TimeZone>(time: &DateTime<Tz>, days: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() - Duration::days(days)
    }

    /// Subtract hours from a DateTime
    pub fn subtract_hours<Tz: TimeZone>(time: &DateTime<Tz>, hours: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() - Duration::hours(hours)
    }

    /// Subtract minutes from a DateTime
    pub fn subtract_minutes<Tz: TimeZone>(time: &DateTime<Tz>, minutes: i64) -> DateTime<Tz>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.clone() - Duration::minutes(minutes)
    }

    /// Get the start of day (midnight) for a DateTime
    pub fn start_of_day<Tz: TimeZone + Clone>(time: &DateTime<Tz>) -> Option<DateTime<Tz>>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.date_naive()
            .and_hms_opt(0, 0, 0)
            .and_then(|naive| time.timezone().from_local_datetime(&naive).single())
    }

    /// Get the end of day (23:59:59) for a DateTime
    pub fn end_of_day<Tz: TimeZone + Clone>(time: &DateTime<Tz>) -> Option<DateTime<Tz>>
    where
        Tz::Offset: std::fmt::Display,
    {
        time.date_naive()
            .and_hms_opt(23, 59, 59)
            .and_then(|naive| time.timezone().from_local_datetime(&naive).single())
    }

    /// Check if a DateTime falls within business hours (9 AM - 5 PM)
    pub fn is_business_hours<Tz: TimeZone>(time: &DateTime<Tz>) -> bool
    where
        Tz::Offset: std::fmt::Display,
    {
        let hour = time.hour();
        hour >= 9 && hour < 17
    }

    /// Calculate the number of days between two DateTimes
    pub fn days_between<Tz: TimeZone>(time1: &DateTime<Tz>, time2: &DateTime<Tz>) -> i64
    where
        Tz::Offset: std::fmt::Display,
    {
        let diff = if time1 > time2 {
            time1.clone() - time2.clone()
        } else {
            time2.clone() - time1.clone()
        };
        diff.num_days()
    }

    /// Calculate the number of hours between two DateTimes
    pub fn hours_between<Tz: TimeZone>(time1: &DateTime<Tz>, time2: &DateTime<Tz>) -> i64
    where
        Tz::Offset: std::fmt::Display,
    {
        let diff = if time1 > time2 {
            time1.clone() - time2.clone()
        } else {
            time2.clone() - time1.clone()
        };
        diff.num_hours()
    }
}

#[cfg(test)]
#[cfg(feature = "datetime")]
mod tests {
    use super::*;
    use chrono::{Utc, Duration, TimeZone};

    #[test]
    fn test_is_after() {
        let now = Utc::now();
        let future = now + Duration::hours(1);
        let past = now - Duration::hours(1);

        assert!(chrono_ops::is_after(&future, &now));
        assert!(!chrono_ops::is_after(&past, &now));
    }

    #[test]
    fn test_is_between() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let middle = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let before = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        assert!(chrono_ops::is_between(&middle, &start, &end));
        assert!(!chrono_ops::is_between(&before, &start, &end));
    }

    #[test]
    fn test_date_extraction() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 15, 14, 30, 45).unwrap();

        assert_eq!(chrono_ops::extract_year(&dt), 2024);
        assert_eq!(chrono_ops::extract_month(&dt), 3);
        assert_eq!(chrono_ops::extract_day(&dt), 15);
        assert_eq!(chrono_ops::extract_hour(&dt), 14);
        assert_eq!(chrono_ops::extract_minute(&dt), 30);
        assert_eq!(chrono_ops::extract_second(&dt), 45);
    }

    #[test]
    fn test_date_arithmetic() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap();
        let future = chrono_ops::add_days(&dt, 10);
        let past = chrono_ops::subtract_days(&dt, 5);

        assert_eq!(chrono_ops::extract_day(&future), 25);
        assert_eq!(chrono_ops::extract_day(&past), 10);
    }

    #[test]
    fn test_is_weekend() {
        // Saturday, March 16, 2024
        let saturday = Utc.with_ymd_and_hms(2024, 3, 16, 12, 0, 0).unwrap();
        // Monday, March 18, 2024
        let monday = Utc.with_ymd_and_hms(2024, 3, 18, 12, 0, 0).unwrap();

        assert!(chrono_ops::is_weekend(&saturday));
        assert!(!chrono_ops::is_weekend(&monday));
        assert!(chrono_ops::is_weekday(&monday));
    }

    #[test]
    fn test_is_business_hours() {
        let morning = Utc.with_ymd_and_hms(2024, 3, 15, 10, 0, 0).unwrap();
        let evening = Utc.with_ymd_and_hms(2024, 3, 15, 18, 0, 0).unwrap();

        assert!(chrono_ops::is_business_hours(&morning));
        assert!(!chrono_ops::is_business_hours(&evening));
    }
}

