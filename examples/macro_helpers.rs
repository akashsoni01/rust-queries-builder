// Demonstrates helper macros that reduce boilerplate code
// Shows before/after comparisons for common query patterns
// cargo run --example macro_helpers

use rust_queries_builder::LazyQuery;
use key_paths_derive::Keypath;

// Import all the helper macros
use rust_queries_builder::{
    lazy_query, query, collect_lazy, filter_collect,
    count_where, find_first, exists_where, paginate,
    sum_where, avg_where, select_all, select_where,
};

#[derive(Debug, Clone, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
    rating: f64,
    active: bool,
}

fn create_products() -> Vec<Product> {
    vec![
        Product { id: 1, name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string(), stock: 15, rating: 4.8, active: true },
        Product { id: 2, name: "Mouse".to_string(), price: 29.99, category: "Electronics".to_string(), stock: 50, rating: 4.5, active: true },
        Product { id: 3, name: "Keyboard".to_string(), price: 129.99, category: "Electronics".to_string(), stock: 30, rating: 4.7, active: true },
        Product { id: 4, name: "Desk".to_string(), price: 299.99, category: "Furniture".to_string(), stock: 20, rating: 4.6, active: true },
        Product { id: 5, name: "Chair".to_string(), price: 199.99, category: "Furniture".to_string(), stock: 0, rating: 4.3, active: false },
    ]
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Macro Helpers Demo - Reducing Boilerplate                    ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let products = create_products();

    // ============================================================================
    // EXAMPLE 1: Simple Collection
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 1: collect_lazy! - Simple collection");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let results: Vec<_> = LazyQuery::new(&products).collect();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let results = collect_lazy!(&products);");
    println!("```\n");

    let results = collect_lazy!(&products);
    println!("Result: {} products collected\n", results.len());

    // ============================================================================
    // EXAMPLE 2: Filter and Collect
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 2: filter_collect! - Filter and collect");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let electronics: Vec<_> = LazyQuery::new(&products)");
    println!("    .where_(Product::category(), |cat| cat == \"Electronics\")");
    println!("    .collect();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let electronics = filter_collect!(");
    println!("    &products,");
    println!("    Product::category(),");
    println!("    |cat| cat == \"Electronics\"");
    println!(");");
    println!("```\n");

    let electronics = filter_collect!(
        &products,
        Product::category(),
        |cat| cat == "Electronics"
    );
    println!("Result: {} electronics\n", electronics.len());

    // ============================================================================
    // EXAMPLE 3: Count with Filter
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 3: count_where! - Count with filter");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let count = LazyQuery::new(&products)");
    println!("    .where_(Product::stock(), |&s| s > 0)");
    println!("    .count();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let count = count_where!(&products, Product::stock(), |&s| s > 0);");
    println!("```\n");

    let count = count_where!(&products, Product::stock(), |&s| s > 0);
    println!("Result: {} products in stock\n", count);

    // ============================================================================
    // EXAMPLE 4: Find First
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 4: find_first! - Find first matching item");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let found = LazyQuery::new(&products)");
    println!("    .where_(Product::price(), |&p| p > 500.0)");
    println!("    .first();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let found = find_first!(&products, Product::price(), |&p| p > 500.0);");
    println!("```\n");

    let found = find_first!(&products, Product::price(), |&p| p > 500.0);
    if let Some(p) = found {
        println!("Result: Found {} at ${:.2}\n", p.name, p.price);
    }

    // ============================================================================
    // EXAMPLE 5: Existence Check
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 5: exists_where! - Check if any item matches");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let has_furniture = LazyQuery::new(&products)");
    println!("    .where_(Product::category(), |cat| cat == \"Furniture\")");
    println!("    .any();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let has_furniture = exists_where!(");
    println!("    &products,");
    println!("    Product::category(),");
    println!("    |cat| cat == \"Furniture\"");
    println!(");");
    println!("```\n");

    let has_furniture = exists_where!(
        &products,
        Product::category(),
        |cat| cat == "Furniture"
    );
    println!("Result: Has furniture = {}\n", has_furniture);

    // ============================================================================
    // EXAMPLE 6: Pagination
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 6: paginate! - Quick pagination");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let page_2: Vec<_> = LazyQuery::new(&products)");
    println!("    .skip_lazy(1 * 2)  // page * size");
    println!("    .take_lazy(2)");
    println!("    .collect();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let page_2 = paginate!(&products, page: 1, size: 2);");
    println!("```\n");

    let page_2 = paginate!(&products, page: 1, size: 2);
    println!("Result: Page 2 has {} items\n", page_2.len());

    // ============================================================================
    // EXAMPLE 7: Sum with Filter
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 7: sum_where! - Sum with filter");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let total: f64 = LazyQuery::new(&products)");
    println!("    .where_(Product::active(), |&a| a)");
    println!("    .sum_by(Product::price());");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let total = sum_where!(&products, Product::price(), Product::active(), |&a| a);");
    println!("```\n");

    let total = sum_where!(&products, Product::price(), Product::active(), |&a| a);
    println!("Result: Total active products value = ${:.2}\n", total);

    // ============================================================================
    // EXAMPLE 8: Average with Filter
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 8: avg_where! - Average with filter");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let avg = LazyQuery::new(&products)");
    println!("    .where_(Product::category(), |cat| cat == \"Electronics\")");
    println!("    .avg_by(Product::price())");
    println!("    .unwrap_or(0.0);");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let avg = avg_where!(");
    println!("    &products,");
    println!("    Product::price(),");
    println!("    Product::category(),");
    println!("    |cat| cat == \"Electronics\"");
    println!(").unwrap_or(0.0);");
    println!("```\n");

    let avg = avg_where!(
        &products,
        Product::price(),
        Product::category(),
        |cat| cat == "Electronics"
    ).unwrap_or(0.0);
    println!("Result: Average electronics price = ${:.2}\n", avg);

    // ============================================================================
    // EXAMPLE 9: Select All
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 9: select_all! - Select field from all items");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let names: Vec<String> = LazyQuery::new(&products)");
    println!("    .select_lazy(Product::name())");
    println!("    .collect();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let names = select_all!(&products, Product::name());");
    println!("```\n");

    let names: Vec<String> = select_all!(&products, Product::name());
    println!("Result: {} product names\n", names.len());

    // ============================================================================
    // EXAMPLE 10: Select with Filter
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Example 10: select_where! - Select field with filter");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("❌ Without macro (verbose):");
    println!("```rust");
    println!("let furniture_names: Vec<String> = LazyQuery::new(&products)");
    println!("    .where_(Product::category(), |cat| cat == \"Furniture\")");
    println!("    .select_lazy(Product::name())");
    println!("    .collect();");
    println!("```\n");

    println!("✅ With macro (concise):");
    println!("```rust");
    println!("let furniture_names = select_where!(");
    println!("    &products,");
    println!("    Product::name(),");
    println!("    Product::category(),");
    println!("    |cat| cat == \"Furniture\"");
    println!(");");
    println!("```\n");

    let furniture_names: Vec<String> = select_where!(
        &products,
        Product::name(),
        Product::category(),
        |cat| cat == "Furniture"
    );
    println!("Result: {} furniture items\n", furniture_names.len());

    // ============================================================================
    // COMPARISON: Complex Query
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Complex Example: Before & After");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Scenario: Filter electronics, under $500, in stock, get first 5\n");

    println!("❌ WITHOUT MACROS (13 lines):");
    println!("```rust");
    println!("let results: Vec<_> = LazyQuery::new(&products)");
    println!("    .where_(");
    println!("        Product::category(),");
    println!("        |cat| cat == \"Electronics\"");
    println!("    )");
    println!("    .where_(Product::price(), |&p| p < 500.0)");
    println!("    .where_(Product::stock(), |&s| s > 0)");
    println!("    .take_lazy(5)");
    println!("    .collect();");
    println!("```\n");

    // Actual verbose version
    let verbose_results: Vec<_> = LazyQuery::new(&products)
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p < 500.0)
        .where_(Product::stock(), |&s| s > 0)
        .take_lazy(5)
        .collect();

    println!("✅ WITH MACROS (Shorter, but still need multiple filters):");
    println!("```rust");
    println!("let results = lazy_query!(&products)");
    println!("    .where_(Product::category(), |cat| cat == \"Electronics\")");
    println!("    .where_(Product::price(), |&p| p < 500.0)");
    println!("    .where_(Product::stock(), |&s| s > 0)");
    println!("    .take_lazy(5)");
    println!("    .collect();");
    println!("```\n");

    let macro_results = lazy_query!(&products)
        .where_(Product::category(), |cat| cat == "Electronics")
        .where_(Product::price(), |&p| p < 500.0)
        .where_(Product::stock(), |&s| s > 0)
        .take_lazy(5)
        .collect();

    println!("Both approaches found {} items ✅\n", verbose_results.len());
    assert_eq!(verbose_results.len(), macro_results.len());

    // ============================================================================
    // CODE REDUCTION METRICS
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Code Reduction Metrics");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Pattern Comparisons:\n");

    println!("  1. Simple collect:");
    println!("     Before: LazyQuery::new(&data).collect()");
    println!("     After:  collect_lazy!(&data)");
    println!("     Saved:  ~20 characters\n");

    println!("  2. Filter + collect:");
    println!("     Before: LazyQuery::new(&data).where_(...).collect()");
    println!("     After:  filter_collect!(&data, field, pred)");
    println!("     Saved:  ~35 characters\n");

    println!("  3. Count with filter:");
    println!("     Before: LazyQuery::new(&data).where_(...).count()");
    println!("     After:  count_where!(&data, field, pred)");
    println!("     Saved:  ~30 characters\n");

    println!("  4. Pagination:");
    println!("     Before: LazyQuery::new(&data).skip_lazy(p*s).take_lazy(s).collect()");
    println!("     After:  paginate!(&data, page: p, size: s)");
    println!("     Saved:  ~45 characters\n");

    println!("  5. Sum with filter:");
    println!("     Before: LazyQuery::new(&data).where_(...).sum_by(...)");
    println!("     After:  sum_where!(&data, sum_field, filter_field, pred)");
    println!("     Saved:  ~25 characters\n");

    // ============================================================================
    // ALL MACRO DEMONSTRATIONS
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("All Available Macros - Quick Reference");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("1. lazy_query!(&data)");
    let _q1 = lazy_query!(&products);
    println!("   → LazyQuery::new(&data)\n");

    println!("2. query!(&data)");
    let _q2 = query!(&products);
    println!("   → Query::new(&data)\n");

    println!("3. collect_lazy!(&data)");
    let _r3 = collect_lazy!(&products);
    println!("   → LazyQuery::new(&data).collect()\n");

    println!("4. filter_collect!(&data, field, pred)");
    let _r4 = filter_collect!(&products, Product::active(), |&a| a);
    println!("   → LazyQuery::new(&data).where_(field, pred).collect()\n");

    println!("5. count_where!(&data, field, pred)");
    let _r5 = count_where!(&products, Product::active(), |&a| a);
    println!("   → LazyQuery::new(&data).where_(field, pred).count()\n");

    println!("6. find_first!(&data, field, pred)");
    let _r6 = find_first!(&products, Product::id(), |&id| id == 1);
    println!("   → LazyQuery::new(&data).where_(field, pred).first()\n");

    println!("7. exists_where!(&data, field, pred)");
    let _r7 = exists_where!(&products, Product::active(), |&a| a);
    println!("   → LazyQuery::new(&data).where_(field, pred).any()\n");

    println!("8. paginate!(&data, page: p, size: s)");
    let _r8 = paginate!(&products, page: 0, size: 3);
    println!("   → LazyQuery::new(&data).skip_lazy(p*s).take_lazy(s).collect()\n");

    println!("9. sum_where!(&data, sum_field, filter_field, pred)");
    let _r9 = sum_where!(&products, Product::price(), Product::active(), |&a| a);
    println!("   → LazyQuery::new(&data).where_(filter_field, pred).sum_by(sum_field)\n");

    println!("10. avg_where!(&data, avg_field, filter_field, pred)");
    let _r10 = avg_where!(&products, Product::price(), Product::active(), |&a| a);
    println!("    → LazyQuery::new(&data).where_(filter_field, pred).avg_by(avg_field)\n");

    println!("11. select_all!(&data, field)");
    let _r11: Vec<String> = select_all!(&products, Product::name());
    println!("    → LazyQuery::new(&data).select_lazy(field).collect()\n");

    println!("12. select_where!(&data, select_field, filter_field, pred)");
    let _r12: Vec<String> = select_where!(&products, Product::name(), Product::active(), |&a| a);
    println!("    → LazyQuery::new(&data).where_(filter, pred).select_lazy(field).collect()\n");

    // ============================================================================
    // Summary
    // ============================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Summary                                                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ 12 helper macros provided:");
    println!("   • lazy_query! - Create LazyQuery");
    println!("   • query! - Create Query");
    println!("   • collect_lazy! - Quick collect");
    println!("   • filter_collect! - Filter and collect");
    println!("   • count_where! - Count with filter");
    println!("   • find_first! - Find first match");
    println!("   • exists_where! - Existence check");
    println!("   • paginate! - Easy pagination");
    println!("   • sum_where! - Sum with filter");
    println!("   • avg_where! - Average with filter");
    println!("   • select_all! - Select all");
    println!("   • select_where! - Select with filter\n");

    println!("📊 Benefits:");
    println!("   • Less typing (20-45 characters saved per operation)");
    println!("   • More readable code");
    println!("   • Common patterns encapsulated");
    println!("   • Same performance (zero-cost abstraction)");
    println!("   • Type-safe (compile-time checked)\n");

    println!("💡 When to use:");
    println!("   • Use macros for simple, common patterns");
    println!("   • Use full API for complex queries");
    println!("   • Mix and match as needed\n");

    println!("✓ Macro helpers demo complete!\n");
}

