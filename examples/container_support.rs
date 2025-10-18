// Demonstrates support for various container types
// Shows how to query Vec, HashMap, HashSet, BTreeMap, VecDeque, etc.
// cargo run --example container_support

use rust_queries_builder::{Query, LazyQuery};
use key_paths_derive::Keypath;
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList};

#[derive(Debug, Clone, Keypath, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Product {
    id: u32,
    name: String,
    price: u32, // Using u32 for Hash/Ord
    category: String,
}

fn create_sample_product(id: u32, name: &str, price: u32, category: &str) -> Product {
    Product {
        id,
        name: name.to_string(),
        price,
        category: category.to_string(),
    }
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Container Support Demo                                       ║");
    println!("║  Query various collection types                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ============================================================================
    // CONTAINER 1: Vec<T>
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Container 1: Vec<T>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let vec_products = vec![
        create_sample_product(1, "Laptop", 999, "Electronics"),
        create_sample_product(2, "Mouse", 29, "Electronics"),
        create_sample_product(3, "Desk", 299, "Furniture"),
    ];

    // Query Vec directly
    let vec_query = Query::new(&vec_products)
        .where_(Product::category(), |cat| cat == "Electronics");
    let vec_results = vec_query.all();
    println!("  Vec: Found {} electronics", vec_results.len());

    // Lazy query on Vec
    let lazy_count = LazyQuery::new(&vec_products)
        .where_(Product::price(), |&p| p < 100)
        .count();
    println!("  Vec (lazy): {} items under $100", lazy_count);

    // ============================================================================
    // CONTAINER 2: VecDeque<T>
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 2: VecDeque<T>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut deque_products = VecDeque::new();
    deque_products.push_back(create_sample_product(1, "Keyboard", 129, "Electronics"));
    deque_products.push_back(create_sample_product(2, "Monitor", 349, "Electronics"));
    deque_products.push_back(create_sample_product(3, "Chair", 199, "Furniture"));

    // Convert to owned Vec for querying
    let deque_vec: Vec<Product> = deque_products.iter().cloned().collect();
    let deque_query = Query::new(&deque_vec);
    let deque_count = deque_query.count();
    println!("  VecDeque: {} total items", deque_count);

    // More efficient: use make_contiguous for zero-copy slice access
    let contiguous = deque_products.make_contiguous();
    let contiguous_query = Query::new(contiguous);
    println!("  VecDeque (zero-copy): {} items", contiguous_query.count());

    // ============================================================================
    // CONTAINER 3: HashSet<T>
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 3: HashSet<T>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut set_products = HashSet::new();
    set_products.insert(create_sample_product(1, "Tablet", 499, "Electronics"));
    set_products.insert(create_sample_product(2, "Phone", 799, "Electronics"));
    set_products.insert(create_sample_product(3, "Lamp", 39, "Furniture"));

    // Collect from HashSet to Vec for querying
    let set_vec: Vec<Product> = set_products.iter().cloned().collect();
    let set_query = Query::new(&set_vec)
        .where_(Product::price(), |&p| p > 500);
    let expensive = set_query.all();
    println!("  HashSet: {} expensive items", expensive.len());

    // Lazy on HashSet (convert to owned Vec first)
    let set_owned: Vec<Product> = set_products.iter().cloned().collect();
    let lazy_set: Vec<_> = LazyQuery::new(&set_owned)
        .where_(Product::category(), |cat| cat == "Electronics")
        .collect();
    println!("  HashSet (lazy): {} electronics", lazy_set.len());

    // ============================================================================
    // CONTAINER 4: BTreeSet<T>
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 4: BTreeSet<T>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut btree_set = BTreeSet::new();
    btree_set.insert(create_sample_product(1, "Webcam", 79, "Electronics"));
    btree_set.insert(create_sample_product(2, "Microphone", 129, "Electronics"));
    btree_set.insert(create_sample_product(3, "Bookshelf", 149, "Furniture"));

    let btree_vec: Vec<Product> = btree_set.iter().cloned().collect();
    let btree_query = Query::new(&btree_vec);
    println!("  BTreeSet: {} total items (sorted order!)", btree_query.count());
    
    // BTreeSet items are in sorted order
    for (i, item) in btree_vec.iter().take(3).enumerate() {
        println!("    {}. {} (ID: {})", i + 1, item.name, item.id);
    }

    // ============================================================================
    // CONTAINER 5: HashMap<K, V> - Query Values
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 5: HashMap<K, V> - Querying values");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut map_products = HashMap::new();
    map_products.insert("prod1", create_sample_product(1, "Speaker", 199, "Electronics"));
    map_products.insert("prod2", create_sample_product(2, "Headphones", 149, "Electronics"));
    map_products.insert("prod3", create_sample_product(3, "Ottoman", 249, "Furniture"));

    // Query HashMap values (convert to owned Vec)
    let map_vec: Vec<Product> = map_products.values().cloned().collect();
    let map_query = Query::new(&map_vec)
        .where_(Product::category(), |cat| cat == "Electronics");
    let electronics = map_query.all();
    println!("  HashMap: {} electronics", electronics.len());

    // ============================================================================
    // CONTAINER 6: BTreeMap<K, V> - Query Values
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 6: BTreeMap<K, V> - Querying values");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut btree_map = BTreeMap::new();
    btree_map.insert(1, create_sample_product(1, "Router", 89, "Electronics"));
    btree_map.insert(2, create_sample_product(2, "Switch", 129, "Electronics"));
    btree_map.insert(3, create_sample_product(3, "Sofa", 899, "Furniture"));

    let btree_map_vec: Vec<Product> = btree_map.values().cloned().collect();
    let btree_map_query = Query::new(&btree_map_vec);
    let avg_price = btree_map_query.sum(Product::price()) as f64 / btree_map.len() as f64;
    println!("  BTreeMap: Average price ${:.2}", avg_price);

    // ============================================================================
    // CONTAINER 7: Arrays [T; N]
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 7: Arrays [T; N]");
    println!("═══════════════════════════════════════════════════════════════\n");

    let array_products = [
        create_sample_product(1, "USB Cable", 15, "Electronics"),
        create_sample_product(2, "HDMI Cable", 25, "Electronics"),
        create_sample_product(3, "Power Strip", 35, "Electronics"),
    ];

    // Query array directly (as slice)
    let array_query = Query::new(&array_products);
    let total = array_query.sum(Product::price());
    println!("  Array: Total value ${}", total);

    // Lazy on array
    let lazy_array: Vec<_> = LazyQuery::new(&array_products)
        .where_(Product::price(), |&p| p > 20)
        .collect();
    println!("  Array (lazy): {} items over $20", lazy_array.len());

    // ============================================================================
    // CONTAINER 8: LinkedList<T>
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 8: LinkedList<T>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut list_products = LinkedList::new();
    list_products.push_back(create_sample_product(1, "SSD", 159, "Electronics"));
    list_products.push_back(create_sample_product(2, "HDD", 79, "Electronics"));

    let list_vec: Vec<Product> = list_products.iter().cloned().collect();
    let list_query = Query::new(&list_vec);
    println!("  LinkedList: {} items", list_query.count());

    // ============================================================================
    // CONTAINER 9: Option<T> and Result<T, E>
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Container 9: Option<T> and Result<T, E>");
    println!("═══════════════════════════════════════════════════════════════\n");

    let maybe_product = Some(create_sample_product(1, "Mystery Box", 99, "Special"));
    
    if let Some(ref product) = maybe_product {
        let option_query = Query::new(std::slice::from_ref(product));
        println!("  Option (Some): {} items", option_query.count());
    }

    let none_product: Option<Product> = None;
    let none_count = none_product.iter().count();
    println!("  Option (None): {} items", none_count);

    // ============================================================================
    // Summary
    // ============================================================================
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Supported Containers Summary                                  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ Supported container types:");
    println!("   • Vec<T>              - Standard vector");
    println!("   • &[T]                - Slices");
    println!("   • [T; N]              - Fixed-size arrays");
    println!("   • VecDeque<T>         - Double-ended queue");
    println!("   • LinkedList<T>       - Doubly-linked list");
    println!("   • HashSet<T>          - Unordered set");
    println!("   • BTreeSet<T>         - Ordered set");
    println!("   • HashMap<K, V>       - Query values");
    println!("   • BTreeMap<K, V>      - Query values (sorted)");
    println!("   • Option<T>           - 0 or 1 item");
    println!("   • Result<T, E>        - 0 or 1 item\n");

    println!("📝 Usage patterns:");
    println!("   • Direct: Query::new(&container) for Vec, slices, arrays");
    println!("   • Convert: Collect to Vec for Sets and Maps");
    println!("   • Lazy: LazyQuery::new(&slice) for any slice\n");

    println!("💡 Tips:");
    println!("   • Vec/slice: Direct support, most efficient");
    println!("   • Sets: Iterate to Vec, then query");
    println!("   • Maps: Use .values().collect() to query values");
    println!("   • VecDeque: Use .as_slices() for zero-copy");
    println!("   • For custom types: Implement Queryable trait\n");

    println!("✓ Container support demo complete!\n");
}

