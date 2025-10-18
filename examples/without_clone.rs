// Demonstrates that the query builder works without Clone derive
// Most operations only require references, not cloning
// cargo run --example without_clone

use rust_queries_builder::{Query, JoinQuery};
use key_paths_derive::Keypath;

// Notice: NO Clone derive! This struct cannot be cloned.
#[derive(Debug, Keypath)]
struct Employee {
    id: u32,
    name: String,
    email: String,
    department: String,
    salary: f64,
    // Imagine this contains large data that's expensive to clone
    large_data: Vec<u8>,
}

// Another struct without Clone
#[derive(Debug, Keypath)]
struct Department {
    id: u32,
    name: String,
    budget: f64,
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  Query Builder Without Clone - Performance Optimization   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let employees = vec![
        Employee {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            department: "Engineering".to_string(),
            salary: 95000.0,
            large_data: vec![0; 1000], // Large data - expensive to clone!
        },
        Employee {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            department: "Engineering".to_string(),
            salary: 87000.0,
            large_data: vec![0; 1000],
        },
        Employee {
            id: 3,
            name: "Carol".to_string(),
            email: "carol@example.com".to_string(),
            department: "Sales".to_string(),
            salary: 75000.0,
            large_data: vec![0; 1000],
        },
    ];

    let departments = vec![
        Department {
            id: 1,
            name: "Engineering".to_string(),
            budget: 1000000.0,
        },
        Department {
            id: 2,
            name: "Sales".to_string(),
            budget: 500000.0,
        },
    ];

    println!("✅ Operations that DON'T require Clone:\n");

    // 1. WHERE filtering - returns Vec<&T>
    println!("1. WHERE filtering (returns references)");
    let query = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering");
    let engineering = query.all();
    println!("   Found {} engineering employees", engineering.len());
    for emp in &engineering {
        println!("     - {}: ${:.0}", emp.name, emp.salary);
    }

    // 2. COUNT - no cloning needed
    println!("\n2. COUNT aggregation");
    let count = Query::new(&employees)
        .where_(Employee::salary(), |&sal| sal > 80000.0)
        .count();
    println!("   {} employees earn over $80k", count);

    // 3. SELECT - only clones the selected field
    println!("\n3. SELECT (only selected fields are cloned)");
    let names: Vec<String> = Query::new(&employees)
        .select(Employee::name());
    println!("   Employee names: {:?}", names);

    // 4. FIRST - returns Option<&T>
    println!("\n4. FIRST (returns reference)");
    let query = Query::new(&employees)
        .where_(Employee::salary(), |&sal| sal > 90000.0);
    if let Some(emp) = query.first() {
        println!("   First high earner: {} (${:.0})", emp.name, emp.salary);
    }

    // 5. SUM/AVG aggregations - no cloning
    println!("\n5. Aggregations (SUM/AVG)");
    let eng_query = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering");
    let total = eng_query.sum(Employee::salary());
    let avg = eng_query.avg(Employee::salary()).unwrap_or(0.0);
    println!("   Engineering total: ${:.0}", total);
    println!("   Engineering average: ${:.0}", avg);

    // 6. MIN/MAX - no cloning
    println!("\n6. MIN/MAX");
    let min = Query::new(&employees).min_float(Employee::salary());
    let max = Query::new(&employees).max_float(Employee::salary());
    println!("   Salary range: ${:.0} - ${:.0}", min.unwrap(), max.unwrap());

    // 7. LIMIT - returns Vec<&T>
    println!("\n7. LIMIT (returns references)");
    let query = Query::new(&employees);
    let first_two = query.limit(2);
    println!("   First 2 employees:");
    for emp in &first_two {
        println!("     - {}", emp.name);
    }

    // 8. SKIP/Pagination - returns Vec<&T>
    println!("\n8. SKIP/Pagination (returns references)");
    let query = Query::new(&employees);
    let page_2 = query.skip(2).limit(1);
    println!("   Page 2:");
    for emp in &page_2 {
        println!("     - {}", emp.name);
    }

    // 9. EXISTS - just checks
    println!("\n9. EXISTS check");
    let has_sales = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Sales")
        .exists();
    println!("   Has Sales employees: {}", has_sales);

    // 10. JOIN - no Clone required on L or R!
    println!("\n10. JOIN operations (no Clone required!)");
    let results = JoinQuery::new(&employees, &departments)
        .inner_join(
            Employee::department(),
            Department::name(),
            |emp, dept| {
                // Mapper only clones what it needs for the result
                (emp.name.clone(), dept.budget)
            },
        );
    println!("   Employee-Department pairs:");
    for (name, budget) in &results {
        println!("     - {} works in dept with ${:.0} budget", name, budget);
    }

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Operations that REQUIRE Clone (only when needed)         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("⚠️  The following operations require Clone because they return owned Vec<T>:");
    println!("   - order_by() / order_by_desc()");
    println!("   - order_by_float() / order_by_float_desc()");
    println!("   - group_by()");
    println!("\n   To use these, add #[derive(Clone)] to your struct:");
    println!("   ```rust");
    println!("   #[derive(Clone, Keypath)]  // Add Clone here");
    println!("   struct Employee {{ ... }}");
    println!("   ```");

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Performance Benefits                                      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("✅ Zero cloning for most operations");
    println!("✅ Work with large structs efficiently");
    println!("✅ No unnecessary memory allocations");
    println!("✅ Only clone when you actually need owned data");
    println!("✅ Pay for what you use");

    println!("\n✓ Example complete! Most operations work without Clone.\n");
}

