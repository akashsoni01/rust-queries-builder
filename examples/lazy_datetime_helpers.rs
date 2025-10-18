// Demonstrates datetime helper functions with lazy evaluation
// This example shows:
// 1. All datetime::chrono_ops helper functions with LazyQuery
// 2. Performance benefits of lazy evaluation with helper functions
// 3. Early termination with datetime helpers
// 4. SQL equivalents for reference
//
// Run with: cargo run --example lazy_datetime_helpers --features datetime --release

#[cfg(feature = "datetime")]
use chrono::{DateTime, Utc, Duration, TimeZone};
use rust_queries_builder::{LazyQuery, datetime::chrono_ops};
use key_paths_derive::Keypath;
use std::time::Instant;

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
    println!("=== Lazy DateTime Helper Functions Demo ===\n");
    println!("⚠️  This example requires the 'datetime' feature to be enabled.");
    println!("Run with: cargo run --example lazy_datetime_helpers --features datetime --release");
}

#[cfg(feature = "datetime")]
fn create_large_dataset(size: usize) -> Vec<Event> {
    let base_time = Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
    
    (0..size)
        .map(|i| {
            let days_offset = (i % 365) as i64;
            let hour = (i % 24) as u32;
            let category = if i % 3 == 0 { "Work" } else { "Personal" }.to_string();
            
            Event {
                id: i as u32,
                title: format!("Event {}", i),
                scheduled_at: base_time + Duration::days(days_offset) + Duration::hours(hour as i64),
                created_at: base_time - Duration::days((i % 100) as i64),
                category,
                priority: (i % 5 + 1) as u32,
            }
        })
        .collect()
}

#[cfg(feature = "datetime")]
fn main() {
    println!("=== Lazy DateTime Helper Functions Demo ===\n");
    println!("Demonstrates all helper functions with lazy evaluation and early termination\n");

    // Create large dataset
    let dataset_size = 50_000;
    println!("Creating dataset with {} events...", dataset_size);
    let events = create_large_dataset(dataset_size);
    println!("Dataset created!\n");

    let now = Utc::now();
    let reference_date = Utc.with_ymd_and_hms(2024, 10, 20, 12, 0, 0).unwrap();

    // ============================================================================
    // COMPARISON FUNCTIONS WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. COMPARISON FUNCTIONS (Lazy with Early Termination)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- is_after with early termination ---
    println!("--- is_after: Find first 10 events after reference date ---");
    let start = Instant::now();
    let ref_date = reference_date.clone();
    let after_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_after(dt, &ref_date)
        })
        .take_lazy(10)  // Early termination!
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in {:?}", after_events.len(), duration);
    for event in after_events.iter().take(3) {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }
    println!("SQL: SELECT * FROM events WHERE scheduled_at > '2024-10-20' LIMIT 10;\n");

    // --- is_before with early termination ---
    println!("--- is_before: Find first 5 events before reference date ---");
    let start = Instant::now();
    let ref_date = reference_date.clone();
    let before_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_before(dt, &ref_date)
        })
        .take_lazy(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in {:?}", before_events.len(), duration);
    println!("SQL: SELECT * FROM events WHERE scheduled_at < '2024-10-20' LIMIT 5;\n");

    // --- is_between with early termination ---
    println!("--- is_between: Find events in date range (first 20) ---");
    let start = Instant::now();
    let start_date = Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2024, 10, 31, 23, 59, 59).unwrap();
    let between_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_between(dt, &start_date, &end_date)
        })
        .take_lazy(20)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in {:?}", between_events.len(), duration);
    println!("SQL: SELECT * FROM events WHERE scheduled_at BETWEEN '2024-10-01' AND '2024-10-31' LIMIT 20;\n");

    // --- is_past / is_future with any() ---
    println!("--- is_past: Check if ANY past events exist (early termination) ---");
    let start = Instant::now();
    let now_clone = now.clone();
    let has_past = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), move |dt| {
            chrono_ops::is_past(dt, &now_clone)
        })
        .any();
    let duration = start.elapsed();
    
    println!("Past events exist: {} (checked in {:?})", has_past, duration);
    println!("SQL: SELECT EXISTS(SELECT 1 FROM events WHERE scheduled_at < NOW());\n");

    // ============================================================================
    // DAY TYPE FUNCTIONS WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. DAY TYPE FUNCTIONS (Lazy)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- is_weekend with early termination ---
    println!("--- is_weekend: Find first 15 weekend events ---");
    let start = Instant::now();
    let weekend_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::is_weekend(dt)
        })
        .take_lazy(15)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} weekend events in {:?}", weekend_events.len(), duration);
    for event in weekend_events.iter().take(3) {
        println!("  • {} - {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d (%A)")
        );
    }
    println!("SQL: SELECT * FROM events WHERE EXTRACT(DOW FROM scheduled_at) IN (0, 6) LIMIT 15;\n");

    // --- is_weekday count (full scan) ---
    println!("--- is_weekday: Count all weekday events ---");
    let start = Instant::now();
    let weekday_count = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::is_weekday(dt)
        })
        .count();
    let duration = start.elapsed();
    
    println!("Found {} weekday events in {:?}", weekday_count, duration);
    println!("SQL: SELECT COUNT(*) FROM events WHERE EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5;\n");

    // --- is_business_hours with early termination ---
    println!("--- is_business_hours: Find first 10 business hour events ---");
    let start = Instant::now();
    let business_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::is_business_hours(dt)
        })
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} business hour events in {:?}", business_events.len(), duration);
    println!("SQL: SELECT * FROM events WHERE EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16 LIMIT 10;\n");

    // --- day_of_week with map ---
    println!("--- day_of_week: Get day of week for first 5 events ---");
    let start = Instant::now();
    let days: Vec<(String, u32)> = LazyQuery::new(&events)
        .map_items(|e| {
            (e.title.clone(), chrono_ops::day_of_week(&e.scheduled_at))
        })
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Day of week for first 5 events (in {:?}):", duration);
    for (title, dow) in days {
        let day_name = match dow {
            0 => "Monday", 1 => "Tuesday", 2 => "Wednesday",
            3 => "Thursday", 4 => "Friday", 5 => "Saturday", 6 => "Sunday",
            _ => "Unknown",
        };
        println!("  • {} - {} ({})", title, day_name, dow);
    }
    println!("SQL: SELECT title, EXTRACT(DOW FROM scheduled_at) FROM events LIMIT 5;\n");

    // ============================================================================
    // EXTRACTION FUNCTIONS WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. EXTRACTION FUNCTIONS (Lazy)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- extract_year/month/day with filtering ---
    println!("--- extract_year/month: Find first 10 events in October 2024 ---");
    let start = Instant::now();
    let october_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::extract_year(dt) == 2024 && chrono_ops::extract_month(dt) == 10
        })
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} October 2024 events in {:?}", october_events.len(), duration);
    for event in october_events.iter().take(3) {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }
    println!("SQL: SELECT * FROM events WHERE YEAR(scheduled_at) = 2024 AND MONTH(scheduled_at) = 10 LIMIT 10;\n");

    // --- extract_hour with filtering ---
    println!("--- extract_hour: Find events at specific hour (14:00-14:59) ---");
    let start = Instant::now();
    let hour_14_events: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::extract_hour(dt) == 14
        })
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events at hour 14 in {:?}", hour_14_events.len(), duration);
    println!("SQL: SELECT * FROM events WHERE EXTRACT(HOUR FROM scheduled_at) = 14 LIMIT 10;\n");

    // ============================================================================
    // ARITHMETIC FUNCTIONS WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. ARITHMETIC FUNCTIONS (Lazy)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- days_between with filtering ---
    println!("--- days_between: Find events planned >30 days in advance (first 10) ---");
    let start = Instant::now();
    let well_planned: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |scheduled| {
            events.iter().any(|e| {
                e.scheduled_at == *scheduled && 
                chrono_ops::days_between(scheduled, &e.created_at) > 30
            })
        })
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} well-planned events in {:?}", well_planned.len(), duration);
    for event in well_planned.iter().take(3) {
        let days = chrono_ops::days_between(&event.scheduled_at, &event.created_at);
        println!("  • {} - {} days in advance", event.title, days);
    }
    println!("SQL: SELECT * FROM events WHERE DATEDIFF(scheduled_at, created_at) > 30 LIMIT 10;\n");

    // --- hours_between with map ---
    println!("--- hours_between: Calculate hours between creation and schedule (first 5) ---");
    let start = Instant::now();
    let hours_data: Vec<(String, i64)> = LazyQuery::new(&events)
        .map_items(|e| {
            let hours = chrono_ops::hours_between(&e.scheduled_at, &e.created_at);
            (e.title.clone(), hours)
        })
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Hours between creation and schedule (in {:?}):", duration);
    for (title, hours) in hours_data {
        println!("  • {} - {} hours", title, hours);
    }
    println!("SQL: SELECT title, TIMESTAMPDIFF(HOUR, created_at, scheduled_at) FROM events LIMIT 5;\n");

    // --- add_days demonstration ---
    println!("--- add_days: Calculate dates 7 days in future (first 5 events) ---");
    let start = Instant::now();
    let future_dates: Vec<(String, String, String)> = LazyQuery::new(&events)
        .map_items(|e| {
            let future = chrono_ops::add_days(&e.scheduled_at, 7);
            (
                e.title.clone(),
                e.scheduled_at.format("%Y-%m-%d").to_string(),
                future.format("%Y-%m-%d").to_string()
            )
        })
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Future dates calculated in {:?}:", duration);
    for (title, current, future) in future_dates {
        println!("  • {} - {} → {}", title, current, future);
    }
    println!("SQL: SELECT title, scheduled_at, scheduled_at + INTERVAL '7 days' FROM events LIMIT 5;\n");

    // ============================================================================
    // UTILITY FUNCTIONS WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. UTILITY FUNCTIONS (Lazy)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- start_of_day ---
    println!("--- start_of_day: Get midnight for first 5 events ---");
    let start = Instant::now();
    let start_of_days: Vec<(String, String, String)> = LazyQuery::new(&events)
        .map_items(|e| {
            let start_of_day = chrono_ops::start_of_day(&e.scheduled_at)
                .unwrap_or(e.scheduled_at.clone());
            (
                e.title.clone(),
                e.scheduled_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                start_of_day.format("%Y-%m-%d %H:%M:%S").to_string()
            )
        })
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Start of day calculated in {:?}:", duration);
    for (title, original, start_of_day) in start_of_days {
        println!("  • {} - {} → {}", title, original, start_of_day);
    }
    println!("SQL: SELECT title, scheduled_at, DATE_TRUNC('day', scheduled_at) FROM events LIMIT 5;\n");

    // --- end_of_day ---
    println!("--- end_of_day: Get end of day (23:59:59) for first 5 events ---");
    let start = Instant::now();
    let end_of_days: Vec<(String, String)> = LazyQuery::new(&events)
        .map_items(|e| {
            let end_of_day = chrono_ops::end_of_day(&e.scheduled_at)
                .unwrap_or(e.scheduled_at.clone());
            (
                e.title.clone(),
                end_of_day.format("%Y-%m-%d %H:%M:%S").to_string()
            )
        })
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("End of day calculated in {:?}:", duration);
    for (title, end_of_day) in end_of_days {
        println!("  • {} - {}", title, end_of_day);
    }
    println!("SQL: SELECT title, DATE_TRUNC('day', scheduled_at) + INTERVAL '23:59:59' FROM events LIMIT 5;\n");

    // ============================================================================
    // COMPLEX QUERIES WITH LAZY EVALUATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("6. COMPLEX QUERIES (Lazy with Early Termination)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // --- Complex: High-priority weekend events during business hours ---
    println!("--- Complex: High-priority (>=4) weekend events (first 10) ---");
    let start = Instant::now();
    let complex_query: Vec<_> = LazyQuery::new(&events)
        .where_(Event::priority(), |&p| p >= 4)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::is_weekend(dt)
        })
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in {:?}", complex_query.len(), duration);
    for event in complex_query.iter().take(3) {
        println!("  • {} - {} (Priority: {})", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d (%A)"),
            event.priority
        );
    }
    println!("SQL: SELECT * FROM events");
    println!("     WHERE priority >= 4");
    println!("     AND EXTRACT(DOW FROM scheduled_at) IN (0, 6)");
    println!("     LIMIT 10;\n");

    // --- Complex: Weekday business hours in specific month ---
    println!("--- Complex: October weekday business hours events (first 15) ---");
    let start = Instant::now();
    let oct_weekday_business: Vec<_> = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| {
            chrono_ops::extract_month(dt) == 10 &&
            chrono_ops::is_weekday(dt) &&
            chrono_ops::is_business_hours(dt)
        })
        .take_lazy(15)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in {:?}", oct_weekday_business.len(), duration);
    println!("SQL: SELECT * FROM events");
    println!("     WHERE EXTRACT(MONTH FROM scheduled_at) = 10");
    println!("     AND EXTRACT(DOW FROM scheduled_at) BETWEEN 1 AND 5");
    println!("     AND EXTRACT(HOUR FROM scheduled_at) BETWEEN 9 AND 16");
    println!("     LIMIT 15;\n");

    // ============================================================================
    // PERFORMANCE COMPARISON
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("7. PERFORMANCE SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Benchmark: First match vs full count
    println!("--- Benchmark: first() vs count() with is_weekend ---");
    
    let start = Instant::now();
    let _first = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| chrono_ops::is_weekend(dt))
        .first();
    let first_duration = start.elapsed();
    
    let start = Instant::now();
    let _count = LazyQuery::new(&events)
        .where_(Event::scheduled_at(), |dt| chrono_ops::is_weekend(dt))
        .count();
    let count_duration = start.elapsed();
    
    println!("  first() (early termination): {:?}", first_duration);
    println!("  count() (full scan): {:?}", count_duration);
    println!("  Speedup: {}x faster", count_duration.as_micros() / first_duration.as_micros().max(1));

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ Lazy datetime helper functions demo complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    println!("\n📝 Summary:");
    println!("  • Dataset size: {} events", dataset_size);
    println!("  • All datetime helpers demonstrated with lazy evaluation");
    println!("  • Early termination with take_lazy(), first(), any()");
    println!("  • Performance: Microsecond range for most operations");
    
    println!("\n💡 Key Benefits:");
    println!("  • Early termination: Stop as soon as you have enough results");
    println!("  • Iterator fusion: Rust optimizes chained operations");
    println!("  • Memory efficient: No intermediate collections");
    println!("  • Same helper functions work with both Query and LazyQuery");
}

