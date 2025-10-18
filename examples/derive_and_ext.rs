use rust_queries_builder::{QueryExt, QueryBuilder};
use key_paths_derive::Keypath;

#[derive(Debug, Clone, Keypath, QueryBuilder)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

fn main() {
    println!("Derive Macros and Extension Traits Example");
    println!("===========================================\n");

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

    println!("1. Using Extension Trait - Direct .query() on Vec");
    println!("   Query: products.query().where_(price > 100).all()");
    
    let query = products
        .query()
        .where_(Product::price(), |&p| p > 100.0);
    let expensive = query.all();
    
    println!("   Found {} expensive products:", expensive.len());
    for product in &expensive {
        println!("   - {} (${:.2})", product.name, product.price);
    }
    println!();

    println!("2. Using Extension Trait - Direct .lazy_query() on Vec");
    println!("   Query: products.lazy_query().where_(stock < 10).collect()");
    
    let low_stock: Vec<_> = products
        .lazy_query()
        .where_(Product::stock(), |&s| s < 10)
        .collect();
    
    println!("   Found {} low stock products:", low_stock.len());
    for product in &low_stock {
        println!("   - {} (stock: {})", product.name, product.stock);
    }
    println!();

    println!("3. Using Extension Trait - Chained Operations");
    println!("   Query: products.lazy_query().where_(category == Electronics).take(2).select(name).collect()");
    
    let names: Vec<String> = products
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .take_lazy(2)
        .select_lazy(Product::name())
        .collect();
    
    println!("   First 2 electronics:");
    for name in &names {
        println!("   - {}", name);
    }
    println!();

    println!("4. Using QueryBuilder Derive - Static Methods");
    println!("   Query: Product::query(&products).where_(price > 50).count()");
    
    let query4 = Product::query(&products)
        .where_(Product::price(), |&p| p > 50.0);
    let count = query4.count();
    
    println!("   Products over $50: {}", count);
    println!();

    println!("5. Using QueryBuilder Derive - Lazy Static Methods");
    println!("   Query: Product::lazy_query(&products).first()");
    
    if let Some(first) = Product::lazy_query(&products).first() {
        println!("   First product: {} (${:.2})", first.name, first.price);
    }
    println!();

    println!("6. Complex Chain with Extension Trait");
    println!("   Query: products.lazy_query()");
    println!("          .where_(category == Electronics)");
    println!("          .where_(price < 500)");
    println!("          .map(|p| format!(\"{{}} - ${{:.2}}\", p.name, p.price))");
    println!("          .collect()");
    
    let formatted: Vec<String> = products
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p < 500.0)
        .map_items(|p| format!("{} - ${:.2}", p.name, p.price))
        .collect();
    
    println!("   Affordable electronics:");
    for item in &formatted {
        println!("   - {}", item);
    }
    println!();

    println!("7. Aggregation with Extension Trait");
    println!("   Query: products.lazy_query().sum_by(Product::stock())");
    
    let total_stock = products
        .lazy_query()
        .sum_by(Product::stock());
    
    println!("   Total stock across all products: {}", total_stock);
    println!();

    println!("8. Find with Extension Trait");
    println!("   Query: products.lazy_query().find(|p| p.name.contains(\"Chair\"))");
    
    if let Some(chair) = products.lazy_query().find(|p| p.name.contains("Chair")) {
        println!("   Found: {} in {}", chair.name, chair.category);
    }
    println!();

    println!("9. Early Termination with Extension Trait");
    println!("   Query: products.lazy_query().where_(price > 1000).any()");
    
    let has_luxury = products
        .lazy_query()
        .where_(Product::price(), |&p| p > 1000.0)
        .any();
    
    println!("   Has products over $1000: {}", has_luxury);
    println!();

    println!("10. Slice Extension Trait");
    println!("    Query: (&products[..]).lazy_query().count()");
    
    let slice_count = (&products[..])
        .lazy_query()
        .count();
    
    println!("    Count from slice: {}", slice_count);
    println!();

    println!("Summary:");
    println!("--------");
    println!("✓ Extension trait QueryExt adds .query() and .lazy_query() to containers");
    println!("✓ Derive macro QueryBuilder adds static methods Product::query() and Product::lazy_query()");
    println!("✓ Both approaches provide the same query functionality");
    println!("✓ Extension trait is more ergonomic: products.query() vs Query::new(&products)");
    println!("✓ All iterator optimizations (fusion, early termination) still apply");
}

