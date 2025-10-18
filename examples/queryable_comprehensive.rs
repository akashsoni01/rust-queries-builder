//! Comprehensive demonstration of Queryable trait with LazyQuery support
//!
//! This example shows how ALL Queryable container types (HashMap, HashSet, 
//! VecDeque, LinkedList, BTreeMap, BTreeSet, Vec, arrays, etc.) now have
//! full access to lazy query operations through the QueryableExt trait.

use rust_queries_core::QueryableExt;
use key_paths_derive::Keypath;
use std::collections::{HashMap, HashSet, VecDeque, LinkedList, BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Keypath)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    stock: u32,
}

impl Product {
    fn new(id: u32, name: &str, price: f64, category: &str, stock: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
            category: category.to_string(),
            stock,
        }
    }
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Queryable Comprehensive: All Containers with Lazy Queries    ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ============================================================================
    // 1. Vec - Direct QueryExt support (slice-based)
    // ============================================================================
    println!("┌────────────────────────────────────────┐");
    println!("│ 1. Vec<T> - Direct QueryExt Support   │");
    println!("└────────────────────────────────────────┘");

    let products_vec = vec![
        Product::new(1, "Laptop", 999.99, "Electronics", 5),
        Product::new(2, "Mouse", 29.99, "Electronics", 50),
        Product::new(3, "Desk", 299.99, "Furniture", 10),
        Product::new(4, "Chair", 199.99, "Furniture", 15),
    ];

    // Using QueryExt (optimized for slices)
    let expensive: Vec<_> = products_vec
        .lazy_query()
        .where_(Product::price(), |&p| p > 100.0)
        .collect();
    
    println!("Expensive products (>$100): {} items", expensive.len());
    for p in &expensive {
        println!("  • {} - ${:.2}", p.name, p.price);
    }

    // ============================================================================
    // 2. HashMap - Values are queryable
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 2. HashMap<K,V> - Queryable Values    │");
    println!("└────────────────────────────────────────┘");

    let mut products_map: HashMap<u32, Product> = HashMap::new();
    products_map.insert(1, Product::new(1, "Laptop", 999.99, "Electronics", 5));
    products_map.insert(2, Product::new(2, "Mouse", 29.99, "Electronics", 50));
    products_map.insert(3, Product::new(3, "Desk", 299.99, "Furniture", 10));
    products_map.insert(4, Product::new(4, "Chair", 199.99, "Furniture", 15));

    // Using QueryableExt for HashMap
    let electronics: Vec<_> = products_map
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .collect();
    
    println!("Electronics from HashMap: {} items", electronics.len());
    for p in &electronics {
        println!("  • {} - ${:.2}", p.name, p.price);
    }

    // ============================================================================
    // 3. HashSet - Unordered unique items (using simple IDs)
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 3. HashSet<T> - Unique Items          │");
    println!("└────────────────────────────────────────┘");

    let mut ids_set: HashSet<u32> = HashSet::new();
    ids_set.insert(1);
    ids_set.insert(2);
    ids_set.insert(3);
    ids_set.insert(10);
    ids_set.insert(50);
    
    let total_count = ids_set.lazy_query().count();
    
    println!("Total unique IDs in HashSet: {} items", total_count);

    // ============================================================================
    // 4. VecDeque - Double-ended queue
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 4. VecDeque<T> - Double-ended Queue   │");
    println!("└────────────────────────────────────────┘");

    let mut products_deque: VecDeque<Product> = VecDeque::new();
    products_deque.push_back(Product::new(1, "Laptop", 999.99, "Electronics", 5));
    products_deque.push_back(Product::new(2, "Mouse", 29.99, "Electronics", 50));
    products_deque.push_back(Product::new(3, "Desk", 299.99, "Furniture", 10));
    
    // Aggregate operations with QueryableExt
    let total_value = products_deque.lazy_query().sum_by(Product::price());
    let avg_price = products_deque.lazy_query().avg_by(Product::price()).unwrap();
    
    println!("Total value: ${:.2}", total_value);
    println!("Average price: ${:.2}", avg_price);

    // ============================================================================
    // 5. LinkedList - Doubly-linked list
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 5. LinkedList<T> - Linked List        │");
    println!("└────────────────────────────────────────┘");

    let mut products_list: LinkedList<Product> = LinkedList::new();
    products_list.push_back(Product::new(1, "Laptop", 999.99, "Electronics", 5));
    products_list.push_back(Product::new(2, "Mouse", 29.99, "Electronics", 50));
    products_list.push_back(Product::new(3, "Desk", 299.99, "Furniture", 10));
    
    let first_furniture = products_list
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Furniture")
        .first();
    
    if let Some(product) = first_furniture {
        println!("First furniture item: {} - ${:.2}", product.name, product.price);
    }

    // ============================================================================
    // 6. BTreeMap - Sorted map (queries values)
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 6. BTreeMap<K,V> - Sorted Map         │");
    println!("└────────────────────────────────────────┘");

    let mut products_btree: BTreeMap<u32, Product> = BTreeMap::new();
    products_btree.insert(1, Product::new(1, "Laptop", 999.99, "Electronics", 5));
    products_btree.insert(2, Product::new(2, "Mouse", 29.99, "Electronics", 50));
    products_btree.insert(3, Product::new(3, "Desk", 299.99, "Furniture", 10));
    products_btree.insert(4, Product::new(4, "Chair", 199.99, "Furniture", 15));
    
    let furniture_count = products_btree
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Furniture")
        .count();
    
    println!("Furniture items in BTreeMap: {} items", furniture_count);

    // ============================================================================
    // 7. BTreeSet - Sorted unique items (using simple IDs)
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 7. BTreeSet<T> - Sorted Unique Items  │");
    println!("└────────────────────────────────────────┘");

    let mut ids_btreeset: BTreeSet<u32> = BTreeSet::new();
    ids_btreeset.insert(10);
    ids_btreeset.insert(5);
    ids_btreeset.insert(30);
    ids_btreeset.insert(15);
    
    let total_ids = ids_btreeset.lazy_query().count();
    
    println!("Total unique IDs in BTreeSet: {} items (sorted)", total_ids);

    // ============================================================================
    // 8. Arrays - Fixed-size arrays
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 8. Arrays [T; N] - Fixed Size         │");
    println!("└────────────────────────────────────────┘");

    let products_array = [
        Product::new(1, "Laptop", 999.99, "Electronics", 5),
        Product::new(2, "Mouse", 29.99, "Electronics", 50),
        Product::new(3, "Desk", 299.99, "Furniture", 10),
    ];
    
    let electronics_array: Vec<_> = products_array
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics")
        .collect();
    
    println!("Electronics from array: {} items", electronics_array.len());

    // ============================================================================
    // 9. Complex chaining with multiple operations
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 9. Complex Query Chaining             │");
    println!("└────────────────────────────────────────┘");

    let complex_results: Vec<_> = products_map
        .lazy_query()
        .where_(Product::category(), |cat| cat == "Electronics" || cat == "Furniture")
        .where_(Product::price(), |&p| p > 100.0)
        .where_(Product::stock(), |&s| s >= 5)
        .skip_lazy(0)
        .take_lazy(3)
        .collect();
    
    println!("Complex filtered results: {} items", complex_results.len());
    for p in &complex_results {
        println!("  • {} - ${:.2} (stock: {})", p.name, p.price, p.stock);
    }

    // ============================================================================
    // 10. All aggregation operations
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 10. Aggregation Operations            │");
    println!("└────────────────────────────────────────┘");

    let mut aggregate_deque: VecDeque<Product> = VecDeque::new();
    aggregate_deque.push_back(Product::new(1, "Laptop", 999.99, "Electronics", 5));
    aggregate_deque.push_back(Product::new(2, "Mouse", 29.99, "Electronics", 50));
    aggregate_deque.push_back(Product::new(3, "Keyboard", 79.99, "Electronics", 30));
    aggregate_deque.push_back(Product::new(4, "Monitor", 299.99, "Electronics", 8));

    println!("Aggregation operations on VecDeque:");
    
    let sum = aggregate_deque.lazy_query().sum_by(Product::price());
    println!("  • Sum of prices: ${:.2}", sum);
    
    let avg = aggregate_deque.lazy_query().avg_by(Product::price()).unwrap();
    println!("  • Average price: ${:.2}", avg);
    
    let min = aggregate_deque.lazy_query().min_by_float(Product::price()).unwrap();
    println!("  • Minimum price: ${:.2}", min);
    
    let max = aggregate_deque.lazy_query().max_by_float(Product::price()).unwrap();
    println!("  • Maximum price: ${:.2}", max);
    
    let count = aggregate_deque.lazy_query().count();
    println!("  • Total items: {}", count);

    let exists = aggregate_deque
        .lazy_query()
        .where_(Product::price(), |&p| p > 500.0)
        .any();
    println!("  • Has items over $500: {}", exists);

    // ============================================================================
    // 11. Select/Projection operations
    // ============================================================================
    println!("\n┌────────────────────────────────────────┐");
    println!("│ 11. Select/Projection Operations      │");
    println!("└────────────────────────────────────────┘");

    let names: Vec<String> = products_map
        .lazy_query()
        .select_lazy(Product::name())
        .collect();
    
    println!("All product names from HashMap:");
    for name in &names {
        println!("  • {}", name);
    }

    // ============================================================================
    // Summary
    // ============================================================================
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Summary: Queryable Trait with Full Lazy Support              ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ All Queryable containers now support lazy query operations:\n");
    
    println!("📦 Basic Operations:");
    println!("   • new(data) / from_iter(iter) - Create query");
    println!("   • where_(path, predicate) - Filter by predicate");
    println!("   • all() / collect() - Get all matching items");
    println!("   • first() - Get first matching item");
    println!("   • count() - Count matching items");
    println!("   • take_lazy(n) - Limit results");
    println!("   • skip_lazy(n) - Skip results for pagination");
    println!("   • any() / exists() - Check if any match\n");
    
    println!("📊 Aggregations:");
    println!("   • sum_by(path) - Sum numeric field");
    println!("   • avg_by(path) - Average of f64 field");
    println!("   • min_by(path) / max_by(path) - Min/max of Ord field");
    println!("   • min_by_float(path) / max_by_float(path) - Min/max of f64\n");
    
    println!("🔍 Projection:");
    println!("   • select_lazy(path) - Project field values\n");
    
    println!("📋 Supported Containers:");
    println!("   • Vec<T> ✓");
    println!("   • &[T] (slices) ✓");
    println!("   • [T; N] (arrays) ✓");
    println!("   • HashMap<K, V> ✓ (queries values)");
    println!("   • HashSet<T> ✓");
    println!("   • VecDeque<T> ✓");
    println!("   • LinkedList<T> ✓");
    println!("   • BTreeMap<K, V> ✓ (queries values)");
    println!("   • BTreeSet<T> ✓");
    println!("   • Option<T> ✓");
    println!("   • Result<T, E> ✓\n");
    
    println!("🎯 Key Benefits:");
    println!("   • Unified API across all container types");
    println!("   • Lazy evaluation with early termination");
    println!("   • Zero-cost abstractions");
    println!("   • Type-safe with compile-time checking");
    println!("   • Composable and chainable operations");
    println!("   • No unnecessary allocations\n");

    println!("💡 Usage Pattern:");
    println!("   ```rust");
    println!("   use rust_queries_core::QueryableExt;");
    println!("   ");
    println!("   let map: HashMap<K, Product> = ...;");
    println!("   let results: Vec<_> = map");
    println!("       .lazy_query()");
    println!("       .where_(Product::price(), |&p| p > 100.0)");
    println!("       .where_(Product::stock(), |&s| s > 0)");
    println!("       .take_lazy(10)");
    println!("       .collect();");
    println!("   ```\n");
}

