// Demonstrates datetime operations with the query builder
// This example shows how to:
// 1. Use SystemTime for basic datetime operations
// 2. Use chrono for advanced datetime operations (with datetime feature)
// 3. Filter by date ranges, specific dates, business hours, weekends
// 4. Extract date components (year, month, day)
// 5. Perform date arithmetic
//
// Run with: cargo run --example datetime_operations --features datetime

#[cfg(feature = "datetime")]
use chrono::{DateTime, Utc, Duration, TimeZone};
use rust_queries_builder::Query;
use key_paths_derive::Keypaths;
use std::time::SystemTime;

#[derive(Debug, Clone, Keypaths)]
struct Event {
    id: u32,
    title: String,
    category: String,
    created_at: SystemTime,
    #[cfg(feature = "datetime")]
    scheduled_at: DateTime<Utc>,
    priority: u32,
}

#[cfg(feature = "datetime")]
fn create_sample_events() -> Vec<Event> {
    let now = Utc::now();
    
    vec![
        Event {
            id: 1,
            title: "Team Meeting".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: now + Duration::hours(2),
            priority: 2,
        },
        Event {
            id: 2,
            title: "Conference Call".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 10, 15, 14, 0, 0).unwrap(),
            priority: 3,
        },
        Event {
            id: 3,
            title: "Weekend Brunch".to_string(),
            category: "Personal".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 10, 19, 11, 0, 0).unwrap(), // Saturday
            priority: 1,
        },
        Event {
            id: 4,
            title: "Project Deadline".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 12, 31, 17, 0, 0).unwrap(),
            priority: 5,
        },
        Event {
            id: 5,
            title: "Morning Standup".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: now + Duration::days(1) + Duration::hours(9),
            priority: 2,
        },
        Event {
            id: 6,
            title: "Late Night Gaming".to_string(),
            category: "Personal".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: now + Duration::hours(12),
            priority: 1,
        },
        Event {
            id: 7,
            title: "Quarterly Review".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 12, 15, 10, 0, 0).unwrap(),
            priority: 4,
        },
        Event {
            id: 8,
            title: "Lunch Break".to_string(),
            category: "Personal".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: now + Duration::hours(3),
            priority: 1,
        },
        Event {
            id: 9,
            title: "Sunday Hike".to_string(),
            category: "Personal".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 10, 20, 8, 0, 0).unwrap(), // Sunday
            priority: 2,
        },
        Event {
            id: 10,
            title: "Client Presentation".to_string(),
            category: "Work".to_string(),
            created_at: SystemTime::now(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 11, 5, 15, 30, 0).unwrap(),
            priority: 5,
        },
    ]
}

#[cfg(not(feature = "datetime"))]
fn main() {
    println!("=== DateTime Operations Demo ===\n");
    println!("⚠️  This example requires the 'datetime' feature to be enabled.");
    println!("Run with: cargo run --example datetime_operations --features datetime");
}

#[cfg(feature = "datetime")]
fn main() {
    println!("=== DateTime Operations Demo ===\n");
    println!("This example demonstrates datetime operations using chrono.\n");

    let events = create_sample_events();
    let now = Utc::now();

    println!("Total events: {}\n", events.len());

    // Query 1: Events scheduled after now
    println!("--- Query 1: Upcoming Events (After Now) ---");
    let upcoming = Query::new(&events)
        .where_after(Event::scheduled_at_r(), now);
    
    for event in upcoming.all() {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d %H:%M"));
    }

    // Query 2: Events scheduled before a specific date
    println!("\n--- Query 2: Events Before December 2024 ---");
    let cutoff = Utc.with_ymd_and_hms(2024, 12, 1, 0, 0, 0).unwrap();
    let before_dec = Query::new(&events)
        .where_before(Event::scheduled_at_r(), cutoff);
    
    for event in before_dec.all() {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }

    // Query 3: Events in a specific date range
    println!("\n--- Query 3: Events Between Oct 15 - Nov 15, 2024 ---");
    let start = Utc.with_ymd_and_hms(2024, 10, 15, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 11, 15, 23, 59, 59).unwrap();
    let in_range = Query::new(&events)
        .where_between(Event::scheduled_at_r(), start, end);
    
    for event in in_range.all() {
        println!("  • {} - {} - Priority: {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d"), 
            event.priority
        );
    }

    // Query 4: Events scheduled today
    println!("\n--- Query 4: Events Scheduled Today ---");
    let today = Query::new(&events)
        .where_today(Event::scheduled_at_r(), now);
    
    let today_events = today.all();
    if today_events.is_empty() {
        println!("  No events scheduled for today");
    } else {
        for event in today_events {
            println!("  • {} - {}", event.title, event.scheduled_at.format("%H:%M"));
        }
    }

    // Query 5: Events in December 2024
    println!("\n--- Query 5: Events in December 2024 ---");
    let december = Query::new(&events)
        .where_year(Event::scheduled_at_r(), 2024)
        .where_month(Event::scheduled_at_r(), 12);
    
    for event in december.all() {
        println!("  • {} - {}", event.title, event.scheduled_at.format("%Y-%m-%d"));
    }

    // Query 6: Weekend events
    println!("\n--- Query 6: Weekend Events ---");
    let weekend = Query::new(&events)
        .where_weekend(Event::scheduled_at_r());
    
    for event in weekend.all() {
        println!("  • {} - {} ({})", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d"),
            event.scheduled_at.format("%A")
        );
    }

    // Query 7: Weekday work events
    println!("\n--- Query 7: Weekday Work Events ---");
    let weekday_work = Query::new(&events)
        .where_weekday(Event::scheduled_at_r())
        .where_(Event::category_r(), |cat| cat == "Work");
    
    for event in weekday_work.all() {
        println!("  • {} - {} ({})", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d %H:%M"),
            event.scheduled_at.format("%A")
        );
    }

    // Query 8: Events during business hours
    println!("\n--- Query 8: Events During Business Hours (9 AM - 5 PM) ---");
    let business_hours = Query::new(&events)
        .where_business_hours(Event::scheduled_at_r());
    
    for event in business_hours.all() {
        println!("  • {} - {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d %H:%M")
        );
    }

    // Query 9: High priority events in the future
    println!("\n--- Query 9: High Priority Upcoming Events (Priority >= 4) ---");
    let high_priority = Query::new(&events)
        .where_after(Event::scheduled_at_r(), now)
        .where_(Event::priority_r(), |&p| p >= 4);
    
    for event in high_priority.all() {
        println!("  • {} - {} - Priority: {}", 
            event.title, 
            event.scheduled_at.format("%Y-%m-%d %H:%M"),
            event.priority
        );
    }

    // Query 10: Using datetime helper functions
    println!("\n--- Query 10: Events in Next 7 Days ---");
    let next_week = now + Duration::days(7);
    let this_week = Query::new(&events)
        .where_between(Event::scheduled_at_r(), now, next_week);
    
    for event in this_week.all() {
        let days_from_now = (event.scheduled_at - now).num_days();
        println!("  • {} - {} (in {} days)", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d %H:%M"),
            days_from_now
        );
    }

    // Query 11: Events grouped by month
    println!("\n--- Query 11: Events Grouped by Month ---");
    use std::collections::HashMap;
    use chrono::Datelike;
    
    let mut by_month: HashMap<u32, Vec<&Event>> = HashMap::new();
    for event in &events {
        let month = event.scheduled_at.month();
        by_month.entry(month).or_insert_with(Vec::new).push(event);
    }
    
    for (month, events) in by_month.iter() {
        let month_name = match month {
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        };
        println!("\n  {} 2024 ({} events):", month_name, events.len());
        for event in events {
            println!("    - {} ({})", event.title, event.scheduled_at.format("%d %b"));
        }
    }

    // Query 12: Statistics using datetime operations
    println!("\n--- Query 12: Event Statistics ---");
    let work_events = Query::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work");
    
    let personal_events = Query::new(&events)
        .where_(Event::category_r(), |cat| cat == "Personal");
    
    println!("  Total Events: {}", events.len());
    println!("  Work Events: {}", work_events.count());
    println!("  Personal Events: {}", personal_events.count());
    
    let weekend_count = Query::new(&events)
        .where_weekend(Event::scheduled_at_r())
        .count();
    println!("  Weekend Events: {}", weekend_count);
    
    let business_hours_count = Query::new(&events)
        .where_business_hours(Event::scheduled_at_r())
        .count();
    println!("  Business Hours Events: {}", business_hours_count);

    // Query 13: Complex datetime query
    println!("\n--- Query 13: Complex Query ---");
    println!("High-priority work events on weekdays during business hours:");
    let complex = Query::new(&events)
        .where_(Event::category_r(), |cat| cat == "Work")
        .where_(Event::priority_r(), |&p| p >= 3)
        .where_weekday(Event::scheduled_at_r())
        .where_business_hours(Event::scheduled_at_r());
    
    for event in complex.all() {
        println!("  • {} - {} - Priority: {}", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d %H:%M (%A)"),
            event.priority
        );
    }

    // Query 14: Using datetime module functions directly
    println!("\n--- Query 14: Using datetime Module Functions ---");
    use rust_queries_builder::datetime::chrono_ops;
    
    println!("Events within 24 hours:");
    let now_clone = now.clone();
    let within_24h = Query::new(&events)
        .where_(Event::scheduled_at_r(), move |dt| {
            chrono_ops::is_within_duration(dt, &now_clone, Duration::hours(24))
        });
    
    for event in within_24h.all() {
        let hours = chrono_ops::hours_between(&event.scheduled_at, &now);
        println!("  • {} - in {} hours", event.title, hours);
    }

    // Query 15: Date component extraction
    println!("\n--- Query 15: Events on Specific Days of Month ---");
    println!("Events on the 15th of any month:");
    let on_15th = Query::new(&events)
        .where_day(Event::scheduled_at_r(), 15);
    
    for event in on_15th.all() {
        println!("  • {} - {}", 
            event.title,
            event.scheduled_at.format("%Y-%m-%d")
        );
    }

    println!("\n✓ DateTime operations demo complete!");
    println!("\n💡 Tips:");
    println!("  - Use SystemTime for basic operations without features");
    println!("  - Enable 'datetime' feature for advanced chrono operations");
    println!("  - Combine datetime filters with other query operations");
    println!("  - Use datetime::chrono_ops module for helper functions");
}

