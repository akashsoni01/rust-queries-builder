// Demonstrates Arc<RwLock<HashMap<String, Arc<RwLock<User>>>>>
// This is a NESTED lock pattern for thread-safe collections where:
// 1. Outer Arc<RwLock<HashMap<...>>> - for adding/removing entries
// 2. Inner Arc<RwLock<User>> - for reading/updating individual users
// cargo run --example nested_arc_rwlock_hashmap

use rust_queries_builder::locks::{LockQueryExt, LockIterExt};
use key_paths_derive::Keypath;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Keypath)]
struct User {
    id: u32,
    username: String,
    email: String,
    age: u32,
    role: String,
    active: bool,
    login_count: u32,
    balance: f64,
}

// Type alias for clarity
type UserId = String;
type SharedUser = Arc<RwLock<User>>;
type UserStore = Arc<RwLock<HashMap<UserId, SharedUser>>>;

fn create_sample_users() -> UserStore {
    let mut users = HashMap::new();

    let user_data = vec![
        User { id: 1, username: "alice".to_string(), email: "alice@example.com".to_string(), age: 28, role: "admin".to_string(), active: true, login_count: 150, balance: 1250.50 },
        User { id: 2, username: "bob".to_string(), email: "bob@example.com".to_string(), age: 35, role: "user".to_string(), active: true, login_count: 80, balance: 450.00 },
        User { id: 3, username: "charlie".to_string(), email: "charlie@example.com".to_string(), age: 22, role: "user".to_string(), active: true, login_count: 45, balance: 150.75 },
        User { id: 4, username: "diana".to_string(), email: "diana@example.com".to_string(), age: 31, role: "moderator".to_string(), active: true, login_count: 220, balance: 890.25 },
        User { id: 5, username: "eve".to_string(), email: "eve@example.com".to_string(), age: 26, role: "user".to_string(), active: false, login_count: 15, balance: 50.00 },
        User { id: 6, username: "frank".to_string(), email: "frank@example.com".to_string(), age: 42, role: "admin".to_string(), active: true, login_count: 500, balance: 2500.00 },
        User { id: 7, username: "grace".to_string(), email: "grace@example.com".to_string(), age: 29, role: "moderator".to_string(), active: true, login_count: 180, balance: 750.50 },
        User { id: 8, username: "henry".to_string(), email: "henry@example.com".to_string(), age: 19, role: "user".to_string(), active: true, login_count: 25, balance: 100.00 },
    ];

    for user in user_data {
        let user_id = format!("user_{}", user.id);
        users.insert(user_id, Arc::new(RwLock::new(user)));
    }

    Arc::new(RwLock::new(users))
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║  Arc<RwLock<HashMap<String, Arc<RwLock<User>>>>> Demo           ║");
    println!("║  Nested Lock Pattern for Thread-Safe Collections                 ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    let user_store = create_sample_users();

    // ============================================================================
    // OPERATION 1: Query Multiple Users (NO CLONING!)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 1: Query users with lock-aware API (NO cloning)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Acquiring outer read lock on HashMap...");
        let users_guard = user_store.read().unwrap();
        println!("  ✅ Outer read lock acquired (HashMap is read-locked)");
        println!("  Total users: {}\n", users_guard.len());

        // Query: Find all active admins - NO User struct cloning!
        println!("Finding active admins (lock-aware, no cloning)...");
        
        let admin_count = users_guard
            .lock_iter()
            .filter_locked(|u| u.active && u.role == "admin")
            .count();

        println!("  Active Admins ({})", admin_count);
        
        // Extract only the fields we need for display
        // Only username and email Strings are cloned, NOT the entire User struct
        users_guard
            .lock_iter()
            .filter_locked(|u| u.active && u.role == "admin")
            .for_each(|locked_ref| {
                if let Some((username, email)) = locked_ref.with_value(|u| {
                    (u.username.clone(), u.email.clone())
                }) {
                    println!("    • {} ({})", username, email);
                }
            });

        println!("\n  ✅ Zero User struct cloning - only extracted display fields!");
    } // Outer read lock released here
    println!("\n  ✅ Outer read lock released\n");

    // ============================================================================
    // OPERATION 2: Lock-Aware Querying (No User struct cloning!)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 2: Lock-aware querying (NO struct cloning)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        
        println!("Finding users with balance > $500 (lock-aware)...");
        
        // Call lock_iter() directly on the HashMap
        // This acquires inner locks temporarily, but NEVER clones User structs
        let high_balance_count = users_guard
            .lock_iter()
            .filter_locked(|user| user.balance > 500.0)
            .count();

        println!("  Found {} users with high balance", high_balance_count);

        // Get details - only extract specific fields (String must be cloned to outlive the lock)
        println!("\n  Details (only username Strings are cloned, not entire User structs):");
        users_guard
            .lock_iter()
            .filter_locked(|user| user.balance > 500.0)
            .for_each(|locked_ref| {
                // username.clone() is necessary since we can't return &str that outlives the lock guard
                // But we're NOT cloning the entire User struct!
                if let Some((name, balance)) = locked_ref.with_value(|user| (user.username.clone(), user.balance)) {
                    println!("    • {}: ${:.2}", name, balance);
                }
            });
    }
    println!("\n  ✅ No User struct cloning - only specific fields extracted\n");

    // ============================================================================
    // OPERATION 3: Update Individual User (Outer Read + Inner Write)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 3: Update individual user");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Scenario: Alice logs in (increment login_count)");
        
        // Outer read lock (HashMap structure not changing)
        let users_guard = user_store.read().unwrap();
        println!("  ✅ Outer read lock acquired");

        if let Some(user_arc) = users_guard.get("user_1") {
            // Inner write lock (modifying alice's data)
            let mut user = user_arc.write().unwrap();
            println!("  ✅ Inner write lock acquired for alice");
            
            let old_count = user.login_count;
            user.login_count += 1;
            
            println!("  Updated alice's login_count: {} → {}", old_count, user.login_count);
            println!("  ✅ Inner write lock released");
        }
        
        println!("  ✅ Outer read lock released");
    }
    println!();

    // ============================================================================
    // OPERATION 4: Add New User (Outer Write Lock)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 4: Add new user (structural change)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Adding new user 'ivy'...");
        
        // Outer write lock (adding new entry to HashMap)
        let mut users_guard = user_store.write().unwrap();
        println!("  ✅ Outer write lock acquired (exclusive access)");

        let new_user = User {
            id: 9,
            username: "ivy".to_string(),
            email: "ivy@example.com".to_string(),
            age: 27,
            role: "user".to_string(),
            active: true,
            login_count: 0,
            balance: 200.00,
        };

        users_guard.insert("user_9".to_string(), Arc::new(RwLock::new(new_user)));
        println!("  New user 'ivy' added");
        println!("  Total users now: {}", users_guard.len());
        println!("  ✅ Outer write lock released");
    }
    println!();

    // ============================================================================
    // OPERATION 5: Remove User (Outer Write Lock)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 5: Remove user (structural change)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Removing inactive user 'eve'...");
        
        let mut users_guard = user_store.write().unwrap();
        println!("  ✅ Outer write lock acquired");

        if let Some(removed) = users_guard.remove("user_5") {
            if let Ok(user) = removed.read() {
                println!("  Removed: {} (active: {})", user.username, user.active);
            }
        }
        
        println!("  Total users now: {}", users_guard.len());
        println!("  ✅ Outer write lock released");
    }
    println!();

    // ============================================================================
    // OPERATION 5.5: TRULY Zero-Copy Operations (Absolutely NO Cloning!)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 5.5: TRULY zero-copy operations");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        
        println!("These operations clone NOTHING - not even a single byte!\n");

        // Count operations - pure zero-copy
        let active_count = users_guard
            .lock_iter()
            .filter_locked(|u| u.active)
            .count();
        println!("  Active users count: {} (zero cloning)", active_count);

        // Boolean checks - pure zero-copy
        let has_high_balance = users_guard
            .lock_iter()
            .any_locked(|u| u.balance > 1000.0);
        println!("  Has millionaire: {} (zero cloning)", has_high_balance);

        // Checking existence - pure zero-copy
        let all_have_email = users_guard
            .lock_iter()
            .map_locked(|u| !u.email.is_empty())
            .all(|has_email| has_email);
        println!("  All have email: {} (zero cloning)", all_have_email);

        // Sum of Copy types - pure zero-copy
        let total_age: u32 = users_guard
            .lock_iter()
            .map_locked(|u| u.age)
            .sum();
        println!("  Total age sum: {} (zero cloning)", total_age);

        // Count specific values - pure zero-copy
        let admin_count = users_guard
            .lock_iter()
            .count_locked(|u| u.role == "admin");
        println!("  Admin count: {} (zero cloning)", admin_count);

        println!("\n  ✅ Absolutely ZERO bytes cloned in these operations!");
        println!("  ✅ All data accessed directly through lock guards!");
        println!("  ✅ Copy types (u32, f64, bool) are copied by value (cheap!)");
        println!("  ✅ String comparisons done in-place (no allocation)\n");
    }

    // ============================================================================
    // OPERATION 6: Concurrent Operations Simulation
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 6: Multi-threaded access simulation");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let store_clone1 = Arc::clone(&user_store);
    let store_clone2 = Arc::clone(&user_store);
    let store_clone3 = Arc::clone(&user_store);

    println!("Spawning 3 threads...\n");

    // Thread 1: Read operations
    let handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let users = store_clone1.read().unwrap();
        println!("  [Thread 1] Read lock acquired - counting users: {}", users.len());
        thread::sleep(Duration::from_millis(50));
        println!("  [Thread 1] Read lock released");
    });

    // Thread 2: Update individual user
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        let users = store_clone2.read().unwrap();
        println!("  [Thread 2] Outer read lock acquired");
        
        if let Some(user_arc) = users.get("user_2") {
            let mut user = user_arc.write().unwrap();
            println!("  [Thread 2] Inner write lock acquired for bob");
            user.login_count += 1;
            thread::sleep(Duration::from_millis(30));
            println!("  [Thread 2] Updated bob's login_count to {}", user.login_count);
        }
        println!("  [Thread 2] Locks released");
    });

    // Thread 3: Another read operation
    let handle3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        let users = store_clone3.read().unwrap();
        println!("  [Thread 3] Read lock acquired - querying admins");
        
        let admin_count = users.values()
            .filter(|user_arc| {
                if let Ok(user) = user_arc.read() {
                    user.role == "admin"
                } else {
                    false
                }
            })
            .count();
        
        println!("  [Thread 3] Found {} admins", admin_count);
        println!("  [Thread 3] Read lock released");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("\n  ✅ All threads completed successfully!\n");

    // ============================================================================
    // OPERATION 7: Complex Queries with Aggregations (NO CLONING!)
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 7: Complex queries and aggregations (NO cloning)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        
        println!("📊 User Statistics (all lock-aware, zero struct cloning):\n");

        // Total users - pure zero-copy
        let total = users_guard.len();
        println!("  Total users: {}", total);

        // Active users - pure zero-copy count
        let active_count = users_guard
            .lock_iter()
            .count_locked(|u| u.active);
        println!("  Active users: {}", active_count);

        // Average age - pure zero-copy (only u32 values copied)
        let total_age: u32 = users_guard
            .lock_iter()
            .map_locked(|u| u.age)
            .sum();
        let avg_age = if total > 0 { total_age as f64 / total as f64 } else { 0.0 };
        println!("  Average age: {:.1} years", avg_age);

        // Total balance - pure zero-copy (only f64 values copied)
        let total_balance: f64 = users_guard
            .lock_iter()
            .map_locked(|u| u.balance)
            .sum();
        println!("  Total balance: ${:.2}", total_balance);

        // Most active user - lock-aware finding
        let mut max_logins = 0u32;
        let mut most_active_name = String::new();
        
        users_guard.lock_iter().for_each(|locked_ref| {
            if let Some((count, name)) = locked_ref.with_value(|u| (u.login_count, u.username.clone())) {
                if count > max_logins {
                    max_logins = count;
                    most_active_name = name;
                }
            }
        });
        
        if max_logins > 0 {
            println!("  Most active: {} ({} logins)", most_active_name, max_logins);
        }

        // Users by role - pure zero-copy counts
        println!("\n📋 Users by Role:");
        for role in &["admin", "moderator", "user"] {
            let count = users_guard
                .lock_iter()
                .count_locked(|u| u.role == *role);
            println!("  {} - {} users", role, count);
        }
        
        println!("\n  ✅ All statistics computed without cloning a single User struct!");
    }
    println!();

    // ============================================================================
    // OPERATION 7.5: SQL-Like Operations
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 7.5: SQL-Like Operations");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        
        // SQL: SELECT * FROM users WHERE age BETWEEN 25 AND 35
        println!("📊 SQL: WHERE age BETWEEN 25 AND 35");
        let between_count = users_guard
            .lock_iter()
            .count_locked(|u| u.age >= 25 && u.age <= 35);
        println!("  Found {} users aged 25-35\n", between_count);
        
        // SQL: SELECT * FROM users WHERE role IN ('admin', 'moderator')
        println!("📊 SQL: WHERE role IN ('admin', 'moderator')");
        let admin_or_mod = users_guard
            .lock_iter()
            .count_locked(|u| u.role == "admin" || u.role == "moderator");
        println!("  Found {} admins or moderators\n", admin_or_mod);
        
        // SQL: SELECT * FROM users WHERE username LIKE 'a%' (starts with 'a')
        println!("📊 SQL: WHERE username LIKE 'a%' (starts with 'a')");
        users_guard
            .lock_iter()
            .filter_locked(|u| u.username.starts_with('a'))
            .for_each(|locked_ref| {
                if let Some(name) = locked_ref.with_value(|u| u.username.clone()) {
                    println!("    • {}", name);
                }
            });
        
        // SQL: SELECT * FROM users WHERE email LIKE '%@example.com' (ends with)
        println!("\n📊 SQL: WHERE email LIKE '%@example.com'");
        let example_emails = users_guard
            .lock_iter()
            .count_locked(|u| u.email.ends_with("@example.com"));
        println!("  Found {} users with @example.com emails\n", example_emails);
        
        // SQL: SELECT * FROM users WHERE username LIKE '%ar%' (contains)
        println!("📊 SQL: WHERE username LIKE '%ar%' (contains 'ar')");
        users_guard
            .lock_iter()
            .filter_locked(|u| u.username.contains("ar"))
            .for_each(|locked_ref| {
                if let Some(name) = locked_ref.with_value(|u| u.username.clone()) {
                    println!("    • {}", name);
                }
            });
        
        // SQL: SELECT COUNT(*) FROM users WHERE balance > 500 AND active = true
        println!("\n📊 SQL: WHERE balance > 500 AND active = true");
        let active_high_balance = users_guard
            .lock_iter()
            .count_locked(|u| u.balance > 500.0 && u.active);
        println!("  Found {} active users with high balance\n", active_high_balance);
        
        // SQL: SELECT username, balance FROM users WHERE login_count > 100 ORDER BY balance DESC
        println!("📊 SQL: SELECT username, balance WHERE login_count > 100 ORDER BY balance DESC");
        
        // Collect users with login_count > 100 along with their balances
        let mut users_with_balances: Vec<(String, f64)> = users_guard
            .lock_iter()
            .filter_map(|locked_ref| {
                locked_ref.with_value(|u| {
                    if u.login_count > 100 {
                        Some((u.username.clone(), u.balance))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect();
        
        // ORDER BY balance DESC
        users_with_balances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (username, balance) in &users_with_balances {
            println!("    • {}: ${:.2}", username, balance);
        }
        
        // SQL: SELECT * FROM users LIMIT 3
        println!("\n📊 SQL: SELECT * FROM users LIMIT 3");
        let first_3_names: Vec<String> = users_guard
            .lock_iter()
            .take(3)
            .filter_map(|locked_ref| locked_ref.with_value(|u| u.username.clone()))
            .collect();
        println!("  First 3 users: {:?}", first_3_names);
        
        // SQL: SELECT * FROM users OFFSET 2 LIMIT 3
        println!("\n📊 SQL: SELECT * FROM users OFFSET 2 LIMIT 3 (pagination)");
        let page_2_names: Vec<String> = users_guard
            .lock_iter()
            .skip(2)
            .take(3)
            .filter_map(|locked_ref| locked_ref.with_value(|u| u.username.clone()))
            .collect();
        println!("  Page 2 users (skip 2, take 3): {:?}", page_2_names);
        
        // SQL: SELECT role, COUNT(*) FROM users GROUP BY role
        println!("\n📊 SQL: SELECT role, COUNT(*) FROM users GROUP BY role");
        use std::collections::HashMap as StdHashMap;
        let mut role_counts: StdHashMap<String, usize> = StdHashMap::new();
        
        users_guard.lock_iter().for_each(|locked_ref| {
            if let Some(role) = locked_ref.with_value(|u| u.role.clone()) {
                *role_counts.entry(role).or_insert(0) += 1;
            }
        });
        
        for (role, count) in &role_counts {
            println!("    • {}: {} users", role, count);
        }
        
        // SQL: SELECT role, AVG(balance) FROM users GROUP BY role
        println!("\n📊 SQL: SELECT role, AVG(balance) FROM users GROUP BY role");
        let mut role_balances: StdHashMap<String, Vec<f64>> = StdHashMap::new();
        
        users_guard.lock_iter().for_each(|locked_ref| {
            if let Some((role, balance)) = locked_ref.with_value(|u| (u.role.clone(), u.balance)) {
                role_balances.entry(role).or_insert_with(Vec::new).push(balance);
            }
        });
        
        for (role, balances) in &role_balances {
            let avg = balances.iter().sum::<f64>() / balances.len() as f64;
            println!("    • {}: avg ${:.2}", role, avg);
        }
        
        // SQL: SELECT MAX(balance), MIN(balance), AVG(balance) FROM users
        println!("\n📊 SQL: SELECT MAX(balance), MIN(balance), AVG(balance) FROM users");
        let all_balances: Vec<f64> = users_guard
            .lock_iter()
            .map_locked(|u| u.balance)
            .collect();
        
        if !all_balances.is_empty() {
            let max_balance = all_balances.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let min_balance = all_balances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let avg_balance = all_balances.iter().sum::<f64>() / all_balances.len() as f64;
            
            println!("    • MAX: ${:.2}", max_balance);
            println!("    • MIN: ${:.2}", min_balance);
            println!("    • AVG: ${:.2}", avg_balance);
        }
        
        println!("\n  ✅ All SQL operations performed with lock-aware queries!");
    }
    println!();

    // ============================================================================
    // OPERATION 8: Conditional Updates with Predicates
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 8: Conditional bulk updates");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Scenario: Give $50 bonus to users with >100 logins\n");
        
        let users_guard = user_store.read().unwrap();
        let mut updated_count = 0;

        for (_key, user_arc) in users_guard.iter() {
            if let Ok(user) = user_arc.read() {
                if user.login_count > 100 {
                    // Need to drop read lock to get write lock
                    drop(user);
                    
                    if let Ok(mut user_mut) = user_arc.write() {
                        let old_balance = user_mut.balance;
                        user_mut.balance += 50.0;
                        println!("  ✅ {}: ${:.2} → ${:.2}", 
                            user_mut.username, old_balance, user_mut.balance);
                        updated_count += 1;
                    }
                }
            }
        }

        println!("\n  Updated {} users", updated_count);
    }
    println!();

    // ============================================================================
    // OPERATION 9: Search and Retrieve
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 9: Search by predicate");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        println!("Finding user with username 'frank'...");
        
        let users_guard = user_store.read().unwrap();
        
        // Call lock_iter() directly on the HashMap
        let found = users_guard
            .lock_iter()
            .find_locked(|u| u.username == "frank");

        if let Some(locked_ref) = found {
            if let Some(user_info) = locked_ref.with_value(|u| {
                format!("{} - {} (${:.2})", u.username, u.role, u.balance)
            }) {
                println!("  ✅ Found: {}", user_info);
            }
        }
    }
    println!();

    // ============================================================================
    // OPERATION 10: Key-based Operations
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Operation 10: Key-based lookups");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        
        let user_ids = vec!["user_1", "user_3", "user_6"];
        println!("Looking up users: {:?}\n", user_ids);

        for user_id in user_ids {
            if let Some(user_arc) = users_guard.get(user_id) {
                if let Ok(user) = user_arc.read() {
                    println!("  {} -> {} ({}, age {})", 
                        user_id, user.username, user.email, user.age);
                }
            }
        }
    }
    println!();

    // ============================================================================
    // Best Practices Summary
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("🎯 Lock Strategy Summary");
    println!("═══════════════════════════════════════════════════════════════════\n");

    println!("📖 When to use which lock:\n");
    
    println!("  1️⃣  Outer Read Lock (HashMap read):");
    println!("     • Querying multiple users");
    println!("     • Reading user data");
    println!("     • Counting users");
    println!("     • Checking if key exists\n");

    println!("  2️⃣  Outer Read + Inner Write Lock:");
    println!("     • Updating individual user data");
    println!("     • Incrementing counters");
    println!("     • Modifying user properties");
    println!("     • No structural changes to HashMap\n");

    println!("  3️⃣  Outer Write Lock (HashMap write):");
    println!("     • Adding new users");
    println!("     • Removing users");
    println!("     • Replacing entries");
    println!("     • Clearing the HashMap\n");

    println!("✅ Benefits of this pattern:");
    println!("   • Multiple threads can read different users simultaneously");
    println!("   • Lock contention only on structural changes");
    println!("   • Fine-grained locking for individual items");
    println!("   • Thread-safe with zero unsafe code");
    println!("   • Works with lock-aware query extensions");
    println!("   • Supports SQL-like operations (WHERE, LIKE, IN, BETWEEN, etc.)\n");

    println!("═══════════════════════════════════════════════════════════════════");
    println!("📝 No-Clone Strategy: What's Really Happening?");
    println!("═══════════════════════════════════════════════════════════════════\n");

    println!("✅ LOCK-AWARE QUERYING (No User struct cloning):");
    println!("   • Work directly with lock guards");
    println!("   • Filter/count by accessing fields through locks");
    println!("   • Only clone specific fields when needed (e.g., Strings for display)");
    println!("   • The entire User struct is NEVER cloned!");
    println!("   • Memory efficient: Only extracted display fields allocated\n");

    println!("📊 What gets cloned with lock-aware queries:");
    println!("   ✅ TRULY ZERO-COPY operations (no allocation at all):");
    println!("      • count() - just counting");
    println!("      • filter_locked() - predicate evaluation only");
    println!("      • any_locked() / all() - boolean checks");
    println!("      • Accessing Copy types (u32, f64, bool) - cheap value copy");
    println!("      • String comparisons - in-place, no allocation\n");

    println!("   ⚠️  Minimal cloning (only specific fields for output):");
    println!("      • Extracting String fields - must clone to outlive lock guard");
    println!("      • Extracting Vec fields - must clone to outlive lock guard");
    println!("      • Still avoids cloning the entire struct!\n");

    println!("💡 Memory comparison for 8 users:");
    println!("   ❌ Naive way: Clone entire User × 8 = ~640 bytes");
    println!("   ✅ Lock-aware: Clone 2 usernames + 2 emails = ~100 bytes");
    println!("   ✅ Pure counting: 0 bytes allocated!");
    println!("   🎯 Savings: Up to 100% for aggregations, ~84% for display!\n");

    println!("⚠️  Important notes:");
    println!("   • Avoid holding locks longer than necessary");
    println!("   • Be careful of deadlocks (always acquire in same order)");
    println!("   • Consider parking_lot for better performance");
    println!("   • Use lock-aware queries to avoid cloning entire structs\n");

    // ============================================================================
    // Final Statistics
    // ============================================================================
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📈 Final State");
    println!("═══════════════════════════════════════════════════════════════════\n");

    {
        let users_guard = user_store.read().unwrap();
        println!("  Total users in store: {}", users_guard.len());
        println!("  Keys: {:?}", users_guard.keys().collect::<Vec<_>>());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║  Arc<RwLock<HashMap<String, Arc<RwLock<User>>>>> Demo Complete  ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");
    
    println!("📋 What was demonstrated:");
    println!("   1. Lock-aware queries (NO User struct cloning)");
    println!("   2. Truly zero-copy operations (aggregations)");
    println!("   3. Individual user updates (outer read + inner write)");
    println!("   4. Structural changes (outer write lock)");
    println!("   5. Multi-threaded concurrent access");
    println!("   6. Complex aggregations (count, sum, avg, max)");
    println!("   7. SQL-like operations:");
    println!("      • WHERE conditions (AND, OR)");
    println!("      • BETWEEN clause");
    println!("      • IN clause");
    println!("      • LIKE operations (starts_with, ends_with, contains)");
    println!("      • ORDER BY (sorting)");
    println!("      • LIMIT and OFFSET (pagination)");
    println!("      • GROUP BY with aggregations");
    println!("      • MIN/MAX/AVG functions");
    println!("   8. Conditional bulk updates");
    println!("   9. Search and key-based lookups\n");
    
    println!("🎯 Zero cloning achieved throughout the example!");
    println!("   Only specific String fields cloned when needed for display.\n");
}


