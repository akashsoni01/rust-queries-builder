// Complete API Showcase - Demonstrates ALL operations with examples
// This example provides a complete reference for:
// 1. Query operations (basic, ordering, projection, grouping, aggregations)
// 2. DateTime operations (SystemTime and chrono)
// 3. LockLazyQuery operations (all helper functions)
// 4. JoinQuery operations (all join types)
//
// Run with: cargo run --example complete_api_showcase --features datetime

use rust_queries_builder::{Query, JoinQuery};
use key_paths_derive::Keypath;
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

#[derive(Debug, Clone, Keypath)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    total: f64,
}

#[derive(Debug, Clone)]
struct OrderDetail {
    order_id: u32,
    product_name: String,
    quantity: u32,
    total: f64,
}

fn create_products() -> Vec<Product> {
    let now = SystemTime::now();
    vec![
        Product {
            id: 1,
            name: "Laptop Pro".to_string(),
            price: 1299.99,
            category: "Electronics".to_string(),
            stock: 15,
            rating: 4.8,
            created_at: now - StdDuration::from_secs(86400 * 30),
        },
        Product {
            id: 2,
            name: "Wireless Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
            stock: 50,
            rating: 4.5,
            created_at: now - StdDuration::from_secs(86400 * 20),
        },
        Product {
            id: 3,
            name: "Mechanical Keyboard".to_string(),
            price: 129.99,
            category: "Electronics".to_string(),
            stock: 30,
            rating: 4.7,
            created_at: now - StdDuration::from_secs(86400 * 10),
        },
        Product {
            id: 4,
            name: "Office Chair".to_string(),
            price: 299.99,
            category: "Furniture".to_string(),
            stock: 20,
            rating: 4.6,
            created_at: now - StdDuration::from_secs(86400 * 15),
        },
        Product {
            id: 5,
            name: "Standing Desk".to_string(),
            price: 499.99,
            category: "Furniture".to_string(),
            stock: 10,
            rating: 4.9,
            created_at: now - StdDuration::from_secs(86400 * 5),
        },
    ]
}

fn create_orders() -> Vec<Order> {
    vec![
        Order { id: 1, product_id: 1, quantity: 2, total: 2599.98 },
        Order { id: 2, product_id: 2, quantity: 5, total: 149.95 },
        Order { id: 3, product_id: 3, quantity: 1, total: 129.99 },
        Order { id: 4, product_id: 5, quantity: 3, total: 1499.97 },
    ]
}

#[cfg(feature = "datetime")]
fn create_events() -> Vec<Event> {
    let base = Utc.with_ymd_and_hms(2024, 10, 1, 9, 0, 0).unwrap();
    vec![
        Event {
            id: 1,
            title: "Team Meeting".to_string(),
            scheduled_at: base + Duration::hours(2),
            category: "Work".to_string(),
            priority: 3,
        },
        Event {
            id: 2,
            title: "Weekend Brunch".to_string(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 10, 5, 10, 0, 0).unwrap(), // Saturday
            category: "Personal".to_string(),
            priority: 2,
        },
        Event {
            id: 3,
            title: "Lunch with Client".to_string(),
            scheduled_at: base + Duration::hours(28), // Next day at 1 PM
            category: "Work".to_string(),
            priority: 5,
        },
        Event {
            id: 4,
            title: "Gym Session".to_string(),
            scheduled_at: base + Duration::hours(10), // 7 PM same day
            category: "Personal".to_string(),
            priority: 1,
        },
        Event {
            id: 5,
            title: "Project Deadline".to_string(),
            scheduled_at: Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 0).unwrap(),
            category: "Work".to_string(),
            priority: 5,
        },
    ]
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          COMPLETE API SHOWCASE - ALL OPERATIONS              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let products = create_products();
    let orders = create_orders();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 1: BASIC QUERY OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. BASIC QUERY OPERATIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // new(data: &[T])
    println!("✓ new(data: &[T]) - Create a new query");
    let _query = Query::new(&products);
    println!("  Created query from {} products\n", products.len());

    // where_(path, predicate)
    println!("✓ where_(path, predicate) - Filter by predicate");
    let electronics = Query::new(&products)
        .where_(Product::category(), |cat| cat == "Electronics");
    println!("  Electronics count: {}\n", electronics.count());

    // all()
    println!("✓ all() - Get all matching items");
    let all_electronics = electronics.all();
    println!("  Found {} electronics:", all_electronics.len());
    for p in &all_electronics {
        println!("    • {}", p.name);
    }
    println!();

    // first()
    println!("✓ first() - Get first matching item");
    let query_first = Query::new(&products);
    let first_product = query_first.first();
    if let Some(p) = first_product {
        println!("  First product: {}\n", p.name);
    }

    // count()
    println!("✓ count() - Count matching items");
    let count = Query::new(&products)
        .where_(Product::price(), |&price| price > 100.0)
        .count();
    println!("  Products over $100: {}\n", count);

    // limit(n)
    println!("✓ limit(n) - Limit results");
    let query_limit = Query::new(&products);
    let limited = query_limit.limit(3);
    println!("  First 3 products:");
    for p in &limited {
        println!("    • {}", p.name);
    }
    println!();

    // skip(n)
    println!("✓ skip(n) - Skip results for pagination");
    let query_skip = Query::new(&products);
    let page2 = query_skip.skip(2).limit(2);
    println!("  Page 2 (skip 2, take 2):");
    for p in &page2 {
        println!("    • {}", p.name);
    }
    println!();

    // exists()
    println!("✓ exists() - Check if any match");
    let has_expensive = Query::new(&products)
        .where_(Product::price(), |&price| price > 1000.0)
        .exists();
    println!("  Has products over $1000: {}\n", has_expensive);

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 2: ORDERING OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. ORDERING OPERATIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // order_by(path)
    println!("✓ order_by(path) - Sort ascending");
    let sorted_by_name = Query::new(&products).order_by(Product::name());
    println!("  Products sorted by name:");
    for p in sorted_by_name.iter().take(3) {
        println!("    • {}", p.name);
    }
    println!();

    // order_by_desc(path)
    println!("✓ order_by_desc(path) - Sort descending");
    let sorted_desc = Query::new(&products).order_by_desc(Product::stock());
    println!("  Products sorted by stock (descending):");
    for p in sorted_desc.iter().take(3) {
        println!("    • {} - Stock: {}", p.name, p.stock);
    }
    println!();

    // order_by_float(path)
    println!("✓ order_by_float(path) - Sort f64 ascending");
    let sorted_by_price = Query::new(&products).order_by_float(Product::price());
    println!("  Products sorted by price:");
    for p in sorted_by_price.iter().take(3) {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!();

    // order_by_float_desc(path)
    println!("✓ order_by_float_desc(path) - Sort f64 descending");
    let sorted_by_rating = Query::new(&products).order_by_float_desc(Product::rating());
    println!("  Products sorted by rating (descending):");
    for p in sorted_by_rating.iter().take(3) {
        println!("    • {} - Rating: {:.1}", p.name, p.rating);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 3: PROJECTION & GROUPING
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. PROJECTION & GROUPING");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // select(path)
    println!("✓ select(path) - Project field");
    let names = Query::new(&products).select(Product::name());
    println!("  All product names:");
    for name in &names {
        println!("    • {}", name);
    }
    println!();

    // group_by(path)
    println!("✓ group_by(path) - Group by field");
    let by_category = Query::new(&products).group_by(Product::category());
    println!("  Products grouped by category:");
    for (category, items) in &by_category {
        println!("    {} ({} items):", category, items.len());
        for item in items {
            println!("      - {}", item.name);
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 4: AGGREGATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. AGGREGATIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let electronics_query = Query::new(&products)
        .where_(Product::category(), |cat| cat == "Electronics");

    // sum(path)
    println!("✓ sum(path) - Sum numeric field");
    let total_price = electronics_query.sum(Product::price());
    println!("  Total electronics value: ${:.2}\n", total_price);

    // avg(path)
    println!("✓ avg(path) - Average of f64 field");
    let avg_price = electronics_query.avg(Product::price()).unwrap_or(0.0);
    println!("  Average electronics price: ${:.2}\n", avg_price);

    // min(path) / max(path)
    println!("✓ min(path) / max(path) - Min/max of Ord field");
    let min_stock = electronics_query.min(Product::stock());
    let max_stock = electronics_query.max(Product::stock());
    println!("  Min stock: {:?}", min_stock);
    println!("  Max stock: {:?}\n", max_stock);

    // min_float(path) / max_float(path)
    println!("✓ min_float(path) / max_float(path) - Min/max of f64 field");
    let min_price = electronics_query.min_float(Product::price());
    let max_price = electronics_query.max_float(Product::price());
    println!("  Min price: ${:.2}", min_price.unwrap_or(0.0));
    println!("  Max price: ${:.2}\n", max_price.unwrap_or(0.0));

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 5: DATETIME OPERATIONS (SystemTime)
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. DATETIME OPERATIONS - SystemTime (always available)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let now = SystemTime::now();
    let two_weeks_ago = now - StdDuration::from_secs(86400 * 14);
    let one_week_ago = now - StdDuration::from_secs(86400 * 7);

    // where_after_systemtime
    println!("✓ where_after_systemtime(path, time) - Filter after SystemTime");
    let recent = Query::new(&products)
        .where_after_systemtime(Product::created_at(), two_weeks_ago);
    println!("  Products created in last 2 weeks: {}\n", recent.count());

    // where_before_systemtime
    println!("✓ where_before_systemtime(path, time) - Filter before SystemTime");
    let old = Query::new(&products)
        .where_before_systemtime(Product::created_at(), two_weeks_ago);
    println!("  Products created before 2 weeks ago: {}\n", old.count());

    // where_between_systemtime
    println!("✓ where_between_systemtime(path, start, end) - Filter within range");
    let in_range = Query::new(&products)
        .where_between_systemtime(Product::created_at(), two_weeks_ago, one_week_ago);
    println!("  Products created 1-2 weeks ago: {}\n", in_range.count());

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 6: DATETIME OPERATIONS (Chrono)
    // ═══════════════════════════════════════════════════════════════════════
    #[cfg(feature = "datetime")]
    {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("6. DATETIME OPERATIONS - Chrono (with 'datetime' feature)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let events = create_events();
        let cutoff = Utc.with_ymd_and_hms(2024, 10, 2, 0, 0, 0).unwrap();
        let oct_start = Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
        let oct_end = Utc.with_ymd_and_hms(2024, 10, 31, 23, 59, 59).unwrap();

        // where_after
        println!("✓ where_after(path, time) - Filter after datetime");
        let after = Query::new(&events)
            .where_after(Event::scheduled_at(), cutoff);
        println!("  Events after Oct 2: {}\n", after.count());

        // where_before
        println!("✓ where_before(path, time) - Filter before datetime");
        let before = Query::new(&events)
            .where_before(Event::scheduled_at(), cutoff);
        println!("  Events before Oct 2: {}\n", before.count());

        // where_between
        println!("✓ where_between(path, start, end) - Filter within range");
        let between = Query::new(&events)
            .where_between(Event::scheduled_at(), oct_start, oct_end);
        println!("  Events in October: {}\n", between.count());

        // where_today
        println!("✓ where_today(path, now) - Filter for today");
        let today_ref = Utc.with_ymd_and_hms(2024, 10, 1, 15, 0, 0).unwrap();
        let today = Query::new(&events)
            .where_today(Event::scheduled_at(), today_ref);
        println!("  Events on Oct 1: {}\n", today.count());

        // where_year
        println!("✓ where_year(path, year) - Filter by year");
        let year_2024 = Query::new(&events)
            .where_year(Event::scheduled_at(), 2024);
        println!("  Events in 2024: {}\n", year_2024.count());

        // where_month
        println!("✓ where_month(path, month) - Filter by month (1-12)");
        let october = Query::new(&events)
            .where_month(Event::scheduled_at(), 10);
        println!("  Events in October: {}\n", october.count());

        // where_day
        println!("✓ where_day(path, day) - Filter by day (1-31)");
        let first_day = Query::new(&events)
            .where_day(Event::scheduled_at(), 1);
        println!("  Events on the 1st: {}\n", first_day.count());

        // where_weekend
        println!("✓ where_weekend(path) - Filter for weekends");
        let weekend = Query::new(&events)
            .where_weekend(Event::scheduled_at());
        println!("  Weekend events:");
        for e in weekend.all() {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%A, %Y-%m-%d"));
        }
        println!();

        // where_weekday
        println!("✓ where_weekday(path) - Filter for weekdays");
        let weekday = Query::new(&events)
            .where_weekday(Event::scheduled_at());
        println!("  Weekday events: {}\n", weekday.count());

        // where_business_hours
        println!("✓ where_business_hours(path) - Filter for business hours (9 AM - 5 PM)");
        let business = Query::new(&events)
            .where_business_hours(Event::scheduled_at());
        println!("  Events during business hours:");
        for e in business.all() {
            println!("    • {} - {}", e.title, e.scheduled_at.format("%H:%M"));
        }
        println!();
    }

    #[cfg(not(feature = "datetime"))]
    {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("6. DATETIME OPERATIONS - Chrono");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("⚠️  Chrono datetime features require the 'datetime' feature.");
        println!("Run with: cargo run --example complete_api_showcase --features datetime\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 7: JOIN QUERY OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("7. JOIN QUERY OPERATIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // new(left, right)
    println!("✓ new(left, right) - Create a new join query");
    let _join_query = JoinQuery::new(&orders, &products);
    println!("  Created join query: {} orders × {} products\n", orders.len(), products.len());

    // inner_join
    println!("✓ inner_join(left_key, right_key, mapper) - Inner join");
    let inner_results: Vec<OrderDetail> = JoinQuery::new(&orders, &products)
        .inner_join(
            Order::product_id(),
            Product::id(),
            |order, product| OrderDetail {
                order_id: order.id,
                product_name: product.name.clone(),
                quantity: order.quantity,
                total: order.total,
            },
        );
    println!("  Inner join results ({} matches):", inner_results.len());
    for detail in &inner_results {
        println!("    • Order #{}: {} × {} = ${:.2}",
            detail.order_id, detail.product_name, detail.quantity, detail.total);
    }
    println!();

    // left_join
    println!("✓ left_join(left_key, right_key, mapper) - Left join");
    let left_results: Vec<String> = JoinQuery::new(&orders, &products)
        .left_join(
            Order::product_id(),
            Product::id(),
            |order, opt_product| {
                match opt_product {
                    Some(p) => format!("Order #{} - {} (${:.2})", order.id, p.name, order.total),
                    None => format!("Order #{} - Unknown Product (${:.2})", order.id, order.total),
                }
            },
        );
    println!("  Left join results ({} rows):", left_results.len());
    for result in &left_results {
        println!("    • {}", result);
    }
    println!();

    // right_join
    println!("✓ right_join(left_key, right_key, mapper) - Right join");
    let right_results: Vec<String> = JoinQuery::new(&orders, &products)
        .right_join(
            Order::product_id(),
            Product::id(),
            |opt_order, product| {
                match opt_order {
                    Some(o) => format!("{} - Ordered {} times", product.name, o.quantity),
                    None => format!("{} - Not ordered", product.name),
                }
            },
        );
    println!("  Right join results ({} rows):", right_results.len());
    for result in &right_results {
        println!("    • {}", result);
    }
    println!();

    // inner_join_where
    println!("✓ inner_join_where(left_key, right_key, predicate, mapper) - Filtered join");
    let filtered_joins: Vec<OrderDetail> = JoinQuery::new(&orders, &products)
        .inner_join_where(
            Order::product_id(),
            Product::id(),
            |order, product| order.quantity > 1 && product.price > 100.0,
            |order, product| OrderDetail {
                order_id: order.id,
                product_name: product.name.clone(),
                quantity: order.quantity,
                total: order.total,
            },
        );
    println!("  Filtered join (qty > 1 AND price > $100): {} matches", filtered_joins.len());
    for detail in &filtered_joins {
        println!("    • Order #{}: {} × {}", detail.order_id, detail.product_name, detail.quantity);
    }
    println!();

    // cross_join
    println!("✓ cross_join(mapper) - Cartesian product");
    let cross_results: Vec<String> = JoinQuery::new(&orders, &products)
        .cross_join(|order, product| {
            format!("Order #{} could include {}", order.id, product.name)
        });
    println!("  Cross join results: {} combinations", cross_results.len());
    println!("  (Showing first 5):");
    for result in cross_results.iter().take(5) {
        println!("    • {}", result);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 8: COMPLEX EXAMPLES
    // ═══════════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("8. COMPLEX QUERY EXAMPLES");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Complex 1: Multi-condition filtering with aggregation
    println!("Example 1: High-rated electronics under $500");
    let high_value = Query::new(&products)
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&price| price < 500.0)
        .where_(Product::rating(), |&rating| rating >= 4.5)
        .order_by_float_desc(Product::rating());
    
    println!("  Found {} products:", high_value.len());
    for p in &high_value {
        println!("    • {} - ${:.2} (Rating: {:.1})", p.name, p.price, p.rating);
    }
    println!();

    // Complex 2: Category statistics
    println!("Example 2: Category statistics with grouping");
    let grouped = Query::new(&products).group_by(Product::category());
    for (category, items) in &grouped {
        let cat_query = Query::new(items);
        println!("  {} Statistics:", category);
        println!("    • Count: {}", items.len());
        println!("    • Total Value: ${:.2}", cat_query.sum(Product::price()));
        println!("    • Avg Price: ${:.2}", cat_query.avg(Product::price()).unwrap_or(0.0));
        println!("    • Total Stock: {}", cat_query.sum(Product::stock()));
    }
    println!();

    // Complex 3: Pagination with ordering
    println!("Example 3: Paginated results (page 1, 2 items per page)");
    let page1 = Query::new(&products)
        .order_by_float_desc(Product::price());
    let page1_items: Vec<_> = page1.iter().take(2).collect();
    
    println!("  Page 1:");
    for p in page1_items {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    ✓ SHOWCASE COMPLETE                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 All Operations Demonstrated:");
    println!("  ✓ Basic Operations (8): new, where_, all, first, count, limit, skip, exists");
    println!("  ✓ Ordering (4): order_by, order_by_desc, order_by_float, order_by_float_desc");
    println!("  ✓ Projection & Grouping (2): select, group_by");
    println!("  ✓ Aggregations (6): sum, avg, min, max, min_float, max_float");
    println!("  ✓ DateTime SystemTime (3): where_after, where_before, where_between");
    
    #[cfg(feature = "datetime")]
    println!("  ✓ DateTime Chrono (10): where_after, where_before, where_between, where_today,");
    println!("                          where_year, where_month, where_day, where_weekend,");
    println!("                          where_weekday, where_business_hours");
    
    #[cfg(not(feature = "datetime"))]
    println!("  ⚠️  DateTime Chrono (10): Enable with --features datetime");
    
    println!("  ✓ JoinQuery (5): new, inner_join, left_join, right_join,");
    println!("                   inner_join_where, cross_join");
    
    println!("\n💡 Next Steps:");
    println!("  • Check examples/lock_lazy_complete.rs for LockLazyQuery operations");
    println!("  • See examples/advanced_query_builder.rs for more complex queries");
    println!("  • Read the documentation at docs.rs for detailed API reference");
}

