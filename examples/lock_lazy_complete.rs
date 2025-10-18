// Complete LockLazyQuery API Showcase - Demonstrates ALL lock-aware lazy operations
// This example provides a complete reference for:
// 1. Basic lazy operations (where_, count, first, exists, etc.)
// 2. Aggregations (sum, avg, min, max, etc.)
// 3. DateTime operations (SystemTime and chrono)
// 4. Ordering operations (order_by, order_by_float, etc.)
// 5. Grouping operations (group_by)
// 6. SQL-like operations (distinct, limit, skip, etc.)
//
// Run with: cargo run --example lock_lazy_complete --features datetime

use rust_queries_builder::LockLazyQueryable;
use key_paths_derive::Keypath;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::{SystemTime, Duration as StdDuration};

#[cfg(feature = "datetime")]
use chrono::{DateTime, Utc, Duration, TimeZone};

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
    created_at: SystemTime,
}

#[cfg(feature = "datetime")]
#[derive(Debug, Clone, Keypath)]
struct Event {
    id: u32,
    title: String,
    scheduled_at: DateTime<Utc>,
    category: String,
    priority: u32,
}

fn create_locked_products() -> HashMap<String, Arc<RwLock<Product>>> {
    let now = SystemTime::now();
    let mut products = HashMap::new();
    
    products.insert("p1".to_string(), Arc::new(RwLock::new(Product {
        id: 1,
        name: "Laptop Pro".to_string(),
        price: 1299.99,
        category: "Electronics".to_string(),
        stock: 15,
        rating: 4.8,
        created_at: now - StdDuration::from_secs(86400 * 30),
    })));
    
    products.insert("p2".to_string(), Arc::new(RwLock::new(Product {
        id: 2,
        name: "Wireless Mouse".to_string(),
        price: 29.99,
        category: "Electronics".to_string(),
        stock: 50,
        rating: 4.5,
        created_at: now - StdDuration::from_secs(86400 * 20),
    })));
    
    products.insert("p3".to_string(), Arc::new(RwLock::new(Product {
        id: 3,
        name: "Mechanical Keyboard".to_string(),
        price: 129.99,
        category: "Electronics".to_string(),
        stock: 30,
        rating: 4.7,
        created_at: now - StdDuration::from_secs(86400 * 10),
    })));
    
    products.insert("p4".to_string(), Arc::new(RwLock::new(Product {
        id: 4,
        name: "Office Chair".to_string(),
        price: 299.99,
        category: "Furniture".to_string(),
        stock: 20,
        rating: 4.6,
        created_at: now - StdDuration::from_secs(86400 * 15),
    })));
    
    products.insert("p5".to_string(), Arc::new(RwLock::new(Product {
        id: 5,
        name: "Standing Desk".to_string(),
        price: 499.99,
        category: "Furniture".to_string(),
        stock: 10,
        rating: 4.9,
        created_at: now - StdDuration::from_secs(86400 * 5),
    })));
    
    products.insert("p6".to_string(), Arc::new(RwLock::new(Product {
        id: 6,
        name: "USB-C Hub".to_string(),
        price: 49.99,
        category: "Electronics".to_string(),
        stock: 100,
        rating: 4.3,
        created_at: now - StdDuration::from_secs(86400 * 25),
    })));
    
    products
}

#[cfg(feature = "datetime")]
fn create_locked_events() -> HashMap<String, Arc<RwLock<Event>>> {
    let base = Utc.with_ymd_and_hms(2024, 10, 1, 9, 0, 0).unwrap();
    let mut events = HashMap::new();
    
    events.insert("e1".to_string(), Arc::new(RwLock::new(Event {
        id: 1,
        title: "Team Meeting".to_string(),
        scheduled_at: base + Duration::hours(2),
        category: "Work".to_string(),
        priority: 3,
    })));
    
    events.insert("e2".to_string(), Arc::new(RwLock::new(Event {
        id: 2,
        title: "Weekend Brunch".to_string(),
        scheduled_at: Utc.with_ymd_and_hms(2024, 10, 5, 10, 0, 0).unwrap(), // Saturday
        category: "Personal".to_string(),
        priority: 2,
    })));
    
    events.insert("e3".to_string(), Arc::new(RwLock::new(Event {
        id: 3,
        title: "Lunch with Client".to_string(),
        scheduled_at: base + Duration::hours(28), // Next day at 1 PM
        category: "Work".to_string(),
        priority: 5,
    })));
    
    events.insert("e4".to_string(), Arc::new(RwLock::new(Event {
        id: 4,
        title: "Gym Session".to_string(),
        scheduled_at: base + Duration::hours(10), // 7 PM same day
        category: "Personal".to_string(),
        priority: 1,
    })));
    
    events.insert("e5".to_string(), Arc::new(RwLock::new(Event {
        id: 5,
        title: "Project Deadline".to_string(),
        scheduled_at: Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 0).unwrap(),
        category: "Work".to_string(),
        priority: 5,
    })));
    
    events.insert("e6".to_string(), Arc::new(RwLock::new(Event {
        id: 6,
        title: "Morning Workout".to_string(),
        scheduled_at: base + Duration::hours(-1), // 8 AM same day
        category: "Personal".to_string(),
        priority: 2,
    })));
    
    events
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     COMPLETE LOCK LAZY QUERY SHOWCASE - ALL OPERATIONS      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let products = create_locked_products();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 1: BASIC LAZY OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. BASIC LAZY OPERATIONS (Early Termination)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // where_ - Filter by predicate
    println!("✓ where_(path, predicate) - Filter by predicate (lazy)");
    let electronics_query = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics");
    println!("  Set up electronics filter (no evaluation yet)\n");

    // count - Terminal operation
    println!("✓ count() - Count matching items (terminal)");
    let count = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .count();
    println!("  Electronics count: {}\n", count);

    // first - Terminal with early termination
    println!("✓ first() - Get first matching item (terminal, early termination)");
    let first = products
        .lock_lazy_query()
        .where_(Product::price(), |&p| p > 100.0)
        .first();
    if let Some(p) = first {
        println!("  First expensive product: {} (${:.2})\n", p.name, p.price);
    }

    // all - Terminal operation (collect)
    println!("✓ all() - Get all matching items (terminal)");
    let all_electronics = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .all();
    println!("  All electronics ({} items):", all_electronics.len());
    for p in &all_electronics {
        println!("    • {}", p.name);
    }
    println!();

    // exists / any - Early termination
    println!("✓ exists() / any() - Check if any match (terminal, early termination)");
    let has_expensive = products
        .lock_lazy_query()
        .where_(Product::price(), |&p| p > 1000.0)
        .exists();
    println!("  Has products over $1000: {}\n", has_expensive);

    // limit - Lazy operation
    println!("✓ limit(n) - Limit results (lazy)");
    let limited: Vec<_> = products
        .lock_lazy_query()
        .where_(Product::stock(), |&s| s > 0)
        .limit(3)
        .collect();
    println!("  First 3 in-stock products:");
    for p in &limited {
        println!("    • {}", p.name);
    }
    println!();

    // skip - Lazy operation
    println!("✓ skip(n) - Skip results for pagination (lazy)");
    let page2: Vec<_> = products
        .lock_lazy_query()
        .skip(2)
        .limit(2)
        .collect();
    println!("  Page 2 (skip 2, take 2):");
    for p in &page2 {
        println!("    • {}", p.name);
    }
    println!();

    // last - Terminal (consumes iterator)
    println!("✓ last() - Get last matching item (terminal)");
    let last = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .last();
    if let Some(p) = last {
        println!("  Last electronics: {}\n", p.name);
    }

    // nth - Terminal with skip
    println!("✓ nth(n) - Get item at index (terminal)");
    let third = products.lock_lazy_query().nth(2);
    if let Some(p) = third {
        println!("  3rd product: {}\n", p.name);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 2: SELECTION & PROJECTION
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. SELECTION & PROJECTION (Lazy)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // select_lazy - Project field
    println!("✓ select_lazy(path) - Project field (lazy)");
    let names: Vec<String> = products
        .lock_lazy_query()
        .where_(Product::price(), |&p| p < 200.0)
        .select_lazy(Product::name())
        .collect();
    println!("  Affordable product names ({}):", names.len());
    for name in &names {
        println!("    • {}", name);
    }
    println!();

    // distinct - Terminal with HashSet
    println!("✓ distinct(path) - Get distinct values (terminal)");
    let categories = products
        .lock_lazy_query()
        .distinct(Product::category());
    println!("  Distinct categories: {:?}\n", categories);

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 3: AGGREGATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. AGGREGATIONS (Terminal Operations)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let electronics = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics");

    // sum
    println!("✓ sum(path) - Sum numeric field (terminal)");
    let total_price = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .sum(Product::price());
    println!("  Total electronics value: ${:.2}\n", total_price);

    // avg
    println!("✓ avg(path) - Average of f64 field (terminal)");
    let avg_price = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .avg(Product::price());
    println!("  Average electronics price: ${:.2}\n", avg_price.unwrap_or(0.0));

    // min / max
    println!("✓ min(path) / max(path) - Min/max of Ord field (terminal)");
    let min_stock = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .min(Product::stock());
    let max_stock = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .max(Product::stock());
    println!("  Min stock: {:?}", min_stock);
    println!("  Max stock: {:?}\n", max_stock);

    // min_float / max_float
    println!("✓ min_float(path) / max_float(path) - Min/max of f64 field (terminal)");
    let min_price = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .min_float(Product::price());
    let max_price = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .max_float(Product::price());
    println!("  Min price: ${:.2}", min_price.unwrap_or(0.0));
    println!("  Max price: ${:.2}\n", max_price.unwrap_or(0.0));

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 4: ADVANCED FILTERING
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. ADVANCED FILTERING");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // all_match - Check if all items match
    println!("✓ all_match(path, predicate) - Check if all match (terminal)");
    let all_in_stock = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .all_match(Product::stock(), |&s| s > 0);
    println!("  All electronics in stock: {}\n", all_in_stock);

    // find - Find first matching additional predicate
    println!("✓ find(path, predicate) - Find first with condition (terminal)");
    let expensive = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .find(Product::price(), |&p| p > 500.0);
    if let Some(p) = expensive {
        println!("  Found expensive electronics: {} (${:.2})\n", p.name, p.price);
    } else {
        println!("  No expensive electronics found\n");
    }

    // count_where - Count with additional condition
    println!("✓ count_where(path, predicate) - Count with condition (terminal)");
    let expensive_count = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .count_where(Product::price(), |&p| p > 100.0);
    println!("  Expensive electronics count: {}\n", expensive_count);

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 5: DATETIME OPERATIONS (SystemTime)
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. DATETIME OPERATIONS - SystemTime (Always Available)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let now = SystemTime::now();
    let two_weeks_ago = now - StdDuration::from_secs(86400 * 14);
    let one_week_ago = now - StdDuration::from_secs(86400 * 7);

    // where_after_systemtime
    println!("✓ where_after_systemtime(path, time) - Filter after SystemTime (lazy)");
    let recent_count = products
        .lock_lazy_query()
        .where_after_systemtime(Product::created_at(), two_weeks_ago)
        .count();
    println!("  Products created in last 2 weeks: {}\n", recent_count);

    // where_before_systemtime
    println!("✓ where_before_systemtime(path, time) - Filter before SystemTime (lazy)");
    let old_count = products
        .lock_lazy_query()
        .where_before_systemtime(Product::created_at(), two_weeks_ago)
        .count();
    println!("  Products created before 2 weeks ago: {}\n", old_count);

    // where_between_systemtime
    println!("✓ where_between_systemtime(path, start, end) - Filter within range (lazy)");
    let in_range = products
        .lock_lazy_query()
        .where_between_systemtime(Product::created_at(), two_weeks_ago, one_week_ago)
        .all();
    println!("  Products created 1-2 weeks ago: {}", in_range.len());
    for p in &in_range {
        println!("    • {}", p.name);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 6: ORDERING OPERATIONS (require T: Clone)
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("6. ORDERING OPERATIONS (Terminal, require T: Clone)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // order_by
    println!("✓ order_by(path) - Sort ascending (terminal)");
    let sorted_by_name = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .order_by(Product::name());
    println!("  Electronics sorted by name:");
    for p in sorted_by_name.iter().take(3) {
        println!("    • {}", p.name);
    }
    println!();

    // order_by_desc
    println!("✓ order_by_desc(path) - Sort descending (terminal)");
    let sorted_desc = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .order_by_desc(Product::stock());
    println!("  Electronics sorted by stock (desc):");
    for p in sorted_desc.iter().take(3) {
        println!("    • {} - Stock: {}", p.name, p.stock);
    }
    println!();

    // order_by_float
    println!("✓ order_by_float(path) - Sort f64 ascending (terminal)");
    let sorted_by_price = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .order_by_float(Product::price());
    println!("  Electronics sorted by price:");
    for p in sorted_by_price.iter().take(3) {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!();

    // order_by_float_desc
    println!("✓ order_by_float_desc(path) - Sort f64 descending (terminal)");
    let sorted_by_rating = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .order_by_float_desc(Product::rating());
    println!("  Electronics sorted by rating (desc):");
    for p in sorted_by_rating.iter().take(3) {
        println!("    • {} - Rating: {:.1}", p.name, p.rating);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 7: GROUPING OPERATIONS (require T: Clone)
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("7. GROUPING OPERATIONS (Terminal, require T: Clone)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // group_by
    println!("✓ group_by(path) - Group by field (terminal)");
    let by_category = products
        .lock_lazy_query()
        .where_(Product::stock(), |&s| s > 0)
        .group_by(Product::category());
    
    println!("  Products grouped by category:");
    for (category, items) in &by_category {
        println!("    {} ({} items):", category, items.len());
        for item in items {
            println!("      - {} (${:.2})", item.name, item.price);
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 8: DATETIME OPERATIONS (Chrono)
    // ═══════════════════════════════════════════════════════════════════════
    #[cfg(feature = "datetime")]
    {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("8. DATETIME OPERATIONS - Chrono (with 'datetime' feature)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let events = create_locked_events();
        let cutoff = Utc.with_ymd_and_hms(2024, 10, 2, 0, 0, 0).unwrap();
        let oct_start = Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
        let oct_end = Utc.with_ymd_and_hms(2024, 10, 31, 23, 59, 59).unwrap();

        // where_after
        println!("✓ where_after(path, time) - Filter after datetime (lazy)");
        let after = events
            .lock_lazy_query()
            .where_after(Event::scheduled_at(), cutoff)
            .count();
        println!("  Events after Oct 2: {}\n", after);

        // where_before
        println!("✓ where_before(path, time) - Filter before datetime (lazy)");
        let before = events
            .lock_lazy_query()
            .where_before(Event::scheduled_at(), cutoff)
            .count();
        println!("  Events before Oct 2: {}\n", before);

        // where_between
        println!("✓ where_between(path, start, end) - Filter within range (lazy)");
        let between = events
            .lock_lazy_query()
            .where_between(Event::scheduled_at(), oct_start, oct_end)
            .all();
        println!("  Events in October: {}", between.len());
        for e in &between {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%Y-%m-%d"));
        }
        println!();

        // where_today
        println!("✓ where_today(path, now) - Filter for today (lazy)");
        let today_ref = Utc.with_ymd_and_hms(2024, 10, 1, 15, 0, 0).unwrap();
        let today = events
            .lock_lazy_query()
            .where_today(Event::scheduled_at(), today_ref)
            .all();
        println!("  Events on Oct 1: {}", today.len());
        for e in &today {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%H:%M"));
        }
        println!();

        // where_year
        println!("✓ where_year(path, year) - Filter by year (lazy)");
        let year_2024 = events
            .lock_lazy_query()
            .where_year(Event::scheduled_at(), 2024)
            .count();
        println!("  Events in 2024: {}\n", year_2024);

        // where_month
        println!("✓ where_month(path, month) - Filter by month (lazy)");
        let october = events
            .lock_lazy_query()
            .where_month(Event::scheduled_at(), 10)
            .count();
        println!("  Events in October: {}\n", october);

        // where_day
        println!("✓ where_day(path, day) - Filter by day (lazy)");
        let first_day = events
            .lock_lazy_query()
            .where_day(Event::scheduled_at(), 1)
            .count();
        println!("  Events on the 1st: {}\n", first_day);

        // where_weekend
        println!("✓ where_weekend(path) - Filter for weekends (lazy)");
        let weekend = events
            .lock_lazy_query()
            .where_weekend(Event::scheduled_at())
            .all();
        println!("  Weekend events:");
        for e in &weekend {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%A, %Y-%m-%d"));
        }
        println!();

        // where_weekday
        println!("✓ where_weekday(path) - Filter for weekdays (lazy)");
        let weekday = events
            .lock_lazy_query()
            .where_weekday(Event::scheduled_at())
            .count();
        println!("  Weekday events: {}\n", weekday);

        // where_business_hours
        println!("✓ where_business_hours(path) - Filter for business hours (lazy)");
        let business = events
            .lock_lazy_query()
            .where_business_hours(Event::scheduled_at())
            .all();
        println!("  Events during business hours:");
        for e in &business {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%H:%M"));
        }
        println!();
    }

    #[cfg(not(feature = "datetime"))]
    {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("8. DATETIME OPERATIONS - Chrono");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("⚠️  Chrono datetime features require the 'datetime' feature.");
        println!("Run with: cargo run --example lock_lazy_complete --features datetime\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 9: COMPLEX EXAMPLES
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("9. COMPLEX QUERY EXAMPLES");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Complex 1: Multi-filter with early termination
    println!("Example 1: First expensive, high-rated electronics");
    let complex1 = products
        .lock_lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p > 100.0)
        .where_(Product::rating(), |&r| r >= 4.5)
        .first();
    
    if let Some(p) = complex1 {
        println!("  Found: {} - ${:.2} (Rating: {:.1})\n", p.name, p.price, p.rating);
    }

    // Complex 2: Aggregation with filtering
    println!("Example 2: Statistics for high-stock items");
    let high_stock_items = products
        .lock_lazy_query()
        .where_(Product::stock(), |&s| s >= 20);
    
    println!("  Count: {}", high_stock_items.count());
    
    let total_value = products
        .lock_lazy_query()
        .where_(Product::stock(), |&s| s >= 20)
        .sum(Product::price());
    println!("  Total value: ${:.2}", total_value);
    
    let avg_rating = products
        .lock_lazy_query()
        .where_(Product::stock(), |&s| s >= 20)
        .avg(Product::rating());
    println!("  Average rating: {:.2}\n", avg_rating.unwrap_or(0.0));

    // Complex 3: Pagination with ordering
    println!("Example 3: Top 2 products by price");
    let top_products = products
        .lock_lazy_query()
        .order_by_float_desc(Product::price());
    
    println!("  Top 2 most expensive:");
    for p in top_products.iter().take(2) {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║            ✓ LOCK LAZY QUERY SHOWCASE COMPLETE              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 All LockLazyQuery Operations Demonstrated:");
    println!("  ✓ Basic Operations: where_, count, first, all, exists/any,");
    println!("                      limit, skip, last, nth");
    println!("  ✓ Selection: select_lazy, distinct");
    println!("  ✓ Aggregations: sum, avg, min, max, min_float, max_float");
    println!("  ✓ Advanced: all_match, find, count_where");
    println!("  ✓ DateTime SystemTime: where_after, where_before, where_between");
    
    #[cfg(feature = "datetime")]
    println!("  ✓ DateTime Chrono: where_after, where_before, where_between,");
    println!("                     where_today, where_year, where_month, where_day,");
    println!("                     where_weekend, where_weekday, where_business_hours");
    
    #[cfg(not(feature = "datetime"))]
    println!("  ⚠️  DateTime Chrono: Enable with --features datetime");
    
    println!("  ✓ Ordering: order_by, order_by_desc, order_by_float, order_by_float_desc");
    println!("  ✓ Grouping: group_by");

    println!("\n🚀 Performance Benefits:");
    println!("  • Early termination with first(), exists(), any()");
    println!("  • Lazy evaluation - no intermediate collections");
    println!("  • Iterator fusion - Rust optimizes chained operations");
    println!("  • Lock-aware - works with Mutex, RwLock, Arc, etc.");

    println!("\n💡 Next Steps:");
    println!("  • Check examples/complete_api_showcase.rs for Query operations");
    println!("  • See examples/lazy_evaluation.rs for performance comparisons");
    println!("  • Read the documentation for more advanced patterns");
}

