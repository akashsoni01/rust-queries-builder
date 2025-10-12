# DateTime Operations Guide

This guide covers the datetime operations available in rust-queries-builder, including both standard `SystemTime` operations and advanced `chrono` integration.

## Table of Contents

- [Overview](#overview)
- [Feature Flag](#feature-flag)
- [SystemTime Operations](#systemtime-operations)
- [Chrono Operations](#chrono-operations)
- [Query Builder Integration](#query-builder-integration)
- [Helper Functions](#helper-functions)
- [Examples](#examples)

## Overview

The datetime module provides comprehensive datetime comparison and manipulation operations for the query builder. It supports:

- **Standard Library**: Basic operations with `std::time::SystemTime`
- **Chrono (optional)**: Advanced datetime operations with timezone support

## Feature Flag

To enable chrono-based datetime operations, add the `datetime` feature to your `Cargo.toml`:

```toml
[dependencies]
rust-queries-builder = { version = "0.7.0", features = ["datetime"] }
chrono = "0.4"
```

Without the feature flag, you can still use `SystemTime` for basic datetime operations.

## SystemTime Operations

These operations are available without any feature flags:

### Basic Comparisons

```rust
use rust_queries_builder::Query;
use std::time::{SystemTime, Duration};

let events = vec![/* ... */];

// Filter events after a specific time
let cutoff = SystemTime::now() - Duration::from_secs(3600); // 1 hour ago
let recent = Query::new(&events)
    .where_after_systemtime(Event::created_at_r(), cutoff);

// Filter events before a specific time
let old = Query::new(&events)
    .where_before_systemtime(Event::created_at_r(), cutoff);

// Filter events within a time range
let start = SystemTime::now() - Duration::from_secs(7200);
let end = SystemTime::now();
let range = Query::new(&events)
    .where_between_systemtime(Event::created_at_r(), start, end);
```

## Chrono Operations

With the `datetime` feature enabled, you get access to powerful chrono-based operations:

### DateTime Comparisons

```rust
use rust_queries_builder::Query;
use chrono::{Utc, Duration};

let events = vec![/* ... */];
let now = Utc::now();

// After a specific datetime
let upcoming = Query::new(&events)
    .where_after(Event::scheduled_at_r(), now);

// Before a specific datetime
let past = Query::new(&events)
    .where_before(Event::scheduled_at_r(), now);

// Between two datetimes
let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
let year_events = Query::new(&events)
    .where_between(Event::scheduled_at_r(), start, end);
```

### Date-Specific Filters

```rust
// Events scheduled today
let today = Query::new(&events)
    .where_today(Event::scheduled_at_r(), Utc::now());

// Events in a specific year
let this_year = Query::new(&events)
    .where_year(Event::scheduled_at_r(), 2024);

// Events in a specific month (1-12)
let december = Query::new(&events)
    .where_month(Event::scheduled_at_r(), 12);

// Events on a specific day (1-31)
let first_of_month = Query::new(&events)
    .where_day(Event::scheduled_at_r(), 1);
```

### Day of Week Filters

```rust
// Weekend events (Saturday and Sunday)
let weekend = Query::new(&events)
    .where_weekend(Event::scheduled_at_r());

// Weekday events (Monday through Friday)
let weekday = Query::new(&events)
    .where_weekday(Event::scheduled_at_r());
```

### Time-Based Filters

```rust
// Events during business hours (9 AM - 5 PM)
let business_hours = Query::new(&events)
    .where_business_hours(Event::scheduled_at_r());
```

### Combining DateTime Filters

```rust
// Complex query: High-priority work events on weekdays during business hours
let filtered = Query::new(&events)
    .where_(Event::category_r(), |cat| cat == "Work")
    .where_(Event::priority_r(), |&p| p >= 3)
    .where_weekday(Event::scheduled_at_r())
    .where_business_hours(Event::scheduled_at_r());
```

## Query Builder Integration

All datetime query methods integrate seamlessly with the query builder:

```rust
use rust_queries_builder::Query;
use chrono::{Utc, Duration};

#[derive(Keypaths)]
struct Task {
    id: u32,
    title: String,
    due_date: DateTime<Utc>,
    status: String,
    priority: u32,
}

let tasks = vec![/* ... */];
let now = Utc::now();

// Find overdue high-priority tasks
let overdue = Query::new(&tasks)
    .where_before(Task::due_date_r(), now)
    .where_(Task::status_r(), |s| s == "pending")
    .where_(Task::priority_r(), |&p| p >= 4)
    .order_by_float_desc(Task::priority_r());

// Find tasks due this week on weekdays
let this_week = Query::new(&tasks)
    .where_between(
        Task::due_date_r(),
        now,
        now + Duration::days(7)
    )
    .where_weekday(Task::due_date_r());

// Count tasks by time period
let urgent_count = Query::new(&tasks)
    .where_between(
        Task::due_date_r(),
        now,
        now + Duration::hours(24)
    )
    .count();
```

## Helper Functions

The `datetime::chrono_ops` module provides utility functions for datetime operations:

### Date Arithmetic

```rust
use rust_queries_builder::datetime::chrono_ops;
use chrono::Utc;

let now = Utc::now();

// Add/subtract time
let future = chrono_ops::add_days(&now, 7);
let past = chrono_ops::subtract_hours(&now, 24);
let later = chrono_ops::add_minutes(&now, 30);

// Calculate differences
let days = chrono_ops::days_between(&date1, &date2);
let hours = chrono_ops::hours_between(&time1, &time2);
```

### Date Extraction

```rust
use rust_queries_builder::datetime::chrono_ops;

let dt = Utc::now();

let year = chrono_ops::extract_year(&dt);      // 2024
let month = chrono_ops::extract_month(&dt);    // 1-12
let day = chrono_ops::extract_day(&dt);        // 1-31
let hour = chrono_ops::extract_hour(&dt);      // 0-23
let minute = chrono_ops::extract_minute(&dt);  // 0-59
let second = chrono_ops::extract_second(&dt);  // 0-59
```

### Date Checks

```rust
use rust_queries_builder::datetime::chrono_ops;

let now = Utc::now();
let date = Utc.with_ymd_and_hms(2024, 10, 19, 10, 0, 0).unwrap();

// Check relationships
let is_past = chrono_ops::is_past(&date, &now);
let is_future = chrono_ops::is_future(&date, &now);
let same_day = chrono_ops::is_same_day(&date1, &date2);

// Check day type
let weekend = chrono_ops::is_weekend(&date);
let weekday = chrono_ops::is_weekday(&date);
let business = chrono_ops::is_business_hours(&date);

// Check proximity
let within = chrono_ops::is_within_duration(&date, &now, Duration::hours(24));
```

### Start/End of Day

```rust
use rust_queries_builder::datetime::chrono_ops;

let dt = Utc::now();

let start = chrono_ops::start_of_day(&dt);  // 00:00:00
let end = chrono_ops::end_of_day(&dt);      // 23:59:59
```

## Examples

### Example 1: Event Scheduling System

```rust
use rust_queries_builder::Query;
use chrono::{Utc, Duration};
use key_paths_derive::Keypaths;

#[derive(Debug, Clone, Keypaths)]
struct Event {
    id: u32,
    title: String,
    scheduled_at: DateTime<Utc>,
    category: String,
}

let events = vec![/* ... */];
let now = Utc::now();

// Find upcoming events in the next 24 hours
let soon = Query::new(&events)
    .where_between(
        Event::scheduled_at_r(),
        now,
        now + Duration::hours(24)
    )
    .all();

// Find all weekend events
let weekend_events = Query::new(&events)
    .where_weekend(Event::scheduled_at_r())
    .all();

// Find work events during business hours
let work_hours = Query::new(&events)
    .where_(Event::category_r(), |c| c == "Work")
    .where_business_hours(Event::scheduled_at_r())
    .all();
```

### Example 2: Task Management

```rust
use rust_queries_builder::Query;
use chrono::{Utc, Duration};

// Overdue tasks
let overdue = Query::new(&tasks)
    .where_before(Task::due_date_r(), Utc::now())
    .where_(Task::status_r(), |s| s != "completed");

// Tasks due today
let today = Query::new(&tasks)
    .where_today(Task::due_date_r(), Utc::now());

// Upcoming tasks this week
let this_week = Query::new(&tasks)
    .where_between(
        Task::due_date_r(),
        Utc::now(),
        Utc::now() + Duration::days(7)
    );

// Emergency tasks (due within 2 hours)
let emergency = Query::new(&tasks)
    .where_between(
        Task::due_date_r(),
        Utc::now(),
        Utc::now() + Duration::hours(2)
    )
    .count();
```

### Example 3: Analytics and Reporting

```rust
use rust_queries_builder::Query;
use chrono::{Utc, TimeZone};

// Monthly report: Events in December 2024
let december = Query::new(&events)
    .where_year(Event::scheduled_at_r(), 2024)
    .where_month(Event::scheduled_at_r(), 12);

// Weekend vs Weekday analysis
let weekend_count = Query::new(&events)
    .where_weekend(Event::scheduled_at_r())
    .count();

let weekday_count = Query::new(&events)
    .where_weekday(Event::scheduled_at_r())
    .count();

// Business hours coverage
let business_hours_events = Query::new(&events)
    .where_business_hours(Event::scheduled_at_r())
    .count();

println!("Weekend events: {}", weekend_count);
println!("Weekday events: {}", weekday_count);
println!("Business hours: {}", business_hours_events);
```

### Example 4: Custom DateTime Logic

```rust
use rust_queries_builder::{Query, datetime::chrono_ops};
use chrono::{Utc, Duration};

let events = vec![/* ... */];
let now = Utc::now();

// Events within next 48 hours using helper function
let near_future = Query::new(&events)
    .where_(Event::scheduled_at_r(), |dt| {
        chrono_ops::is_within_duration(dt, &now, Duration::hours(48)) &&
        chrono_ops::is_future(dt, &now)
    });

// Events on the 1st or 15th of any month
let paydays = Query::new(&events)
    .where_(Event::scheduled_at_r(), |dt| {
        let day = chrono_ops::extract_day(dt);
        day == 1 || day == 15
    });
```

## Best Practices

1. **Choose the Right Type**
   - Use `SystemTime` for simple timestamp comparisons
   - Use `DateTime<Utc>` for timezone-aware operations
   - Use `DateTime<Local>` when working with user's local time

2. **Enable Features Wisely**
   - Only enable the `datetime` feature if you need chrono
   - This keeps dependencies minimal for simple use cases

3. **Combine Filters**
   - Chain multiple datetime filters for complex queries
   - Combine with other query operations (where_, order_by, etc.)

4. **Performance Considerations**
   - DateTime operations are generally fast
   - Consider indexing or caching for very large datasets
   - Use `first()` or `exists()` for early termination when appropriate

5. **Error Handling**
   - Most operations use `unwrap()` in examples for clarity
   - In production, properly handle timezone conversion errors
   - Validate date ranges before queries

## Running the Example

To see all datetime operations in action:

```bash
# With datetime feature
cargo run --example datetime_operations --features datetime

# Without feature (shows feature requirement message)
cargo run --example datetime_operations
```

## API Reference

### Query Methods (Available with datetime feature)

- `where_after<Tz>` - Filter after a DateTime
- `where_before<Tz>` - Filter before a DateTime
- `where_between<Tz>` - Filter between two DateTimes
- `where_today<Tz>` - Filter for today's date
- `where_year` - Filter by year
- `where_month` - Filter by month (1-12)
- `where_day` - Filter by day (1-31)
- `where_weekend` - Filter for weekends
- `where_weekday` - Filter for weekdays
- `where_business_hours` - Filter for business hours (9 AM - 5 PM)

### Query Methods (Always available)

- `where_after_systemtime` - Filter after a SystemTime
- `where_before_systemtime` - Filter before a SystemTime
- `where_between_systemtime` - Filter between two SystemTimes

### Helper Functions (datetime::chrono_ops)

**Comparisons:**
- `is_after`, `is_before`, `is_between`
- `is_past`, `is_future`, `is_today`
- `is_same_day`, `is_within_duration`

**Day Type:**
- `is_weekend`, `is_weekday`, `is_business_hours`
- `day_of_week`

**Extraction:**
- `extract_year`, `extract_month`, `extract_day`
- `extract_hour`, `extract_minute`, `extract_second`

**Arithmetic:**
- `add_days`, `add_hours`, `add_minutes`
- `subtract_days`, `subtract_hours`, `subtract_minutes`
- `days_between`, `hours_between`

**Utility:**
- `start_of_day`, `end_of_day`

## See Also

- [Main README](README.md)
- [Query Builder Guide](QUERYABLE_GUIDE.md)
- [Examples](examples/)
- [Chrono Documentation](https://docs.rs/chrono/)

