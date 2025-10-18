// Demonstrates full SQL-like query syntax for locked data structures
// This example shows:
// 1. Complete Query API on HashMap<K, Arc<RwLock<V>>>
// 2. WHERE, SELECT, ORDER BY, GROUP BY, aggregations
// 3. Lazy queries with early termination
// 4. No data copying until final collection
// 5. SQL equivalents for each operation
//
// cargo run --example sql_like_lock_queries --release

use rust_queries_builder::{LockQueryable, LockLazyQueryable};
use key_paths_derive::Keypath;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
    active: bool,
}

type ProductMap = HashMap<String, Arc<RwLock<Product>>>;

fn create_product_catalog() -> ProductMap {
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

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  SQL-Like Query Syntax for Locked Data (HashMap)                ║");
    println!("║  Full Query API on Arc<RwLock<T>> without copying!              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let products = create_product_catalog();
    println!("Created product catalog with {} items\n", products.len());

    // ============================================================================
    // 1. WHERE - FILTERING
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. WHERE - Filtering with Key-Paths");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Single WHERE clause
    println!("--- WHERE category = 'Electronics' ---");
    let start = Instant::now();
    let electronics = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .all();
    let duration = start.elapsed();
    
    println!("  Found: {} products in {:?}", electronics.len(), duration);
    println!("  SQL: SELECT * FROM products WHERE category = 'Electronics';\n");

    // Multiple WHERE clauses
    println!("--- WHERE category = 'Electronics' AND price > 100 ---");
    let start = Instant::now();
    let expensive_electronics = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p > 100.0)
        .all();
    let duration = start.elapsed();
    
    println!("  Found: {} products in {:?}", expensive_electronics.len(), duration);
    for p in expensive_electronics.iter().take(3) {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!("  SQL: SELECT * FROM products WHERE category = 'Electronics' AND price > 100;\n");

    // ============================================================================
    // 2. SELECT - PROJECTION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. SELECT - Projection");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- SELECT name FROM products ---");
    let start = Instant::now();
    let names = products
        .lock_query()
        .select(Product::name());
    let duration = start.elapsed();
    
    println!("  Found: {} names in {:?}", names.len(), duration);
    for name in names.iter().take(5) {
        println!("    • {}", name);
    }
    println!("  SQL: SELECT name FROM products;\n");

    println!("--- SELECT price WHERE category = 'Furniture' ---");
    let start = Instant::now();
    let furniture_prices = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Furniture")
        .select(Product::price());
    let duration = start.elapsed();
    
    println!("  Found: {} prices in {:?}", furniture_prices.len(), duration);
    for price in &furniture_prices {
        println!("    • ${:.2}", price);
    }
    println!("  SQL: SELECT price FROM products WHERE category = 'Furniture';\n");

    // ============================================================================
    // 3. ORDER BY - SORTING
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. ORDER BY - Sorting");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- ORDER BY price ASC ---");
    let start = Instant::now();
    let sorted_by_price = products
        .lock_query()
        .order_by_float(Product::price());
    let duration = start.elapsed();
    
    println!("  Sorted {} products in {:?}", sorted_by_price.len(), duration);
    println!("  Top 3 cheapest:");
    for (i, p) in sorted_by_price.iter().take(3).enumerate() {
        println!("    {}. {} - ${:.2}", i + 1, p.name, p.price);
    }
    println!("  SQL: SELECT * FROM products ORDER BY price ASC;\n");

    println!("--- ORDER BY rating DESC ---");
    let start = Instant::now();
    let sorted_by_rating = products
        .lock_query()
        .order_by_float_desc(Product::rating());
    let duration = start.elapsed();
    
    println!("  Sorted {} products in {:?}", sorted_by_rating.len(), duration);
    println!("  Top 3 rated:");
    for (i, p) in sorted_by_rating.iter().take(3).enumerate() {
        println!("    {}. {} - Rating: {:.1}", i + 1, p.name, p.rating);
    }
    println!("  SQL: SELECT * FROM products ORDER BY rating DESC;\n");

    // ============================================================================
    // 4. GROUP BY - GROUPING
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. GROUP BY - Grouping");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- GROUP BY category ---");
    let start = Instant::now();
    let by_category = products
        .lock_query()
        .group_by(Product::category());
    let duration = start.elapsed();
    
    println!("  Grouped into {} categories in {:?}", by_category.len(), duration);
    for (category, items) in &by_category {
        println!("\n  {}: {} products", category, items.len());
        for item in items {
            println!("    • {} - ${:.2}", item.name, item.price);
        }
    }
    println!("\n  SQL: SELECT category, COUNT(*) FROM products GROUP BY category;\n");

    // ============================================================================
    // 5. AGGREGATIONS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. AGGREGATIONS (COUNT, SUM, AVG, MIN, MAX)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Aggregations for Electronics Category ---");
    let electronics_query = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Electronics");

    let start = Instant::now();
    let count = electronics_query.count();
    let total_value = electronics_query.sum(Product::price());
    let avg_price = electronics_query.avg(Product::price()).unwrap_or(0.0);
    let min_price = electronics_query.min_float(Product::price()).unwrap_or(0.0);
    let max_price = electronics_query.max_float(Product::price()).unwrap_or(0.0);
    let total_stock = electronics_query.sum(Product::stock());
    let duration = start.elapsed();
    
    println!("  Count: {} (in {:?})", count, duration);
    println!("  Total Value: ${:.2}", total_value);
    println!("  Average Price: ${:.2}", avg_price);
    println!("  Min Price: ${:.2}", min_price);
    println!("  Max Price: ${:.2}", max_price);
    println!("  Total Stock: {}", total_stock);
    
    println!("\n  SQL:");
    println!("    SELECT COUNT(*), SUM(price), AVG(price), MIN(price), MAX(price), SUM(stock)");
    println!("    FROM products WHERE category = 'Electronics';\n");

    // ============================================================================
    // 6. LIMIT - PAGINATION
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("6. LIMIT - Pagination");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- LIMIT 3 ---");
    let start = Instant::now();
    let first_3 = products
        .lock_query()
        .limit(3);
    let duration = start.elapsed();
    
    println!("  Found: {} products in {:?}", first_3.len(), duration);
    for (i, p) in first_3.iter().enumerate() {
        println!("    {}. {} - ${:.2}", i + 1, p.name, p.price);
    }
    println!("  SQL: SELECT * FROM products LIMIT 3;\n");

    // ============================================================================
    // 7. COMPLEX QUERY - Multiple Clauses
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("7. COMPLEX QUERY - Multiple Clauses");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Complex: Electronics, price < 200, rating > 4.5, ORDER BY price ---");
    let start = Instant::now();
    let results = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p < 200.0)
        .where_(Product::rating(), |&r| r > 4.5)
        .order_by_float(Product::price());
    let duration = start.elapsed();
    
    println!("  Found: {} products in {:?}", results.len(), duration);
    for p in &results {
        println!("    • {} - ${:.2} - Rating: {:.1}", p.name, p.price, p.rating);
    }
    println!("\n  SQL:");
    println!("    SELECT * FROM products");
    println!("    WHERE category = 'Electronics'");
    println!("    AND price < 200");
    println!("    AND rating > 4.5");
    println!("    ORDER BY price ASC;\n");

    // ============================================================================
    // 8. LAZY QUERIES - Early Termination
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("8. LAZY QUERIES - Early Termination");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Lazy: WHERE active = true LIMIT 3 ---");
    let start = Instant::now();
    let active_products: Vec<_> = products
        .lock_lazy_query()
        .where_(Product::active(), |&a| a)
        .take_lazy(3)
        .collect();
    let duration = start.elapsed();
    
    println!("  Found: {} products in {:?} (stopped after 3!)", active_products.len(), duration);
    for (i, p) in active_products.iter().enumerate() {
        println!("    {}. {}", i + 1, p.name);
    }
    println!("  SQL: SELECT * FROM products WHERE active = true LIMIT 3;\n");

    println!("--- Lazy: SELECT name WHERE rating > 4.5 LIMIT 5 ---");
    let start = Instant::now();
    let top_names: Vec<String> = products
        .lock_lazy_query()
        .where_(Product::rating(), |&r| r > 4.5)
        .select_lazy(Product::name())
        .take(5)
        .collect();
    let duration = start.elapsed();
    
    println!("  Found: {} names in {:?}", top_names.len(), duration);
    for name in &top_names {
        println!("    • {}", name);
    }
    println!("  SQL: SELECT name FROM products WHERE rating > 4.5 LIMIT 5;\n");

    // ============================================================================
    // 9. EXISTS - Existence Check
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("9. EXISTS - Existence Check");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- EXISTS (SELECT * WHERE price > 1000) ---");
    let start = Instant::now();
    let has_expensive = products
        .lock_query()
        .where_(Product::price(), |&p| p > 1000.0)
        .exists();
    let duration = start.elapsed();
    
    println!("  Exists: {} (checked in {:?})", has_expensive, duration);
    println!("  SQL: SELECT EXISTS(SELECT 1 FROM products WHERE price > 1000);\n");

    // ============================================================================
    // 10. FIRST - Find First Match
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("10. FIRST - Find First Match");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- FIRST WHERE category = 'Furniture' ---");
    let start = Instant::now();
    let first_furniture = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Furniture")
        .first();
    let duration = start.elapsed();
    
    if let Some(p) = first_furniture {
        println!("  Found: {} - ${:.2} (in {:?})", p.name, p.price, duration);
    }
    println!("  SQL: SELECT * FROM products WHERE category = 'Furniture' LIMIT 1;\n");

    // ============================================================================
    // 11. AGGREGATIONS BY GROUP
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("11. GROUP BY with Aggregations");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Category Statistics ---");
    let start = Instant::now();
    let grouped = products
        .lock_query()
        .group_by(Product::category());
    let duration = start.elapsed();
    
    println!("  Grouped in {:?}\n", duration);
    for (category, items) in &grouped {
        let total_value: f64 = items.iter().map(|p| p.price).sum();
        let avg_price = total_value / items.len() as f64;
        let total_stock: u32 = items.iter().map(|p| p.stock).sum();
        
        println!("  {} Statistics:", category);
        println!("    Products: {}", items.len());
        println!("    Total Value: ${:.2}", total_value);
        println!("    Avg Price: ${:.2}", avg_price);
        println!("    Total Stock: {}", total_stock);
    }
    
    println!("\n  SQL:");
    println!("    SELECT category, COUNT(*), SUM(price), AVG(price)");
    println!("    FROM products GROUP BY category;\n");

    // ============================================================================
    // 12. COMPLEX WITH ORDER AND LIMIT
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("12. Complex Query: WHERE + ORDER BY + LIMIT");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Top 3 Electronics by Rating ---");
    let start = Instant::now();
    let top_electronics = products
        .lock_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::active(), |&a| a)
        .order_by_float_desc(Product::rating());
    let duration = start.elapsed();
    
    println!("  Found top {} electronics in {:?}", top_electronics.len().min(3), duration);
    for (i, p) in top_electronics.iter().take(3).enumerate() {
        println!("    {}. {} - Rating: {:.1} - ${:.2}", i + 1, p.name, p.rating, p.price);
    }
    
    println!("\n  SQL:");
    println!("    SELECT * FROM products");
    println!("    WHERE category = 'Electronics' AND active = true");
    println!("    ORDER BY rating DESC");
    println!("    LIMIT 3;\n");

    // ============================================================================
    // 13. SUMMARY STATISTICS
    // ============================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("13. Summary Statistics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let start = Instant::now();
    
    let total_products = products.lock_query().count();
    let total_value = products.lock_query().sum(Product::price());
    let avg_price = products.lock_query().avg(Product::price()).unwrap_or(0.0);
    let min_price = products.lock_query().min_float(Product::price()).unwrap_or(0.0);
    let max_price = products.lock_query().max_float(Product::price()).unwrap_or(0.0);
    let total_stock = products.lock_query().sum(Product::stock());
    let active_count = products.lock_query().where_(Product::active(), |&a| a).count();
    
    let duration = start.elapsed();
    
    println!("  Catalog Statistics (computed in {:?}):", duration);
    println!("    Total Products: {}", total_products);
    println!("    Active Products: {}", active_count);
    println!("    Total Value: ${:.2}", total_value);
    println!("    Average Price: ${:.2}", avg_price);
    println!("    Price Range: ${:.2} - ${:.2}", min_price, max_price);
    println!("    Total Stock: {} units", total_stock);
    
    println!("\n  SQL:");
    println!("    SELECT COUNT(*), SUM(price), AVG(price), MIN(price), MAX(price), SUM(stock)");
    println!("    FROM products;\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ SQL-Like Lock Queries Demo Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📝 Summary:");
    println!("  ✅ WHERE clauses with key-paths");
    println!("  ✅ SELECT (projection)");
    println!("  ✅ ORDER BY (ASC/DESC)");
    println!("  ✅ GROUP BY");
    println!("  ✅ Aggregations (COUNT, SUM, AVG, MIN, MAX)");
    println!("  ✅ LIMIT");
    println!("  ✅ EXISTS");
    println!("  ✅ FIRST");
    println!("  ✅ Lazy queries with early termination");
    
    println!("\n💡 Key Benefits:");
    println!("  • Full SQL-like syntax on locked data");
    println!("  • No data copying during filtering");
    println!("  • Type-safe key-paths");
    println!("  • Works with Arc<RwLock<T>> and Arc<Mutex<T>>");
    println!("  • Lazy evaluation available");
    println!("  • Early termination support");
    
    println!("\n🎯 Use Cases:");
    println!("  • Thread-safe product catalogs");
    println!("  • Shared user session stores");
    println!("  • Real-time inventory systems");
    println!("  • Configuration caches");
    println!("  • Multi-threaded data processing");
}

