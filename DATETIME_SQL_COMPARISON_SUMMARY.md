# DateTime Helper Functions with SQL Equivalents

## Overview

Created a comprehensive example (`datetime_helper_functions.rs`) demonstrating **all 20+ datetime helper functions** with **SQL equivalents** for PostgreSQL, MySQL, and SQLite.

## What Was Created

### Example File: `datetime_helper_functions.rs`

A complete reference guide showing:
- All datetime helper functions from `datetime::chrono_ops`
- SQL equivalents for each operation (PostgreSQL, MySQL, SQLite)
- Real-world use cases
- Complex query examples

### Functions Covered

#### 1. Comparison Functions (8 functions)
- `is_after` → `WHERE scheduled_at > '2024-10-20'`
- `is_before` → `WHERE scheduled_at < '2024-10-20'`
- `is_between` → `WHERE scheduled_at BETWEEN start AND end`
- `is_today` → `WHERE DATE(scheduled_at) = CURRENT_DATE`
- `is_same_day` → `WHERE DATE(e1.scheduled_at) = DATE(e2.scheduled_at)`
- `is_past` → `WHERE scheduled_at < NOW()`
- `is_future` → `WHERE scheduled_at > NOW()`
- `is_within_duration` → `WHERE ABS(EXTRACT(EPOCH FROM (scheduled_at - ref))) <= seconds`

#### 2. Day Type Functions (4 functions)
- `is_weekend` → `WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6)`
- `is_weekday` → `WHERE EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5`
- `is_business_hours` → `WHERE EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16`
- `day_of_week` → `SELECT EXTRACT(DOW FROM scheduled_at)`

#### 3. Extraction Functions (6 functions)
- `extract_year` → `SELECT EXTRACT(YEAR FROM scheduled_at)`
- `extract_month` → `SELECT EXTRACT(MONTH FROM scheduled_at)`
- `extract_day` → `SELECT EXTRACT(DAY FROM scheduled_at)`
- `extract_hour` → `SELECT EXTRACT(HOUR FROM scheduled_at)`
- `extract_minute` → `SELECT EXTRACT(MINUTE FROM scheduled_at)`
- `extract_second` → `SELECT EXTRACT(SECOND FROM scheduled_at)`

#### 4. Arithmetic Functions (8 functions)
- `add_days` → `SELECT scheduled_at + INTERVAL '7 days'`
- `add_hours` → `SELECT scheduled_at + INTERVAL '3 hours'`
- `add_minutes` → `SELECT scheduled_at + INTERVAL '30 minutes'`
- `subtract_days` → `SELECT scheduled_at - INTERVAL '7 days'`
- `subtract_hours` → `SELECT scheduled_at - INTERVAL '2 hours'`
- `subtract_minutes` → `SELECT scheduled_at - INTERVAL '15 minutes'`
- `days_between` → `SELECT DATEDIFF(scheduled_at, created_at)`
- `hours_between` → `SELECT TIMESTAMPDIFF(HOUR, created_at, scheduled_at)`

#### 5. Utility Functions (2 functions)
- `start_of_day` → `SELECT DATE_TRUNC('day', scheduled_at)`
- `end_of_day` → `SELECT DATE_TRUNC('day', scheduled_at) + INTERVAL '23:59:59'`

## Example Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. COMPARISON FUNCTIONS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

--- is_after: Check if datetime is after another ---
Events after 2024-10-20: 1
SQL: SELECT * FROM events WHERE scheduled_at > '2024-10-20 12:00:00';

--- is_weekend: Check if datetime is on weekend (Sat/Sun) ---
Weekend events: 1
  • Weekend Brunch - 2024-10-19 (Saturday)
SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6);  -- PostgreSQL
     SELECT * FROM events WHERE DAYOFWEEK(scheduled_at) IN (1, 7);         -- MySQL
     SELECT * FROM events WHERE strftime('%w', scheduled_at) IN ('0', '6'); -- SQLite

--- add_days: Add days to a datetime ---
Events + 7 days:
  • Team Meeting - 2024-10-15 → 2024-10-22
SQL: SELECT *, scheduled_at + INTERVAL '7 days' as future_date FROM events;        -- PostgreSQL
     SELECT *, DATE_ADD(scheduled_at, INTERVAL 7 DAY) as future_date FROM events;  -- MySQL
```

## Running the Example

```bash
cargo run --example datetime_helper_functions --features datetime
```

## Key Benefits

### 1. Side-by-Side Comparison
See Rust datetime operations alongside their SQL equivalents for easy learning and migration.

### 2. Multi-Database Support
SQL examples provided for:
- **PostgreSQL**: `EXTRACT`, `INTERVAL`, `DATE_TRUNC`
- **MySQL**: `DAYOFWEEK`, `DATE_ADD`, `TIMESTAMPDIFF`
- **SQLite**: `strftime`, `DATE`

### 3. Real-World Queries
Complex examples showing:
- Events planned >30 days in advance
- High-priority weekday business hours events
- Monthly event filtering

### 4. Advantages Over SQL
- ✅ **Type-safe** at compile time
- ✅ **No string-based queries**
- ✅ **Works on in-memory data**
- ✅ **No database connection needed**
- ✅ **Consistent API** across different databases
- ✅ **No SQL injection** vulnerabilities

## Code Comparison

### Rust (Type-Safe)
```rust
let weekend = events.iter()
    .filter(|e| chrono_ops::is_weekend(&e.scheduled_at))
    .collect::<Vec<_>>();

let october = events.iter()
    .filter(|e| chrono_ops::extract_year(&e.scheduled_at) == 2024)
    .filter(|e| chrono_ops::extract_month(&e.scheduled_at) == 10)
    .collect::<Vec<_>>();
```

### SQL (String-Based)
```sql
-- PostgreSQL
SELECT * FROM events 
WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6);

SELECT * FROM events 
WHERE EXTRACT(YEAR FROM scheduled_at) = 2024 
AND EXTRACT(MONTH FROM scheduled_at) = 10;
```

## Example Structure

1. **Comparison Functions** - 8 operations
2. **Day Type Functions** - 4 operations  
3. **Extraction Functions** - 6 operations
4. **Arithmetic Functions** - 8 operations
5. **Utility Functions** - 2 operations
6. **Complex Real-World Queries** - 3 examples

## Documentation

- Each function includes:
  - Description
  - Rust implementation
  - SQL equivalent (PostgreSQL)
  - SQL equivalent (MySQL)
  - SQL equivalent (SQLite where applicable)
  - Output examples

## Perfect For

- **Learning**: See how datetime operations translate between Rust and SQL
- **Migration**: Moving from SQL databases to in-memory Rust queries
- **Reference**: Quick lookup of SQL equivalents
- **Documentation**: Understanding datetime capabilities
- **Training**: Teaching datetime operations with concrete examples

## Files

- `examples/datetime_helper_functions.rs` - Complete example (**NEW**)
- `DATETIME_GUIDE.md` - Full datetime documentation
- `README.md` - Updated with new example

## Run It

```bash
# See all 20+ helper functions with SQL equivalents
cargo run --example datetime_helper_functions --features datetime
```

Expected output: **~250 lines** demonstrating all datetime helpers with SQL comparisons for PostgreSQL, MySQL, and SQLite! 📅✨

