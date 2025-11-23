//! Example demonstrating AND/OR operators for functional filter composition
//!
//! This example shows how to use the `and()` and `or()` methods to compose
//! complex filter expressions with explicit logical operators.

use rust_queries_builder::QueryableExt;
use key_paths_derive::Keypath;

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
}

fn main() {
    println!("AND/OR Operators Example");
    println!("=======================\n");

    let products = vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
            stock: 5,
            rating: 4.5,
        },
        Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
            stock: 50,
            rating: 4.0,
        },
        Product {
            id: 3,
            name: "Keyboard".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
            stock: 30,
            rating: 4.8,
        },
        Product {
            id: 4,
            name: "Monitor".to_string(),
            price: 299.99,
            category: "Electronics".to_string(),
            stock: 12,
            rating: 4.2,
        },
        Product {
            id: 5,
            name: "Desk Chair".to_string(),
            price: 199.99,
            category: "Furniture".to_string(),
            stock: 8,
            rating: 4.7,
        },
        Product {
            id: 6,
            name: "Premium Laptop".to_string(),
            price: 1999.99,
            category: "Electronics".to_string(),
            stock: 3,
            rating: 4.9,
        },
    ];

    println!("1. Using AND operator (explicit)");
    println!("   Query: price < 100 AND stock > 10");
    
    let results: Vec<_> = products
        .lazy_query()
        .where_(Product::price(), |&p| p < 100.0)
        .and(Product::stock(), |&s| s > 10)
        .collect();
    
    println!("   Found {} products:", results.len());
    for product in results {
        println!("   - {} (${:.2}, stock: {})", product.name, product.price, product.stock);
    }
    println!();

    println!("2. Using OR operator");
    println!("   Query: price < 50 OR category == 'Furniture'");
    
    let results: Vec<_> = products
        .lazy_query()
        .where_(Product::price(), |&p| p < 50.0)
        .or(Product::category(), |c| c == "Furniture")
        .collect();
    
    println!("   Found {} products:", results.len());
    for product in results {
        println!("   - {} (${:.2}, category: {})", product.name, product.price, product.category);
    }
    println!();

    println!("3. Complex composition: AND then OR");
    println!("   Query: (price < 100 AND stock > 10) OR (category == 'Furniture')");
    
    // Note: This creates two filter groups - first AND group, then OR group
    // Items pass if: (all AND groups pass) OR (any OR group passes)
    let results: Vec<_> = products
        .lazy_query()
        .where_(Product::price(), |&p| p < 100.0)
        .and(Product::stock(), |&s| s > 10)
        .or(Product::category(), |c| c == "Furniture")
        .collect();
    
    println!("   Found {} products:", results.len());
    for product in results {
        println!("   - {} (${:.2}, stock: {}, category: {})", 
                 product.name, product.price, product.stock, product.category);
    }
    println!();

    println!("4. Multiple AND conditions");
    println!("   Query: price < 200 AND stock > 5 AND rating > 4.5");
    
    let results: Vec<_> = products
        .lazy_query()
        .where_(Product::price(), |&p| p < 200.0)
        .and(Product::stock(), |&s| s > 5)
        .and(Product::rating(), |&r| r > 4.5)
        .collect();
    
    println!("   Found {} products:", results.len());
    for product in results {
        println!("   - {} (${:.2}, stock: {}, rating: {:.1})", 
                 product.name, product.price, product.stock, product.rating);
    }
    println!();

    println!("5. Multiple OR conditions");
    println!("   Query: price > 500 OR category == 'Furniture' OR rating > 4.8");
    
    let results: Vec<_> = products
        .lazy_query()
        .where_(Product::price(), |&p| p > 500.0)
        .or(Product::category(), |c| c == "Furniture")
        .or(Product::rating(), |&r| r > 4.8)
        .collect();
    
    println!("   Found {} products:", results.len());
    for product in results {
        println!("   - {} (${:.2}, category: {}, rating: {:.1})", 
                 product.name, product.price, product.category, product.rating);
    }
    println!();

    println!("Summary:");
    println!("--------");
    println!("✓ where_() - adds filter (implicitly AND with previous)");
    println!("✓ and() - explicitly adds AND filter to current AND group");
    println!("✓ or() - explicitly adds OR filter to current OR group (or creates new OR group)");
    println!("✓ Filters are grouped: AND groups require all filters, OR groups require any filter");
    println!("✓ Evaluation: (all AND groups pass) OR (any OR group passes)");
}

