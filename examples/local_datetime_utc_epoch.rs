//! Comprehensive example demonstrating local datetime operations over UTC epoch with i64 timestamp aggregators.
//!
//! This example shows how to work with local datetime values stored as Unix timestamps in milliseconds,
//! including timezone conversions, local time filtering, and handling of different timezones.
//! 
//! Key concepts demonstrated:
//! - UTC epoch timestamps with local timezone interpretation
//! - Timezone-aware filtering and aggregations
//! - Local business hours and timezone-specific operations
//! - Cross-timezone event handling

use rust_queries_builder::{Query, Keypath};
use chrono::{DateTime, Utc, TimeZone, Timelike, FixedOffset};

#[derive(Debug, Clone, Keypath)]
struct LocalEvent {
    id: u32,
    name: String,
    utc_timestamp: i64,        // UTC timestamp in milliseconds
    local_timezone: String,    // Timezone identifier (e.g., "America/New_York", "Europe/London")
    category: String,
    duration_minutes: f64,
    is_business_hours: bool,   // Whether event is during local business hours (9 AM - 5 PM)
}

impl LocalEvent {
    fn new(
        id: u32, 
        name: &str, 
        utc_timestamp: i64, 
        local_timezone: &str, 
        category: &str, 
        duration_minutes: f64
    ) -> Self {
        // Determine if the event is during local business hours
        let is_business_hours = Self::is_local_business_hours(utc_timestamp, local_timezone);
        
        Self {
            id,
            name: name.to_string(),
            utc_timestamp,
            local_timezone: local_timezone.to_string(),
            category: category.to_string(),
            duration_minutes,
            is_business_hours,
        }
    }
    
    /// Check if a UTC timestamp falls within local business hours (9 AM - 5 PM)
    fn is_local_business_hours(utc_timestamp: i64, timezone: &str) -> bool {
        // Convert UTC timestamp to local time
        if let Some(dt_utc) = DateTime::from_timestamp_millis(utc_timestamp) {
            // For simplicity, we'll use fixed offsets for major timezones
            // In a real application, you'd use chrono-tz for proper timezone handling
            let offset = match timezone {
                "America/New_York" => FixedOffset::west_opt(5 * 3600).unwrap(), // EST (UTC-5)
                "America/Los_Angeles" => FixedOffset::west_opt(8 * 3600).unwrap(), // PST (UTC-8)
                "Europe/London" => FixedOffset::east_opt(0).unwrap(), // GMT (UTC+0)
                "Europe/Paris" => FixedOffset::east_opt(1 * 3600).unwrap(), // CET (UTC+1)
                "Asia/Tokyo" => FixedOffset::east_opt(9 * 3600).unwrap(), // JST (UTC+9)
                "Asia/Shanghai" => FixedOffset::east_opt(8 * 3600).unwrap(), // CST (UTC+8)
                _ => FixedOffset::east_opt(0).unwrap(), // Default to UTC
            };
            
            let local_dt = dt_utc.with_timezone(&offset);
            let hour = local_dt.hour();
            hour >= 9 && hour < 17 // 9 AM to 5 PM
        } else {
            false
        }
    }
    
    /// Get the local datetime for this event
    fn get_local_datetime(&self) -> Option<DateTime<FixedOffset>> {
        if let Some(dt_utc) = DateTime::from_timestamp_millis(self.utc_timestamp) {
            let offset = match self.local_timezone.as_str() {
                "America/New_York" => FixedOffset::west_opt(5 * 3600).unwrap(),
                "America/Los_Angeles" => FixedOffset::west_opt(8 * 3600).unwrap(),
                "Europe/London" => FixedOffset::east_opt(0).unwrap(),
                "Europe/Paris" => FixedOffset::east_opt(1 * 3600).unwrap(),
                "Asia/Tokyo" => FixedOffset::east_opt(9 * 3600).unwrap(),
                "Asia/Shanghai" => FixedOffset::east_opt(8 * 3600).unwrap(),
                _ => FixedOffset::east_opt(0).unwrap(),
            };
            Some(dt_utc.with_timezone(&offset))
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Local DateTime over UTC Epoch Demo ===\n");

    // Create sample events across different timezones
    // All timestamps are in UTC, but events are interpreted in their local timezones
    let events = vec![
        // New York events (EST, UTC-5)
        LocalEvent::new(1, "NY Morning Meeting", 1704067200000, "America/New_York", "Work", 60.0), // 2024-01-01 10:00:00 EST
        LocalEvent::new(2, "NY Evening Conference", 1704096000000, "America/New_York", "Work", 120.0), // 2024-01-01 18:00:00 EST
        LocalEvent::new(3, "NY Late Night Event", 1704110400000, "America/New_York", "Social", 180.0), // 2024-01-01 22:00:00 EST
        
        // Los Angeles events (PST, UTC-8)
        LocalEvent::new(4, "LA Business Meeting", 1704067200000, "America/Los_Angeles", "Work", 90.0), // 2024-01-01 07:00:00 PST
        LocalEvent::new(5, "LA Afternoon Workshop", 1704081600000, "America/Los_Angeles", "Education", 240.0), // 2024-01-01 12:00:00 PST
        LocalEvent::new(6, "LA Evening Social", 1704103200000, "America/Los_Angeles", "Social", 120.0), // 2024-01-01 19:00:00 PST
        
        // London events (GMT, UTC+0)
        LocalEvent::new(7, "London Morning Briefing", 1704067200000, "Europe/London", "Work", 30.0), // 2024-01-01 15:00:00 GMT
        LocalEvent::new(8, "London Afternoon Seminar", 1704081600000, "Europe/London", "Education", 180.0), // 2024-01-01 20:00:00 GMT
        LocalEvent::new(9, "London Evening Gala", 1704103200000, "Europe/London", "Social", 300.0), // 2024-01-01 23:00:00 GMT
        
        // Tokyo events (JST, UTC+9)
        LocalEvent::new(10, "Tokyo Early Meeting", 1704067200000, "Asia/Tokyo", "Work", 60.0), // 2024-01-01 00:00:00 JST
        LocalEvent::new(11, "Tokyo Lunch Meeting", 1704081600000, "Asia/Tokyo", "Work", 90.0), // 2024-01-01 05:00:00 JST
        LocalEvent::new(12, "Tokyo Evening Event", 1704103200000, "Asia/Tokyo", "Social", 150.0), // 2024-01-01 08:00:00 JST
        
        // Paris events (CET, UTC+1)
        LocalEvent::new(13, "Paris Morning Coffee", 1704067200000, "Europe/Paris", "Social", 30.0), // 2024-01-01 16:00:00 CET
        LocalEvent::new(14, "Paris Business Lunch", 1704081600000, "Europe/Paris", "Work", 120.0), // 2024-01-01 21:00:00 CET
        LocalEvent::new(15, "Paris Evening Dinner", 1704103200000, "Europe/Paris", "Social", 180.0), // 2024-01-01 00:00:00 CET (next day)
        
        // Shanghai events (CST, UTC+8)
        LocalEvent::new(16, "Shanghai Morning Call", 1704067200000, "Asia/Shanghai", "Work", 45.0), // 2024-01-01 23:00:00 CST
        LocalEvent::new(17, "Shanghai Afternoon Meeting", 1704081600000, "Asia/Shanghai", "Work", 90.0), // 2024-01-01 04:00:00 CST
        LocalEvent::new(18, "Shanghai Evening Networking", 1704103200000, "Asia/Shanghai", "Social", 120.0), // 2024-01-01 07:00:00 CST
    ];

    println!("🌍 Sample Events (UTC timestamps with local timezone interpretation):");
    for event in &events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("  {}: {} - {} ({}) [UTC: {}, Local: {}]", 
                     event.id, event.name, event.local_timezone,
                     event.category, 
                     DateTime::from_timestamp_millis(event.utc_timestamp)
                         .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                         .format("%Y-%m-%d %H:%M:%S UTC"),
                     local_dt.format("%Y-%m-%d %H:%M:%S %z"));
        }
    }

    // ============================================================================
    // Timezone-aware Aggregations
    // ============================================================================
    
    println!("\n🔍 Timezone-aware Aggregations:");
    
    // Find earliest and latest events across all timezones
    let earliest_utc = Query::new(&events).min_timestamp(LocalEvent::utc_timestamp());
    let latest_utc = Query::new(&events).max_timestamp(LocalEvent::utc_timestamp());
    
    println!("  Earliest UTC timestamp: {} ({})", 
             earliest_utc.unwrap_or(0),
             DateTime::from_timestamp_millis(earliest_utc.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .format("%Y-%m-%d %H:%M:%S UTC"));
    
    println!("  Latest UTC timestamp: {} ({})", 
             latest_utc.unwrap_or(0),
             DateTime::from_timestamp_millis(latest_utc.unwrap_or(0))
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .format("%Y-%m-%d %H:%M:%S UTC"));

    // ============================================================================
    // Business Hours Analysis
    // ============================================================================
    
    println!("\n💼 Business Hours Analysis:");
    
    // Events during local business hours
    let business_hours_query = Query::new(&events)
        .where_(LocalEvent::is_business_hours(), |&is_business| is_business);
    let business_hours_events = business_hours_query.all();
    
    println!("  Events during local business hours (9 AM - 5 PM): {} events", business_hours_events.len());
    for event in &business_hours_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    - {}: {} ({}) - {}", 
                     event.id, event.name, event.local_timezone,
                     local_dt.format("%H:%M %z"));
        }
    }
    
    // Events outside business hours
    let after_hours_query = Query::new(&events)
        .where_(LocalEvent::is_business_hours(), |&is_business| !is_business);
    let after_hours_events = after_hours_query.all();
    
    println!("\n  Events outside local business hours: {} events", after_hours_events.len());
    for event in &after_hours_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    - {}: {} ({}) - {}", 
                     event.id, event.name, event.local_timezone,
                     local_dt.format("%H:%M %z"));
        }
    }

    // ============================================================================
    // Timezone-specific Filtering
    // ============================================================================
    
    println!("\n🌐 Timezone-specific Filtering:");
    
    // Events in specific timezones
    const TIMEZONES: &[&str] = &["America/New_York", "America/Los_Angeles", "Europe/London", "Asia/Tokyo"];
    
    for timezone in TIMEZONES {
        let tz_query = Query::new(&events)
            .where_(LocalEvent::local_timezone(), move |tz| tz == timezone);
        let tz_events = tz_query.all();
        
        println!("  {} events: {} events", timezone, tz_events.len());
        for event in &tz_events {
            if let Some(local_dt) = event.get_local_datetime() {
                println!("    - {}: {} - {}", 
                         event.id, event.name,
                         local_dt.format("%Y-%m-%d %H:%M:%S %z"));
            }
        }
    }

    // ============================================================================
    // Cross-timezone Event Analysis
    // ============================================================================
    
    println!("\n🔄 Cross-timezone Event Analysis:");
    
    // Events happening at the same UTC time but different local times
    let same_utc_time = 1704067200000; // 2024-01-01 15:00:00 UTC
    let simultaneous_query = Query::new(&events)
        .where_(LocalEvent::utc_timestamp(), move |&ts| ts == same_utc_time);
    let simultaneous_events = simultaneous_query.all();
    
    println!("  Events happening simultaneously (UTC: {}):", 
             DateTime::from_timestamp_millis(same_utc_time)
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .format("%Y-%m-%d %H:%M:%S UTC"));
    
    for event in &simultaneous_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    - {}: {} ({}) - Local: {}", 
                     event.id, event.name, event.local_timezone,
                     local_dt.format("%Y-%m-%d %H:%M:%S %z"));
        }
    }

    // ============================================================================
    // Local Time Range Filtering
    // ============================================================================
    
    println!("\n⏰ Local Time Range Filtering:");
    
    // Find events that occur during morning hours (6 AM - 12 PM) in their local timezone
    let morning_events: Vec<_> = events
        .iter()
        .filter(|event| {
            if let Some(local_dt) = event.get_local_datetime() {
                let hour = local_dt.hour();
                hour >= 6 && hour < 12
            } else {
                false
            }
        })
        .collect();
    
    println!("  Events during local morning hours (6 AM - 12 PM): {} events", morning_events.len());
    for event in &morning_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    - {}: {} ({}) - {}", 
                     event.id, event.name, event.local_timezone,
                     local_dt.format("%H:%M %z"));
        }
    }
    
    // Find events that occur during evening hours (6 PM - 12 AM) in their local timezone
    let evening_events: Vec<_> = events
        .iter()
        .filter(|event| {
            if let Some(local_dt) = event.get_local_datetime() {
                let hour = local_dt.hour();
                hour >= 18 || hour < 6
            } else {
                false
            }
        })
        .collect();
    
    println!("\n  Events during local evening hours (6 PM - 12 AM): {} events", evening_events.len());
    for event in &evening_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    - {}: {} ({}) - {}", 
                     event.id, event.name, event.local_timezone,
                     local_dt.format("%H:%M %z"));
        }
    }

    // ============================================================================
    // Duration Analysis by Timezone
    // ============================================================================
    
    println!("\n📊 Duration Analysis by Timezone:");
    
    for timezone in TIMEZONES {
        let tz_query = Query::new(&events)
            .where_(LocalEvent::local_timezone(), move |tz| tz == timezone);
        
        let avg_duration = tz_query.avg(LocalEvent::duration_minutes()).unwrap_or(0.0);
        let total_duration = tz_query.sum(LocalEvent::duration_minutes());
        let event_count = tz_query.count();
        
        println!("  {}: {} events, avg duration: {:.1} min, total: {} min", 
                 timezone, event_count, avg_duration, total_duration);
    }

    // ============================================================================
    // Category Analysis with Timezone Context
    // ============================================================================
    
    println!("\n📈 Category Analysis with Timezone Context:");
    
    const CATEGORIES: &[&str] = &["Work", "Social", "Education"];
    
    for category in CATEGORIES {
        let cat_query = Query::new(&events)
            .where_(LocalEvent::category(), move |cat| cat == category);
        let cat_events = cat_query.all();
        
        println!("  {} events: {} events", category, cat_events.len());
        
        // Group by timezone
        let mut by_timezone = std::collections::HashMap::new();
        for event in &cat_events {
            by_timezone.entry(&event.local_timezone)
                .or_insert_with(Vec::new)
                .push(event);
        }
        
        for (timezone, events) in &by_timezone {
            println!("    {}: {} events", timezone, events.len());
            for event in events {
                if let Some(local_dt) = event.get_local_datetime() {
                    println!("      - {}: {} - {}", 
                             event.id, event.name,
                             local_dt.format("%H:%M %z"));
                }
            }
        }
    }

    // ============================================================================
    // UTC vs Local Time Comparison
    // ============================================================================
    
    println!("\n🕐 UTC vs Local Time Comparison:");
    
    // Show how the same UTC timestamp appears in different timezones
    let sample_utc = 1704067200000; // 2024-01-01 15:00:00 UTC
    let sample_query = Query::new(&events)
        .where_(LocalEvent::utc_timestamp(), move |&ts| ts == sample_utc);
    let sample_events = sample_query.all();
    
    println!("  UTC timestamp: {} ({})", 
             sample_utc,
             DateTime::from_timestamp_millis(sample_utc)
                 .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
                 .format("%Y-%m-%d %H:%M:%S UTC"));
    
    println!("  Local time representations:");
    for event in &sample_events {
        if let Some(local_dt) = event.get_local_datetime() {
            println!("    {}: {} - {}", 
                     event.local_timezone,
                     local_dt.format("%Y-%m-%d %H:%M:%S %z"),
                     local_dt.format("%A, %B %d, %Y at %I:%M %p"));
        }
    }

    // ============================================================================
    // Timezone Offset Analysis
    // ============================================================================
    
    println!("\n🌍 Timezone Offset Analysis:");
    
    // Calculate timezone offsets from UTC
    let mut timezone_offsets = std::collections::HashMap::new();
    
    for event in &events {
        if let Some(local_dt) = event.get_local_datetime() {
            let offset_seconds = local_dt.offset().local_minus_utc();
            let offset_hours = offset_seconds / 3600;
            timezone_offsets.insert(event.local_timezone.clone(), offset_hours);
        }
    }
    
    let mut sorted_offsets: Vec<_> = timezone_offsets.iter().collect();
    sorted_offsets.sort_by(|a, b| a.1.cmp(b.1));
    
    println!("  Timezone offsets from UTC:");
    for (timezone, offset_hours) in &sorted_offsets {
        let offset_str = if **offset_hours >= 0 {
            format!("UTC+{}", offset_hours)
        } else {
            format!("UTC{}", offset_hours)
        };
        println!("    {}: {}", timezone, offset_str);
    }

    println!("\n✅ Local DateTime over UTC Epoch Demo Complete!");
    println!("\nKey Features Demonstrated:");
    println!("  • UTC timestamp storage with local timezone interpretation");
    println!("  • Timezone-aware business hours detection");
    println!("  • Cross-timezone event filtering and analysis");
    println!("  • Local time range filtering (morning/evening hours)");
    println!("  • Simultaneous event detection across timezones");
    println!("  • Duration analysis by timezone");
    println!("  • Category analysis with timezone context");
    println!("  • UTC vs local time comparison");
    println!("  • Timezone offset calculation and display");
    println!("  • Integration with i64 timestamp aggregators");
}
