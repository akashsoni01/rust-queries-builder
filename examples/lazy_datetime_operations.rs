// Demonstrates lazy datetime operations with the query builder
// This example shows how to:
// 1. Use LazyQuery with datetime filters for efficient processing
// 2. Benefit from early termination with datetime queries
// 3. Chain datetime operations lazily
// 4. Compare performance of lazy vs eager datetime queries
//
// Run with: cargo run --example lazy_datetime_operations --features datetime

#[cfg(feature = "datetime")]
use chrono::{DateTime, Utc, Duration, TimeZone};
use rust_queries_builder::LazyQuery;
use key_paths_derive::Keypaths;
use std::time::Instant;

#[derive(Debug, Clone, Keypaths)]
struct Event {
    id: u32,
    title: String,
    scheduled_at: DateTime<Utc>,
    category: String,
    priority: u32,
}

#[cfg(feature = "datetime")]
fn create_large_event_dataset(size: usize) -> Vec<Event> {
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
                category,
                priority: (i % 5 + 1) as u32,
            }
        })
        .collect()
}

#[cfg(not(feature = "datetime"))]
fn main() {
    println!("=== Lazy DateTime Operations Demo ===\n");
    println!("⚠️  This example requires the 'datetime' feature to be enabled.");
    println!("Run with: cargo run --example lazy_datetime_operations --features datetime");
}

#[cfg(feature = "datetime")]
fn main() {
    println!("=== Lazy DateTime Operations Demo ===\n");
    println!("This example demonstrates lazy evaluation with datetime operations.\n");

    // Create a large dataset to showcase lazy evaluation benefits
    let dataset_size = 100_000;
    println!("Creating dataset with {} events...", dataset_size);
    let events = create_large_event_dataset(dataset_size);
    println!("Dataset created!\n");

    let now = Utc::now();
    let start_of_2024 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end_of_2024 = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

    // Example 1: Basic lazy datetime filtering
    println!("--- Example 1: Basic Lazy DateTime Filtering ---");
    let start = Instant::now();
    let upcoming: Vec<_> = LazyQuery::new(&events)
        .where_after(Event::scheduled_at_r(), start_of_2024)
        .take_lazy(10)  // Only process until we find 10 items!
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} upcoming events in {:?}", upcoming.len(), duration);
    for event in upcoming.iter().take(3) {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }

    // Example 2: Date range with early termination
    println!("\n--- Example 2: Date Range Query with Early Termination ---");
    let start = Instant::now();
    let in_range: Vec<_> = LazyQuery::new(&events)
        .where_between(Event::scheduled_at_r(), start_of_2024, end_of_2024)
        .take_lazy(5)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in date range in {:?}", in_range.len(), duration);

    // Example 3: Weekend events with lazy evaluation
    println!("\n--- Example 3: Weekend Events (Lazy) ---");
    let start = Instant::now();
    let weekend_count = LazyQuery::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .count();
    let duration = start.elapsed();
    
    println!("Found {} weekend events in {:?}", weekend_count, duration);

    // Example 4: Complex lazy query with multiple datetime filters
    println!("\n--- Example 4: Complex Lazy Query ---");
    println!("Finding high-priority work events on weekdays during business hours...");
    
    let start = Instant::now();
    let complex_query: Vec<_> = LazyQuery::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work")
        .where_(Event::priority_r(), |&p| p >= 4)
        .where_weekday(Event::scheduled_at_r())
        .where_business_hours(Event::scheduled_at_r())
        .take_lazy(20)  // Stop after finding 20 matches
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} matching events in {:?}", complex_query.len(), duration);
    for event in complex_query.iter().take(5) {
        println!("  • {} - {} (Priority: {})", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d %H:%M"),
            event.priority
        );
    }

    // Example 5: Monthly filtering with lazy evaluation
    println!("\n--- Example 5: Events in Specific Month (Lazy) ---");
    let start = Instant::now();
    let december: Vec<_> = LazyQuery::new(&events)
        .where_year(Event::scheduled_at_r(), 2024)
        .where_month(Event::scheduled_at_r(), 12)
        .take_lazy(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} December events in {:?}", december.len(), duration);

    // Example 6: First matching item (early termination)
    println!("\n--- Example 6: First Matching Event (Early Termination) ---");
    let start = Instant::now();
    let first_weekend = LazyQuery::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .where_(Event::priority_r(), |&p| p == 5)
        .first();
    let duration = start.elapsed();
    
    if let Some(event) = first_weekend {
        println!("Found first high-priority weekend event in {:?}:", duration);
        println!("  • {} - {} (Priority: {})", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d %H:%M"),
            event.priority
        );
    }

    // Example 7: Using any() for existence check (early termination)
    println!("\n--- Example 7: Check if Any Weekend Work Events Exist ---");
    let start = Instant::now();
    let has_weekend_work = LazyQuery::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work")
        .where_weekend(Event::scheduled_at_r())
        .any();
    let duration = start.elapsed();
    
    println!("Weekend work events exist: {} (checked in {:?})", has_weekend_work, duration);

    // Example 8: Chaining multiple datetime operations
    println!("\n--- Example 8: Chained DateTime Operations ---");
    let start = Instant::now();
    let chained: Vec<_> = LazyQuery::new(&events)
        .where_year(Event::scheduled_at_r(), 2024)
        .where_month(Event::scheduled_at_r(), 6) // June
        .where_weekday(Event::scheduled_at_r())
        .where_business_hours(Event::scheduled_at_r())
        .skip_lazy(10)  // Skip first 10
        .take_lazy(5)   // Take next 5
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} events in June 2024 (weekdays, business hours) in {:?}", 
        chained.len(), duration);

    // Example 9: Today's events with lazy evaluation
    println!("\n--- Example 9: Today's High Priority Events ---");
    let start = Instant::now();
    let today: Vec<_> = LazyQuery::new(&events)
        .where_today(Event::scheduled_at_r(), now)
        .where_(Event::priority_r(), |&p| p >= 3)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} high-priority events today in {:?}", today.len(), duration);

    // Example 10: Lazy aggregations with datetime filtering
    println!("\n--- Example 10: Lazy Aggregations ---");
    
    let start = Instant::now();
    let weekend_work_count = LazyQuery::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work")
        .where_weekend(Event::scheduled_at_r())
        .count();
    let duration1 = start.elapsed();
    
    let start = Instant::now();
    let weekday_personal_count = LazyQuery::new(&events)
        .where_(Event::category_r(), |cat| cat == "Personal")
        .where_weekday(Event::scheduled_at_r())
        .count();
    let duration2 = start.elapsed();
    
    println!("Weekend work events: {} (in {:?})", weekend_work_count, duration1);
    println!("Weekday personal events: {} (in {:?})", weekday_personal_count, duration2);

    // Example 11: Performance comparison - Lazy vs filtering entire dataset
    println!("\n--- Example 11: Performance Comparison ---");
    println!("Finding first 100 events in 2024 December...");
    
    let start = Instant::now();
    let lazy_result: Vec<_> = LazyQuery::new(&events)
        .where_year(Event::scheduled_at_r(), 2024)
        .where_month(Event::scheduled_at_r(), 12)
        .take_lazy(100)
        .collect();
    let lazy_duration = start.elapsed();
    
    println!("✅ Lazy query: Found {} events in {:?}", lazy_result.len(), lazy_duration);
    println!("   Benefits: Early termination, iterator fusion, no intermediate collections");

    // Example 12: Map and collect with datetime filtering
    println!("\n--- Example 12: Lazy Map with DateTime Filter ---");
    let start = Instant::now();
    let titles: Vec<String> = LazyQuery::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .where_(Event::priority_r(), |&p| p >= 4)
        .map_items(|e| e.title.clone())
        .take(10)
        .collect();
    let duration = start.elapsed();
    
    println!("Found {} high-priority weekend event titles in {:?}", titles.len(), duration);
    for (i, title) in titles.iter().take(3).enumerate() {
        println!("  {}. {}", i + 1, title);
    }

    // Example 13: Fold operation with datetime filtering
    println!("\n--- Example 13: Lazy Fold with DateTime Filter ---");
    let start = Instant::now();
    let total_priority = LazyQuery::new(&events)
        .where_business_hours(Event::scheduled_at_r())
        .fold(0u32, |acc, event| acc + event.priority);
    let duration = start.elapsed();
    
    println!("Total priority of business hours events: {} (computed in {:?})", 
        total_priority, duration);

    // Example 14: All match - check if condition holds for filtered items
    println!("\n--- Example 14: All Match with DateTime Filter ---");
    let start = Instant::now();
    let all_high_priority = LazyQuery::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .where_(Event::category_r(), |cat| cat == "Work")
        .take_lazy(100)
        .all_match(|event| event.priority >= 3);
    let duration = start.elapsed();
    
    println!("All weekend work events (first 100) have priority >= 3: {} (in {:?})", 
        all_high_priority, duration);

    // Example 15: Combining SystemTime and DateTime operations
    println!("\n--- Example 15: Statistics Summary ---");
    
    let total_events = events.len();
    let weekend_events = LazyQuery::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .count();
    let weekday_events = LazyQuery::new(&events)
        .where_weekday(Event::scheduled_at_r())
        .count();
    let business_hours_events = LazyQuery::new(&events)
        .where_business_hours(Event::scheduled_at_r())
        .count();
    let work_events = LazyQuery::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work")
        .count();
    
    println!("\nDataset Statistics:");
    println!("  Total events: {}", total_events);
    println!("  Weekend events: {} ({:.1}%)", 
        weekend_events, 
        (weekend_events as f64 / total_events as f64) * 100.0
    );
    println!("  Weekday events: {} ({:.1}%)", 
        weekday_events,
        (weekday_events as f64 / total_events as f64) * 100.0
    );
    println!("  Business hours: {} ({:.1}%)", 
        business_hours_events,
        (business_hours_events as f64 / total_events as f64) * 100.0
    );
    println!("  Work category: {} ({:.1}%)", 
        work_events,
        (work_events as f64 / total_events as f64) * 100.0
    );

    println!("\n✓ Lazy datetime operations demo complete!");
    println!("\n💡 Key Benefits of Lazy DateTime Queries:");
    println!("  • Early termination: Stop as soon as you have enough results");
    println!("  • Iterator fusion: Rust optimizes chained operations");
    println!("  • No intermediate collections: Memory efficient");
    println!("  • Composable: Build complex queries step by step");
    println!("  • Same datetime operations as eager queries");
}

