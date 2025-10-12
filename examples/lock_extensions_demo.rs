//! Example: Lock Extensions Demo
//!
//! Demonstrates using parking_lot and tokio lock extensions with:
//! 1. Direct .lock_query() and .lock_lazy_query() calls
//! 2. JOIN operations with extension traits
//! 3. Selecting limited fields with select_lazy
//! 4. Using .all() method on lazy queries
//!
//! To run this example:
//! cargo run --example lock_extensions_demo --features parking_lot
//! 
//! Or with tokio (requires tokio runtime):
//! cargo run --example lock_extensions_demo --features tokio

#[cfg(feature = "parking_lot")]
use rust_queries_builder::lock_ext::{
    ParkingLotRwLockWrapper, ParkingLotQueryExt, ParkingLotJoinExt
};

use key_paths_derive::Keypaths;
use std::collections::HashMap;

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

#[derive(Debug, Clone, Keypaths)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    total: f64,
}

#[cfg(feature = "parking_lot")]
fn demo_parking_lot() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║       parking_lot Lock Extensions Demo                  ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Create test data with parking_lot locks
    let mut products: HashMap<String, ParkingLotRwLockWrapper<Product>> = HashMap::new();
    
    products.insert("p1".to_string(), ParkingLotRwLockWrapper::new(Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 999.99,
        category: "Electronics".to_string(),
        stock: 15,
    }));
    
    products.insert("p2".to_string(), ParkingLotRwLockWrapper::new(Product {
        id: 2,
        name: "Mouse".to_string(),
        price: 29.99,
        category: "Electronics".to_string(),
        stock: 50,
    }));
    
    products.insert("p3".to_string(), ParkingLotRwLockWrapper::new(Product {
        id: 3,
        name: "Desk".to_string(),
        price: 299.99,
        category: "Furniture".to_string(),
        stock: 8,
    }));
    
    products.insert("p4".to_string(), ParkingLotRwLockWrapper::new(Product {
        id: 4,
        name: "Monitor".to_string(),
        price: 399.99,
        category: "Electronics".to_string(),
        stock: 0,
    }));

    println!("Created {} products with parking_lot RwLock\n", products.len());

    // ========================================================================
    // 1. Direct .lock_query() calls
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. Direct .lock_query() Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let electronics = products
        .lock_query()  // Direct method call!
        .where_(Product::category_r(), |c| c == "Electronics")
        .all();
    
    println!("Found {} electronics:", electronics.len());
    for p in &electronics {
        println!("  • {} - ${:.2} (stock: {})", p.name, p.price, p.stock);
    }
    println!();

    // ========================================================================
    // 2. Direct .lock_lazy_query() with .all()
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. Lazy Query with .all() Method");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let available = products
        .lock_lazy_query()  // Direct method call!
        .where_(Product::stock_r(), |&s| s > 0)
        .all();  // New .all() method (alias for collect)
    
    println!("Products in stock ({}):", available.len());
    for p in &available {
        println!("  • {} - {} units", p.name, p.stock);
    }
    println!();

    // ========================================================================
    // 3. Selecting Limited Fields with select_lazy
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. Select Limited Fields (Efficient Projection)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Select only names (not full objects)
    let names: Vec<String> = products
        .lock_lazy_query()
        .where_(Product::price_r(), |&p| p > 100.0)
        .select_lazy(Product::name_r())
        .collect();
    
    println!("Expensive product names only:");
    for name in &names {
        println!("  • {}", name);
    }
    println!("💡 Only cloned String fields, not full Product objects!\n");

    // Select only IDs
    let ids: Vec<u32> = products
        .lock_lazy_query()
        .where_(Product::stock_r(), |&s| s > 10)
        .select_lazy(Product::id_r())
        .collect();
    
    println!("IDs of well-stocked products: {:?}", ids);
    println!("💡 Only cloned u32 fields!\n");

    // Select prices and compute sum
    let total: f64 = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .select_lazy(Product::price_r())
        .sum();
    
    println!("Total value of electronics: ${:.2}", total);
    println!("💡 Computed without cloning full objects!\n");

    // ========================================================================
    // 4. JOIN Operations with Extension Traits
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. JOIN Operations with Extension Traits");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut orders: HashMap<String, ParkingLotRwLockWrapper<Order>> = HashMap::new();
    
    orders.insert("o1".to_string(), ParkingLotRwLockWrapper::new(Order {
        id: 101,
        product_id: 1,
        quantity: 2,
        total: 1999.98,
    }));
    
    orders.insert("o2".to_string(), ParkingLotRwLockWrapper::new(Order {
        id: 102,
        product_id: 2,
        quantity: 5,
        total: 149.95,
    }));
    
    orders.insert("o3".to_string(), ParkingLotRwLockWrapper::new(Order {
        id: 103,
        product_id: 1,
        quantity: 1,
        total: 999.99,
    }));

    // INNER JOIN using extension trait
    println!("INNER JOIN: Products with their orders");
    let results = products
        .lock_join(&orders)  // Direct .lock_join() call!
        .inner_join(
            Product::id_r(),
            Order::product_id_r(),
            |product, order| {
                (product.name.clone(), order.quantity, order.total)
            }
        );
    
    println!("Found {} product-order pairs:", results.len());
    for (name, qty, total) in &results {
        println!("  • {} - {} units - ${:.2}", name, qty, total);
    }
    println!();

    // LEFT JOIN using extension trait
    println!("LEFT JOIN: All products with optional orders");
    let all_products = products
        .lock_join(&orders)  // Direct .lock_join() call!
        .left_join(
            Product::id_r(),
            Order::product_id_r(),
            |product, order_opt| {
                match order_opt {
                    Some(order) => format!("{} - Ordered: {} units", product.name, order.quantity),
                    None => format!("{} - No orders yet", product.name),
                }
            }
        );
    
    for result in &all_products {
        println!("  • {}", result);
    }
    println!();

    // ========================================================================
    // 5. Complex Query Chains
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. Complex Query Chains");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Chained WHERE clauses with select_lazy
    let expensive_electronics: Vec<String> = products
        .lock_lazy_query()
        .where_(Product::category_r(), |c| c == "Electronics")
        .where_(Product::price_r(), |&p| p > 200.0)
        .where_(Product::stock_r(), |&s| s > 0)
        .select_lazy(Product::name_r())
        .collect();
    
    println!("Expensive electronics in stock:");
    for name in &expensive_electronics {
        println!("  • {}", name);
    }
    println!();

    // Aggregations
    let avg_price = products
        .lock_query()
        .where_(Product::stock_r(), |&s| s > 0)
        .avg(Product::price_r())
        .unwrap_or(0.0);
    
    let total_stock: u32 = products
        .lock_query()
        .sum(Product::stock_r());
    
    println!("Statistics:");
    println!("  • Average price (in stock): ${:.2}", avg_price);
    println!("  • Total stock: {} units", total_stock);
    println!();

    // ========================================================================
    // Summary
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ Demo Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Features Demonstrated:");
    println!("  ✅ Direct .lock_query() calls via extension traits");
    println!("  ✅ Direct .lock_lazy_query() calls");
    println!("  ✅ New .all() method on lazy queries");
    println!("  ✅ select_lazy() for efficient field projection");
    println!("  ✅ JOIN operations via .lock_join()");
    println!("  ✅ INNER JOIN and LEFT JOIN");
    println!("  ✅ Complex query chains");
    println!("  ✅ Aggregations (AVG, SUM)");
    
    println!("\n💡 Key Benefits:");
    println!("  • parking_lot: 10-30% faster than std::sync");
    println!("  • No poisoning overhead");
    println!("  • 8x smaller memory footprint");
    println!("  • Fair unlocking policy");
    println!("  • Seamless query integration");
}

#[cfg(not(feature = "parking_lot"))]
fn demo_parking_lot() {
    println!("\n⚠️  parking_lot feature not enabled!");
    println!("Run with: cargo run --example lock_extensions_demo --features parking_lot\n");
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Lock Extensions Demo - parking_lot & tokio Support       ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    demo_parking_lot();
}

