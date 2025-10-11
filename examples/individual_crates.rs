// Example: Using Individual Crates (rust-queries-core + rust-queries-derive)
//
// This example demonstrates how to use the individual crates directly
// instead of the umbrella rust-queries-builder crate.
//
// Cargo.toml:
// [dependencies]
// rust-queries-core = "0.6.0"
// rust-queries-derive = "0.6.0"
// key-paths-core = "1.0.1"
// key-paths-derive = "0.5.0"

// Import from individual crates
use rust_queries_core::{Query, QueryExt};  // Core functionality (LazyQuery available via QueryExt)
use rust_queries_derive::QueryBuilder;      // Derive macro
use key_paths_derive::Keypaths;             // Key-paths

#[derive(Debug, Clone, Keypaths, QueryBuilder)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

fn main() {
    println!("Individual Crates Example");
    println!("=========================\n");
    println!("Using rust-queries-core + rust-queries-derive directly\n");

    let products = vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
            stock: 5,
        },
        Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
            stock: 50,
        },
        Product {
            id: 3,
            name: "Keyboard".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
            stock: 30,
        },
        Product {
            id: 4,
            name: "Monitor".to_string(),
            price: 299.99,
            category: "Electronics".to_string(),
            stock: 12,
        },
        Product {
            id: 5,
            name: "Desk Chair".to_string(),
            price: 199.99,
            category: "Furniture".to_string(),
            stock: 8,
        },
    ];

    println!("1. QueryExt from rust_queries_core");
    println!("   products.query().where_(price > 100).all()");
    
    let query = products
        .query()  // From QueryExt trait
        .where_(Product::price_r(), |&p| p > 100.0);
    let expensive = query.all();
    
    println!("   Found {} expensive products:", expensive.len());
    for product in expensive {
        println!("   - {} (${:.2})", product.name, product.price);
    }
    println!();

    println!("2. QueryBuilder from rust_queries_derive");
    println!("   Product::query(&products).where_(stock < 10).all()");
    
    let query2 = Product::query(&products)  // From QueryBuilder derive
        .where_(Product::stock_r(), |&s| s < 10);
    let low_stock = query2.all();
    
    println!("   Found {} low stock products:", low_stock.len());
    for product in low_stock {
        println!("   - {} (stock: {})", product.name, product.stock);
    }
    println!();

    println!("3. LazyQuery from rust_queries_core");
    println!("   products.lazy_query().where_(...).collect()");
    
    let electronics: Vec<_> = products
        .lazy_query()  // From QueryExt trait
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .collect();
    
    println!("   Found {} electronics:", electronics.len());
    for product in electronics {
        println!("   - {}", product.name);
    }
    println!();

    println!("4. Aggregations with LazyQuery");
    println!("   products.lazy_query().sum_by(Product::price_r())");
    
    let total_value: f64 = products
        .lazy_query()
        .sum_by(Product::price_r());
    
    println!("   Total inventory value: ${:.2}", total_value);
    println!();

    println!("5. Early termination with lazy queries");
    println!("   products.lazy_query().where_(...).first()");
    
    if let Some(first_cheap) = products
        .lazy_query()
        .where_(Product::price_r(), |&p| p < 50.0)
        .first()
    {
        println!("   First cheap product: {} (${:.2})", first_cheap.name, first_cheap.price);
    }
    println!();

    println!("6. Using Query (eager) from rust_queries_core");
    println!("   Query::new(&products).where_(...).count()");
    
    let query3 = Query::new(&products)  // Traditional approach
        .where_(Product::price_r(), |&p| p > 50.0);
    let count = query3.count();
    
    println!("   Products over $50: {}", count);
    println!();

    println!("Summary:");
    println!("--------");
    println!("✓ rust_queries_core provides: Query, LazyQuery, QueryExt");
    println!("✓ rust_queries_derive provides: #[derive(QueryBuilder)]");
    println!("✓ key_paths_derive provides: #[derive(Keypaths)]");
    println!("✓ All features work with individual crates!");
}

