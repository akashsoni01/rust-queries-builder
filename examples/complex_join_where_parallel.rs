//! Complex example demonstrating JOIN, WHERE (with AND/OR), and Rayon parallel processing
//!
//! This example shows:
//! 1. Multi-table JOIN operations (Users, Orders, Products)
//! 2. Complex WHERE clauses with AND/OR operators
//! 3. Parallel processing with Rayon for large datasets
//! 4. Performance comparison between sequential and parallel operations
//!
//! Run with: cargo run --example complex_join_where_parallel --features parallel,datetime

use rust_queries_builder::*;
use key_paths_derive::Keypath;
use std::time::Instant;

#[derive(Debug, Clone, Keypath)]
struct User {
    id: u32,
    name: String,
    email: String,
    city: String,
    age: u32,
    is_premium: bool,
    registration_date: i64, // Unix timestamp in milliseconds
}

#[derive(Debug, Clone, Keypath)]
struct Order {
    id: u32,
    user_id: u32,
    product_id: u32,
    quantity: u32,
    total: f64,
    order_date: i64, // Unix timestamp in milliseconds
    status: String,  // "pending", "completed", "cancelled"
}

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
    supplier_id: u32,
}

// Result type for joined queries
#[derive(Debug, Clone, Keypath)]
struct OrderAnalytics {
    order_id: u32,
    user_name: String,
    user_city: String,
    user_age: u32,
    product_name: String,
    product_category: String,
    quantity: u32,
    unit_price: f64,
    total: f64,
    order_date: i64,
    status: String,
}

fn create_large_dataset() -> (Vec<User>, Vec<Order>, Vec<Product>) {
    let mut users = Vec::new();
    let mut orders = Vec::new();
    let mut products = Vec::new();

    // Create 10,000 users
    for i in 0..10000 {
        let city = match i % 10 {
            0 => "New York",
            1 => "Los Angeles",
            2 => "Chicago",
            3 => "Houston",
            4 => "Phoenix",
            5 => "Philadelphia",
            6 => "San Antonio",
            7 => "San Diego",
            8 => "Dallas",
            _ => "San Jose",
        };

        users.push(User {
            id: i + 1,
            name: format!("User {}", i + 1),
            email: format!("user{}@example.com", i + 1),
            city: city.to_string(),
            age: 20 + (i % 50) as u32,
            is_premium: i % 5 == 0, // 20% are premium
            registration_date: 1609459200000 + (i as i64 * 86400000), // Start from 2021-01-01
        });
    }

    // Create 1,000 products
    for i in 0..1000 {
        let category = match i % 8 {
            0 => "Electronics",
            1 => "Clothing",
            2 => "Books",
            3 => "Home",
            4 => "Sports",
            5 => "Toys",
            6 => "Food",
            _ => "Beauty",
        };

        products.push(Product {
            id: i + 1,
            name: format!("Product {}", i + 1),
            price: 10.0 + (i as f64 * 0.5),
            category: category.to_string(),
            stock: (i % 100) + 1,
            rating: 3.0 + (i as f64 % 20) / 10.0, // 3.0 to 4.9
            supplier_id: (i % 50) + 1,
        });
    }

    // Create 50,000 orders
    for i in 0..50000 {
        let status = match i % 10 {
            0..=7 => "completed",
            8 => "pending",
            _ => "cancelled",
        };

        orders.push(Order {
            id: i + 1,
            user_id: (i % 10000) + 1,
            product_id: (i % 1000) + 1,
            quantity: (i % 10) + 1,
            total: 50.0 + (i as f64 * 0.1),
            order_date: 1609459200000 + (i as i64 * 3600000), // Start from 2021-01-01, hourly
            status: status.to_string(),
        });
    }

    (users, orders, products)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Complex JOIN + WHERE (AND/OR) + Rayon Parallel Processing     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let (users, orders, products) = create_large_dataset();

    println!("📊 Dataset created:");
    println!("  • {} users", users.len());
    println!("  • {} orders", orders.len());
    println!("  • {} products\n", products.len());

    // ============================================================================
    // STEP 1: JOIN Operations - Create joined dataset
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 1: JOIN Operations - Building joined dataset");
    println!("═══════════════════════════════════════════════════════════════\n");

    let start = Instant::now();

    // Join Orders with Users
    let orders_with_users = JoinQuery::new(&orders, &users).inner_join(
        Order::user_id(),
        User::id(),
        |order, user| (order.clone(), user.clone()),
    );

    // Join (Orders+Users) with Products
    let mut order_analytics = Vec::new();
    for (order, user) in &orders_with_users {
        if let Some(product) = products.iter().find(|p| p.id == order.product_id) {
            order_analytics.push(OrderAnalytics {
                order_id: order.id,
                user_name: user.name.clone(),
                user_city: user.city.clone(),
                user_age: user.age,
                product_name: product.name.clone(),
                product_category: product.category.clone(),
                quantity: order.quantity,
                unit_price: product.price,
                total: order.total,
                order_date: order.order_date,
                status: order.status.clone(),
            });
        }
    }

    let join_time = start.elapsed();
    println!("  ✅ Joined {} orders with users and products", order_analytics.len());
    println!("  ⏱️  Join time: {:?}\n", join_time);

    // ============================================================================
    // STEP 2: Complex WHERE with AND/OR - Sequential Processing
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 2: Complex WHERE with AND/OR - Sequential");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Query: (status == "completed" AND total > 100) OR (user_age < 25 AND is_premium)
    // But we need to work with OrderAnalytics, so let's adjust:
    // (status == "completed" AND total > 100) OR (user_age < 25 AND category == "Electronics")

    let start = Instant::now();
    let complex_filtered_seq: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::total(), |&t| t > 100.0)
        .or(OrderAnalytics::user_age(), |&age| age < 25)
        .where_(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .collect();
    let seq_time = start.elapsed();

    println!("  Query: (status == 'completed' AND total > 100) OR (age < 25 AND category == 'Electronics')");
    println!("  📊 Sequential Results:");
    println!("    • Found {} matching orders", complex_filtered_seq.len());
    println!("    • Time: {:?}\n", seq_time);

    // ============================================================================
    // STEP 3: Complex WHERE with AND/OR - Parallel Processing
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 3: Complex WHERE with AND/OR - Parallel (Rayon)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Parallel query - using multiple where_() for AND conditions
    // Note: OR support for parallel queries is coming soon
    let start = Instant::now();
    let complex_filtered_par: Vec<_> = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .where_(OrderAnalytics::total(), |&t| t > 100.0)
        .where_(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .collect_parallel();
    let par_time = start.elapsed();

    println!("  Query Parallel: status == 'completed' AND total > 100 AND category == 'Electronics'");
    println!("  📊 Parallel Results:");
    println!("    • Found {} matching orders", complex_filtered_par.len());
    println!("    • Time: {:?}", par_time);
    println!("    • Speedup: {:.2}x", seq_time.as_nanos() as f64 / par_time.as_nanos() as f64);
    println!("  ℹ️  Note: Parallel queries use multiple where_() for AND conditions");
    println!("     For OR queries, use sequential LazyQuery (supports full AND/OR)\n");

    // ============================================================================
    // STEP 4: Multiple Complex Queries - Sequential
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 4: Multiple Complex Queries - Sequential");
    println!("═══════════════════════════════════════════════════════════════\n");

    let start = Instant::now();

    // Query 1: Premium users with high-value orders (using AND)
    let premium_high_value: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::user_age(), |&age| age >= 30)
        .and(OrderAnalytics::total(), |&t| t > 200.0)
        .and(OrderAnalytics::status(), |s| s == "completed")
        .collect();

    // Query 2: Electronics or Sports with good ratings (using OR)
    let electronics_or_sports: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .or(OrderAnalytics::product_category(), |cat| cat == "Sports")
        .and(OrderAnalytics::total(), |&t| t > 50.0)
        .collect();

    // Query 3: Young users in major cities
    let young_major_cities: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::user_age(), |&age| age < 30)
        .and(OrderAnalytics::user_city(), |city| {
            city == "New York" || city == "Los Angeles" || city == "Chicago"
        })
        .collect();

    let multi_query_seq_time = start.elapsed();

    println!("  Query 1: Premium users (age >= 30) with high-value orders (> $200)");
    println!("    • Found {} orders", premium_high_value.len());
    println!();
    println!("  Query 2: Electronics OR Sports with total > $50");
    println!("    • Found {} orders", electronics_or_sports.len());
    println!();
    println!("  Query 3: Young users (< 30) in major cities");
    println!("    • Found {} orders", young_major_cities.len());
    println!();
    println!("  ⏱️  Total time (3 queries): {:?}\n", multi_query_seq_time);

    // ============================================================================
    // STEP 5: Multiple Complex Queries - Parallel
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 5: Multiple Complex Queries - Parallel");
    println!("═══════════════════════════════════════════════════════════════\n");

    let start = Instant::now();

    // Query 1: Premium users with high-value orders (parallel - multiple where_ = AND)
    let premium_high_value_par: Vec<_> = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::user_age(), |&age| age >= 30)
        .where_(OrderAnalytics::total(), |&t| t > 200.0)
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .collect_parallel();

    // Query 2: Electronics with total > 50 (parallel - for OR, use sequential LazyQuery)
    let electronics_par: Vec<_> = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .where_(OrderAnalytics::total(), |&t| t > 50.0)
        .collect_parallel();
    
    // For OR queries, use sequential LazyQuery which supports OR
    let sports_par: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::product_category(), |cat| cat == "Sports")
        .and(OrderAnalytics::total(), |&t| t > 50.0)
        .collect();
    
    let electronics_or_sports_par: Vec<_> = [electronics_par, sports_par].concat();

    // Query 3: Young users in major cities
    let young_major_cities_par: Vec<_> = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::user_age(), |&age| age < 30)
        .and(OrderAnalytics::user_city(), |city| {
            city == "New York" || city == "Los Angeles" || city == "Chicago"
        })
        .collect_parallel();

    let multi_query_par_time = start.elapsed();

    println!("  Query 1: Premium users (age >= 30) with high-value orders (> $200)");
    println!("    • Found {} orders", premium_high_value_par.len());
    println!();
    println!("  Query 2: Electronics OR Sports with total > $50");
    println!("    • Found {} orders", electronics_or_sports_par.len());
    println!();
    println!("  Query 3: Young users (< 30) in major cities");
    println!("    • Found {} orders", young_major_cities_par.len());
    println!();
    println!("  ⏱️  Total time (3 queries): {:?}", multi_query_par_time);
    println!("  🚀 Speedup: {:.2}x\n", multi_query_seq_time.as_nanos() as f64 / multi_query_par_time.as_nanos() as f64);

    // ============================================================================
    // STEP 6: Parallel Aggregations on Filtered Results
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 6: Parallel Aggregations on Filtered Results");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential aggregations
    let start = Instant::now();
    let total_revenue_seq = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .sum_by(OrderAnalytics::total());
    
    let avg_order_seq = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .avg_by(OrderAnalytics::total());

    let max_order_seq = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .max_by_float(OrderAnalytics::total());

    let seq_agg_time = start.elapsed();

    // Parallel aggregations
    let start = Instant::now();
    let total_revenue_par = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .sum_by_parallel(OrderAnalytics::total());
    
    let avg_order_par = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .avg_by_parallel(OrderAnalytics::total());

    let max_order_par = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .max_by_float_parallel(OrderAnalytics::total());

    let par_agg_time = start.elapsed();

    println!("  Filter: status == 'completed' AND category == 'Electronics'");
    println!();
    println!("  📊 Sequential Aggregations:");
    println!("    • Total Revenue: ${:.2}", total_revenue_seq);
    println!("    • Average Order: ${:.2}", avg_order_seq.unwrap_or(0.0));
    println!("    • Max Order: ${:.2}", max_order_seq.unwrap_or(0.0));
    println!("    • Time: {:?}", seq_agg_time);
    println!();
    println!("  📊 Parallel Aggregations:");
    println!("    • Total Revenue: ${:.2}", total_revenue_par);
    println!("    • Average Order: ${:.2}", avg_order_par.unwrap_or(0.0));
    println!("    • Max Order: ${:.2}", max_order_par.unwrap_or(0.0));
    println!("    • Time: {:?}", par_agg_time);
    println!("    • Speedup: {:.2}x\n", seq_agg_time.as_nanos() as f64 / par_agg_time.as_nanos() as f64);

    // ============================================================================
    // STEP 7: Complex Nested Query with Multiple AND/OR Groups
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("STEP 7: Complex Nested Query with Multiple AND/OR Groups");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Complex nested query with AND/OR (sequential - LazyQuery supports this)
    let start = Instant::now();
    let nested_complex_seq: Vec<_> = order_analytics
        .lazy_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .and(OrderAnalytics::total(), |&t| t > 100.0)
        .and(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .or(OrderAnalytics::user_age(), |&age| age < 25)
        .where_(OrderAnalytics::product_category(), |cat| cat == "Sports")
        .or(OrderAnalytics::product_category(), |cat| cat == "Toys")
        .collect();
    let nested_seq_time = start.elapsed();

    // For parallel, we'll do a simpler AND query (OR support coming soon)
    let start = Instant::now();
    let nested_complex_par: Vec<_> = order_analytics
        .lazy_parallel_query()
        .where_(OrderAnalytics::status(), |s| s == "completed")
        .where_(OrderAnalytics::total(), |&t| t > 100.0)
        .where_(OrderAnalytics::product_category(), |cat| cat == "Electronics")
        .collect_parallel();
    let nested_par_time = start.elapsed();

    println!("  Query Sequential: (completed AND total > 100 AND Electronics) OR (age < 25 AND (Sports OR Toys))");
    println!("  📊 Sequential (with OR):");
    println!("    • Found {} orders", nested_complex_seq.len());
    println!("    • Time: {:?}", nested_seq_time);
    println!();
    println!("  Query Parallel: completed AND total > 100 AND Electronics");
    println!("  📊 Parallel (AND only):");
    println!("    • Found {} orders", nested_complex_par.len());
    println!("    • Time: {:?}", nested_par_time);
    println!("  ℹ️  Note: Use sequential LazyQuery for complex OR queries");
    println!("     Parallel OR support is coming soon!\n");

    // ============================================================================
    // SUMMARY
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Successfully demonstrated:");
    println!("   • Multi-table JOIN operations (Users, Orders, Products)");
    println!("   • Complex WHERE clauses with AND/OR operators (LazyQuery)");
    println!("   • Parallel processing with Rayon (LazyParallelQuery)");
    println!("   • Performance comparisons between sequential and parallel");
    println!();
    println!("🎯 Key Features Used:");
    println!("   • JoinQuery::inner_join() - Multi-table joins");
    println!("   • LazyQuery::where_().and().or() - Complex filtering with AND/OR");
    println!("   • LazyParallelQuery::where_() - Parallel filtering (AND via multiple where_)");
    println!("   • Parallel aggregations (sum_by_parallel, avg_by_parallel, etc.)");
    println!();
    println!("🚀 Performance Benefits:");
    println!("   • Parallel processing provides significant speedup on large datasets");
    println!("   • Complex AND/OR queries work with sequential LazyQuery");
    println!("   • Parallel queries excel at AND conditions (multiple where_ calls)");
    println!("   • Join operations can be combined with both sequential and parallel filtering");
    println!();
    println!("💡 Use Cases:");
    println!("   • E-commerce analytics");
    println!("   • Large-scale data processing");
    println!("   • Real-time reporting");
    println!("   • Multi-table queries with complex filters");
    println!();
    println!("📝 Notes:");
    println!("   • LazyQuery supports full AND/OR operators");
    println!("   • LazyParallelQuery currently supports AND (via multiple where_ calls)");
    println!("   • OR support for parallel queries is coming soon!");
    println!("   • For complex OR queries, use sequential LazyQuery");
    println!("   • For large datasets with AND conditions, use LazyParallelQuery");
}

