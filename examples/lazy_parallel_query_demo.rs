//! Comprehensive example demonstrating lazy parallel query operations
//!
//! This example shows how to use LazyParallelQuery with all available features:
//! - Parallel filtering operations
//! - Parallel aggregations
//! - Parallel datetime operations
//! - Parallel timestamp operations
//! - Performance comparison with sequential operations
//!
//! Run with: cargo run --example lazy_parallel_query_demo --features parallel

use rust_queries_builder::*;
use key_paths_derive::Keypaths;
use std::time::Instant;

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    created_at: i64,        // Unix timestamp in milliseconds
}

#[derive(Debug, Clone, Keypaths)]
struct Event {
    id: u32,
    name: String,
    timestamp: i64,         // Unix timestamp in milliseconds
    value: f64,
    category: String,
}

impl Product {
    fn new(id: u32, name: &str, price: f64, category: &str, stock: u32, created_at: i64) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
            category: category.to_string(),
            stock,
            created_at,
        }
    }
}

impl Event {
    fn new(id: u32, name: &str, timestamp: i64, value: f64, category: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            timestamp,
            value,
            category: category.to_string(),
        }
    }
}

fn create_large_dataset() -> (Vec<Product>, Vec<Event>) {
    let mut products = Vec::new();
    let mut events = Vec::new();
    
    // Create 50,000 products for better parallel performance demonstration
    for i in 0..50000 {
        let category = match i % 5 {
            0 => "Electronics",
            1 => "Clothing",
            2 => "Books",
            3 => "Home",
            _ => "Sports",
        };
        
        let price = 10.0 + (i as f64 * 0.1);
        let stock = (i % 100) + 1;
        let created_at = 1609459200000 + (i as i64 * 86400000); // Start from 2021-01-01
        
        products.push(Product::new(
            i,
            &format!("Product {}", i),
            price,
            category,
            stock,
            created_at,
        ));
    }
    
    // Create 25,000 events
    for i in 0..25000 {
        let category = match i % 4 {
            0 => "Purchase",
            1 => "View",
            2 => "Click",
            _ => "Share",
        };
        
        let timestamp = 1609459200000 + (i as i64 * 3600000); // Start from 2021-01-01, hourly
        let value = 1.0 + (i as f64 * 0.01);
        
        events.push(Event::new(
            i,
            &format!("Event {}", i),
            timestamp,
            value,
            category,
        ));
    }
    
    (products, events)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Lazy Parallel Query Demo - All Features                        ║");
    println!("║  Parallel Processing with Lazy Evaluation                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let (products, events) = create_large_dataset();
    
    println!("📊 Dataset created:");
    println!("  • {} products", products.len());
    println!("  • {} events", events.len());

    // ============================================================================
    // 1. BASIC PARALLEL FILTERING OPERATIONS
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("1. BASIC PARALLEL FILTERING OPERATIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential vs Parallel filtering
    let start = Instant::now();
    let electronics_seq: Vec<_> = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .collect();
    let seq_time = start.elapsed();
    
    let start = Instant::now();
    let electronics_par: Vec<_> = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .collect_parallel();
    let par_time = start.elapsed();

    println!("  🔍 Filtering Performance Results:");
    println!("    Electronics with price > $100:");
    println!("      Sequential: {:?} ({} items)", seq_time, electronics_seq.len());
    println!("      Parallel:   {:?} ({} items)", par_time, electronics_par.len());
    println!("      Speedup:    {:.2}x", seq_time.as_nanos() as f64 / par_time.as_nanos() as f64);

    // ============================================================================
    // 2. PARALLEL AGGREGATION OPERATIONS
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("2. PARALLEL AGGREGATION OPERATIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sum aggregation
    let start = Instant::now();
    let total_price_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .sum_by(Product::price_r());
    let sum_seq_time = start.elapsed();
    
    let start = Instant::now();
    let total_price_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .sum_by_parallel(Product::price_r());
    let sum_par_time = start.elapsed();

    println!("  📊 Sum Aggregation Results:");
    println!("    Total price (Electronics):");
    println!("      Sequential: {:?} (sum: {:.2})", sum_seq_time, total_price_seq);
    println!("      Parallel:   {:?} (sum: {:.2})", sum_par_time, total_price_par);
    println!("      Speedup:    {:.2}x", sum_seq_time.as_nanos() as f64 / sum_par_time.as_nanos() as f64);

    // Average aggregation
    let start = Instant::now();
    let avg_price_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .avg_by(Product::price_r());
    let avg_seq_time = start.elapsed();
    
    let start = Instant::now();
    let avg_price_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .avg_by_parallel(Product::price_r());
    let avg_par_time = start.elapsed();

    println!("  📊 Average Aggregation Results:");
    println!("    Average price (Electronics):");
    println!("      Sequential: {:?} (avg: {:?})", avg_seq_time, avg_price_seq);
    println!("      Parallel:   {:?} (avg: {:?})", avg_par_time, avg_price_par);
    println!("      Speedup:    {:.2}x", avg_seq_time.as_nanos() as f64 / avg_par_time.as_nanos() as f64);

    // Min/Max aggregations
    let start = Instant::now();
    let min_price_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .min_by_float(Product::price_r());
    let min_seq_time = start.elapsed();
    
    let start = Instant::now();
    let min_price_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .min_by_float_parallel(Product::price_r());
    let min_par_time = start.elapsed();

    println!("  📊 Min/Max Aggregation Results:");
    println!("    Min price (Electronics):");
    println!("      Sequential: {:?} (min: {:?})", min_seq_time, min_price_seq);
    println!("      Parallel:   {:?} (min: {:?})", min_par_time, min_price_par);
    println!("      Speedup:    {:.2}x", min_seq_time.as_nanos() as f64 / min_par_time.as_nanos() as f64);

    // ============================================================================
    // 3. PARALLEL TERMINAL OPERATIONS
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("3. PARALLEL TERMINAL OPERATIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Count operation
    let start = Instant::now();
    let count_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .count();
    let count_seq_time = start.elapsed();
    
    let start = Instant::now();
    let count_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .count_parallel();
    let count_par_time = start.elapsed();

    println!("  🔢 Count Operation Results:");
    println!("    Electronics count:");
    println!("      Sequential: {:?} (count: {})", count_seq_time, count_seq);
    println!("      Parallel:   {:?} (count: {})", count_par_time, count_par);
    println!("      Speedup:    {:.2}x", count_seq_time.as_nanos() as f64 / count_par_time.as_nanos() as f64);

    // Any operation
    let start = Instant::now();
    let any_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .any();
    let any_seq_time = start.elapsed();
    
    let start = Instant::now();
    let any_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .any_parallel();
    let any_par_time = start.elapsed();

    println!("  ✅ Any Operation Results:");
    println!("    Any Electronics exist:");
    println!("      Sequential: {:?} (any: {})", any_seq_time, any_seq);
    println!("      Parallel:   {:?} (any: {})", any_par_time, any_par);
    println!("      Speedup:    {:.2}x", any_seq_time.as_nanos() as f64 / any_par_time.as_nanos() as f64);

    // ============================================================================
    // 4. PARALLEL i64 TIMESTAMP OPERATIONS
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("4. PARALLEL i64 TIMESTAMP OPERATIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Timestamp filtering
    let cutoff_time = 1640995200000; // 2022-01-01
    
    let start = Instant::now();
    let recent_products_seq: Vec<_> = products
        .lazy_query()
        .where_after_timestamp(Product::created_at_r(), cutoff_time)
        .collect();
    let timestamp_filter_seq_time = start.elapsed();
    
    let start = Instant::now();
    let recent_products_par: Vec<_> = products
        .lazy_parallel_query()
        .where_after_timestamp_parallel(Product::created_at_r(), cutoff_time)
        .collect_parallel();
    let timestamp_filter_par_time = start.elapsed();

    println!("  ⏰ Timestamp Filtering Results:");
    println!("    Products created after 2022-01-01:");
    println!("      Sequential: {:?} ({} items)", timestamp_filter_seq_time, recent_products_seq.len());
    println!("      Parallel:   {:?} ({} items)", timestamp_filter_par_time, recent_products_par.len());
    println!("      Speedup:    {:.2}x", timestamp_filter_seq_time.as_nanos() as f64 / timestamp_filter_par_time.as_nanos() as f64);

    // Timestamp aggregations
    let start = Instant::now();
    let min_timestamp_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .min_timestamp(Product::created_at_r());
    let min_timestamp_seq_time = start.elapsed();
    
    let start = Instant::now();
    let min_timestamp_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .min_timestamp_parallel(Product::created_at_r());
    let min_timestamp_par_time = start.elapsed();

    println!("  ⏰ Timestamp Aggregation Results:");
    println!("    Min timestamp (Electronics):");
    println!("      Sequential: {:?} (min: {:?})", min_timestamp_seq_time, min_timestamp_seq);
    println!("      Parallel:   {:?} (min: {:?})", min_timestamp_par_time, min_timestamp_par);
    println!("      Speedup:    {:.2}x", min_timestamp_seq_time.as_nanos() as f64 / min_timestamp_par_time.as_nanos() as f64);

    // ============================================================================
    // 5. PARALLEL PROJECTION AND MAPPING
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("5. PARALLEL PROJECTION AND MAPPING");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Select operation
    let start = Instant::now();
    let names_seq: Vec<_> = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .select_lazy(Product::name_r())
        .collect();
    let select_seq_time = start.elapsed();
    
    let start = Instant::now();
    let names_par: Vec<_> = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .select_parallel(Product::name_r());
    let select_par_time = start.elapsed();

    println!("  🎯 Select Operation Results:");
    println!("    Electronics names:");
    println!("      Sequential: {:?} ({} names)", select_seq_time, names_seq.len());
    println!("      Parallel:   {:?} ({} names)", select_par_time, names_par.len());
    println!("      Speedup:    {:.2}x", select_seq_time.as_nanos() as f64 / select_par_time.as_nanos() as f64);

    // Map operation
    let start = Instant::now();
    let prices_seq: Vec<_> = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .map_items(|p| p.price)
        .collect();
    let map_seq_time = start.elapsed();
    
    let start = Instant::now();
    let prices_par: Vec<_> = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .map_items_parallel(|p| p.price);
    let map_par_time = start.elapsed();

    println!("  🗺️  Map Operation Results:");
    println!("    Electronics prices:");
    println!("      Sequential: {:?} ({} prices)", map_seq_time, prices_seq.len());
    println!("      Parallel:   {:?} ({} prices)", map_par_time, prices_par.len());
    println!("      Speedup:    {:.2}x", map_seq_time.as_nanos() as f64 / map_par_time.as_nanos() as f64);

    // ============================================================================
    // 6. COMPLEX PARALLEL QUERIES
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("6. COMPLEX PARALLEL QUERIES");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Complex query with multiple filters and operations
    let start = Instant::now();
    let complex_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .where_(Product::stock_r(), |&s| s > 10)
        .where_after_timestamp(Product::created_at_r(), cutoff_time)
        .collect();
    let complex_seq_time = start.elapsed();
    
    let start = Instant::now();
    let complex_par = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .where_(Product::stock_r(), |&s| s > 10)
        .where_after_timestamp_parallel(Product::created_at_r(), cutoff_time)
        .collect_parallel();
    let complex_par_time = start.elapsed();

    println!("  🔧 Complex Query Results:");
    println!("    Electronics: price > $100, stock > 10, created after 2022-01-01:");
    println!("      Sequential: {:?} ({} items)", complex_seq_time, complex_seq.len());
    println!("      Parallel:   {:?} ({} items)", complex_par_time, complex_par.len());
    println!("      Speedup:    {:.2}x", complex_seq_time.as_nanos() as f64 / complex_par_time.as_nanos() as f64);

    // ============================================================================
    // 7. PARALLEL PAGINATION OPERATIONS
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("7. PARALLEL PAGINATION OPERATIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Take operation
    let start = Instant::now();
    let first_100_seq: Vec<_> = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .take_lazy(100)
        .collect();
    let take_seq_time = start.elapsed();
    
    let start = Instant::now();
    let first_100_par: Vec<_> = products
        .lazy_parallel_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .take_parallel(100);
    let take_par_time = start.elapsed();

    println!("  📄 Pagination Results:");
    println!("    First 100 Electronics:");
    println!("      Sequential: {:?} ({} items)", take_seq_time, first_100_seq.len());
    println!("      Parallel:   {:?} ({} items)", take_par_time, first_100_par.len());
    println!("      Speedup:    {:.2}x", take_seq_time.as_nanos() as f64 / take_par_time.as_nanos() as f64);

    // ============================================================================
    // 8. SUMMARY
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("8. SUMMARY - Lazy Parallel Query Benefits");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Successfully demonstrated LazyParallelQuery with all features:");
    println!("   • Parallel filtering operations (where_, where_after_timestamp, etc.)");
    println!("   • Parallel aggregation operations (sum_by_parallel, avg_by_parallel, etc.)");
    println!("   • Parallel terminal operations (collect_parallel, count_parallel, etc.)");
    println!("   • Parallel projection operations (select_parallel, map_items_parallel)");
    println!("   • Parallel timestamp operations (min_timestamp_parallel, etc.)");
    println!("   • Parallel pagination operations (take_parallel, skip_parallel)");

    println!("\n✅ Key Features of LazyParallelQuery:");
    println!("   • Deferred execution - no work until results needed");
    println!("   • Parallel processing - utilizes multiple CPU cores");
    println!("   • Thread-safe - all operations are Send + Sync");
    println!("   • Composable - build complex queries by composition");
    println!("   • Early termination - short-circuits when possible");
    println!("   • Zero-cost when parallel feature is disabled");

    println!("\n🚀 Performance Benefits:");
    println!("   • Significant speedup on large datasets (50,000+ items)");
    println!("   • Automatic CPU core utilization");
    println!("   • Better performance for CPU-intensive operations");
    println!("   • Maintains lazy evaluation benefits");

    println!("\n🎯 Perfect for:");
    println!("   • Large datasets requiring parallel processing");
    println!("   • CPU-intensive aggregations and filtering");
    println!("   • Multi-core systems");
    println!("   • Real-time analytics with large data volumes");
    println!("   • High-performance applications");

    println!("\n🎉 LazyParallelQuery is now available with all LazyQuery features!");
    println!("   Enable with: cargo run --features parallel");
    println!("   Use with: collection.lazy_parallel_query()");
}
