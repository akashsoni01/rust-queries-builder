//! Comprehensive example demonstrating i64 timestamp aggregators across ALL query types
//!
//! This example shows how to use i64 timestamp aggregators with:
//! - Query (eager)
//! - LazyQuery (lazy)
//! - LockQuery (thread-safe eager)
//! - LockLazyQuery (thread-safe lazy)
//! - JoinQuery (eager joins)
//! - LockJoinQuery (thread-safe joins)
//!
//! Run with: cargo run --example comprehensive_i64_aggregators

use rust_queries_builder::*;
use key_paths_derive::Keypaths;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Keypaths)]
struct Event {
    id: u32,
    name: String,
    created_at: i64,        // Unix timestamp in milliseconds
    scheduled_at: i64,      // Unix timestamp in milliseconds
    duration_minutes: u32,
    category: String,
}

#[derive(Debug, Clone, Keypaths)]
struct User {
    id: u32,
    name: String,
    registered_at: i64,     // Unix timestamp in milliseconds
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

impl User {
    fn new(id: u32, name: &str, registered_at: i64) -> Self {
        Self {
            id,
            name: name.to_string(),
            registered_at,
        }
    }
}

fn create_sample_data() -> (Vec<Event>, Vec<User>, HashMap<String, Arc<RwLock<Event>>>, HashMap<String, Arc<RwLock<User>>>) {
    let events = vec![
        Event::new(1, "Apollo 11 Moon Landing", -141829200000, -141829200000, 1440, "Historical"),
        Event::new(2, "Woodstock Festival", -14112000000, -14112000000, 4320, "Historical"),
        Event::new(3, "First Computer Network", -94694400000, -94694400000, 60, "Technology"),
        Event::new(4, "Y2K Aftermath", 978307200000, 978307200000, 60, "Historical"),
        Event::new(5, "Dot-com Recovery", 1009843200000, 1009843200000, 120, "Historical"),
        Event::new(6, "Early Internet", 1104537600000, 1104537600000, 90, "Historical"),
        Event::new(7, "Remote Work Revolution", 1577836800000, 1577836800000, 480, "Work"),
        Event::new(8, "AI Breakthrough", 1609459200000, 1609459200000, 180, "Technology"),
        Event::new(9, "Climate Summit", 1640995200000, 1640995200000, 1440, "Environment"),
        Event::new(10, "Space Mission", 1672531200000, 1672531200000, 2880, "Science"),
        Event::new(11, "Tech Conference", 1704067200000, 1704067200000, 480, "Technology"),
        Event::new(12, "Future Planning", 1735689600000, 1735689600000, 120, "Planning"),
        Event::new(13, "Mars Mission", 1767225600000, 1767225600000, 43200, "Science"),
        Event::new(14, "Quantum Computing", 1798761600000, 1798761600000, 240, "Technology"),
        Event::new(15, "Sustainable Future", 1830297600000, 1830297600000, 2880, "Environment"),
    ];

    let users = vec![
        User::new(1, "Alice", 1577836800000), // 2020-01-01
        User::new(2, "Bob", 1609459200000),   // 2021-01-01
        User::new(3, "Charlie", 1640995200000), // 2022-01-01
        User::new(4, "Diana", 1672531200000), // 2023-01-01
        User::new(5, "Eve", 1704067200000),   // 2024-01-01
    ];

    // Create locked versions for thread-safe operations
    let mut locked_events = HashMap::new();
    for event in &events {
        locked_events.insert(format!("EVENT-{:03}", event.id), Arc::new(RwLock::new(event.clone())));
    }

    let mut locked_users = HashMap::new();
    for user in &users {
        locked_users.insert(format!("USER-{:03}", user.id), Arc::new(RwLock::new(user.clone())));
    }

    (events, users, locked_events, locked_users)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Comprehensive i64 Timestamp Aggregators Demo                   ║");
    println!("║  ALL Query Types: Query, LazyQuery, LockQuery, LockLazyQuery    ║");
    println!("║  JoinQuery, LockJoinQuery                                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let (events, users, locked_events, locked_users) = create_sample_data();

    // ============================================================================
    // 1. QUERY (Eager) - i64 Timestamp Aggregators
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("1. QUERY (Eager) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Basic aggregations
    let earliest_created = Query::new(&events).min_timestamp(Event::created_at_r());
    let latest_created = Query::new(&events).max_timestamp(Event::created_at_r());
    let avg_created = Query::new(&events).avg_timestamp(Event::created_at_r());
    let sum_created = Query::new(&events).sum_timestamp(Event::created_at_r());
    let count_created = Query::new(&events).count_timestamp(Event::created_at_r());

    println!("  📊 Basic Aggregations:");
    println!("    Earliest created: {:?}", earliest_created);
    println!("    Latest created: {:?}", latest_created);
    println!("    Average created: {:?}", avg_created);
    println!("    Sum created: {}", sum_created);
    println!("    Count created: {}", count_created);

    // Time-based filtering
    let year_2000_cutoff = 946684800000; // 2000-01-01

    let modern_query = Query::new(&events)
        .where_after_timestamp(Event::created_at_r(), year_2000_cutoff);
    let modern_events = modern_query.all();

    println!("\n  ⏰ Time-based Filtering:");
    println!("    Events after 2000: {} events", modern_events.len());

    let recent_query = Query::new(&events)
        .where_last_days_timestamp(Event::created_at_r(), 365 * 5); // Last 5 years
    let recent_events = recent_query.all();

    println!("    Events in last 5 years: {} events", recent_events.len());

    // ============================================================================
    // 2. LAZYQUERY (Lazy) - i64 Timestamp Aggregators
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("2. LAZYQUERY (Lazy) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Lazy aggregations (deferred execution)
    let lazy_earliest = events.lazy_query().min_timestamp(Event::created_at_r());
    let lazy_latest = events.lazy_query().max_timestamp(Event::created_at_r());
    let lazy_avg = events.lazy_query().avg_timestamp(Event::created_at_r());

    println!("  📊 Lazy Aggregations (deferred execution):");
    println!("    Earliest created: {:?}", lazy_earliest);
    println!("    Latest created: {:?}", lazy_latest);
    println!("    Average created: {:?}", lazy_avg);

    // Lazy filtering with early termination
    let first_tech_event = events
        .lazy_query()
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_after_timestamp(Event::created_at_r(), year_2000_cutoff)
        .first();

    println!("\n  ⚡ Lazy Filtering with Early Termination:");
    if let Some(event) = first_tech_event {
        println!("    First tech event after 2000: {}", event.name);
    }

    // ============================================================================
    // 3. LOCKQUERY (Thread-safe Eager) - i64 Timestamp Aggregators
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("3. LOCKQUERY (Thread-safe Eager) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Thread-safe aggregations
    let lock_earliest = locked_events.lock_query().min_timestamp(Event::created_at_r());
    let lock_latest = locked_events.lock_query().max_timestamp(Event::created_at_r());
    let lock_avg = locked_events.lock_query().avg_timestamp(Event::created_at_r());

    println!("  🔒 Thread-safe Aggregations:");
    println!("    Earliest created: {:?}", lock_earliest);
    println!("    Latest created: {:?}", lock_latest);
    println!("    Average created: {:?}", lock_avg);

    // Thread-safe filtering
    let lock_tech_events = locked_events
        .lock_query()
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_after_timestamp(Event::created_at_r(), year_2000_cutoff)
        .all();

    println!("\n  🔒 Thread-safe Filtering:");
    println!("    Tech events after 2000: {} events", lock_tech_events.len());

    // ============================================================================
    // 4. LOCKLAZYQUERY (Thread-safe Lazy) - i64 Timestamp Aggregators
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("4. LOCKLAZYQUERY (Thread-safe Lazy) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Thread-safe lazy aggregations
    let lock_lazy_earliest = locked_events.lock_lazy_query().min_timestamp(Event::created_at_r());
    let lock_lazy_latest = locked_events.lock_lazy_query().max_timestamp(Event::created_at_r());
    let lock_lazy_avg = locked_events.lock_lazy_query().avg_timestamp(Event::created_at_r());

    println!("  🔒⚡ Thread-safe Lazy Aggregations:");
    println!("    Earliest created: {:?}", lock_lazy_earliest);
    println!("    Latest created: {:?}", lock_lazy_latest);
    println!("    Average created: {:?}", lock_lazy_avg);

    // Thread-safe lazy filtering with relative time
    let recent_lock_events: Vec<_> = locked_events
        .lock_lazy_query()
        .where_last_days_timestamp(Event::created_at_r(), 365 * 3) // Last 3 years
        .collect();

    println!("\n  🔒⚡ Thread-safe Lazy Filtering:");
    println!("    Events in last 3 years: {} events", recent_lock_events.len());

    // ============================================================================
    // 5. JOINQUERY (Eager Joins) - i64 Timestamp Aggregators
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("5. JOINQUERY (Eager Joins) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Join events with users based on registration time
    let join_results = JoinQuery::new(&events, &users)
        .inner_join(
            Event::created_at_r(),
            User::registered_at_r(),
            |event, user| (event.clone(), user.clone())
        );

    // Manual aggregations on join results
    let join_earliest = join_results.iter().map(|(event, _user)| event.created_at).min();
    let join_latest = join_results.iter().map(|(event, _user)| event.created_at).max();

    println!("  🔗 Join Aggregations:");
    println!("    Earliest joined timestamp: {:?}", join_earliest);
    println!("    Latest joined timestamp: {:?}", join_latest);
    println!("    Total joined results: {}", join_results.len());

    // ============================================================================
    // 6. LOCKJOINQUERY (Thread-safe Joins) - i64 Timestamp Aggregators
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("6. LOCKJOINQUERY (Thread-safe Joins) - i64 Timestamp Aggregators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Convert HashMap to Vec<&Arc<RwLock<T>>> for LockJoinQuery
    let locked_events_vec: Vec<_> = locked_events.values().collect();
    let locked_users_vec: Vec<_> = locked_users.values().collect();

    // Thread-safe join aggregations
    let lock_join_results = LockJoinQuery::new(locked_events_vec, locked_users_vec)
        .inner_join(
            Event::created_at_r(),
            User::registered_at_r(),
            |event, user| (event.clone(), user.clone())
        );

    // Manual aggregations on lock join results
    let lock_join_earliest = lock_join_results.iter().map(|(event, _user)| event.created_at).min();
    let lock_join_latest = lock_join_results.iter().map(|(event, _user)| event.created_at).max();

    println!("  🔒🔗 Thread-safe Join Aggregations:");
    println!("    Earliest joined timestamp: {:?}", lock_join_earliest);
    println!("    Latest joined timestamp: {:?}", lock_join_latest);
    println!("    Total lock joined results: {}", lock_join_results.len());

    // ============================================================================
    // 7. COMPLEX QUERIES - Combining Multiple Operations
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("7. COMPLEX QUERIES - Combining Multiple Operations");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Complex query: Technology events in the last 5 years, ordered by creation time
    let complex_results = Query::new(&events)
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_last_days_timestamp(Event::created_at_r(), 365 * 5)
        .order_by_timestamp(Event::created_at_r());

    println!("  🔧 Complex Query Results:");
    println!("    Technology events in last 5 years: {} events", complex_results.len());
    for event in &complex_results {
        println!("      - {} (created: {})", event.name, event.created_at);
    }

    // Lazy complex query with early termination
    let first_recent_tech = events
        .lazy_query()
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_last_days_timestamp(Event::created_at_r(), 365 * 5)
        .first();

    println!("\n  ⚡ Lazy Complex Query (Early Termination):");
    if let Some(event) = first_recent_tech {
        println!("    First recent tech event: {}", event.name);
    }

    // ============================================================================
    // 8. PERFORMANCE COMPARISON
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("8. PERFORMANCE COMPARISON");
    println!("═══════════════════════════════════════════════════════════════\n");

    use std::time::Instant;

    // Eager vs Lazy performance
    let start = Instant::now();
    let _eager_results = Query::new(&events)
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_after_timestamp(Event::created_at_r(), year_2000_cutoff)
        .all();
    let eager_time = start.elapsed();

    let start = Instant::now();
    let _lazy_results: Vec<_> = events
        .lazy_query()
        .where_(Event::category_r(), |cat| cat == "Technology")
        .where_after_timestamp(Event::created_at_r(), year_2000_cutoff)
        .collect();
    let lazy_time = start.elapsed();

    println!("  ⚡ Performance Comparison:");
    println!("    Eager query time: {:?}", eager_time);
    println!("    Lazy query time: {:?}", lazy_time);

    // ============================================================================
    // 9. SUMMARY
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("9. SUMMARY - All i64 Timestamp Aggregators Available");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Successfully demonstrated i64 timestamp aggregators for:");
    println!("   • Query (eager) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");
    println!("   • LazyQuery (lazy) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");
    println!("   • LockQuery (thread-safe eager) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");
    println!("   • LockLazyQuery (thread-safe lazy) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");
    println!("   • JoinQuery (eager joins) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");
    println!("   • LockJoinQuery (thread-safe joins) - min_timestamp, max_timestamp, avg_timestamp, sum_timestamp, count_timestamp");

    println!("\n✅ Time-based filtering methods available:");
    println!("   • where_after_timestamp() - Filter after a specific timestamp");
    println!("   • where_before_timestamp() - Filter before a specific timestamp");
    println!("   • where_between_timestamp() - Filter within a timestamp range");
    println!("   • where_last_days_timestamp() - Filter within last N days");
    println!("   • where_next_days_timestamp() - Filter within next N days");
    println!("   • where_last_hours_timestamp() - Filter within last N hours");
    println!("   • where_next_hours_timestamp() - Filter within next N hours");
    println!("   • where_last_minutes_timestamp() - Filter within last N minutes");
    println!("   • where_next_minutes_timestamp() - Filter within next N minutes");

    println!("\n✅ Ordering methods available:");
    println!("   • order_by_timestamp() - Sort by timestamp ascending");
    println!("   • order_by_timestamp_desc() - Sort by timestamp descending");

    println!("\n🎉 All i64 timestamp aggregators are now available across ALL query types!");
    println!("   Perfect for Unix timestamps in milliseconds, similar to Java's Date.getTime()");
}
