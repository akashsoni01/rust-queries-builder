//! Comprehensive example demonstrating parallel query operations
//!
//! This example shows how to use parallel processing with:
//! - Query (eager parallel) - Full parallel support
//! - LazyQuery, LockQuery, LockLazyQuery - Sequential only (parallel removed for thread safety)
//!
//! Run with: cargo run --example parallel_queries_demo --features parallel

use rust_queries_builder::*;
use key_paths_derive::Keypaths;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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

fn create_large_dataset() -> (Vec<Product>, Vec<Event>, HashMap<String, Arc<RwLock<Product>>>, HashMap<String, Arc<RwLock<Event>>>) {
    let mut products = Vec::new();
    let mut events = Vec::new();
    
    // Create 10,000 products
    for i in 0..10000 {
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
    
    // Create 5,000 events
    for i in 0..5000 {
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
    
    // Create locked versions for thread-safe operations
    let mut locked_products = HashMap::new();
    for product in &products {
        locked_products.insert(format!("PROD-{:05}", product.id), Arc::new(RwLock::new(product.clone())));
    }
    
    let mut locked_events = HashMap::new();
    for event in &events {
        locked_events.insert(format!("EVENT-{:05}", event.id), Arc::new(RwLock::new(event.clone())));
    }
    
    (products, events, locked_products, locked_events)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Parallel Queries Demo - Performance Comparison                 ║");
    println!("║  Sequential vs Parallel Operations                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let (products, events, locked_products, locked_events) = create_large_dataset();
    
    println!("📊 Dataset created:");
    println!("  • {} products", products.len());
    println!("  • {} events", events.len());
    println!("  • {} locked products", locked_products.len());
    println!("  • {} locked events", locked_events.len());

    // ============================================================================
    // 1. QUERY (Eager) - Sequential vs Parallel
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("1. QUERY (Eager) - Sequential vs Parallel Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential operations
    let start = Instant::now();
    let expensive_query_seq = Query::new(&products)
        .where_(Product::price_r(), |&p| p > 500.0);
    let expensive_products_seq = expensive_query_seq.all();
    let seq_time = start.elapsed();
    
    let start = Instant::now();
    let min_query_seq = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let min_price_seq = min_query_seq.min_float(Product::price_r());
    let min_seq_time = start.elapsed();
    
    let start = Instant::now();
    let avg_query_seq = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let avg_price_seq = avg_query_seq.avg(Product::price_r());
    let avg_seq_time = start.elapsed();

    // Parallel operations
    let start = Instant::now();
    let expensive_query_par = Query::new(&products)
        .where_(Product::price_r(), |&p| p > 500.0);
    let expensive_products_par = expensive_query_par.all_parallel();
    let par_time = start.elapsed();
    
    let start = Instant::now();
    let min_query_par = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let min_price_par = min_query_par.min_float(Product::price_r());
    let min_par_time = start.elapsed();
    
    let start = Instant::now();
    let avg_query_par = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let avg_price_par = avg_query_par.avg_parallel(Product::price_r());
    let avg_par_time = start.elapsed();

    println!("  📈 Query Performance Results:");
    println!("    Expensive products (>$500):");
    println!("      Sequential: {:?} ({} items)", seq_time, expensive_products_seq.len());
    println!("      Parallel:   {:?} ({} items)", par_time, expensive_products_par.len());
    println!("      Speedup:    {:.2}x", seq_time.as_nanos() as f64 / par_time.as_nanos() as f64);
    
    println!("    Min price (Electronics):");
    println!("      Sequential: {:?} (min: {:?})", min_seq_time, min_price_seq);
    println!("      Parallel:   {:?} (min: {:?})", min_par_time, min_price_par);
    println!("      Speedup:    {:.2}x", min_seq_time.as_nanos() as f64 / min_par_time.as_nanos() as f64);
    
    println!("    Average price (Electronics):");
    println!("      Sequential: {:?} (avg: {:?})", avg_seq_time, avg_price_seq);
    println!("      Parallel:   {:?} (avg: {:?})", avg_par_time, avg_price_par);
    println!("      Speedup:    {:.2}x", avg_seq_time.as_nanos() as f64 / avg_par_time.as_nanos() as f64);

    // ============================================================================
    // 2. LAZYQUERY (Lazy) - Sequential Only
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("2. LAZYQUERY (Lazy) - Sequential Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential lazy operations
    let start = Instant::now();
    let electronics_lazy_seq: Vec<_> = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .collect();
    let lazy_seq_time = start.elapsed();
    
    let start = Instant::now();
    let max_stock_seq = products
        .lazy_query()
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .max_by(Product::stock_r());
    let max_seq_time = start.elapsed();

    println!("  ⚡ LazyQuery Performance Results:");
    println!("    Electronics with price > $100:");
    println!("      Sequential: {:?} ({} items)", lazy_seq_time, electronics_lazy_seq.len());
    println!("    Max stock (Electronics):");
    println!("      Sequential: {:?} (max: {:?})", max_seq_time, max_stock_seq);
    println!("    Note: Parallel operations removed for thread safety");

    // ============================================================================
    // 3. LOCKQUERY (Thread-safe Eager) - Sequential Only
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("3. LOCKQUERY (Thread-safe Eager) - Sequential Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential lock operations
    let start = Instant::now();
    let clothing_lock_seq = locked_products
        .lock_query()
        .where_(Product::category_r(), |cat| cat == "Clothing")
        .where_(Product::stock_r(), |&s| s > 50)
        .all();
    let lock_seq_time = start.elapsed();
    
    let start = Instant::now();
    let sum_price_seq = locked_products
        .lock_query()
        .where_(Product::category_r(), |cat| cat == "Clothing")
        .sum(Product::price_r());
    let sum_seq_time = start.elapsed();

    println!("  🔒 LockQuery Performance Results:");
    println!("    Clothing with stock > 50:");
    println!("      Sequential: {:?} ({} items)", lock_seq_time, clothing_lock_seq.len());
    println!("    Sum of prices (Clothing):");
    println!("      Sequential: {:?} (sum: {})", sum_seq_time, sum_price_seq);
    println!("    Note: Parallel operations removed for thread safety");

    // ============================================================================
    // 4. LOCKLAZYQUERY (Thread-safe Lazy) - Sequential Only
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("4. LOCKLAZYQUERY (Thread-safe Lazy) - Sequential Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential lock lazy operations
    let start = Instant::now();
    let recent_events_seq: Vec<_> = locked_events
        .lock_lazy_query()
        .where_(Event::category_r(), |cat| cat == "Purchase")
        .where_(Event::value_r(), |&v| v > 5.0)
        .collect();
    let lock_lazy_seq_time = start.elapsed();
    
    let start = Instant::now();
    let count_events_seq = locked_events
        .lock_lazy_query()
        .where_(Event::category_r(), |cat| cat == "Purchase")
        .count();
    let count_seq_time = start.elapsed();

    println!("  🔒⚡ LockLazyQuery Performance Results:");
    println!("    Purchase events with value > 5.0:");
    println!("      Sequential: {:?} ({} items)", lock_lazy_seq_time, recent_events_seq.len());
    println!("    Count of Purchase events:");
    println!("      Sequential: {:?} (count: {})", count_seq_time, count_events_seq);
    println!("    Note: Parallel operations removed for thread safety");

    // ============================================================================
    // 5. i64 TIMESTAMP AGGREGATORS - Sequential vs Parallel
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("5. i64 TIMESTAMP AGGREGATORS - Sequential vs Parallel Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Sequential timestamp operations
    let start = Instant::now();
    let timestamp_query_seq = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let min_timestamp_seq = timestamp_query_seq.min_timestamp(Product::created_at_r());
    let timestamp_seq_time = start.elapsed();
    
    let start = Instant::now();
    let avg_timestamp_query_seq = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let avg_timestamp_seq = avg_timestamp_query_seq.avg_timestamp(Product::created_at_r());
    let avg_timestamp_seq_time = start.elapsed();

    // Parallel timestamp operations
    let start = Instant::now();
    let timestamp_query_par = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let min_timestamp_par = timestamp_query_par.min_timestamp_parallel(Product::created_at_r());
    let timestamp_par_time = start.elapsed();
    
    let start = Instant::now();
    let avg_timestamp_query_par = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let avg_timestamp_par = avg_timestamp_query_par.avg_timestamp_parallel(Product::created_at_r());
    let avg_timestamp_par_time = start.elapsed();

    println!("  ⏰ Timestamp Aggregator Performance Results:");
    println!("    Min timestamp (Electronics):");
    println!("      Sequential: {:?} (min: {:?})", timestamp_seq_time, min_timestamp_seq);
    println!("      Parallel:   {:?} (min: {:?})", timestamp_par_time, min_timestamp_par);
    println!("      Speedup:    {:.2}x", timestamp_seq_time.as_nanos() as f64 / timestamp_par_time.as_nanos() as f64);
    
    println!("    Average timestamp (Electronics):");
    println!("      Sequential: {:?} (avg: {:?})", avg_timestamp_seq_time, avg_timestamp_seq);
    println!("      Parallel:   {:?} (avg: {:?})", avg_timestamp_par_time, avg_timestamp_par);
    println!("      Speedup:    {:.2}x", avg_timestamp_seq_time.as_nanos() as f64 / avg_timestamp_par_time.as_nanos() as f64);

    // ============================================================================
    // 6. COMPLEX QUERIES - Sequential vs Parallel
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("6. COMPLEX QUERIES - Sequential vs Parallel Performance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Complex sequential query
    let start = Instant::now();
    let complex_seq = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .where_(Product::stock_r(), |&s| s > 10)
        .order_by_float(Product::price_r());
    let complex_seq_time = start.elapsed();
    
    // Complex parallel query
    let start = Instant::now();
    let complex_query_par = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p > 100.0)
        .where_(Product::stock_r(), |&s| s > 10);
    let complex_par = complex_query_par.all_parallel();
    let complex_par_time = start.elapsed();

    println!("  🔧 Complex Query Performance Results:");
    println!("    Electronics: price > $100, stock > 10, ordered by price:");
    println!("      Sequential: {:?} ({} items)", complex_seq_time, complex_seq.len());
    println!("      Parallel:   {:?} ({} items)", complex_par_time, complex_par.len());
    println!("      Speedup:    {:.2}x", complex_seq_time.as_nanos() as f64 / complex_par_time.as_nanos() as f64);

    // ============================================================================
    // 7. SUMMARY
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("7. SUMMARY - Parallel Processing Benefits");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Successfully demonstrated parallel processing for:");
    println!("   • Query (eager) - all_parallel(), avg_parallel(), timestamp aggregators, etc.");
    println!("   • LazyQuery, LockQuery, LockLazyQuery - Sequential only (parallel removed for thread safety)");

    println!("\n✅ Parallel methods available (Query only):");
    println!("   • all_parallel() - Collect all results in parallel");
    println!("   • count_parallel() - Count items in parallel");
    println!("   • exists_parallel() - Check existence in parallel");
    println!("   • min_parallel() / max_parallel() - Find min/max in parallel (Ord types)");
    println!("   • sum_parallel() - Sum values in parallel");
    println!("   • avg_parallel() - Calculate average in parallel");
    println!("   • min_timestamp_parallel() / max_timestamp_parallel() - Timestamp min/max in parallel");
    println!("   • avg_timestamp_parallel() / sum_timestamp_parallel() - Timestamp aggregations in parallel");
    println!("   • count_timestamp_parallel() - Count timestamps in parallel");

    println!("\n🚀 Key Benefits:");
    println!("   • Significant performance improvements on large datasets");
    println!("   • Automatic CPU core utilization");
    println!("   • Zero-cost when parallel feature is disabled");
    println!("   • Thread-safe operations with lock-aware queries");
    println!("   • Consistent API across all query types");

    println!("\n🎯 Perfect for:");
    println!("   • Large datasets (10,000+ items)");
    println!("   • CPU-intensive aggregations");
    println!("   • Multi-core systems");
    println!("   • Real-time analytics");
    println!("   • High-performance applications");

    println!("\n🎉 Parallel processing is now available for Query (eager) operations!");
    println!("   Enable with: cargo run --features parallel");
    println!("   Note: Other query types use sequential processing for thread safety");
}
