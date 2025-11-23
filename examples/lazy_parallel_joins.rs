//! Example demonstrating lazy and parallel join operations
//!
//! This example shows:
//! 1. Lazy joins with deferred execution and early termination
//! 2. Parallel joins with Rayon for better performance
//! 3. Combining joins with WHERE filters and AND/OR operators
//!
//! Run with: cargo run --example lazy_parallel_joins --features parallel

use rust_queries_builder::*;
use key_paths_derive::Keypath;
use std::time::Instant;

#[derive(Debug, Clone, Keypath)]
struct User {
    id: u32,
    name: String,
    email: String,
    city: String,
    age: u32,
}

#[derive(Debug, Clone, Keypath)]
struct Order {
    id: u32,
    user_id: u32,
    product_id: u32,
    quantity: u32,
    total: f64,
    status: String,
}

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
}

fn create_sample_data() -> (Vec<User>, Vec<Order>, Vec<Product>) {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            city: "New York".to_string(),
            age: 30,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            city: "Los Angeles".to_string(),
            age: 25,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            city: "Chicago".to_string(),
            age: 35,
        },
    ];

    let orders = vec![
        Order {
            id: 1001,
            user_id: 1,
            product_id: 101,
            quantity: 1,
            total: 999.99,
            status: "completed".to_string(),
        },
        Order {
            id: 1002,
            user_id: 1,
            product_id: 102,
            quantity: 2,
            total: 59.98,
            status: "completed".to_string(),
        },
        Order {
            id: 1003,
            user_id: 2,
            product_id: 103,
            quantity: 1,
            total: 199.99,
            status: "pending".to_string(),
        },
        Order {
            id: 1004,
            user_id: 2,
            product_id: 101,
            quantity: 1,
            total: 999.99,
            status: "completed".to_string(),
        },
        Order {
            id: 1005,
            user_id: 3,
            product_id: 102,
            quantity: 3,
            total: 89.97,
            status: "completed".to_string(),
        },
    ];

    let products = vec![
        Product {
            id: 101,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
        },
        Product {
            id: 102,
            name: "Mouse".to_string(),
            price: 29.99,
            category: "Electronics".to_string(),
        },
        Product {
            id: 103,
            name: "Desk Chair".to_string(),
            price: 199.99,
            category: "Furniture".to_string(),
        },
    ];

    (users, orders, products)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║  Lazy and Parallel Join Operations                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let (users, orders, products) = create_sample_data();

    println!("📊 Dataset:");
    println!("  • {} users", users.len());
    println!("  • {} orders", orders.len());
    println!("  • {} products\n", products.len());

    // ============================================================================
    // 1. LAZY INNER JOIN - Deferred Execution
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("1. Lazy Inner Join - Deferred Execution");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Build lazy join (nothing executes yet!)
    let lazy_join = LazyJoinQuery::new(&users, &orders)
        .inner_join_lazy(
            User::id(),
            Order::user_id(),
            |user, order| (user.name.clone(), order.id, order.total),
        );

    println!("  ✅ Lazy join built (no execution yet)");

    // Early termination - only take first 3 results
    let first_3: Vec<_> = lazy_join.take(3).collect();
    println!("  📊 First 3 user-order pairs:");
    for (name, order_id, total) in &first_3 {
        println!("    • {} - Order #{} - ${:.2}", name, order_id, total);
    }
    println!();

    // ============================================================================
    // 2. LAZY LEFT JOIN - All Users with Optional Orders
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("2. Lazy Left Join - All Users with Optional Orders");
    println!("═══════════════════════════════════════════════════════════════\n");

    let lazy_left_join = LazyJoinQuery::new(&users, &orders)
        .left_join_lazy(
            User::id(),
            Order::user_id(),
            |user, order| match order {
                Some(o) => format!("{} has order #{} (${:.2})", user.name, o.id, o.total),
                None => format!("{} has no orders", user.name),
            },
        );

    let all_users: Vec<_> = lazy_left_join.collect();
    println!("  📊 All users with their orders:");
    for info in &all_users {
        println!("    • {}", info);
    }
    println!();

    // ============================================================================
    // 3. PARALLEL INNER JOIN - Rayon-Powered
    // ============================================================================
    #[cfg(feature = "parallel")]
    {
        println!("═══════════════════════════════════════════════════════════════");
        println!("3. Parallel Inner Join - Rayon-Powered");
        println!("═══════════════════════════════════════════════════════════════\n");

        let start = Instant::now();
        let parallel_results: Vec<_> = JoinQuery::new(&users, &orders)
            .inner_join_parallel(
                User::id(),
                Order::user_id(),
                |user, order| (user.name.clone(), order.id, order.total),
            );
        let par_time = start.elapsed();

        println!("  📊 Parallel join results ({} pairs):", parallel_results.len());
        for (name, order_id, total) in &parallel_results {
            println!("    • {} - Order #{} - ${:.2}", name, order_id, total);
        }
        println!("  ⏱️  Time: {:?}\n", par_time);
    }

    // ============================================================================
    // 4. PARALLEL LEFT JOIN
    // ============================================================================
    #[cfg(feature = "parallel")]
    {
        println!("═══════════════════════════════════════════════════════════════");
        println!("4. Parallel Left Join");
        println!("═══════════════════════════════════════════════════════════════\n");

        let start = Instant::now();
        let parallel_left: Vec<_> = JoinQuery::new(&users, &orders)
            .left_join_parallel(
                User::id(),
                Order::user_id(),
                |user, order| match order {
                    Some(o) => format!("{} has order #{}", user.name, o.id),
                    None => format!("{} has no orders", user.name),
                },
            );
        let par_time = start.elapsed();

        println!("  📊 Parallel left join results:");
        for info in &parallel_left {
            println!("    • {}", info);
        }
        println!("  ⏱️  Time: {:?}\n", par_time);
    }

    // ============================================================================
    // 5. PARALLEL JOIN WITH WHERE FILTER
    // ============================================================================
    #[cfg(feature = "parallel")]
    {
        println!("═══════════════════════════════════════════════════════════════");
        println!("5. Parallel Join with WHERE Filter");
        println!("═══════════════════════════════════════════════════════════════\n");

        // Join with filter: only high-value orders (> $100)
        let start = Instant::now();
        let high_value: Vec<_> = JoinQuery::new(&users, &orders)
            .inner_join_where_parallel(
                User::id(),
                Order::user_id(),
                |_user, order| order.total > 100.0,
                |user, order| (user.name.clone(), order.total),
            );
        let par_time = start.elapsed();

        println!("  Query: Users with orders > $100");
        println!("  📊 Results:");
        for (name, total) in &high_value {
            println!("    • {} - ${:.2}", name, total);
        }
        println!("  ⏱️  Time: {:?}\n", par_time);
    }

    // ============================================================================
    // 6. JOIN + LAZY QUERY WITH AND/OR
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("6. Join + Lazy Query with AND/OR Operators");
    println!("═══════════════════════════════════════════════════════════════\n");

    // First, join orders with products
    let orders_with_products: Vec<_> = JoinQuery::new(&orders, &products)
        .inner_join(
            Order::product_id(),
            Product::id(),
            |order, product| (order.clone(), product.clone()),
        );

    println!("  ✅ Joined {} orders with products", orders_with_products.len());

    // Now query the joined results with AND/OR
    // Create a simple struct to hold the joined data
    #[derive(Debug, Clone, Keypath)]
    struct OrderProductPair {
        order_id: u32,
        order_total: f64,
        order_status: String,
        product_name: String,
        product_category: String,
    }

    let order_products: Vec<OrderProductPair> = orders_with_products
        .into_iter()
        .map(|(order, product)| OrderProductPair {
            order_id: order.id,
            order_total: order.total,
            order_status: order.status,
            product_name: product.name,
            product_category: product.category,
        })
        .collect();

    let filtered: Vec<_> = order_products
        .lazy_query()
        .where_(OrderProductPair::order_status(), |s| s == "completed")
        .and(OrderProductPair::order_total(), |&t| t > 50.0)
        .or(OrderProductPair::product_category(), |c| c == "Furniture")
        .collect();

    println!("  Query: (status == 'completed' AND total > 50) OR (category == 'Furniture')");
    println!("  📊 Results:");
    for op in &filtered {
        println!(
            "    • Order #{}: {} (${:.2}, {})",
            op.order_id, op.product_name, op.order_total, op.product_category
        );
    }
    println!();

    // ============================================================================
    // 7. JOIN + PARALLEL QUERY WITH AND/OR
    // ============================================================================
    #[cfg(feature = "parallel")]
    {
        println!("═══════════════════════════════════════════════════════════════");
        println!("7. Join + Parallel Query with AND/OR Operators");
        println!("═══════════════════════════════════════════════════════════════\n");

        // Re-create order_products for parallel query
        let (_, orders2, products2) = create_sample_data();
        let orders_with_products2: Vec<_> = JoinQuery::new(&orders2, &products2)
            .inner_join(
                Order::product_id(),
                Product::id(),
                |order, product| (order.clone(), product.clone()),
            );

        let order_products2: Vec<OrderProductPair> = orders_with_products2
            .into_iter()
            .map(|(order, product)| OrderProductPair {
                order_id: order.id,
                order_total: order.total,
                order_status: order.status,
                product_name: product.name,
                product_category: product.category,
            })
            .collect();

        let start = Instant::now();
        let filtered_par: Vec<_> = order_products2
            .lazy_parallel_query()
            .where_(OrderProductPair::order_status(), |s| s == "completed")
            .and(OrderProductPair::order_total(), |&t| t > 50.0)
            .or(OrderProductPair::product_category(), |c| c == "Furniture")
            .collect_parallel();
        let par_time = start.elapsed();

        println!("  Query: (status == 'completed' AND total > 50) OR (category == 'Furniture')");
        println!("  📊 Parallel Results:");
        for op in &filtered_par {
            println!(
                "    • Order #{}: {} (${:.2}, {})",
                op.order_id, op.product_name, op.order_total, op.product_category
            );
        }
        println!("  ⏱️  Time: {:?}\n", par_time);
    }

    // ============================================================================
    // SUMMARY
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Successfully demonstrated:");
    println!("   • Lazy joins with deferred execution and early termination");
    println!("   • Parallel joins with Rayon for better performance");
    println!("   • Join + WHERE filters with AND/OR operators");
    println!("   • Join + Parallel queries with AND/OR operators");
    println!();
    println!("🎯 Key Features:");
    println!("   • LazyJoinQuery::inner_join_lazy() - Returns iterator");
    println!("   • LazyJoinQuery::left_join_lazy() - Returns iterator");
    println!("   • JoinQuery::inner_join_parallel() - Parallel processing");
    println!("   • JoinQuery::left_join_parallel() - Parallel processing");
    println!("   • JoinQuery::inner_join_where_parallel() - Parallel with filter");
    println!();
    println!("💡 Use Cases:");
    println!("   • Lazy joins: Early termination, memory efficiency");
    println!("   • Parallel joins: Large datasets (10,000+ items)");
    println!("   • Combined: Join + filter with complex AND/OR logic");
}

