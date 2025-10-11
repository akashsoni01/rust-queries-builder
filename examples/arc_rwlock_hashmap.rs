// Demonstrates querying HashMap<String, Arc<RwLock<Product>>>
// This is a common pattern for thread-safe shared data
// Shows all lazy query operations
// cargo run --example arc_rwlock_hashmap

use rust_queries_builder::LazyQuery;
use key_paths_derive::Keypaths;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
    active: bool,
}

type ProductId = String;
type SharedProduct = Arc<RwLock<Product>>;
type ProductMap = HashMap<ProductId, SharedProduct>;

fn create_sample_data() -> ProductMap {
    let mut map = HashMap::new();

    let products = vec![
        Product { id: 1, name: "Laptop Pro".to_string(), price: 1299.99, category: "Electronics".to_string(), stock: 15, rating: 4.8, active: true },
        Product { id: 2, name: "Wireless Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.5, active: true },
        Product { id: 3, name: "Mechanical Keyboard".to_string(), price: 129.99, category: "Electronics".to_string(), stock: 30, rating: 4.7, active: true },
        Product { id: 4, name: "Office Chair".to_string(), price: 299.99, category: "Furniture".to_string(), stock: 20, rating: 4.6, active: true },
        Product { id: 5, name: "Standing Desk".to_string(), price: 499.99, category: "Furniture".to_string(), stock: 10, rating: 4.9, active: true },
        Product { id: 6, name: "USB-C Hub".to_string(), price: 49.99, category: "Electronics".to_string(), stock: 100, rating: 4.3, active: true },
        Product { id: 7, name: "Monitor 27\"".to_string(), price: 349.99, category: "Electronics".to_string(), stock: 25, rating: 4.7, active: true },
        Product { id: 8, name: "Desk Lamp".to_string(), price: 39.99, category: "Furniture".to_string(), stock: 40, rating: 4.2, active: true },
        Product { id: 9, name: "Webcam HD".to_string(), price: 79.99, category: "Electronics".to_string(), stock: 35, rating: 4.4, active: true },
        Product { id: 10, name: "Bookshelf".to_string(), price: 149.99, category: "Furniture".to_string(), stock: 15, rating: 4.5, active: false },
    ];

    for product in products {
        let id = format!("PROD-{:03}", product.id);
        map.insert(id, Arc::new(RwLock::new(product)));
    }

    map
}

// Helper to extract products from Arc<RwLock<Product>> for querying
fn extract_products(map: &ProductMap) -> Vec<Product> {
    map.values()
        .filter_map(|arc_lock| {
            arc_lock.read().ok().map(|guard| guard.clone())
        })
        .collect()
}

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Arc<RwLock<T>> HashMap Lazy Query Demo                         ║");
    println!("║  Thread-safe shared data with lazy evaluation                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let product_map = create_sample_data();
    println!("Created product catalog:");
    println!("  Total products: {}", product_map.len());
    println!("  Keys: {:?}\n", product_map.keys().take(3).collect::<Vec<_>>());

    // Extract products for querying
    let products = extract_products(&product_map);
    println!("Extracted {} products from Arc<RwLock<Product>>\n", products.len());

    // ============================================================================
    // LAZY OPERATION 1: where_ - Lazy Filtering
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 1: where_ (filtering)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Building filtered query (nothing executes yet)...");
    let electronics_query = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::active_r(), |&active| active)
        .where_(Product::stock_r(), |&stock| stock > 20);

    println!("  ✅ Query built (deferred execution)\n");

    println!("Collecting results (executes now)...");
    let electronics: Vec<_> = electronics_query.collect();
    println!("  Found {} electronics in stock", electronics.len());
    for p in &electronics {
        println!("    • {}: {} in stock", p.name, p.stock);
    }

    // ============================================================================
    // LAZY OPERATION 2: select_lazy - Lazy Projection
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 2: select_lazy (projection)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Selecting product names (lazy)...");
    let names: Vec<String> = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Furniture")
        .select_lazy(Product::name_r())
        .collect();

    println!("  Furniture names ({}):", names.len());
    for name in &names {
        println!("    • {}", name);
    }

    // ============================================================================
    // LAZY OPERATION 3: take_lazy - Early Termination
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 3: take_lazy (early termination)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Getting first 3 electronics...");
    let first_3: Vec<_> = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .take_lazy(3)
        .collect();

    println!("  First 3 electronics:");
    for (i, p) in first_3.iter().enumerate() {
        println!("    {}. {} - ${:.2}", i + 1, p.name, p.price);
    }
    println!("  ✅ Stopped after finding 3 items!");

    // ============================================================================
    // LAZY OPERATION 4: skip_lazy - Pagination
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 4: skip_lazy (pagination)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Getting page 2 (skip 3, take 3)...");
    let page_2: Vec<_> = LazyQuery::new(&products)
        .where_(Product::active_r(), |&active| active)
        .skip_lazy(3)
        .take_lazy(3)
        .collect();

    println!("  Page 2 items:");
    for (i, p) in page_2.iter().enumerate() {
        println!("    {}. {}", i + 4, p.name);
    }

    // ============================================================================
    // LAZY OPERATION 5: first - Short-Circuit Search
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 5: first (short-circuit)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Finding first expensive item (>$1000)...");
    let expensive = LazyQuery::new(&products)
        .where_(Product::price_r(), |&price| price > 1000.0)
        .first();

    match expensive {
        Some(p) => println!("  Found: {} - ${:.2}", p.name, p.price),
        None => println!("  Not found"),
    }
    println!("  ✅ Stopped at first match!");

    // ============================================================================
    // LAZY OPERATION 6: any - Existence Check
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 6: any (existence check)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Checking if any furniture exists...");
    let has_furniture = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Furniture")
        .any();

    println!("  Has furniture: {}", has_furniture);
    println!("  ✅ Stopped immediately after finding first match!");

    // ============================================================================
    // LAZY OPERATION 7: count - Count Matching Items
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 7: count");
    println!("═══════════════════════════════════════════════════════════════\n");

    let electronics_count = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .count();

    println!("  Electronics count: {}", electronics_count);

    // ============================================================================
    // LAZY OPERATION 8: sum_by - Aggregation
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 8: sum_by (aggregation)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let total_value: f64 = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .sum_by(Product::price_r());

    println!("  Total electronics value: ${:.2}", total_value);

    // ============================================================================
    // LAZY OPERATION 9: avg_by - Average
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 9: avg_by");
    println!("═══════════════════════════════════════════════════════════════\n");

    let avg_price = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Furniture")
        .avg_by(Product::price_r())
        .unwrap_or(0.0);

    println!("  Average furniture price: ${:.2}", avg_price);

    // ============================================================================
    // LAZY OPERATION 10: min_by_float / max_by_float
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 10: min_by_float / max_by_float");
    println!("═══════════════════════════════════════════════════════════════\n");

    let min_price = LazyQuery::new(&products)
        .where_(Product::active_r(), |&active| active)
        .min_by_float(Product::price_r())
        .unwrap_or(0.0);

    let max_price = LazyQuery::new(&products)
        .where_(Product::active_r(), |&active| active)
        .max_by_float(Product::price_r())
        .unwrap_or(0.0);

    println!("  Price range for active products:");
    println!("    Min: ${:.2}", min_price);
    println!("    Max: ${:.2}", max_price);

    // ============================================================================
    // LAZY OPERATION 11: find - Find with Predicate
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 11: find (with predicate)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let high_rated = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .find(|item| item.rating > 4.7);

    if let Some(product) = high_rated {
        println!("  First highly-rated electronic:");
        println!("    {}: Rating {:.1}", product.name, product.rating);
    }

    // ============================================================================
    // LAZY OPERATION 12: for_each - Iteration
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 12: for_each (iteration)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("  Low stock alerts:");
    LazyQuery::new(&products)
        .where_(Product::stock_r(), |&stock| stock < 20)
        .for_each(|product| {
            println!("    ⚠️  {}: Only {} in stock", product.name, product.stock);
        });

    // ============================================================================
    // LAZY OPERATION 13: fold - Custom Aggregation
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 14: fold (custom aggregation)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let total_inventory_value = LazyQuery::new(&products)
        .where_(Product::active_r(), |&active| active)
        .fold(0.0, |acc, product| {
            acc + (product.price * product.stock as f64)
        });

    println!("  Total inventory value: ${:.2}", total_inventory_value);

    // ============================================================================
    // LAZY OPERATION 14: all_match - Validation
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 15: all_match (validation)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let all_priced = LazyQuery::new(&products)
        .all_match(|item| item.price > 0.0);

    println!("  All products have valid prices: {}", all_priced);

    // ============================================================================
    // LAZY OPERATION 15: into_iter - For Loop
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 16: into_iter (for loop)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("  High-value products (>$300):");
    for product in LazyQuery::new(&products)
        .where_(Product::price_r(), |&p| p > 300.0)
        .take_lazy(5)
    {
        println!("    • {}: ${:.2}", product.name, product.price);
    }

    // ============================================================================
    // LAZY OPERATION 16: map_items - Transform
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Lazy Operation 17: map_items (transformation)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let price_tags: Vec<String> = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .map_items(|p| format!("{}: ${:.2}", p.name, p.price))
        .take(5)
        .collect();

    println!("  Electronics price tags:");
    for tag in &price_tags {
        println!("    • {}", tag);
    }

    // ============================================================================
    // COMPLEX EXAMPLE: Multi-Stage Lazy Pipeline
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Complex Example: Multi-stage lazy pipeline");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Building complex query:");
    println!("  1. Filter by category (Electronics)");
    println!("  2. Filter by price range ($50-$500)");
    println!("  3. Filter by rating (>4.5)");
    println!("  4. Select names");
    println!("  5. Take first 3");
    println!();

    let results: Vec<String> = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&p| p >= 50.0 && p <= 500.0)
        .where_(Product::rating_r(), |&r| r > 4.5)
        .select_lazy(Product::name_r())
        .take(3)
        .collect();

    println!("  Results:");
    for (i, name) in results.iter().enumerate() {
        println!("    {}. {}", i + 1, name);
    }
    println!("  ✅ All operations fused and executed lazily!");

    // ============================================================================
    // THREAD-SAFE UPDATES WITH RWLOCK
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Bonus: Thread-safe updates with RwLock");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Update a product through the Arc<RwLock>
    if let Some(product_arc) = product_map.get("PROD-002") {
        println!("Updating PROD-002 stock...");
        
        // Write lock for mutation
        if let Ok(mut product) = product_arc.write() {
            let old_stock = product.stock;
            product.stock = 25;
            println!("  Stock updated: {} → {}", old_stock, product.stock);
        }
    }

    // Query again to see the update
    let updated_products = extract_products(&product_map);
    let mouse = LazyQuery::new(&updated_products)
        .find(|p| p.id == 2);

    if let Some(product) = mouse {
        println!("  Verified update: {} now has {} in stock", product.name, product.stock);
    }

    // ============================================================================
    // PRACTICAL EXAMPLE: Price Analysis by Category
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Practical Example: Price analysis by category");
    println!("═══════════════════════════════════════════════════════════════\n");

    let categories = vec!["Electronics", "Furniture"];

    for category in categories {
        println!("  {} Statistics:", category);
        
        // Filter products by category first
        let cat_products: Vec<Product> = products.iter()
            .filter(|p| p.category == category)
            .cloned()
            .collect();

        let count = LazyQuery::new(&cat_products).count();
        let total = LazyQuery::new(&cat_products).sum_by(Product::price_r());
        let avg = LazyQuery::new(&cat_products).avg_by(Product::price_r()).unwrap_or(0.0);
        let min = LazyQuery::new(&cat_products).min_by_float(Product::price_r()).unwrap_or(0.0);
        let max = LazyQuery::new(&cat_products).max_by_float(Product::price_r()).unwrap_or(0.0);

        println!("    Count: {}", count);
        println!("    Total: ${:.2}", total);
        println!("    Average: ${:.2}", avg);
        println!("    Range: ${:.2} - ${:.2}\n", min, max);
    }

    // ============================================================================
    // COMBINING WITH HASHMAP OPERATIONS
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("HashMap Integration: Query by key patterns");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Filter HashMap by key pattern
    let electronics_keys: Vec<String> = product_map
        .iter()
        .filter(|(_key, value)| {
            // Read the value to check category
            if let Ok(guard) = value.read() {
                guard.category == "Electronics"
            } else {
                false
            }
        })
        .map(|(key, _value)| key.clone())
        .collect();

    println!("  Electronics product IDs:");
    for key in electronics_keys.iter().take(5) {
        println!("    • {}", key);
    }

    // ============================================================================
    // PERFORMANCE DEMONSTRATION
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Performance: Lazy vs Eager comparison");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Scenario: Find first product with rating > 4.7 from {} products\n", products.len());

    println!("  Eager approach (hypothetical):");
    println!("    - Filter all {} products", products.len());
    println!("    - Collect filtered results");
    println!("    - Take first item");
    println!("    - Wasted work: Check all items\n");

    println!("  Lazy approach (actual):");
    let _first_rated = LazyQuery::new(&products)
        .where_(Product::rating_r(), |&r| r > 4.7)
        .first();
    println!("    - Starts checking products");
    println!("    - Stops at first match");
    println!("    - ✅ Early termination - checks ~3-5 items only!");

    // ============================================================================
    // Summary
    // ============================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Summary: All Lazy Operations Demonstrated                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("✅ Lazy Query Operations:");
    println!("   1. ✅ where_ - Lazy filtering");
    println!("   2. ✅ select_lazy - Lazy projection");
    println!("   3. ✅ take_lazy - Early termination");
    println!("   4. ✅ skip_lazy - Pagination");
    println!("   5. ✅ first - Short-circuit search");
    println!("   6. ✅ any - Existence check (short-circuit)");
    println!("   7. ✅ count - Count items");
    println!("   8. ✅ sum_by - Sum aggregation");
    println!("   9. ✅ avg_by - Average aggregation");
    println!("   10. ✅ min_by_float - Minimum value");
    println!("   11. ✅ max_by_float - Maximum value");
    println!("   12. ✅ find - Find with predicate (short-circuit)");
    println!("   13. ✅ for_each - Iteration");
    println!("   14. ✅ fold - Custom aggregation");
    println!("   15. ✅ all_match - Validation (short-circuit)");
    println!("   16. ✅ into_iter - For loop support");
    println!("   17. ✅ map_items - Transformation\n");

    println!("✅ Arc<RwLock<T>> Benefits:");
    println!("   • Thread-safe shared access");
    println!("   • Interior mutability");
    println!("   • Multiple readers, single writer");
    println!("   • Reference counting (Arc)");
    println!("   • Can be queried after extracting data\n");

    println!("✅ HashMap<K, Arc<RwLock<V>>> Benefits:");
    println!("   • Fast key-based lookup");
    println!("   • Thread-safe value access");
    println!("   • Can query all values");
    println!("   • Can filter by key patterns");
    println!("   • Perfect for shared state/caches\n");

    println!("🎯 Use Cases:");
    println!("   • Shared product catalogs");
    println!("   • User session stores");
    println!("   • Configuration caches");
    println!("   • Real-time inventory systems");
    println!("   • Multi-threaded data processing");
    println!("   • Web server state management\n");

    println!("✓ Arc<RwLock<T>> HashMap with all lazy operations demo complete!\n");
}

