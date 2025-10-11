// Demonstrates advanced query builder with SQL-like operations using keypaths
// This example shows how to:
// 1. Select specific fields (projection)
// 2. Order results by fields (ascending/descending)
// 3. Group by fields with aggregations
// 4. Limit and paginate results
// 5. Compute aggregates (count, sum, avg, min, max)
// 6. Chain complex queries
// cargo run --example advanced_query_builder

use rust_queries_builder::Query;
use key_paths_derive::Keypaths;

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
}

// Helper function to create sample products
fn create_product_catalog() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Laptop Pro".to_string(),
            price: 1299.99,
            category: "Electronics".to_string(),
            stock: 15,
            rating: 4.8,
        },
        Product {
            id: 2,
            name: "Wireless Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
            stock: 50,
            rating: 4.5,
        },
        Product {
            id: 3,
            name: "Mechanical Keyboard".to_string(),
            price: 129.99,
            category: "Electronics".to_string(),
            stock: 30,
            rating: 4.7,
        },
        Product {
            id: 4,
            name: "Office Chair".to_string(),
            price: 299.99,
            category: "Furniture".to_string(),
            stock: 20,
            rating: 4.6,
        },
        Product {
            id: 5,
            name: "Standing Desk".to_string(),
            price: 499.99,
            category: "Furniture".to_string(),
            stock: 10,
            rating: 4.9,
        },
        Product {
            id: 6,
            name: "USB-C Hub".to_string(),
            price: 49.99,
            category: "Electronics".to_string(),
            stock: 100,
            rating: 4.3,
        },
        Product {
            id: 7,
            name: "Monitor 27\"".to_string(),
            price: 349.99,
            category: "Electronics".to_string(),
            stock: 25,
            rating: 4.7,
        },
        Product {
            id: 8,
            name: "Desk Lamp".to_string(),
            price: 39.99,
            category: "Furniture".to_string(),
            stock: 40,
            rating: 4.2,
        },
        Product {
            id: 9,
            name: "Webcam HD".to_string(),
            price: 79.99,
            category: "Electronics".to_string(),
            stock: 35,
            rating: 4.4,
        },
        Product {
            id: 10,
            name: "Bookshelf".to_string(),
            price: 149.99,
            category: "Furniture".to_string(),
            stock: 15,
            rating: 4.5,
        },
    ]
}

fn main() {
    println!("=== Advanced Query Builder Demo ===\n");

    let products = create_product_catalog();
    println!("Total products in catalog: {}\n", products.len());

    // Query 1: Select all product names
    println!("--- Query 1: Select All Product Names ---");
    let names = Query::new(&products).select(Product::name_r());
    println!("Product names ({}):", names.len());
    for name in &names {
        println!("  • {}", name);
    }

    // Query 2: Order by price (ascending)
    println!("\n--- Query 2: Products Ordered by Price (Ascending) ---");
    let ordered = Query::new(&products).order_by_float(Product::price_r());
    for product in ordered.iter().take(5) {
        println!("  • {} - ${:.2}", product.name, product.price);
    }

    // Query 3: Order by rating (descending)
    println!("\n--- Query 3: Top-Rated Products (Descending) ---");
    let top_rated = Query::new(&products).order_by_float_desc(Product::rating_r());
    for product in top_rated.iter().take(5) {
        println!("  • {} - Rating: {:.1}", product.name, product.rating);
    }

    // Query 4: Group by category
    println!("\n--- Query 4: Products Grouped by Category ---");
    let by_category = Query::new(&products).group_by(Product::category_r());
    for (category, items) in &by_category {
        println!("  {}: {} products", category, items.len());
        for item in items {
            println!("    - {} (${:.2})", item.name, item.price);
        }
    }

    // Query 5: Aggregations - Electronics statistics
    println!("\n--- Query 5: Electronics Category Statistics ---");
    let electronics_query = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");

    println!("  Count: {}", electronics_query.count());
    println!("  Total Value: ${:.2}", electronics_query.sum(Product::price_r()));
    println!("  Average Price: ${:.2}", electronics_query.avg(Product::price_r()).unwrap_or(0.0));
    println!("  Min Price: ${:.2}", electronics_query.min_float(Product::price_r()).unwrap_or(0.0));
    println!("  Max Price: ${:.2}", electronics_query.max_float(Product::price_r()).unwrap_or(0.0));
    println!("  Total Stock: {}", electronics_query.sum(Product::stock_r()));

    // Query 6: Complex filtering with ordering
    println!("\n--- Query 6: Electronics Under $200, Ordered by Rating ---");
    let affordable_electronics = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&price| price < 200.0)
        .order_by_float_desc(Product::rating_r());

    for product in &affordable_electronics {
        println!(
            "  • {} - ${:.2} - Rating: {:.1}",
            product.name, product.price, product.rating
        );
    }

    // Query 7: Limit results
    println!("\n--- Query 7: First 3 Products ---");
    let query7 = Query::new(&products);
    let first_three = query7.limit(3);
    for product in &first_three {
        println!("  • {} (ID: {})", product.name, product.id);
    }

    // Query 8: Pagination
    println!("\n--- Query 8: Pagination (Page 2, 3 items per page) ---");
    let query8 = Query::new(&products);
    let page_2 = query8.skip(3).limit(3);
    for product in &page_2 {
        println!("  • {} (ID: {})", product.name, product.id);
    }

    // Query 9: First matching item
    println!("\n--- Query 9: Find First Product Over $1000 ---");
    let query9 = Query::new(&products)
        .where_(Product::price_r(), |&price| price > 1000.0);
    let expensive = query9.first();

    if let Some(product) = expensive {
        println!("  Found: {} - ${:.2}", product.name, product.price);
    } else {
        println!("  No products found over $1000");
    }

    // Query 10: Check existence
    println!("\n--- Query 10: Check if Any Furniture Exists ---");
    let has_furniture = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Furniture")
        .exists();
    println!("  Furniture available: {}", has_furniture);

    // Query 11: Multiple aggregations by group
    println!("\n--- Query 11: Category Statistics ---");
    let grouped = Query::new(&products).group_by(Product::category_r());

    for (category, items) in &grouped {
        let cat_query = Query::new(items);
        println!("\n  {} Statistics:", category);
        println!("    Products: {}", items.len());
        println!("    Total Value: ${:.2}", cat_query.sum(Product::price_r()));
        println!("    Avg Price: ${:.2}", cat_query.avg(Product::price_r()).unwrap_or(0.0));
        println!("    Total Stock: {}", cat_query.sum(Product::stock_r()));
        println!("    Avg Rating: {:.2}", cat_query.avg(Product::rating_r()).unwrap_or(0.0));
    }

    // Query 12: Complex multi-stage query
    println!("\n--- Query 12: Top 3 Highly-Rated Products (Rating > 4.5) by Price ---");
    let top_products = Query::new(&products)
        .where_(Product::rating_r(), |&rating| rating > 4.5)
        .order_by_float_desc(Product::price_r());

    for (i, product) in top_products.iter().take(3).enumerate() {
        println!(
            "  {}. {} - ${:.2} - Rating: {:.1}",
            i + 1,
            product.name,
            product.price,
            product.rating
        );
    }

    // Query 13: Select multiple fields (simulated with tuples)
    println!("\n--- Query 13: Select Name and Price for Electronics ---");
    let query13 = Query::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let electronics = query13.all();

    for product in electronics {
        println!("  • {} - ${:.2}", product.name, product.price);
    }

    // Query 14: Stock analysis
    println!("\n--- Query 14: Low Stock Alert (Stock < 20) ---");
    let low_stock = Query::new(&products)
        .where_(Product::stock_r(), |&stock| stock < 20)
        .order_by(Product::stock_r());

    for product in &low_stock {
        println!("  ⚠️  {} - Only {} in stock", product.name, product.stock);
    }

    // Query 15: Price range query with multiple conditions
    println!("\n--- Query 15: Mid-Range Products ($50-$300) with Good Ratings (>4.5) ---");
    let mid_range = Query::new(&products)
        .where_(Product::price_r(), |&price| price >= 50.0 && price <= 300.0)
        .where_(Product::rating_r(), |&rating| rating > 4.5)
        .order_by_float(Product::price_r());

    for product in &mid_range {
        println!(
            "  • {} - ${:.2} - Rating: {:.1} - Stock: {}",
            product.name, product.price, product.rating, product.stock
        );
    }

    // Query 16: Revenue calculation
    println!("\n--- Query 16: Potential Revenue by Category ---");
    let by_category = Query::new(&products).group_by(Product::category_r());

    for (category, items) in &by_category {
        let revenue: f64 = items.iter().map(|p| p.price * p.stock as f64).sum();
        println!("  {}: ${:.2}", category, revenue);
    }

    println!("\n✓ Advanced query builder demo complete!");
}

