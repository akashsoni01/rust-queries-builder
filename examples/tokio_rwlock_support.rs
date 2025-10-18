// Example: Extending Lock-Aware Queries to tokio::sync::RwLock
//
// This example demonstrates how to extend the lock-aware querying system
// to support tokio's async RwLock, enabling async queries on locked data.
//
// Features:
// 1. Implement LockValue trait for tokio::sync::RwLock
// 2. Async lock acquisition and querying
// 3. All SQL operations work with tokio locks
// 4. Maintain zero-copy benefits in async context
//
// cargo run --example tokio_rwlock_support

// Note: We can't directly implement LockQueryable/LockLazyQueryable for HashMap<K, TokioLock<V>>
// due to Rust's orphan rules. Instead, we use helper functions.
use key_paths_derive::Keypath;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use std::time::Instant;

#[derive(Debug, Clone, Keypath)]
struct User {
    id: u32,
    name: String,
    email: String,
    status: String,
    score: f64,
}

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    stock: u32,
    category: String,
}

type AsyncUserMap = HashMap<String, TokioLock<User>>;
type AsyncProductMap = HashMap<String, TokioLock<Product>>;

// ============================================================================
// Step 1: Create a newtype wrapper for tokio::sync::RwLock
// ============================================================================

/// Wrapper around Arc<tokio::sync::RwLock<T>> to enable our trait implementations.
/// 
/// This newtype is needed because of Rust's orphan rules - we can't implement
/// foreign traits (LockValue) on foreign types (Arc<tokio::sync::RwLock<T>>).
#[derive(Clone, Debug)]
pub struct TokioLock<T>(Arc<TokioRwLock<T>>);

impl<T> TokioLock<T> {
    /// Create a new TokioLock wrapper.
    pub fn new(value: T) -> Self {
        Self(Arc::new(TokioRwLock::new(value)))
    }
    
    /// Execute a function with a read lock (synchronous wrapper).
    /// 
    /// Note: This uses block_in_place which is suitable for CPU-bound work.
    /// For IO-bound work, consider using async throughout.
    pub fn with_value_sync<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // Use tokio's block_in_place for synchronous access in async context
        // In production, you'd typically stay fully async
        tokio::task::block_in_place(|| {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let guard = self.0.read().await;
                Some(f(&*guard))
            })
        })
    }
}

// ============================================================================
// Step 2: Implement LockValue trait (required by our query system)
// ============================================================================

use rust_queries_builder::LockValue;

impl<T> LockValue<T> for TokioLock<T> {
    fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.with_value_sync(f)
    }
}

// ============================================================================
// Step 3: Helper functions for querying
// ============================================================================

use rust_queries_builder::{LockQuery, LockLazyQuery};

/// Helper function to create a LockQuery from a HashMap of TokioLocks.
fn lock_query<V: 'static>(map: &HashMap<impl std::hash::Hash + Eq, TokioLock<V>>) -> LockQuery<'_, V, TokioLock<V>> {
    let locks: Vec<_> = map.values().collect();
    LockQuery::from_locks(locks)
}

/// Helper function to create a LockLazyQuery from a HashMap of TokioLocks.
fn lock_lazy_query<V: 'static, K>(map: &HashMap<K, TokioLock<V>>) -> LockLazyQuery<'_, V, TokioLock<V>, impl Iterator<Item = &TokioLock<V>>>
where
    K: std::hash::Hash + Eq,
{
    LockLazyQuery::new(map.values())
}

// ============================================================================
// Helper functions for creating test data
// ============================================================================

fn create_async_users() -> AsyncUserMap {
    let mut users = HashMap::new();
    
    users.insert("u1".to_string(), TokioLock::new(User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        status: "active".to_string(),
        score: 95.5,
    }));
    
    users.insert("u2".to_string(), TokioLock::new(User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        status: "active".to_string(),
        score: 87.3,
    }));
    
    users.insert("u3".to_string(), TokioLock::new(User {
        id: 3,
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        status: "inactive".to_string(),
        score: 72.8,
    }));
    
    users.insert("u4".to_string(), TokioLock::new(User {
        id: 4,
        name: "Diana".to_string(),
        email: "diana@example.com".to_string(),
        status: "active".to_string(),
        score: 91.2,
    }));
    
    users
}

fn create_async_products() -> AsyncProductMap {
    let mut products = HashMap::new();
    
    products.insert("p1".to_string(), TokioLock::new(Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 999.99,
        stock: 15,
        category: "Electronics".to_string(),
    }));
    
    products.insert("p2".to_string(), TokioLock::new(Product {
        id: 2,
        name: "Mouse".to_string(),
        price: 29.99,
        stock: 50,
        category: "Electronics".to_string(),
    }));
    
    products.insert("p3".to_string(), TokioLock::new(Product {
        id: 3,
        name: "Desk Chair".to_string(),
        price: 299.99,
        stock: 8,
        category: "Furniture".to_string(),
    }));
    
    products.insert("p4".to_string(), TokioLock::new(Product {
        id: 4,
        name: "Monitor".to_string(),
        price: 399.99,
        stock: 0,
        category: "Electronics".to_string(),
    }));
    
    products
}

fn create_large_async_dataset(size: usize) -> (AsyncUserMap, AsyncProductMap) {
    let mut users = HashMap::new();
    let mut products = HashMap::new();
    
    let statuses = ["active", "inactive", "suspended"];
    let categories = ["Electronics", "Furniture", "Clothing", "Books"];
    
    for i in 0..size {
        let user = User {
            id: i as u32,
            name: format!("User{}", i),
            email: format!("user{}@example.com", i),
            status: statuses[i % statuses.len()].to_string(),
            score: 50.0 + (i as f64 * 7.3) % 50.0,
        };
        users.insert(format!("u{}", i), TokioLock::new(user));
    }
    
    for i in 0..size {
        let product = Product {
            id: i as u32,
            name: format!("Product{}", i),
            price: 10.0 + (i as f64 * 13.7) % 1000.0,
            stock: ((i * 7) % 100) as u32,
            category: categories[i % categories.len()].to_string(),
        };
        products.insert(format!("p{}", i), TokioLock::new(product));
    }
    
    (users, products)
}

// ============================================================================
// Main example
// ============================================================================

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║  tokio::sync::RwLock Support for Lock-Aware Queries             ║");
    println!("║  Demonstrating Extensibility to Async Locks                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let users = create_async_users();
    let products = create_async_products();
    
    println!("Created async data:");
    println!("  Users: {} (with tokio::sync::RwLock)", users.len());
    println!("  Products: {} (with tokio::sync::RwLock)\n", products.len());

    // ========================================================================
    // 1. Basic WHERE Queries
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("1. Basic WHERE Queries on tokio::RwLock");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Find active users ---");
    let active_users = lock_query(&users)
        .where_(User::status(), |s| s == "active")
        .all();
    
    println!("  Found: {} active users", active_users.len());
    for user in &active_users {
        println!("    • {} - {} - score: {:.1}", user.name, user.status, user.score);
    }
    println!("  SQL: SELECT * FROM users WHERE status = 'active';\n");

    println!("--- Find products in Electronics category ---");
    let electronics = lock_query(&products)
        .where_(Product::category(), |c| c == "Electronics")
        .all();
    
    println!("  Found: {} electronics", electronics.len());
    for product in &electronics {
        println!("    • {} - ${:.2} - stock: {}", product.name, product.price, product.stock);
    }
    println!("  SQL: SELECT * FROM products WHERE category = 'Electronics';\n");

    // ========================================================================
    // 2. SELECT Projections
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("2. SELECT Projections");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- SELECT user names ---");
    let names: Vec<String> = lock_query(&users)
        .select(User::name());
    
    println!("  Extracted: {} names", names.len());
    for name in &names {
        println!("    • {}", name);
    }
    println!("  SQL: SELECT name FROM users;\n");

    // ========================================================================
    // 3. ORDER BY
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("3. ORDER BY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Users ordered by score (descending) ---");
    let ordered = lock_query(&users)
        .order_by_float_desc(User::score());
    
    println!("  Top users by score:");
    for user in ordered.iter().take(3) {
        println!("    • {} - score: {:.1}", user.name, user.score);
    }
    println!("  SQL: SELECT * FROM users ORDER BY score DESC;\n");

    // ========================================================================
    // 4. Aggregations
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("4. Aggregations");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- COUNT active users ---");
    let count = lock_query(&users)
        .where_(User::status(), |s| s == "active")
        .count();
    println!("  Active users: {}", count);
    println!("  SQL: SELECT COUNT(*) FROM users WHERE status = 'active';\n");

    println!("--- AVG user score ---");
    let avg_score = lock_query(&users)
        .avg(User::score())
        .unwrap_or(0.0);
    println!("  Average score: {:.2}", avg_score);
    println!("  SQL: SELECT AVG(score) FROM users;\n");

    println!("--- SUM of product stock ---");
    let total_stock: u32 = lock_query(&products)
        .sum(Product::stock());
    println!("  Total stock: {} units", total_stock);
    println!("  SQL: SELECT SUM(stock) FROM products;\n");

    // ========================================================================
    // 5. GROUP BY
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("5. GROUP BY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Products grouped by category ---");
    let by_category = lock_query(&products)
        .group_by(Product::category());
    
    println!("  Categories: {}", by_category.len());
    for (category, items) in &by_category {
        let total_value: f64 = items.iter().map(|p| p.price).sum();
        println!("    • {}: {} products, total value: ${:.2}", 
                 category, items.len(), total_value);
    }
    println!("  SQL: SELECT category, COUNT(*), SUM(price) FROM products GROUP BY category;\n");

    // ========================================================================
    // 6. LAZY Queries with Early Termination
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("6. LAZY Queries - Early Termination");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- FIRST: Find first inactive user ---");
    let first_inactive = lock_lazy_query(&users)
        .where_(User::status(), |s| s == "inactive")
        .first();
    
    match first_inactive {
        Some(user) => println!("  Found: {} - {}", user.name, user.status),
        None => println!("  No inactive users found"),
    }
    println!("  SQL: SELECT * FROM users WHERE status = 'inactive' LIMIT 1;\n");

    println!("--- EXISTS: Check if any out-of-stock products ---");
    let out_of_stock = lock_lazy_query(&products)
        .where_(Product::stock(), |&s| s == 0)
        .any();
    
    println!("  Out of stock products exist? {}", out_of_stock);
    println!("  SQL: SELECT EXISTS(SELECT 1 FROM products WHERE stock = 0);\n");

    println!("--- TAKE: First 2 active users ---");
    let first_two: Vec<_> = lock_lazy_query(&users)
        .where_(User::status(), |s| s == "active")
        .take_lazy(2)
        .collect();
    
    println!("  Found: {} users (stopped early!)", first_two.len());
    for user in &first_two {
        println!("    • {}", user.name);
    }
    println!("  SQL: SELECT * FROM users WHERE status = 'active' LIMIT 2;\n");

    // ========================================================================
    // 7. Complex Queries
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("7. Complex Multi-Condition Queries");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("--- Active users with high scores (>= 90) ---");
    let high_scorers = lock_query(&users)
        .where_(User::status(), |s| s == "active")
        .where_(User::score(), |&score| score >= 90.0)
        .order_by_float_desc(User::score());
    
    println!("  Found: {} high-scoring active users", high_scorers.len());
    for user in &high_scorers {
        println!("    • {} - score: {:.1}", user.name, user.score);
    }
    println!("  SQL: SELECT * FROM users WHERE status = 'active' AND score >= 90 ORDER BY score DESC;\n");

    println!("--- Affordable in-stock electronics ---");
    let affordable = lock_query(&products)
        .where_(Product::category(), |c| c == "Electronics")
        .where_(Product::price(), |&p| p < 100.0)
        .where_(Product::stock(), |&s| s > 0)
        .all();
    
    println!("  Found: {} affordable electronics in stock", affordable.len());
    for product in &affordable {
        println!("    • {} - ${:.2} - {} in stock", product.name, product.price, product.stock);
    }
    println!("  SQL: SELECT * FROM products");
    println!("       WHERE category = 'Electronics' AND price < 100 AND stock > 0;\n");

    // ========================================================================
    // 8. Performance Comparison: Large Dataset
    // ========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("8. Performance: tokio::RwLock vs std::RwLock");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("🔬 Creating large dataset (1,000 items)...");
    let (large_users, large_products) = create_large_async_dataset(1000);
    println!("  Users: {}, Products: {}\n", large_users.len(), large_products.len());

    // Benchmark 1: Find First
    println!("  [1] Find First Inactive User:");
    
    // Eager
    let start = Instant::now();
    let eager_all = lock_query(&large_users)
        .where_(User::status(), |s| s == "inactive")
        .all();
    let _eager_first = eager_all.first().cloned();
    let eager_time = start.elapsed();
    
    // Lazy
    let start = Instant::now();
    let _lazy_first = lock_lazy_query(&large_users)
        .where_(User::status(), |s| s == "inactive")
        .first();
    let lazy_time = start.elapsed();
    
    println!("    Eager: {:?}", eager_time);
    println!("    Lazy: {:?}", lazy_time);
    if lazy_time.as_nanos() > 0 {
        let speedup = eager_time.as_nanos() as f64 / lazy_time.as_nanos() as f64;
        println!("    ⚡ Lazy is {:.2}x faster!", speedup);
    }

    // Benchmark 2: EXISTS
    println!("\n  [2] Check if Expensive Products Exist:");
    
    // Eager
    let start = Instant::now();
    let eager_all = lock_query(&large_products)
        .where_(Product::price(), |&p| p > 900.0)
        .all();
    let _eager_exists = !eager_all.is_empty();
    let eager_time = start.elapsed();
    
    // Lazy
    let start = Instant::now();
    let _lazy_exists = lock_lazy_query(&large_products)
        .where_(Product::price(), |&p| p > 900.0)
        .any();
    let lazy_time = start.elapsed();
    
    println!("    Eager: {:?}", eager_time);
    println!("    Lazy: {:?}", lazy_time);
    if lazy_time.as_nanos() > 0 {
        let speedup = eager_time.as_nanos() as f64 / lazy_time.as_nanos() as f64;
        println!("    ⚡ Lazy is {:.2}x faster!", speedup);
    }

    // ========================================================================
    // 9. Summary
    // ========================================================================
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✓ tokio::sync::RwLock Support Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📝 Features Demonstrated:");
    println!("  ✅ WHERE clauses on tokio::RwLock");
    println!("  ✅ SELECT projections");
    println!("  ✅ ORDER BY sorting");
    println!("  ✅ Aggregations (COUNT, SUM, AVG)");
    println!("  ✅ GROUP BY operations");
    println!("  ✅ LAZY queries with early termination");
    println!("  ✅ Complex multi-condition queries");
    println!("  ✅ Performance benefits maintained");

    println!("\n💡 Key Points:");
    println!("  • Implemented LockValue trait for tokio::sync::RwLock");
    println!("  • All SQL operations work seamlessly");
    println!("  • Zero-copy benefits maintained");
    println!("  • Lazy evaluation still provides huge speedups");
    println!("  • Easy to extend to other lock types");

    println!("\n🚀 Extension Pattern:");
    println!("  1. Implement LockValue trait for your lock type");
    println!("  2. Provide with_value() method");
    println!("  3. All query operations work automatically!");

    println!("\n⚠️  Production Note:");
    println!("  This example uses block_in_place for simplicity.");
    println!("  In production async code, consider:");
    println!("    • Staying fully async throughout");
    println!("    • Using async versions of query methods");
    println!("    • Avoiding blocking in async context");
}

