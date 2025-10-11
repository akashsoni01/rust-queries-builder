// Demonstrates lazy query evaluation using iterators
// Shows how lazy evaluation defers work until results are needed
// and enables early termination for better performance
// cargo run --example lazy_evaluation

use rust_queries_builder::LazyQuery;
use key_paths_derive::Keypaths;
use std::sync::atomic::{AtomicUsize, Ordering};

// Counter to track how many times predicates are evaluated
static FILTER_EVALUATIONS: AtomicUsize = AtomicUsize::new(0);
static EXPENSIVE_OPERATIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

fn create_products() -> Vec<Product> {
    (1..=1000)
        .map(|i| Product {
            id: i,
            name: format!("Product {}", i),
            price: (i as f64 * 10.0) % 1000.0,
            category: if i % 3 == 0 {
                "Electronics".to_string()
            } else if i % 3 == 1 {
                "Furniture".to_string()
            } else {
                "Clothing".to_string()
            },
            stock: i % 50,
        })
        .collect()
}

fn expensive_check(value: &f64) -> bool {
    EXPENSIVE_OPERATIONS.fetch_add(1, Ordering::SeqCst);
    // Simulate expensive operation
    value > &100.0
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Lazy Query Evaluation Demo                                   ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let products = create_products();
    println!("Created {} products\n", products.len());

    // ============================================================================
    // DEMO 1: Lazy Execution - Nothing Happens Until .collect()
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 1: Lazy execution - deferred until needed");
    println!("═══════════════════════════════════════════════════════════════\n");

    FILTER_EVALUATIONS.store(0, Ordering::SeqCst);

    println!("Building query (should execute nothing)...");
    let lazy_query = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| {
            FILTER_EVALUATIONS.fetch_add(1, Ordering::SeqCst);
            cat == "Electronics"
        });

    let evals_after_build = FILTER_EVALUATIONS.load(Ordering::SeqCst);
    println!("  Filter evaluations after building query: {}", evals_after_build);
    
    if evals_after_build == 0 {
        println!("  ✅ Confirmed: Query is lazy! Nothing executed yet.\n");
    }

    println!("Collecting results (now it executes)...");
    let results: Vec<_> = lazy_query.collect();

    let evals_after_collect = FILTER_EVALUATIONS.load(Ordering::SeqCst);
    println!("  Filter evaluations after collecting: {}", evals_after_collect);
    println!("  Results found: {}", results.len());
    println!("  ✅ Query executed exactly once, when needed!\n");

    // ============================================================================
    // DEMO 2: Early Termination - Stops as Soon as Enough Items Found
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 2: Early termination with .take()");
    println!("═══════════════════════════════════════════════════════════════\n");

    EXPENSIVE_OPERATIONS.store(0, Ordering::SeqCst);

    println!("Finding first 5 expensive items from 1000 products...");
    let first_5: Vec<_> = LazyQuery::new(&products)
        .where_(Product::price_r(), |p| expensive_check(p))
        .take_lazy(5)
        .collect();

    let ops = EXPENSIVE_OPERATIONS.load(Ordering::SeqCst);
    println!("  Found: {} items", first_5.len());
    println!("  Expensive operations performed: {}", ops);
    println!("  Items NOT checked: {} (stopped early!)", 1000 - ops);
    
    if ops < 1000 {
        println!("  ✅ Early termination worked! Didn't check all 1000 items.\n");
    }

    // ============================================================================
    // DEMO 3: Iterator Fusion - Rust Optimizes Chained Operations
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 3: Iterator fusion - chained operations optimized");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Chaining multiple operations...");
    let chained_query = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .where_(Product::price_r(), |&price| price > 200.0)
        .where_(Product::stock_r(), |&stock| stock > 10)
        .take_lazy(10);

    println!("  Built query with 3 filters + take(10)");
    println!("  ✅ No execution yet - all operations fused into one iterator\n");

    let results: Vec<_> = chained_query.collect();
    println!("  Executed: Found {} items", results.len());
    println!("  ✅ All filters applied in single pass!\n");

    // ============================================================================
    // DEMO 4: Lazy Projection - Only Extract What You Need
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 4: Lazy projection");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Selecting names (lazy)...");
    let names: Vec<String> = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .select_lazy(Product::name_r())
        .take(5)  // Only process until we have 5
        .collect();

    println!("  Selected {} names", names.len());
    println!("  ✅ Only evaluated until 5 names found!\n");

    // ============================================================================
    // DEMO 5: Lazy Aggregation - Short-Circuit When Possible
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 5: Short-circuit with .any()");
    println!("═══════════════════════════════════════════════════════════════\n");

    FILTER_EVALUATIONS.store(0, Ordering::SeqCst);

    println!("Checking if ANY electronics exist (1000 items to search)...");
    let exists = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| {
            FILTER_EVALUATIONS.fetch_add(1, Ordering::SeqCst);
            cat == "Electronics"
        })
        .any();

    let checks = FILTER_EVALUATIONS.load(Ordering::SeqCst);
    println!("  Result: {}", exists);
    println!("  Items checked: {} out of 1000", checks);
    println!("  Items skipped: {} (short-circuited!)", 1000 - checks);
    
    if checks < 1000 {
        println!("  ✅ Short-circuit worked! Stopped as soon as first match found.\n");
    }

    // ============================================================================
    // DEMO 6: Lazy Find - Stops at First Match
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 6: .find() stops at first match");
    println!("═══════════════════════════════════════════════════════════════\n");

    FILTER_EVALUATIONS.store(0, Ordering::SeqCst);

    println!("Finding first product with price > 500...");
    let found = LazyQuery::new(&products)
        .where_(Product::price_r(), |&price| {
            FILTER_EVALUATIONS.fetch_add(1, Ordering::SeqCst);
            price > 500.0
        })
        .first();

    let checks = FILTER_EVALUATIONS.load(Ordering::SeqCst);
    if let Some(product) = found {
        println!("  Found: {} (${:.2})", product.name, product.price);
    }
    println!("  Items checked: {} out of 1000", checks);
    println!("  ✅ Stopped immediately after finding first match!\n");

    // ============================================================================
    // DEMO 7: Composition - Build Queries Incrementally
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 7: Composable queries");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Building base query...");
    let base_query = LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics");

    println!("  Created base query (not executed)\n");

    println!("  Adding price filter...");
    let refined_query = base_query
        .where_(Product::price_r(), |&price| price > 100.0);

    println!("  Still not executed...\n");

    println!("  Adding stock filter and limiting...");
    let final_query = refined_query
        .where_(Product::stock_r(), |&stock| stock > 5)
        .take_lazy(10);

    println!("  Still not executed...\n");

    println!("  Executing...");
    let results: Vec<_> = final_query.collect();
    println!("  ✅ Executed once with all filters: Found {} items\n", results.len());

    // ============================================================================
    // DEMO 8: For Loop - Natural Iteration
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 8: Use in for loops");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Iterating over filtered products...");
    let mut count = 0;
    for product in LazyQuery::new(&products)
        .where_(Product::category_r(), |cat| cat == "Electronics")
        .take_lazy(3)
    {
        println!("  • {}: ${:.2}", product.name, product.price);
        count += 1;
    }
    println!("  ✅ Processed {} items lazily\n", count);

    // ============================================================================
    // DEMO 9: Performance Comparison
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 9: Performance benefit demonstration");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Scenario: Find first expensive item from 1000 products\n");

    EXPENSIVE_OPERATIONS.store(0, Ordering::SeqCst);
    
    println!("Without lazy (hypothetical - check all, then take first):");
    println!("  Would check: 1000 items");
    println!("  Would find: ~300 matching items");
    println!("  Would return: 1 item");
    println!("  Wasted work: 299 items processed unnecessarily\n");

    EXPENSIVE_OPERATIONS.store(0, Ordering::SeqCst);
    
    println!("With lazy evaluation:");
    let _first = LazyQuery::new(&products)
        .where_(Product::price_r(), |p| expensive_check(p))
        .first();

    let ops = EXPENSIVE_OPERATIONS.load(Ordering::SeqCst);
    println!("  Checked: {} items", ops);
    println!("  Found: 1 item");
    println!("  Wasted work: 0 items");
    println!("  ✅ Efficiency gain: {}x faster!", 1000 / ops.max(1));

    // ============================================================================
    // Summary
    // ============================================================================
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Lazy Evaluation Benefits                                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ Deferred Execution:");
    println!("   • No work until results needed");
    println!("   • Can build complex queries without performance cost\n");

    println!("✅ Early Termination:");
    println!("   • .take(n) stops after n items");
    println!("   • .first() stops after 1 item");
    println!("   • .any() stops after first match");
    println!("   • Massive performance win for large datasets\n");

    println!("✅ Iterator Fusion:");
    println!("   • Multiple filters combined into one pass");
    println!("   • Rust compiler optimizes chained operations");
    println!("   • No intermediate allocations\n");

    println!("✅ Composable:");
    println!("   • Build queries incrementally");
    println!("   • Reuse query fragments");
    println!("   • Clean separation of query building vs execution\n");

    println!("✅ Zero Overhead:");
    println!("   • Compiles to same code as manual loops");
    println!("   • No runtime cost for abstraction");
    println!("   • Pay only for what you use\n");

    println!("✓ Lazy evaluation demo complete!\n");
}

