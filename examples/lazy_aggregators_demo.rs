//! Example: Lazy Lock Query Aggregators and SQL Functions
//!
//! Demonstrates all aggregation functions and SQL-like operations
//! available in LockLazyQuery for efficient data analysis.
//!
//! Features demonstrated:
//! 1. Aggregators: sum, avg, min, max, min_float, max_float
//! 2. SQL functions: exists, limit, skip, distinct
//! 3. Advanced: last, nth, all_match, find, count_where
//!
//! cargo run --example lazy_aggregators_demo --release

use rust_queries_builder::{LockQueryable, LockLazyQueryable};
use key_paths_derive::Keypaths;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    stock: u32,
    category: String,
    rating: f64,
}

#[derive(Debug, Clone, Keypaths)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    total: f64,
    status: String,
}

fn create_products() -> HashMap<String, Arc<RwLock<Product>>> {
    let mut products = HashMap::new();
    
    products.insert("p1".to_string(), Arc::new(RwLock::new(Product {
        id: 1,
        name: "Laptop Pro".to_string(),
        price: 1299.99,
        stock: 15,
        category: "Electronics".to_string(),
        rating: 4.8,
    })));
    
    products.insert("p2".to_string(), Arc::new(RwLock::new(Product {
        id: 2,
        name: "Wireless Mouse".to_string(),
        price: 29.99,
        stock: 50,
        category: "Electronics".to_string(),
        rating: 4.5,
    })));
    
    products.insert("p3".to_string(), Arc::new(RwLock::new(Product {
        id: 3,
        name: "Mechanical Keyboard".to_string(),
        price: 149.99,
        stock: 30,
        category: "Electronics".to_string(),
        rating: 4.7,
    })));
    
    products.insert("p4".to_string(), Arc::new(RwLock::new(Product {
        id: 4,
        name: "Desk Chair".to_string(),
        price: 299.99,
        stock: 8,
        category: "Furniture".to_string(),
        rating: 4.3,
    })));
    
    products.insert("p5".to_string(), Arc::new(RwLock::new(Product {
        id: 5,
        name: "Standing Desk".to_string(),
        price: 599.99,
        stock: 12,
        category: "Furniture".to_string(),
        rating: 4.6,
    })));
    
    products.insert("p6".to_string(), Arc::new(RwLock::new(Product {
        id: 6,
        name: "Monitor 4K".to_string(),
        price: 399.99,
        stock: 0,
        category: "Electronics".to_string(),
        rating: 4.9,
    })));
    
    products.insert("p7".to_string(), Arc::new(RwLock::new(Product {
        id: 7,
        name: "USB-C Hub".to_string(),
        price: 49.99,
        stock: 100,
        category: "Electronics".to_string(),
        rating: 4.2,
    })));
    
    products.insert("p8".to_string(), Arc::new(RwLock::new(Product {
        id: 8,
        name: "Office Lamp".to_string(),
        price: 79.99,
        stock: 25,
        category: "Furniture".to_string(),
        rating: 4.4,
    })));
    
    products
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Lazy Lock Query - Aggregators & SQL Functions Demo        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    
    let products = create_products();
    println!("Created {} products\n", products.len());
    
    // ========================================================================
    // PART 1: AGGREGATION FUNCTIONS
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: Aggregation Functions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("--- [1] SUM - Calculate total values ---");
    
    // Sum of all prices
    let total_value: f64 = products
        .lock_lazy_query()
        .sum(Product::price_r());
    println!("  Total inventory value: ${:.2}", total_value);
    
    // Sum of electronics prices only
    let electronics_value: f64 = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .sum(Product::price_r());
    println!("  Electronics value: ${:.2}", electronics_value);
    
    // Sum of stock quantities
    let total_stock: u32 = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s > 0)
        .sum(Product::stock_r());
    println!("  Total stock units: {}", total_stock);
    println!("  SQL: SELECT SUM(stock) FROM products WHERE stock > 0;\n");
    
    println!("--- [2] AVG - Calculate averages ---");
    
    // Average price of all products
    let avg_price = products
        .lock_lazy_query()
        .avg(Product::price_r());
    println!("  Average product price: ${:.2}", avg_price.unwrap_or(0.0));
    
    // Average rating for electronics
    let avg_electronics_rating = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .avg(Product::rating_r());
    println!("  Average electronics rating: {:.2}", avg_electronics_rating.unwrap_or(0.0));
    
    // Average price of in-stock items
    let avg_in_stock = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s > 0)
        .avg(Product::price_r());
    println!("  Average price (in stock): ${:.2}", avg_in_stock.unwrap_or(0.0));
    println!("  SQL: SELECT AVG(price) FROM products WHERE stock > 0;\n");
    
    println!("--- [3] MIN / MAX - Find extremes ---");
    
    // Minimum and maximum prices
    let min_price = products
        .lock_lazy_query()
        .min_float(Product::price_r());
    let max_price = products
        .lock_lazy_query()
        .max_float(Product::price_r());
    println!("  Price range: ${:.2} - ${:.2}", min_price.unwrap_or(0.0), max_price.unwrap_or(0.0));
    
    // Minimum stock level
    let min_stock = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s > 0)
        .min(Product::stock_r());
    println!("  Minimum stock level (>0): {:?} units", min_stock);
    
    // Highest rated product
    let highest_rating = products
        .lock_lazy_query()
        .max_float(Product::rating_r());
    println!("  Highest rating: {:.1}", highest_rating.unwrap_or(0.0));
    println!("  SQL: SELECT MIN(stock), MAX(rating) FROM products;\n");
    
    // ========================================================================
    // PART 2: SQL-LIKE FUNCTIONS
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: SQL-like Functions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("--- [1] EXISTS - Check existence ---");
    
    // Check if expensive items exist
    let has_expensive = products
        .lock_lazy_query()
        .where_(Product::price_r(), |&p| p > 1000.0)
        .exists();
    println!("  Expensive items (>$1000): {}", if has_expensive { "Yes ✓" } else { "No ✗" });
    
    // Check if out-of-stock items exist
    let has_out_of_stock = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s == 0)
        .exists();
    println!("  Out of stock items: {}", if has_out_of_stock { "Yes ✓" } else { "No ✗" });
    println!("  SQL: SELECT EXISTS(SELECT 1 FROM products WHERE stock = 0);\n");
    
    println!("--- [2] LIMIT / SKIP - Pagination ---");
    
    // Get top 3 most expensive products
    let top_3: Vec<_> = products
        .lock_lazy_query()
        .limit(3)
        .collect();
    println!("  Top 3 products (first 3):");
    for p in &top_3 {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    
    // Pagination: Get page 2 (skip 3, take 3)
    let page_2: Vec<_> = products
        .lock_lazy_query()
        .skip(3)
        .limit(3)
        .collect();
    println!("\n  Page 2 (skip 3, take 3):");
    for p in &page_2 {
        println!("    • {} - ${:.2}", p.name, p.price);
    }
    println!("  SQL: SELECT * FROM products LIMIT 3 OFFSET 3;\n");
    
    println!("--- [3] DISTINCT - Get unique values ---");
    
    // Get all unique categories
    let categories: Vec<String> = products
        .lock_lazy_query()
        .distinct(Product::category_r());
    println!("  Unique categories: {:?}", categories);
    
    // Get unique categories for in-stock items only
    let in_stock_categories: Vec<String> = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s > 0)
        .distinct(Product::category_r());
    println!("  Categories (in stock): {:?}", in_stock_categories);
    println!("  SQL: SELECT DISTINCT category FROM products WHERE stock > 0;\n");
    
    // ========================================================================
    // PART 3: ADVANCED FUNCTIONS
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: Advanced Functions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("--- [1] FIRST / LAST / NTH ---");
    
    // Get first product
    let first = products
        .lock_lazy_query()
        .first();
    if let Some(p) = first {
        println!("  First product: {} - ${:.2}", p.name, p.price);
    }
    
    // Get last product (note: less efficient for lazy queries)
    let last = products
        .lock_lazy_query()
        .last();
    if let Some(p) = last {
        println!("  Last product: {} - ${:.2}", p.name, p.price);
    }
    
    // Get 3rd product (0-indexed, so nth(2))
    let third = products
        .lock_lazy_query()
        .nth(2);
    if let Some(p) = third {
        println!("  Third product: {} - ${:.2}", p.name, p.price);
    }
    println!();
    
    println!("--- [2] ALL_MATCH - Verify conditions ---");
    
    // Check if all electronics have rating > 4.0
    let all_well_rated = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .all_match(Product::rating_r(), |&r| r > 4.0);
    println!("  All electronics well-rated (>4.0): {}", 
             if all_well_rated { "Yes ✓" } else { "No ✗" });
    
    // Check if all products are in stock
    let all_in_stock = products
        .lock_lazy_query()
        .all_match(Product::stock_r(), |&s| s > 0);
    println!("  All products in stock: {}", 
             if all_in_stock { "Yes ✓" } else { "No ✗" });
    println!();
    
    println!("--- [3] FIND - Search with condition ---");
    
    // Find first expensive electronics item
    let expensive_electronics = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .find(Product::price_r(), |&p| p > 500.0);
    if let Some(p) = expensive_electronics {
        println!("  First expensive electronics: {} - ${:.2}", p.name, p.price);
    }
    
    // Find first highly-rated furniture
    let good_furniture = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Furniture")
        .find(Product::rating_r(), |&r| r > 4.5);
    if let Some(p) = good_furniture {
        println!("  First highly-rated furniture: {} - {:.1}★", p.name, p.rating);
    }
    println!();
    
    println!("--- [4] COUNT_WHERE - Conditional counting ---");
    
    // Count expensive electronics
    let expensive_count = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .count_where(Product::price_r(), |&p| p > 100.0);
    println!("  Expensive electronics (>$100): {}", expensive_count);
    
    // Count well-stocked items
    let well_stocked = products
        .lock_lazy_query()
        .count_where(Product::stock_r(), |&s| s > 20);
    println!("  Well-stocked items (>20 units): {}", well_stocked);
    
    // Count highly-rated products
    let highly_rated = products
        .lock_lazy_query()
        .count_where(Product::rating_r(), |&r| r >= 4.5);
    println!("  Highly-rated products (≥4.5★): {}", highly_rated);
    println!();
    
    // ========================================================================
    // PART 4: COMPLEX QUERY CHAINS
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: Complex Query Chains");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("--- Business Intelligence Queries ---\n");
    
    // Query 1: High-value inventory analysis
    println!("  [Query 1] High-value in-stock electronics:");
    let high_value_electronics = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .where_(Product::stock_r(), |&s| s > 0)
        .where_(Product::price_r(), |&p| p > 100.0);
    
    let hv_count = high_value_electronics.count();
    let hv_total: f64 = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .where_(Product::stock_r(), |&s| s > 0)
        .where_(Product::price_r(), |&p| p > 100.0)
        .sum(Product::price_r());
    let hv_avg = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .where_(Product::stock_r(), |&s| s > 0)
        .where_(Product::price_r(), |&p| p > 100.0)
        .avg(Product::price_r());
    
    println!("    Count: {}", hv_count);
    println!("    Total value: ${:.2}", hv_total);
    println!("    Average price: ${:.2}", hv_avg.unwrap_or(0.0));
    
    // Query 2: Category statistics
    println!("\n  [Query 2] Per-category statistics:");
    for category in &categories {
        let cat_count = products
            .lock_lazy_query()
            .where_(Product::category_r(), |c| c == category)
            .count();
        
        let cat_avg = products
            .lock_lazy_query()
            .where_(Product::category_r(), |c| c == category)
            .avg(Product::price_r())
            .unwrap_or(0.0);
        
        let cat_stock: u32 = products
            .lock_lazy_query()
            .where_(Product::category_r(), |c| c == category)
            .sum(Product::stock_r());
        
        println!("    {}: {} items, avg ${:.2}, {} units in stock", 
                 category, cat_count, cat_avg, cat_stock);
    }
    
    // Query 3: Premium products analysis
    println!("\n  [Query 3] Premium products (price > $200, rating > 4.5):");
    let premium_count = products
        .lock_lazy_query()
        .where_(Product::price_r(), |&p| p > 200.0)
        .count_where(Product::rating_r(), |&r| r > 4.5);
    
    let premium_exists = products
        .lock_lazy_query()
        .where_(Product::price_r(), |&p| p > 200.0)
        .find(Product::rating_r(), |&r| r > 4.5);
    
    println!("    Count: {}", premium_count);
    if let Some(p) = premium_exists {
        println!("    Example: {} - ${:.2} - {:.1}★", p.name, p.price, p.rating);
    }
    
    // ========================================================================
    // Summary
    // ========================================================================
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ Demo Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("📊 Aggregation Functions Demonstrated:");
    println!("  ✅ sum() - Calculate totals");
    println!("  ✅ avg() - Calculate averages");
    println!("  ✅ min() / max() - Find extremes (Ord)");
    println!("  ✅ min_float() / max_float() - Find extremes (f64)");
    
    println!("\n🔍 SQL-like Functions Demonstrated:");
    println!("  ✅ exists() - Check existence (early termination)");
    println!("  ✅ limit() - Limit results");
    println!("  ✅ skip() - Skip results (pagination)");
    println!("  ✅ distinct() - Get unique values");
    
    println!("\n⚡ Advanced Functions Demonstrated:");
    println!("  ✅ first() / last() / nth() - Positional access");
    println!("  ✅ all_match() - Verify all items match");
    println!("  ✅ find() - Search with condition");
    println!("  ✅ count_where() - Conditional counting");
    
    println!("\n💡 Key Benefits:");
    println!("  • Lazy evaluation - only processes what's needed");
    println!("  • Early termination - stops at first match for exists/find");
    println!("  • Memory efficient - no intermediate collections");
    println!("  • Type-safe - compile-time checking");
    println!("  • Zero-copy where possible - works with locks directly");
    
    println!("\n🎯 Perfect for:");
    println!("  • Real-time analytics on locked data");
    println!("  • Large datasets with filtering");
    println!("  • Efficient existence checks");
    println!("  • Statistical analysis");
    println!("  • Business intelligence queries");
}

