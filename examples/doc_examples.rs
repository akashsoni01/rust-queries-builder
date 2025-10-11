// Test that all documentation examples compile correctly
// cargo run --example doc_examples

use rust_queries_builder::{Query, JoinQuery};
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

#[derive(Debug, Clone, Keypaths)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, Keypaths)]
struct Order {
    id: u32,
    user_id: u32,
    total: f64,
}

fn main() {
    println!("Testing documentation examples...\n");

    // Example from README - Quick Start
    println!("Test 1: README Quick Start Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.0 },
            Product { id: 3, name: "Desk".to_string(), price: 299.99, category: "Furniture".to_string(), stock: 10, rating: 4.8 },
        ];

        let affordable_query = Query::new(&products)
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .where_(Product::price_r(), |&price| price < 100.0);
        let affordable_electronics = affordable_query.all();

        println!("  Found {} affordable electronics ✅", affordable_electronics.len());
    }

    // Example from README - Filtering
    println!("\nTest 2: Filtering Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
        ];

        let electronics_query = Query::new(&products)
            .where_(Product::category_r(), |cat| cat == "Electronics");
        let electronics = electronics_query.all();

        println!("  Found {} electronics ✅", electronics.len());
    }

    // Example from README - Selecting
    println!("\nTest 3: Selecting Fields Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.0 },
        ];

        let names: Vec<String> = Query::new(&products)
            .select(Product::name_r());

        println!("  Selected {} names ✅", names.len());
    }

    // Example from README - Ordering
    println!("\nTest 4: Ordering Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.0 },
        ];

        let by_price = Query::new(&products).order_by_float(Product::price_r());
        println!("  Ordered {} products ✅", by_price.len());
    }

    // Example from README - Aggregations
    println!("\nTest 5: Aggregations Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.0 },
        ];

        let electronics_query = Query::new(&products)
            .where_(Product::category_r(), |cat| cat == "Electronics");

        let count = electronics_query.count();
        let total_value: f64 = electronics_query.sum(Product::price_r());
        let avg_price = electronics_query.avg(Product::price_r()).unwrap_or(0.0);

        println!("  Count: {}, Total: ${:.2}, Avg: ${:.2} ✅", count, total_value, avg_price);
    }

    // Example from README - Grouping
    println!("\nTest 6: Grouping Example");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.0 },
            Product { id: 3, name: "Desk".to_string(), price: 299.99, category: "Furniture".to_string(), stock: 10, rating: 4.8 },
        ];

        let by_category = Query::new(&products).group_by(Product::category_r());
        println!("  Grouped into {} categories ✅", by_category.len());
    }

    // Example from README - Pagination
    println!("\nTest 7: Pagination Example");
    {
        let products = vec![
            Product { id: 1, name: "P1".to_string(), price: 10.0, category: "A".to_string(), stock: 1, rating: 4.0 },
            Product { id: 2, name: "P2".to_string(), price: 20.0, category: "A".to_string(), stock: 1, rating: 4.0 },
            Product { id: 3, name: "P3".to_string(), price: 30.0, category: "A".to_string(), stock: 1, rating: 4.0 },
        ];

        let query = Query::new(&products);
        let first_10 = query.limit(10);
        let page_1 = query.skip(0).limit(10);

        println!("  Limited to {} products ✅", first_10.len());
        println!("  Page 1 has {} products ✅", page_1.len());
    }

    // Example from README - Join
    println!("\nTest 8: Join Example");
    {
        let users = vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ];

        let orders = vec![
            Order { id: 101, user_id: 1, total: 99.99 },
            Order { id: 102, user_id: 1, total: 149.99 },
        ];

        let user_orders = JoinQuery::new(&users, &orders).inner_join(
            User::id_r(),
            Order::user_id_r(),
            |user, order| (user.name.clone(), order.total),
        );

        println!("  Joined {} user-order pairs ✅", user_orders.len());
    }

    // Example from SQL_COMPARISON - SELECT with WHERE
    println!("\nTest 9: SQL Comparison - SELECT with WHERE");
    {
        #[derive(Clone, Keypaths)]
        struct Employee {
            department: String,
        }

        let employees = vec![
            Employee { department: "Engineering".to_string() },
            Employee { department: "Sales".to_string() },
        ];

        let engineering_query = Query::new(&employees)
            .where_(Employee::department_r(), |dept| dept == "Engineering");
        let engineering = engineering_query.all();

        println!("  Found {} engineering employees ✅", engineering.len());
    }

    // Example from USAGE.md - Complex Filtering
    println!("\nTest 10: USAGE - Complex Filtering");
    {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.5 },
        ];

        let results_query = Query::new(&products)
            .where_(Product::category_r(), |cat| cat == "Electronics")
            .where_(Product::price_r(), |&price| price >= 100.0 && price <= 500.0)
            .where_(Product::stock_r(), |&stock| stock > 10);
        let results = results_query.order_by_float(Product::price_r());

        println!("  Filtered {} products ✅", results.len());
    }

    println!("\n✅ All documentation examples compile and run successfully!");
}

