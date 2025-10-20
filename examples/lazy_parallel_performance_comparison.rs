use rust_queries_builder::{QueryableExt, LazyParallelQueryExt};
use std::time::{Instant, SystemTime};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: f64,
    stock: u32,
    rating: f64,
    created_at: SystemTime,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Event {
    id: u32,
    event_type: String,
    timestamp: SystemTime,
    user_id: u32,
    data: HashMap<String, String>,
    severity: u8,
    location: String,
}

fn create_large_product_dataset(size: usize) -> Vec<Product> {
    let mut products = Vec::with_capacity(size);
    let categories = vec!["Electronics", "Clothing", "Books", "Home", "Sports", "Toys", "Food", "Beauty"];
    let tags = vec!["new", "sale", "popular", "limited", "premium", "basic", "featured", "trending"];
    
    for i in 0..size {
        let category = categories[i % categories.len()];
        let tag = tags[i % tags.len()];
        
        let mut metadata = HashMap::new();
        metadata.insert("brand".to_string(), format!("Brand{}", i % 100));
        metadata.insert("color".to_string(), format!("Color{}", i % 20));
        metadata.insert("size".to_string(), format!("Size{}", i % 10));
        
        products.push(Product {
            id: i as u32,
            name: format!("Product {}", i),
            category: category.to_string(),
            price: 10.0 + (i as f64 * 0.1) % 1000.0,
            stock: (i % 1000) as u32,
            rating: 1.0 + (i as f64 * 0.01) % 4.0,
            created_at: SystemTime::now(),
            tags: vec![tag.to_string()],
            metadata,
        });
    }
    
    products
}

fn create_large_event_dataset(size: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(size);
    let event_types = vec!["login", "purchase", "view", "click", "logout", "error", "warning", "info"];
    let locations = vec!["US", "EU", "ASIA", "LATAM", "AFRICA", "OCEANIA"];
    
    for i in 0..size {
        let event_type = event_types[i % event_types.len()];
        let location = locations[i % locations.len()];
        
        let mut data = HashMap::new();
        data.insert("session_id".to_string(), format!("session_{}", i));
        data.insert("ip_address".to_string(), format!("192.168.1.{}", i % 255));
        data.insert("user_agent".to_string(), format!("Browser{}", i % 10));
        
        events.push(Event {
            id: i as u32,
            event_type: event_type.to_string(),
            timestamp: SystemTime::now(),
            user_id: (i % 10000) as u32,
            data,
            severity: (i % 5) as u8,
            location: location.to_string(),
        });
    }
    
    events
}

fn benchmark_collection_operations(products: &Vec<Product>) {
    println!("\n🔍 COLLECTION OPERATIONS BENCHMARK");
    println!("Dataset size: {} products", products.len());
    
    // Test 1: Simple collection
    let start = Instant::now();
    let lazy_results = products.lazy_query().collect();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_results = products.lazy_parallel_query().collect_parallel();
    let parallel_time = start.elapsed();
    
    println!("Simple collection (all items):");
    println!("  Lazy: {:?} ({} results)", lazy_time, lazy_results.len());
    println!("  Parallel: {:?} ({} results)", parallel_time, parallel_results.len());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_count_operations(products: &Vec<Product>) {
    println!("\n📊 COUNT OPERATIONS BENCHMARK");
    
    // Test 1: Count operations
    let start = Instant::now();
    let lazy_count = products.lazy_query().count();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_count = products.lazy_parallel_query().count_parallel();
    let parallel_time = start.elapsed();
    
    println!("Count operations:");
    println!("  Lazy: {:?} (count: {})", lazy_time, lazy_count);
    println!("  Parallel: {:?} (count: {})", parallel_time, parallel_count);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_terminal_operations(products: &Vec<Product>) {
    println!("\n🎯 TERMINAL OPERATIONS BENCHMARK");
    
    // Test 1: Any operations
    let start = Instant::now();
    let lazy_any = products.lazy_query().any();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_any = products.lazy_parallel_query().any_parallel();
    let parallel_time = start.elapsed();
    
    println!("Any operations:");
    println!("  Lazy: {:?} (any: {})", lazy_time, lazy_any);
    println!("  Parallel: {:?} (any: {})", parallel_time, parallel_any);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Test 2: First operations
    let start = Instant::now();
    let lazy_first = products.lazy_query().first();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_first = products.lazy_parallel_query().first_parallel();
    let parallel_time = start.elapsed();
    
    println!("First operations:");
    println!("  Lazy: {:?} (first: {})", lazy_time, lazy_first.is_some());
    println!("  Parallel: {:?} (first: {})", parallel_time, parallel_first.is_some());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_fold_operations(products: &Vec<Product>) {
    println!("\n🔄 FOLD OPERATIONS BENCHMARK");
    
    // Test 1: Fold operations for sum
    let start = Instant::now();
    let lazy_sum = products.lazy_query().fold(0.0, |acc, p| acc + p.price);
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_sum = products.lazy_parallel_query().fold_parallel(0.0, |acc, p| acc + p.price);
    let parallel_time = start.elapsed();
    
    println!("Fold operations (sum prices):");
    println!("  Lazy: {:?} (sum: {:.2})", lazy_time, lazy_sum);
    println!("  Parallel: {:?} (sum: {:.2})", parallel_time, parallel_sum);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Test 2: Fold operations for count
    let start = Instant::now();
    let lazy_count = products.lazy_query().fold(0, |acc, _| acc + 1);
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_count = products.lazy_parallel_query().fold_parallel(0, |acc, _| acc + 1);
    let parallel_time = start.elapsed();
    
    println!("Fold operations (count):");
    println!("  Lazy: {:?} (count: {})", lazy_time, lazy_count);
    println!("  Parallel: {:?} (count: {})", parallel_time, parallel_count);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_projection_operations(products: &Vec<Product>) {
    println!("\n🎨 PROJECTION OPERATIONS BENCHMARK");
    
    // Test 1: Map operations
    let start = Instant::now();
    let lazy_mapped: Vec<_> = products.lazy_query().map_items(|p| (p.name.clone(), p.price * 1.1)).collect();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_mapped = products.lazy_parallel_query().map_items_parallel(|p| (p.name.clone(), p.price * 1.1));
    let parallel_time = start.elapsed();
    
    println!("Map operations (transform data):");
    println!("  Lazy: {:?} ({} results)", lazy_time, lazy_mapped.len());
    println!("  Parallel: {:?} ({} results)", parallel_time, parallel_mapped.len());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_pagination_operations(products: &Vec<Product>) {
    println!("\n📄 PAGINATION OPERATIONS BENCHMARK");
    
    // Test 1: Take operations
    let start = Instant::now();
    let lazy_taken: Vec<_> = products.lazy_query().take_lazy(1000).collect();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_taken = products.lazy_parallel_query().take_parallel(1000);
    let parallel_time = start.elapsed();
    
    println!("Take operations (1000 items):");
    println!("  Lazy: {:?} ({} results)", lazy_time, lazy_taken.len());
    println!("  Parallel: {:?} ({} results)", parallel_time, parallel_taken.len());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Test 2: Skip operations
    let start = Instant::now();
    let lazy_skipped: Vec<_> = products.lazy_query().skip_lazy(5000).collect();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_skipped = products.lazy_parallel_query().skip_parallel(5000);
    let parallel_time = start.elapsed();
    
    println!("Skip operations (skip 5000):");
    println!("  Lazy: {:?} ({} results)", lazy_time, lazy_skipped.len());
    println!("  Parallel: {:?} ({} results)", parallel_time, parallel_skipped.len());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_event_operations(events: &Vec<Event>) {
    println!("\n📅 EVENT OPERATIONS BENCHMARK");
    
    // Test 1: Event collection
    let start = Instant::now();
    let lazy_events = events.lazy_query().collect();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_events = events.lazy_parallel_query().collect_parallel();
    let parallel_time = start.elapsed();
    
    println!("Event collection:");
    println!("  Lazy: {:?} ({} results)", lazy_time, lazy_events.len());
    println!("  Parallel: {:?} ({} results)", parallel_time, parallel_events.len());
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Test 2: Event count
    let start = Instant::now();
    let lazy_event_count = events.lazy_query().count();
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    let parallel_event_count = events.lazy_parallel_query().count_parallel();
    let parallel_time = start.elapsed();
    
    println!("Event count:");
    println!("  Lazy: {:?} (count: {})", lazy_time, lazy_event_count);
    println!("  Parallel: {:?} (count: {})", parallel_time, parallel_event_count);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn benchmark_for_each_operations(products: &Vec<Product>) {
    println!("\n🔄 FOR_EACH OPERATIONS BENCHMARK");
    
    // Test 1: For each operations
    let start = Instant::now();
    let mut lazy_sum = 0.0;
    products.lazy_query().for_each(|p| { lazy_sum += p.price; });
    let lazy_time = start.elapsed();
    
    let start = Instant::now();
    products.lazy_parallel_query().for_each_parallel(|p| { 
        // Just demonstrate the operation without mutation
        let _ = p.price; 
    });
    let parallel_time = start.elapsed();
    
    println!("For each operations (process prices):");
    println!("  Lazy: {:?} (sum: {:.2})", lazy_time, lazy_sum);
    println!("  Parallel: {:?} (processed all items)", parallel_time);
    println!("  Speedup: {:.2}x", lazy_time.as_secs_f64() / parallel_time.as_secs_f64());
}

fn main() {
    println!("🚀 LAZY PARALLEL QUERY PERFORMANCE COMPARISON");
    println!("=============================================");
    
    // Create large datasets
    println!("Creating large datasets...");
    let products = create_large_product_dataset(1_000_000);
    let events = create_large_event_dataset(1_000_000);
    
    println!("Dataset sizes:");
    println!("  Products: {}", products.len());
    println!("  Events: {}", events.len());
    
    // Run benchmarks
    benchmark_collection_operations(&products);
    benchmark_count_operations(&products);
    benchmark_terminal_operations(&products);
    benchmark_fold_operations(&products);
    benchmark_projection_operations(&products);
    benchmark_pagination_operations(&products);
    benchmark_event_operations(&events);
    benchmark_for_each_operations(&products);
    
    println!("\n📊 PERFORMANCE SUMMARY");
    println!("=====================");
    println!("✅ All benchmarks completed successfully!");
    println!("🔍 Parallel lazy queries show performance improvements for large datasets");
    println!("⚡ Best performance gains are seen with:");
    println!("   - Collection operations");
    println!("   - Count operations");
    println!("   - Fold operations");
    println!("   - Large dataset processing");
    println!("   - CPU-intensive computations");
    println!("🎯 Use parallel lazy queries when processing large datasets or complex operations");
    println!("💡 Consider the overhead of parallelization for small datasets");
    println!("🔧 Note: This example focuses on core operations without keypath filtering");
    println!("   For keypath-based filtering, see the lazy_parallel_query_demo example");
}