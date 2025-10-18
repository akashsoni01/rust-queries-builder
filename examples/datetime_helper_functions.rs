// Demonstrates all datetime helper functions with SQL equivalents
// This example shows:
// 1. All datetime::chrono_ops helper functions
// 2. SQL equivalents for each operation (in comments)
// 3. Real-world use cases
// 4. Performance comparison with SQL approaches
//
// Run with: cargo run --example datetime_helper_functions --features datetime

#[cfg(feature = "datetime")]
use chrono::{DateTime, Utc, Duration, TimeZone};
use rust_queries_builder::{Query, datetime::chrono_ops};
use key_paths_derive::Keypath;

#[derive(Debug, Clone, Keypath)]
struct Event {
    id: u32,
    title: String,
    scheduled_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    category: String,
    priority: u32,
}

#[cfg(not(feature = "datetime"))]
fn main() {
    println!("=== DateTime Helper Functions Demo ===\n");
    println!("⚠️  This example requires the 'datetime' feature to be enabled.");
    println!("Run with: cargo run --example datetime_helper_functions --features datetime");
}

#[cfg(feature = "datetime")]
fn create_sample_events() -> Vec<Event> {
    let base_time = Utc.with_ymd_and_hms(2024, 10, 15, 14, 30, 0).unwrap(); // Tuesday
    
    vec![
        Event {
            id: 1,
            title: "Team Meeting".to_string(),
            scheduled_at: base_time,
            created_at: base_time - Duration::days(2),
            category: "Work".to_string(),
            priority: 3,
        },
        Event {
            id: 2,
            title: "Weekend Brunch".to_string(),
            scheduled_at: base_time + Duration::days(4), // Saturday
            created_at: base_time - Duration::days(5),
            category: "Personal".to_string(),
            priority: 2,
        },
        Event {
            id: 3,
            title: "Project Deadline".to_string(),
            scheduled_at: base_time + Duration::days(30),
            created_at: base_time - Duration::days(60),
            category: "Work".to_string(),
            priority: 5,
        },
        Event {
            id: 4,
            title: "Morning Standup".to_string(),
            scheduled_at: base_time + Duration::days(1) - Duration::hours(5), // Next day, 9:30 AM
            created_at: base_time,
            category: "Work".to_string(),
            priority: 2,
        },
        Event {
            id: 5,
            title: "Late Night Gaming".to_string(),
            scheduled_at: base_time + Duration::hours(8), // Same day, 10:30 PM
            created_at: base_time,
            category: "Personal".to_string(),
            priority: 1,
        },
    ]
}

#[cfg(feature = "datetime")]
fn main() {
    println!("=== DateTime Helper Functions Demo ===\n");
    println!("All helper functions with SQL equivalents\n");

    let events = create_sample_events();
    let now = Utc::now();
    let reference_date = Utc.with_ymd_and_hms(2024, 10, 20, 12, 0, 0).unwrap();
    let start_date = Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2024, 10, 31, 23, 59, 59).unwrap();

    println!("Reference Date: {}", reference_date.format("%Y-%m-%d %H:%M:%S"));
    println!("Date Range: {} to {}\n", 
        start_date.format("%Y-%m-%d"), 
        end_date.format("%Y-%m-%d")
    );

    // ============================================================================
    // COMPARISON FUNCTIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. COMPARISON FUNCTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- is_after ---
    println!("--- is_after: Check if datetime is after another ---");
    // SQL: SELECT * FROM events WHERE scheduled_at > '2024-10-20 12:00:00';
    let ref_date = reference_date.clone();
    let after_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_after(dt, &ref_date)
        })
        .count();
    println!("Events after {}: {}", reference_date.format("%Y-%m-%d"), after_count);
    println!("SQL: SELECT * FROM events WHERE scheduled_at > '2024-10-20 12:00:00';\n");

    // --- is_before ---
    println!("--- is_before: Check if datetime is before another ---");
    // SQL: SELECT * FROM events WHERE scheduled_at < '2024-10-20 12:00:00';
    let ref_date = reference_date.clone();
    let before_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_before(dt, &ref_date)
        })
        .count();
    println!("Events before {}: {}", reference_date.format("%Y-%m-%d"), before_count);
    println!("SQL: SELECT * FROM events WHERE scheduled_at < '2024-10-20 12:00:00';\n");

    // --- is_between ---
    println!("--- is_between: Check if datetime is within a range ---");
    // SQL: SELECT * FROM events WHERE scheduled_at BETWEEN '2024-10-01' AND '2024-10-31 23:59:59';
    let start = start_date.clone();
    let end = end_date.clone();
    let between_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_between(dt, &start, &end)
        })
        .count();
    println!("Events between {} and {}: {}", 
        start_date.format("%Y-%m-%d"), 
        end_date.format("%Y-%m-%d"),
        between_count
    );
    println!("SQL: SELECT * FROM events WHERE scheduled_at BETWEEN '2024-10-01' AND '2024-10-31 23:59:59';\n");

    // --- is_today ---
    println!("--- is_today: Check if datetime is today ---");
    // SQL: SELECT * FROM events WHERE DATE(scheduled_at) = CURRENT_DATE;
    // or:  SELECT * FROM events WHERE scheduled_at >= CURRENT_DATE AND scheduled_at < CURRENT_DATE + INTERVAL '1 day';
    let now_clone = now.clone();
    let today_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_today(dt, &now_clone)
        })
        .count();
    println!("Events today: {}", today_count);
    println!("SQL: SELECT * FROM events WHERE DATE(scheduled_at) = CURRENT_DATE;");
    println!("     (PostgreSQL) WHERE scheduled_at >= CURRENT_DATE AND scheduled_at < CURRENT_DATE + INTERVAL '1 day';\n");

    // --- is_same_day ---
    println!("--- is_same_day: Check if two datetimes are on the same day ---");
    // SQL: SELECT * FROM events e1, events e2 WHERE DATE(e1.scheduled_at) = DATE(e2.scheduled_at);
    for event in &events {
        let same_day_count = events.iter()
            .filter(|e| chrono_ops::is_same_day(&e.scheduled_at, &event.scheduled_at))
            .count();
        if same_day_count > 1 {
            println!("Event '{}' shares day with {} other events", event.title, same_day_count - 1);
        }
    }
    println!("SQL: SELECT * FROM events e1, events e2 WHERE DATE(e1.scheduled_at) = DATE(e2.scheduled_at);\n");

    // --- is_past ---
    println!("--- is_past: Check if datetime is in the past ---");
    // SQL: SELECT * FROM events WHERE scheduled_at < NOW();
    let now_clone = now.clone();
    let past_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_past(dt, &now_clone)
        })
        .count();
    println!("Events in the past: {}", past_count);
    println!("SQL: SELECT * FROM events WHERE scheduled_at < NOW();\n");

    // --- is_future ---
    println!("--- is_future: Check if datetime is in the future ---");
    // SQL: SELECT * FROM events WHERE scheduled_at > NOW();
    let now_clone = now.clone();
    let future_count = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_future(dt, &now_clone)
        })
        .count();
    println!("Events in the future: {}", future_count);
    println!("SQL: SELECT * FROM events WHERE scheduled_at > NOW();\n");

    // --- is_within_duration ---
    println!("--- is_within_duration: Check if datetime is within X hours/days from reference ---");
    // SQL: SELECT * FROM events WHERE ABS(EXTRACT(EPOCH FROM (scheduled_at - '2024-10-20 12:00:00'))) <= 86400;
    // or:  SELECT * FROM events WHERE scheduled_at BETWEEN NOW() - INTERVAL '24 hours' AND NOW() + INTERVAL '24 hours';
    let ref_date = reference_date.clone();
    let within_24h = Query::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_within_duration(dt, &ref_date, Duration::hours(24))
        })
        .count();
    println!("Events within 24 hours of {}: {}", reference_date.format("%Y-%m-%d"), within_24h);
    println!("SQL: SELECT * FROM events WHERE ABS(EXTRACT(EPOCH FROM (scheduled_at - '2024-10-20'))) <= 86400;");
    println!("     (PostgreSQL) WHERE scheduled_at BETWEEN timestamp - INTERVAL '24 hours' AND timestamp + INTERVAL '24 hours';\n");

    // ============================================================================
    // DAY TYPE FUNCTIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. DAY TYPE FUNCTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- is_weekend ---
    println!("--- is_weekend: Check if datetime is on weekend (Sat/Sun) ---");
    // SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6);
    // or:  SELECT * FROM events WHERE DAYOFWEEK(scheduled_at) IN (1, 7); (MySQL)
    let weekend_events = events.iter()
        .filter(|e| chrono_ops::is_weekend(&e.scheduled_at))
        .collect::<Vec<_>>();
    println!("Weekend events: {}", weekend_events.len());
    for event in &weekend_events {
        println!("  • {} - {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d (%A)")
        );
    }
    println!("SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6);  -- PostgreSQL");
    println!("     SELECT * FROM events WHERE DAYOFWEEK(scheduled_at) IN (1, 7);         -- MySQL");
    println!("     SELECT * FROM events WHERE strftime('%w', scheduled_at) IN ('0', '6'); -- SQLite\n");

    // --- is_weekday ---
    println!("--- is_weekday: Check if datetime is on weekday (Mon-Fri) ---");
    // SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5;
    let weekday_events = events.iter()
        .filter(|e| chrono_ops::is_weekday(&e.scheduled_at))
        .collect::<Vec<_>>();
    println!("Weekday events: {}", weekday_events.len());
    for event in &weekday_events {
        println!("  • {} - {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d (%A)")
        );
    }
    println!("SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5;  -- PostgreSQL");
    println!("     SELECT * FROM events WHERE DAYOFWEEK(scheduled_at) BETWEEN 2 AND 6;         -- MySQL\n");

    // --- is_business_hours ---
    println!("--- is_business_hours: Check if datetime is during business hours (9 AM - 5 PM) ---");
    // SQL: SELECT * FROM events WHERE EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16;
    let business_events = events.iter()
        .filter(|e| chrono_ops::is_business_hours(&e.scheduled_at))
        .collect::<Vec<_>>();
    println!("Events during business hours: {}", business_events.len());
    for event in &business_events {
        println!("  • {} - {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d %H:%M")
        );
    }
    println!("SQL: SELECT * FROM events WHERE EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16;  -- PostgreSQL");
    println!("     SELECT * FROM events WHERE HOUR(scheduled_at) BETWEEN 9 AND 16;               -- MySQL\n");

    // --- day_of_week ---
    println!("--- day_of_week: Get day of week (0=Monday, 6=Sunday) ---");
    // SQL: SELECT EXTRACT(DOW FROM scheduled_at) - 1 as day_of_week FROM events;  (adjusted for Monday=0)
    println!("Day of week for each event:");
    for event in &events {
        let dow = chrono_ops::day_of_week(&event.scheduled_at);
        let day_name = match dow {
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown",
        };
        println!("  • {} - {} ({})", event.title, day_name, dow);
    }
    println!("SQL: SELECT *, (EXTRACT(DOW FROM scheduled_at) + 6) % 7 as day_of_week FROM events;  -- PostgreSQL (0=Mon)");
    println!("     SELECT *, DAYOFWEEK(scheduled_at) - 2 as day_of_week FROM events;              -- MySQL\n");

    // ============================================================================
    // EXTRACTION FUNCTIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. EXTRACTION FUNCTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- extract_year, extract_month, extract_day ---
    println!("--- extract_year/month/day: Extract date components ---");
    // SQL: SELECT EXTRACT(YEAR FROM scheduled_at), EXTRACT(MONTH FROM scheduled_at), EXTRACT(DAY FROM scheduled_at) FROM events;
    println!("Date components for each event:");
    for event in &events {
        let year = chrono_ops::extract_year(&event.scheduled_at);
        let month = chrono_ops::extract_month(&event.scheduled_at);
        let day = chrono_ops::extract_day(&event.scheduled_at);
        println!("  • {} - {}-{:02}-{:02}", event.title, year, month, day);
    }
    println!("SQL: SELECT *, EXTRACT(YEAR FROM scheduled_at), EXTRACT(MONTH FROM scheduled_at), EXTRACT(DAY FROM scheduled_at) FROM events;");
    println!("     SELECT *, YEAR(scheduled_at), MONTH(scheduled_at), DAY(scheduled_at) FROM events;  -- MySQL\n");

    // --- extract_hour, extract_minute, extract_second ---
    println!("--- extract_hour/minute/second: Extract time components ---");
    // SQL: SELECT EXTRACT(HOUR FROM scheduled_at), EXTRACT(MINUTE FROM scheduled_at), EXTRACT(SECOND FROM scheduled_at) FROM events;
    println!("Time components for each event:");
    for event in &events {
        let hour = chrono_ops::extract_hour(&event.scheduled_at);
        let minute = chrono_ops::extract_minute(&event.scheduled_at);
        let second = chrono_ops::extract_second(&event.scheduled_at);
        println!("  • {} - {:02}:{:02}:{:02}", event.title, hour, minute, second);
    }
    println!("SQL: SELECT *, EXTRACT(HOUR FROM scheduled_at), EXTRACT(MINUTE FROM scheduled_at), EXTRACT(SECOND FROM scheduled_at) FROM events;");
    println!("     SELECT *, HOUR(scheduled_at), MINUTE(scheduled_at), SECOND(scheduled_at) FROM events;  -- MySQL\n");

    // ============================================================================
    // ARITHMETIC FUNCTIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. ARITHMETIC FUNCTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- add_days ---
    println!("--- add_days: Add days to a datetime ---");
    // SQL: SELECT scheduled_at + INTERVAL '7 days' FROM events;
    println!("Events + 7 days:");
    for event in events.iter().take(3) {
        let future_date = chrono_ops::add_days(&event.scheduled_at, 7);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d"),
            future_date.format("%Y-%m-%d")
        );
    }
    println!("SQL: SELECT *, scheduled_at + INTERVAL '7 days' as future_date FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_ADD(scheduled_at, INTERVAL 7 DAY) as future_date FROM events;  -- MySQL\n");

    // --- add_hours ---
    println!("--- add_hours: Add hours to a datetime ---");
    // SQL: SELECT scheduled_at + INTERVAL '3 hours' FROM events;
    println!("Events + 3 hours:");
    for event in events.iter().take(3) {
        let future_time = chrono_ops::add_hours(&event.scheduled_at, 3);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%H:%M"),
            future_time.format("%H:%M")
        );
    }
    println!("SQL: SELECT *, scheduled_at + INTERVAL '3 hours' as future_time FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_ADD(scheduled_at, INTERVAL 3 HOUR) as future_time FROM events;  -- MySQL\n");

    // --- add_minutes ---
    println!("--- add_minutes: Add minutes to a datetime ---");
    // SQL: SELECT scheduled_at + INTERVAL '30 minutes' FROM events;
    println!("Events + 30 minutes:");
    for event in events.iter().take(3) {
        let future_time = chrono_ops::add_minutes(&event.scheduled_at, 30);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%H:%M"),
            future_time.format("%H:%M")
        );
    }
    println!("SQL: SELECT *, scheduled_at + INTERVAL '30 minutes' as future_time FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_ADD(scheduled_at, INTERVAL 30 MINUTE) as future_time FROM events;  -- MySQL\n");

    // --- subtract_days ---
    println!("--- subtract_days: Subtract days from a datetime ---");
    // SQL: SELECT scheduled_at - INTERVAL '7 days' FROM events;
    println!("Events - 7 days:");
    for event in events.iter().take(3) {
        let past_date = chrono_ops::subtract_days(&event.scheduled_at, 7);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d"),
            past_date.format("%Y-%m-%d")
        );
    }
    println!("SQL: SELECT *, scheduled_at - INTERVAL '7 days' as past_date FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_SUB(scheduled_at, INTERVAL 7 DAY) as past_date FROM events;  -- MySQL\n");

    // --- subtract_hours ---
    println!("--- subtract_hours: Subtract hours from a datetime ---");
    // SQL: SELECT scheduled_at - INTERVAL '2 hours' FROM events;
    println!("Events - 2 hours:");
    for event in events.iter().take(3) {
        let past_time = chrono_ops::subtract_hours(&event.scheduled_at, 2);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%H:%M"),
            past_time.format("%H:%M")
        );
    }
    println!("SQL: SELECT *, scheduled_at - INTERVAL '2 hours' as past_time FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_SUB(scheduled_at, INTERVAL 2 HOUR) as past_time FROM events;  -- MySQL\n");

    // --- subtract_minutes ---
    println!("--- subtract_minutes: Subtract minutes from a datetime ---");
    // SQL: SELECT scheduled_at - INTERVAL '15 minutes' FROM events;
    println!("Events - 15 minutes:");
    for event in events.iter().take(3) {
        let past_time = chrono_ops::subtract_minutes(&event.scheduled_at, 15);
        println!("  • {} - {} → {}", 
            event.title,
            event.scheduled_at.format("%H:%M"),
            past_time.format("%H:%M")
        );
    }
    println!("SQL: SELECT *, scheduled_at - INTERVAL '15 minutes' as past_time FROM events;        -- PostgreSQL");
    println!("     SELECT *, DATE_SUB(scheduled_at, INTERVAL 15 MINUTE) as past_time FROM events;  -- MySQL\n");

    // --- days_between ---
    println!("--- days_between: Calculate days between two datetimes ---");
    // SQL: SELECT EXTRACT(DAYS FROM scheduled_at - created_at) FROM events;
    // or:  SELECT DATEDIFF(scheduled_at, created_at) FROM events;
    println!("Days between creation and schedule:");
    for event in &events {
        let days = chrono_ops::days_between(&event.scheduled_at, &event.created_at);
        println!("  • {} - {} days", event.title, days);
    }
    println!("SQL: SELECT *, EXTRACT(DAY FROM (scheduled_at - created_at)) as days_diff FROM events;  -- PostgreSQL");
    println!("     SELECT *, DATEDIFF(scheduled_at, created_at) as days_diff FROM events;            -- MySQL\n");

    // --- hours_between ---
    println!("--- hours_between: Calculate hours between two datetimes ---");
    // SQL: SELECT EXTRACT(EPOCH FROM scheduled_at - created_at) / 3600 FROM events;
    println!("Hours between creation and schedule:");
    for event in &events {
        let hours = chrono_ops::hours_between(&event.scheduled_at, &event.created_at);
        println!("  • {} - {} hours", event.title, hours);
    }
    println!("SQL: SELECT *, EXTRACT(EPOCH FROM (scheduled_at - created_at)) / 3600 as hours_diff FROM events;  -- PostgreSQL");
    println!("     SELECT *, TIMESTAMPDIFF(HOUR, created_at, scheduled_at) as hours_diff FROM events;           -- MySQL\n");

    // ============================================================================
    // UTILITY FUNCTIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. UTILITY FUNCTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- start_of_day ---
    println!("--- start_of_day: Get midnight (00:00:00) of a date ---");
    // SQL: SELECT DATE_TRUNC('day', scheduled_at) FROM events;
    // or:  SELECT DATE(scheduled_at) FROM events;
    println!("Start of day for each event:");
    for event in events.iter().take(3) {
        if let Some(start) = chrono_ops::start_of_day(&event.scheduled_at) {
            println!("  • {} - {} → {}", 
                event.title,
                event.scheduled_at.format("%Y-%m-%d %H:%M:%S"),
                start.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
    println!("SQL: SELECT *, DATE_TRUNC('day', scheduled_at) as start_of_day FROM events;  -- PostgreSQL");
    println!("     SELECT *, DATE(scheduled_at) as start_of_day FROM events;               -- MySQL\n");

    // --- end_of_day ---
    println!("--- end_of_day: Get end of day (23:59:59) of a date ---");
    // SQL: SELECT DATE_TRUNC('day', scheduled_at) + INTERVAL '1 day' - INTERVAL '1 second' FROM events;
    println!("End of day for each event:");
    for event in events.iter().take(3) {
        if let Some(end) = chrono_ops::end_of_day(&event.scheduled_at) {
            println!("  • {} - {} → {}", 
                event.title,
                event.scheduled_at.format("%Y-%m-%d %H:%M:%S"),
                end.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
    println!("SQL: SELECT *, DATE_TRUNC('day', scheduled_at) + INTERVAL '23:59:59' as end_of_day FROM events;  -- PostgreSQL");
    println!("     SELECT *, TIMESTAMP(DATE(scheduled_at), '23:59:59') as end_of_day FROM events;              -- MySQL\n");

    // ============================================================================
    // COMPLEX REAL-WORLD QUERIES
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("6. COMPLEX REAL-WORLD QUERIES");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Query 1: Events created more than 30 days before scheduled date
    println!("--- Query 1: Events planned well in advance (>30 days) ---");
    let well_planned = events.iter()
        .filter(|e| chrono_ops::days_between(&e.scheduled_at, &e.created_at) > 30)
        .collect::<Vec<_>>();
    println!("Found {} events:", well_planned.len());
    for event in &well_planned {
        let days = chrono_ops::days_between(&event.scheduled_at, &event.created_at);
        println!("  • {} - planned {} days in advance", event.title, days);
    }
    println!("SQL: SELECT * FROM events WHERE EXTRACT(DAY FROM (scheduled_at - created_at)) > 30;");
    println!("     SELECT * FROM events WHERE DATEDIFF(scheduled_at, created_at) > 30;  -- MySQL\n");

    // Query 2: High-priority events on weekdays during business hours
    println!("--- Query 2: High-priority weekday business hours events ---");
    let critical_work = events.iter()
        .filter(|e| e.priority >= 4)
        .filter(|e| chrono_ops::is_weekday(&e.scheduled_at))
        .filter(|e| chrono_ops::is_business_hours(&e.scheduled_at))
        .collect::<Vec<_>>();
    println!("Found {} critical work events:", critical_work.len());
    for event in &critical_work {
        println!("  • {} - {} (Priority: {})", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d %H:%M (%A)"),
            event.priority
        );
    }
    println!("SQL: SELECT * FROM events");
    println!("     WHERE priority >= 4");
    println!("     AND EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5");
    println!("     AND EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16;\n");

    // Query 3: Events in October 2024
    println!("--- Query 3: All events in October 2024 ---");
    let october_events = events.iter()
        .filter(|e| chrono_ops::extract_year(&e.scheduled_at) == 2024)
        .filter(|e| chrono_ops::extract_month(&e.scheduled_at) == 10)
        .collect::<Vec<_>>();
    println!("Found {} October 2024 events:", october_events.len());
    for event in &october_events {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }
    println!("SQL: SELECT * FROM events");
    println!("     WHERE EXTRACT(YEAR FROM scheduled_at) = 2024");
    println!("     AND EXTRACT(MONTH FROM scheduled_at) = 10;\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ DateTime helper functions demo complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    println!("\n📝 Summary:");
    println!("  • Demonstrated all {} helper functions", 20);
    println!("  • Provided SQL equivalents for PostgreSQL, MySQL, and SQLite");
    println!("  • Showed real-world complex query examples");
    println!("\n💡 Key Advantages over SQL:");
    println!("  • Type-safe at compile time");
    println!("  • No string-based queries");
    println!("  • Works on in-memory data");
    println!("  • No database connection needed");
    println!("  • Consistent API across different databases");
}

