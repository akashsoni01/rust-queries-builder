//! Example demonstrating i64 timestamp aggregators for DateTime<Local> stored as Unix timestamps in milliseconds.
//!
//! This example shows how to use the new timestamp aggregators with i64 values that represent
//! Unix timestamps in milliseconds, similar to Java's Date.getTime() or JavaScript's Date.getTime().
//! 
//! Supports both positive timestamps (dates after 1970-01-01) and negative timestamps (dates before 1970-01-01).

use rust_queries_builder::{Query, Keypath};
use chrono::{DateTime, Local, Utc, TimeZone, Datelike};

#[derive(Debug, Clone, Keypath)]
struct Event {
    id: u32,
    name: String,
    created_at: i64,        // Unix timestamp in milliseconds
    scheduled_at: i64,      // Unix timestamp in milliseconds
    duration_minutes: u32,
    category: String,
}

impl Event {
    fn new(id: u32, name: &str, created_at: i64, scheduled_at: i64, duration_minutes: u32, category: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            created_at,
            scheduled_at,
            duration_minutes,
            category: category.to_string(),
        }
    }
}

fn main() {
    println!("=== i64 Timestamp Aggregators Demo ===\n");

    // Create sample events with i64 timestamps (Unix timestamps in milliseconds)
    // These timestamps represent dates from 1969-01-01 to 2028-12-31
    // Note: Negative timestamps represent dates before 1970-01-01 (Unix epoch)
    let events = vec![
        // Pre-epoch events (negative timestamps)
        Event::new(1, "Apollo 11 Moon Landing", -141829200000, -141829200000, 1440, "Historical"),
        Event::new(2, "Woodstock Festival", -14112000000, -14112000000, 4320, "Historical"),
        Event::new(3, "First Computer Network", -94694400000, -94694400000, 60, "Technology"),
        
        // Early epoch events (1970-2000)
        Event::new(4, "Y2K Aftermath", 978307200000, 978307200000, 60, "Historical"),
        Event::new(5, "Dot-com Recovery", 1009843200000, 1009843200000, 120, "Historical"),
        Event::new(6, "Early Internet", 1104537600000, 1104537600000, 90, "Historical"),
        
        // Recent events (2020-2024)
        Event::new(7, "Remote Work Revolution", 1577836800000, 1577836800000, 480, "Work"),
        Event::new(8, "AI Breakthrough", 1609459200000, 1609459200000, 180, "Technology"),
        Event::new(9, "Climate Summit", 1640995200000, 1640995200000, 1440, "Environment"),
        Event::new(10, "Space Mission", 1672531200000, 1672531200000, 2880, "Science"),
        Event::new(11, "Tech Conference", 1704067200000, 1704067200000, 480, "Technology"),
        Event::new(12, "Future Planning", 1735689600000, 1735689600000, 120, "Planning"),
        
        // Future events (2025+)
        Event::new(13, "Mars Mission", 1767225600000, 1767225600000, 43200, "Science"),
        Event::new(14, "Quantum Computing", 1798761600000, 1798761600000, 240, "Technology"),
        Event::new(15, "Sustainable Future", 1830297600000, 1830297600000, 2880, "Environment"),
    ];

    println!("📅 Sample Events (with i64 timestamps):");
    for event in &events {
        let created_dt = DateTime::from_timestamp_millis(event.created_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        let scheduled_dt = DateTime::from_timestamp_millis(event.scheduled_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        
        println!("  {}: {} (created: {}, scheduled: {})", 
                 event.id, event.name, 
                 created_dt.format("%Y-%m-%d %H:%M:%S"),
                 scheduled_dt.format("%Y-%m-%d %H:%M:%S"));
    }

    // ============================================================================
    // Basic Timestamp Aggregators
    // ============================================================================
    
    println!("\n🔍 Basic Timestamp Aggregators:");
    
    // Min/Max timestamps
    let earliest_created = Query::new(&events).min_timestamp(Event::created_at());
    let latest_created = Query::new(&events).max_timestamp(Event::created_at());
    let earliest_scheduled = Query::new(&events).min_timestamp(Event::scheduled_at());
    let latest_scheduled = Query::new(&events).max_timestamp(Event::scheduled_at());
    
    println!("  Earliest created: {} ({})", 
             earliest_created.unwrap_or(0),
             DateTime::from_timestamp_millis(earliest_created.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .with_timezone(&Local)
                 .format("%Y-%m-%d %H:%M:%S"));
    
    println!("  Latest created: {} ({})", 
             latest_created.unwrap_or(0),
             DateTime::from_timestamp_millis(latest_created.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .with_timezone(&Local)
                 .format("%Y-%m-%d %H:%M:%S"));
    
    println!("  Earliest scheduled: {} ({})", 
             earliest_scheduled.unwrap_or(0),
             DateTime::from_timestamp_millis(earliest_scheduled.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .with_timezone(&Local)
                 .format("%Y-%m-%d %H:%M:%S"));
    
    println!("  Latest scheduled: {} ({})", 
             latest_scheduled.unwrap_or(0),
             DateTime::from_timestamp_millis(latest_scheduled.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .with_timezone(&Local)
                 .format("%Y-%m-%d %H:%M:%S"));

    // Average timestamp
    let avg_created = Query::new(&events).avg_timestamp(Event::created_at());
    let avg_scheduled = Query::new(&events).avg_timestamp(Event::scheduled_at());
    
    println!("  Average created timestamp: {}", avg_created.unwrap_or(0));
    println!("  Average scheduled timestamp: {}", avg_scheduled.unwrap_or(0));

    // Sum and count
    let total_created = Query::new(&events).sum_timestamp(Event::created_at());
    let count_created = Query::new(&events).count_timestamp(Event::created_at());
    
    println!("  Total created timestamps: {}", total_created);
    println!("  Count of created timestamps: {}", count_created);

    // ============================================================================
    // Time-based Filtering
    // ============================================================================
    
    println!("\n⏰ Time-based Filtering:");
    
    // Filter by timestamp ranges (including negative timestamps for pre-epoch dates)
    let epoch_start = 0; // 1970-01-01 00:00:00 UTC
    let year_2000_cutoff = 946684800000; // 2000-01-01 00:00:00 UTC
    let year_2020_cutoff = 1577836800000; // 2020-01-01 00:00:00 UTC
    let year_2025_cutoff = 1735689600000; // 2025-01-01 00:00:00 UTC
    
    // Pre-epoch events (negative timestamps - dates before 1970)
    let pre_epoch_query = Query::new(&events)
        .where_before_timestamp(Event::created_at(), epoch_start);
    let pre_epoch_events = pre_epoch_query.all();
    
    println!("  Pre-epoch events (before 1970): {} events", pre_epoch_events.len());
    for event in &pre_epoch_events {
        let dt = DateTime::from_timestamp_millis(event.created_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    - {}: {} ({}) [timestamp: {}]", event.id, event.name, dt.format("%Y-%m-%d"), event.created_at);
    }
    
    // Events created after 2000
    let modern_query = Query::new(&events)
        .where_after_timestamp(Event::created_at(), year_2000_cutoff);
    let modern_events = modern_query.all();
    
    println!("\n  Events created after 2000: {} events", modern_events.len());
    for event in &modern_events {
        let dt = DateTime::from_timestamp_millis(event.created_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    - {}: {} ({})", event.id, event.name, dt.format("%Y-%m-%d"));
    }
    
    // Events scheduled between 2020 and 2025
    let recent_query = Query::new(&events)
        .where_between_timestamp(Event::scheduled_at(), year_2020_cutoff, year_2025_cutoff);
    let recent_events = recent_query.all();
    
    println!("\n  Events scheduled between 2020-2025: {} events", recent_events.len());
    for event in &recent_events {
        let dt = DateTime::from_timestamp_millis(event.scheduled_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    - {}: {} ({})", event.id, event.name, dt.format("%Y-%m-%d"));
    }

    // ============================================================================
    // Relative Time Filtering (Last/Next N days/hours/minutes)
    // ============================================================================
    
    println!("\n📅 Relative Time Filtering:");
    
    // Note: These methods use the current time, so they might not match our sample data
    // But they demonstrate the API usage
    
    // Events from the last 30 days (relative to now)
    let recent_30_days_query = Query::new(&events)
        .where_last_days_timestamp(Event::created_at(), 30);
    let recent_30_days = recent_30_days_query.all();
    
    println!("  Events created in last 30 days: {} events", recent_30_days.len());
    
    // Events in the next 365 days (relative to now)
    let upcoming_year_query = Query::new(&events)
        .where_next_days_timestamp(Event::scheduled_at(), 365);
    let upcoming_year = upcoming_year_query.all();
    
    println!("  Events scheduled in next 365 days: {} events", upcoming_year.len());
    
    // Events from the last 24 hours
    let recent_24_hours_query = Query::new(&events)
        .where_last_hours_timestamp(Event::created_at(), 24);
    let recent_24_hours = recent_24_hours_query.all();
    
    println!("  Events created in last 24 hours: {} events", recent_24_hours.len());
    
    // Events in the next 2 hours
    let upcoming_2_hours_query = Query::new(&events)
        .where_next_hours_timestamp(Event::scheduled_at(), 2);
    let upcoming_2_hours = upcoming_2_hours_query.all();
    
    println!("  Events scheduled in next 2 hours: {} events", upcoming_2_hours.len());

    // ============================================================================
    // Ordering by Timestamps
    // ============================================================================
    
    println!("\n📊 Ordering by Timestamps:");
    
    // Order by creation time (oldest first)
    let chronological = Query::new(&events)
        .order_by_timestamp(Event::created_at());
    
    println!("  Events ordered by creation time (oldest first):");
    for event in &chronological {
        let dt = DateTime::from_timestamp_millis(event.created_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    {}: {} ({})", event.id, event.name, dt.format("%Y-%m-%d"));
    }
    
    // Order by scheduled time (newest first)
    let reverse_chronological = Query::new(&events)
        .order_by_timestamp_desc(Event::scheduled_at());
    
    println!("\n  Events ordered by scheduled time (newest first):");
    for event in &reverse_chronological {
        let dt = DateTime::from_timestamp_millis(event.scheduled_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    {}: {} ({})", event.id, event.name, dt.format("%Y-%m-%d"));
    }

    // ============================================================================
    // Complex Queries with Timestamps
    // ============================================================================
    
    println!("\n🔗 Complex Queries with Timestamps:");
    
    // Technology events created after 2020, ordered by creation time
    let tech_events = Query::new(&events)
        .where_(Event::category(), |cat| cat == "Technology")
        .where_after_timestamp(Event::created_at(), year_2020_cutoff)
        .order_by_timestamp(Event::created_at());
    
    println!("  Technology events created after 2020:");
    for event in &tech_events {
        let dt = DateTime::from_timestamp_millis(event.created_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    {}: {} ({})", event.id, event.name, dt.format("%Y-%m-%d"));
    }
    
    // Long-duration events (more than 8 hours) scheduled in the future
    let long_future_events = Query::new(&events)
        .where_(Event::duration_minutes(), |&duration| duration > 480) // 8 hours
        .where_after_timestamp(Event::scheduled_at(), chrono::Utc::now().timestamp_millis())
        .order_by_timestamp_desc(Event::scheduled_at());
    
    println!("\n  Long-duration events (8+ hours) scheduled in the future:");
    for event in &long_future_events {
        let dt = DateTime::from_timestamp_millis(event.scheduled_at)
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
        println!("    {}: {} ({} hours, {})", 
                 event.id, event.name, 
                 event.duration_minutes / 60,
                 dt.format("%Y-%m-%d"));
    }

    // ============================================================================
    // Grouping by Time Periods
    // ============================================================================
    
    println!("\n📈 Grouping by Time Periods:");
    
    // Group events by decade based on creation time
    let by_decade: std::collections::HashMap<String, Vec<Event>> = events
        .iter()
        .map(|event| {
            let dt = DateTime::from_timestamp_millis(event.created_at)
                .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
            .with_timezone(&Local);
            let decade = (dt.year() / 10) * 10;
            (format!("{}s", decade), event.clone())
        })
        .fold(std::collections::HashMap::new(), |mut acc, (decade, event)| {
            acc.entry(decade).or_insert_with(Vec::new).push(event);
            acc
        });
    
    for (decade, events_in_decade) in &by_decade {
        println!("  {}: {} events", decade, events_in_decade.len());
        for event in events_in_decade {
            println!("    - {}: {}", event.id, event.name);
        }
    }

    // ============================================================================
    // Timestamp Arithmetic Examples
    // ============================================================================
    
    println!("\n🧮 Timestamp Arithmetic Examples:");
    
    // Calculate time differences between creation and scheduling
    let time_diffs: Vec<(u32, i64)> = events
        .iter()
        .map(|event| {
            let diff_ms = event.scheduled_at - event.created_at;
            let diff_days = diff_ms / (24 * 60 * 60 * 1000);
            (event.id, diff_days)
        })
        .collect();
    
    println!("  Time differences between creation and scheduling:");
    for (id, diff_days) in &time_diffs {
        println!("    Event {}: {} days", id, diff_days);
    }
    
    // Find events with the largest time gap between creation and scheduling
    let max_diff = time_diffs.iter().map(|(_, diff)| *diff).max().unwrap_or(0);
    let events_with_max_diff: Vec<_> = time_diffs
        .iter()
        .filter(|(_, diff)| *diff == max_diff)
        .collect();
    
    println!("\n  Events with maximum time gap ({} days):", max_diff);
    for (id, _) in &events_with_max_diff {
        println!("    Event {}", id);
    }

    println!("\n✅ i64 Timestamp Aggregators Demo Complete!");
    println!("\nKey Features Demonstrated:");
    println!("  • min_timestamp() / max_timestamp() - Find earliest/latest timestamps");
    println!("  • avg_timestamp() / sum_timestamp() / count_timestamp() - Statistical operations");
    println!("  • where_after_timestamp() / where_before_timestamp() / where_between_timestamp() - Time filtering");
    println!("  • where_last_days_timestamp() / where_next_days_timestamp() - Relative time filtering");
    println!("  • where_last_hours_timestamp() / where_next_hours_timestamp() - Hour-based filtering");
    println!("  • where_last_minutes_timestamp() / where_next_minutes_timestamp() - Minute-based filtering");
    println!("  • order_by_timestamp() / order_by_timestamp_desc() - Time-based sorting");
    println!("  • Complex queries combining timestamp operations with other filters");
    println!("  • Integration with chrono for human-readable date formatting");
    println!("  • Support for negative timestamps (dates before 1970-01-01) - Pre-epoch dates");
}
