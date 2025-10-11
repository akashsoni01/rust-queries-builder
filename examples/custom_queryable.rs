// Demonstrates how to implement Queryable for custom container types
// Shows how any custom collection can be made queryable
// cargo run --example custom_queryable

use rust_queries_builder::{Query, LazyQuery, Queryable};
use key_paths_derive::Keypaths;
use std::collections::VecDeque;

#[derive(Debug, Clone, Keypaths)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
}

// ============================================================================
// CUSTOM CONTAINER 1: Paginated Collection
// ============================================================================

/// A container that stores items in pages for efficient memory usage
struct PaginatedCollection<T> {
    pages: Vec<Vec<T>>,
    page_size: usize,
}

impl<T> PaginatedCollection<T> {
    fn new(page_size: usize) -> Self {
        Self {
            pages: Vec::new(),
            page_size,
        }
    }

    fn add(&mut self, item: T) {
        if self.pages.is_empty() || self.pages.last().unwrap().len() >= self.page_size {
            self.pages.push(Vec::new());
        }
        self.pages.last_mut().unwrap().push(item);
    }

    fn total_items(&self) -> usize {
        self.pages.iter().map(|p| p.len()).sum()
    }
}

// Implement Queryable to make it queryable!
impl<T> Queryable<T> for PaginatedCollection<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.pages.iter().flat_map(|page| page.iter()))
    }
}

// ============================================================================
// CUSTOM CONTAINER 2: Circular Buffer
// ============================================================================

/// A circular buffer with fixed capacity
struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

// Implement Queryable
impl<T> Queryable<T> for CircularBuffer<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.buffer.iter())
    }
}

// ============================================================================
// CUSTOM CONTAINER 3: Filtered Storage
// ============================================================================

/// A container that only stores items matching a predicate
struct FilteredStorage<T, F>
where
    F: Fn(&T) -> bool,
{
    items: Vec<T>,
    filter: F,
}

impl<T, F> FilteredStorage<T, F>
where
    F: Fn(&T) -> bool,
{
    fn new(filter: F) -> Self {
        Self {
            items: Vec::new(),
            filter,
        }
    }

    fn add(&mut self, item: T) -> bool {
        if (self.filter)(&item) {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Implement Queryable
impl<T, F> Queryable<T> for FilteredStorage<T, F>
where
    F: Fn(&T) -> bool,
{
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.items.iter())
    }
}

// ============================================================================
// CUSTOM CONTAINER 4: CategoryIndex
// ============================================================================

/// A specialized container that maintains an index by category
use std::collections::HashMap;

struct CategoryIndex<T> {
    by_category: HashMap<String, Vec<T>>,
}

impl<T> CategoryIndex<T> {
    fn new() -> Self {
        Self {
            by_category: HashMap::new(),
        }
    }

    fn add(&mut self, category: String, item: T) {
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(item);
    }

    fn total_items(&self) -> usize {
        self.by_category.values().map(|v| v.len()).sum()
    }

    fn categories(&self) -> Vec<&String> {
        self.by_category.keys().collect()
    }
}

// Implement Queryable - iterates over all items across all categories
impl<T> Queryable<T> for CategoryIndex<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.by_category.values().flat_map(|vec| vec.iter()))
    }
}

// ============================================================================
// CUSTOM CONTAINER 5: LazyLoader (Simulated)
// ============================================================================

/// A container that simulates lazy loading from database/file
struct LazyLoader<T> {
    loaded_items: Vec<T>,
    total_count: usize,
}

impl<T> LazyLoader<T> {
    fn new(items: Vec<T>) -> Self {
        let total = items.len();
        Self {
            loaded_items: items,
            total_count: total,
        }
    }

    fn loaded_count(&self) -> usize {
        self.loaded_items.len()
    }

    fn total_count(&self) -> usize {
        self.total_count
    }
}

// Implement Queryable
impl<T> Queryable<T> for LazyLoader<T> {
    fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.loaded_items.iter())
    }
}

// ============================================================================
// MAIN - Demonstrations
// ============================================================================

fn create_sample_product(id: u32, name: &str, price: f64, category: &str, in_stock: bool) -> Product {
    Product {
        id,
        name: name.to_string(),
        price,
        category: category.to_string(),
        in_stock,
    }
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Custom Queryable Implementation Demo                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // ============================================================================
    // DEMO 1: PaginatedCollection
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("Demo 1: PaginatedCollection (custom container)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut paginated = PaginatedCollection::new(3); // 3 items per page
    
    paginated.add(create_sample_product(1, "Laptop", 999.0, "Electronics", true));
    paginated.add(create_sample_product(2, "Mouse", 29.0, "Electronics", true));
    paginated.add(create_sample_product(3, "Keyboard", 129.0, "Electronics", true));
    paginated.add(create_sample_product(4, "Desk", 299.0, "Furniture", true));
    paginated.add(create_sample_product(5, "Chair", 199.0, "Furniture", false));

    println!("  Created paginated collection:");
    println!("    Total items: {}", paginated.total_items());
    println!("    Pages: {}", paginated.pages.len());

    // Now we can query it using the Queryable trait!
    // Collect to owned Vec for querying
    let items: Vec<Product> = paginated.query_iter().cloned().collect();
    let query = Query::new(&items)
        .where_(Product::category_r(), |cat| cat == "Electronics");
    let electronics = query.all();
    
    println!("\n  Querying paginated collection:");
    println!("    Electronics found: {}", electronics.len());
    for product in electronics {
        println!("      • {}: ${:.2}", product.name, product.price);
    }

    // ============================================================================
    // DEMO 2: CircularBuffer
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 2: CircularBuffer (fixed capacity)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut circular = CircularBuffer::new(3); // Capacity: 3
    
    circular.push(create_sample_product(1, "Product 1", 100.0, "A", true));
    circular.push(create_sample_product(2, "Product 2", 200.0, "B", true));
    circular.push(create_sample_product(3, "Product 3", 300.0, "C", true));
    circular.push(create_sample_product(4, "Product 4", 400.0, "D", true)); // Pushes out Product 1

    println!("  Circular buffer (capacity 3, added 4 items):");
    println!("    Current size: {}", circular.len());

    // Query the circular buffer
    let circ_items: Vec<Product> = circular.query_iter().cloned().collect();
    let circ_query = Query::new(&circ_items);
    let avg_price = circ_query.avg(Product::price_r()).unwrap_or(0.0);
    
    println!("\n  Querying circular buffer:");
    println!("    Average price: ${:.2}", avg_price);
    println!("    Items:");
    for (i, product) in circ_items.iter().enumerate() {
        println!("      {}. {}: ${:.2}", i + 1, product.name, product.price);
    }

    // ============================================================================
    // DEMO 3: FilteredStorage
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 3: FilteredStorage (auto-filtering container)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut filtered = FilteredStorage::new(|p: &Product| p.price < 200.0);
    
    println!("  FilteredStorage (only accepts items < $200):");
    println!("    Adding Laptop ($999): {}", filtered.add(create_sample_product(1, "Laptop", 999.0, "Electronics", true)));
    println!("    Adding Mouse ($29): {}", filtered.add(create_sample_product(2, "Mouse", 29.0, "Electronics", true)));
    println!("    Adding Keyboard ($129): {}", filtered.add(create_sample_product(3, "Keyboard", 129.0, "Electronics", true)));
    println!("    Total stored: {}", filtered.len());

    // Query the filtered storage
    let filt_items: Vec<Product> = filtered.query_iter().cloned().collect();
    let filt_query = Query::new(&filt_items)
        .where_(Product::in_stock_r(), |&v| v);
    let in_stock = filt_query.all();
    
    println!("\n  Querying filtered storage:");
    println!("    In stock items: {}", in_stock.len());

    // ============================================================================
    // DEMO 4: CategoryIndex
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 4: CategoryIndex (indexed by category)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut category_index = CategoryIndex::new();
    
    category_index.add("Electronics".to_string(), create_sample_product(1, "Monitor", 349.0, "Electronics", true));
    category_index.add("Electronics".to_string(), create_sample_product(2, "Webcam", 79.0, "Electronics", true));
    category_index.add("Furniture".to_string(), create_sample_product(3, "Desk", 299.0, "Furniture", true));
    category_index.add("Furniture".to_string(), create_sample_product(4, "Lamp", 39.0, "Furniture", true));

    println!("  CategoryIndex:");
    println!("    Categories: {:?}", category_index.categories());
    println!("    Total items: {}", category_index.total_items());

    // Query across all categories
    let idx_items: Vec<Product> = category_index.query_iter().cloned().collect();
    let idx_query = Query::new(&idx_items);
    let expensive = idx_query
        .where_(Product::price_r(), |&p| p > 100.0);
    let expensive_items = expensive.all();
    
    println!("\n  Querying category index:");
    println!("    Expensive items (>$100): {}", expensive_items.len());
    for product in expensive_items {
        println!("      • {}: ${:.2} ({})", product.name, product.price, product.category);
    }

    // ============================================================================
    // DEMO 5: LazyLoader
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 5: LazyLoader (simulated lazy loading)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let loader = LazyLoader::new(vec![
        create_sample_product(1, "Item 1", 50.0, "A", true),
        create_sample_product(2, "Item 2", 150.0, "B", true),
        create_sample_product(3, "Item 3", 250.0, "C", true),
    ]);

    println!("  LazyLoader:");
    println!("    Total available: {}", loader.total_count());
    println!("    Currently loaded: {}", loader.loaded_count());

    // Query loaded items
    let loader_items: Vec<Product> = loader.query_iter().cloned().collect();
    let loader_query = Query::new(&loader_items);
    let total_value = loader_query.sum(Product::price_r());
    
    println!("\n  Querying lazy loader:");
    println!("    Total value: ${:.2}", total_value);

    // ============================================================================
    // DEMO 6: Custom Container with LazyQuery
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 6: Using LazyQuery with custom containers");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut circular = CircularBuffer::new(5);
    for i in 1..=10 {
        circular.push(create_sample_product(
            i,
            &format!("Product {}", i),
            i as f64 * 50.0,
            if i % 2 == 0 { "Even" } else { "Odd" },
            true,
        ));
    }

    println!("  Circular buffer (capacity 5, added 10 items):");
    println!("    Current size: {}", circular.len());

    // Use LazyQuery for early termination!
    let circ_vec: Vec<Product> = circular.query_iter().cloned().collect();
    let first_expensive = LazyQuery::new(&circ_vec)
        .where_(Product::price_r(), |&p| p > 300.0)
        .first();

    if let Some(product) = first_expensive {
        println!("\n  First expensive item (lazy query):");
        println!("    {}: ${:.2}", product.name, product.price);
    }

    // ============================================================================
    // DEMO 7: Implementing Queryable for Wrapper Types
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 7: Queryable for wrapper types");
    println!("═══════════════════════════════════════════════════════════════\n");

    /// A simple wrapper around Vec with metadata
    struct VersionedCollection<T> {
        items: Vec<T>,
        version: u32,
        last_modified: String,
    }

    impl<T> Queryable<T> for VersionedCollection<T> {
        fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
            Box::new(self.items.iter())
        }
    }

    let versioned = VersionedCollection {
        items: vec![
            create_sample_product(1, "V1 Product", 99.0, "Test", true),
            create_sample_product(2, "V2 Product", 199.0, "Test", true),
        ],
        version: 2,
        last_modified: "2025-10-11".to_string(),
    };

    println!("  VersionedCollection:");
    println!("    Version: {}", versioned.version);
    println!("    Last modified: {}", versioned.last_modified);

    let versioned_items: Vec<Product> = versioned.query_iter().cloned().collect();
    let query = Query::new(&versioned_items);
    println!("    Items: {}", query.count());

    // ============================================================================
    // DEMO 8: Real-World Example - Cache with TTL
    // ============================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Demo 8: Cache container (real-world example)");
    println!("═══════════════════════════════════════════════════════════════\n");

    use std::time::{Duration, SystemTime};

    struct CachedItem<T> {
        item: T,
        inserted_at: SystemTime,
        ttl: Duration,
    }

    struct Cache<T> {
        items: Vec<CachedItem<T>>,
    }

    impl<T> Cache<T> {
        fn new() -> Self {
            Self { items: Vec::new() }
        }

        fn insert(&mut self, item: T, ttl: Duration) {
            self.items.push(CachedItem {
                item,
                inserted_at: SystemTime::now(),
                ttl,
            });
        }

        fn valid_items(&self) -> Vec<&T> {
            let now = SystemTime::now();
            self.items
                .iter()
                .filter(|cached| {
                    now.duration_since(cached.inserted_at)
                        .map_or(false, |elapsed| elapsed < cached.ttl)
                })
                .map(|cached| &cached.item)
                .collect()
        }
    }

    // Implement Queryable to query only valid (non-expired) items
    impl<T> Queryable<T> for Cache<T> {
        fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
            let now = SystemTime::now();
            Box::new(
                self.items
                    .iter()
                    .filter(move |cached| {
                        now.duration_since(cached.inserted_at)
                            .map_or(false, |elapsed| elapsed < cached.ttl)
                    })
                    .map(|cached| &cached.item),
            )
        }
    }

    let mut cache = Cache::new();
    cache.insert(create_sample_product(1, "Cached Item 1", 100.0, "A", true), Duration::from_secs(60));
    cache.insert(create_sample_product(2, "Cached Item 2", 200.0, "B", true), Duration::from_secs(60));

    let valid = cache.valid_items();
    println!("  Cache:");
    println!("    Total items: {}", cache.items.len());
    println!("    Valid items: {}", valid.len());

    // Query the cache
    let cache_items: Vec<Product> = cache.query_iter().cloned().collect();
    let cache_query = Query::new(&cache_items);
    println!("    Queryable items: {}", cache_query.count());

    // ============================================================================
    // Summary
    // ============================================================================
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Summary: Implementing Queryable                              ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ How to make any container queryable:\n");
    println!("  1. Implement the Queryable<T> trait:");
    println!("     ```rust");
    println!("     impl<T> Queryable<T> for MyContainer<T> {{");
    println!("         fn query_iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {{");
    println!("             Box::new(self.items.iter())");
    println!("         }}");
    println!("     }}");
    println!("     ```\n");

    println!("  2. Convert to Vec or slice for querying:");
    println!("     ```rust");
    println!("     let items: Vec<&Product> = container.query_iter().collect();");
    println!("     let query = Query::new(&items);");
    println!("     ```\n");

    println!("  3. Now use all query operations:");
    println!("     ```rust");
    println!("     let results = query.where_(...).all();");
    println!("     let count = query.count();");
    println!("     let total = query.sum(field);");
    println!("     ```\n");

    println!("📝 Custom containers demonstrated:");
    println!("   • PaginatedCollection - Items stored in pages");
    println!("   • CircularBuffer - Fixed-capacity FIFO buffer");
    println!("   • FilteredStorage - Auto-filtering container");
    println!("   • CategoryIndex - Indexed by category");
    println!("   • LazyLoader - Simulated lazy loading");
    println!("   • VersionedCollection - Wrapper with metadata");
    println!("   • Cache - TTL-based cache\n");

    println!("💡 Use cases:");
    println!("   • Database result wrappers");
    println!("   • Custom data structures");
    println!("   • Specialized collections");
    println!("   • Caches and buffers");
    println!("   • Event streams");
    println!("   • Any container that holds items!\n");

    println!("✓ Custom Queryable demo complete!\n");
}

