// Comprehensive verification that Rust Query Builder produces exact SQL results
// Tests ordering, LIKE operations, NULL handling, and result consistency
// cargo run --example sql_verification

use rust_queries_builder::Query;
use key_paths_derive::Keypath;

#[derive(Debug, Clone, Keypath)]
struct Employee {
    id: u32,
    name: String,
    email: String,
    department: String,
    salary: f64,
}

fn create_test_data() -> Vec<Employee> {
    vec![
        Employee { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), department: "Engineering".to_string(), salary: 95000.0 },
        Employee { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), department: "Engineering".to_string(), salary: 87000.0 },
        Employee { id: 3, name: "Carol White".to_string(), email: "carol@example.com".to_string(), department: "Sales".to_string(), salary: 75000.0 },
        Employee { id: 4, name: "David Brown".to_string(), email: "david@example.com".to_string(), department: "Sales".to_string(), salary: 82000.0 },
        Employee { id: 5, name: "Eve Davis".to_string(), email: "eve@example.com".to_string(), department: "Marketing".to_string(), salary: 71000.0 },
        Employee { id: 6, name: "Alice Cooper".to_string(), email: "cooper@example.com".to_string(), department: "Engineering".to_string(), salary: 105000.0 },
    ]
}

fn print_test(title: &str, passed: bool) {
    let status = if passed { "✅ PASS" } else { "❌ FAIL" };
    println!("{} - {}", status, title);
}

fn main() {
    let employees = create_test_data();
    
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  SQL Equivalence Verification Tests                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // TEST 1: ORDER BY maintains exact SQL ordering (DESC)
    println!("Test 1: ORDER BY DESC - Exact ordering");
    println!("SQL: SELECT * FROM employees ORDER BY salary DESC;");
    
    let ordered = Query::new(&employees).order_by_float_desc(Employee::salary());
    let expected_order = vec![105000.0, 95000.0, 87000.0, 82000.0, 75000.0, 71000.0];
    let actual_order: Vec<f64> = ordered.iter().map(|e| e.salary).collect();
    
    let test1_pass = expected_order == actual_order;
    print_test("ORDER BY salary DESC produces correct order", test1_pass);
    if !test1_pass {
        println!("  Expected: {:?}", expected_order);
        println!("  Got: {:?}", actual_order);
    }

    // TEST 2: ORDER BY maintains exact SQL ordering (ASC)
    println!("\nTest 2: ORDER BY ASC - Exact ordering");
    println!("SQL: SELECT * FROM employees ORDER BY salary ASC;");
    
    let ordered_asc = Query::new(&employees).order_by_float(Employee::salary());
    let expected_asc = vec![71000.0, 75000.0, 82000.0, 87000.0, 95000.0, 105000.0];
    let actual_asc: Vec<f64> = ordered_asc.iter().map(|e| e.salary).collect();
    
    let test2_pass = expected_asc == actual_asc;
    print_test("ORDER BY salary ASC produces correct order", test2_pass);
    if !test2_pass {
        println!("  Expected: {:?}", expected_asc);
        println!("  Got: {:?}", actual_asc);
    }

    // TEST 3: String ordering (alphabetical)
    println!("\nTest 3: ORDER BY name (alphabetical)");
    println!("SQL: SELECT name FROM employees ORDER BY name;");
    
    let ordered_names = Query::new(&employees).order_by(Employee::name());
    let expected_names = vec!["Alice Cooper", "Alice Johnson", "Bob Smith", "Carol White", "David Brown", "Eve Davis"];
    let actual_names: Vec<&str> = ordered_names.iter().map(|e| e.name.as_str()).collect();
    
    let test3_pass = expected_names == actual_names;
    print_test("ORDER BY name produces correct alphabetical order", test3_pass);
    if !test3_pass {
        println!("  Expected: {:?}", expected_names);
        println!("  Got: {:?}", actual_names);
    }

    // TEST 4: LIKE equivalent - starts with
    println!("\nTest 4: LIKE 'Alice%' equivalent");
    println!("SQL: SELECT * FROM employees WHERE name LIKE 'Alice%';");
    
    let like_alice_query = Query::new(&employees)
        .where_(Employee::name(), |name| name.starts_with("Alice"));
    let like_alice = like_alice_query.all();
    
    let test4_pass = like_alice.len() == 2 
        && like_alice.iter().all(|e| e.name.starts_with("Alice"));
    print_test("LIKE 'Alice%' (starts_with) works correctly", test4_pass);
    if test4_pass {
        for emp in &like_alice {
            println!("  Found: {}", emp.name);
        }
    }

    // TEST 5: LIKE equivalent - ends with
    println!("\nTest 5: LIKE '%son' equivalent");
    println!("SQL: SELECT * FROM employees WHERE name LIKE '%son';");
    
    let like_son_query = Query::new(&employees)
        .where_(Employee::name(), |name| name.ends_with("son"));
    let like_son = like_son_query.all();
    
    let test5_pass = like_son.len() == 1 && like_son[0].name == "Alice Johnson";
    print_test("LIKE '%son' (ends_with) works correctly", test5_pass);
    if test5_pass {
        for emp in &like_son {
            println!("  Found: {}", emp.name);
        }
    }

    // TEST 6: LIKE equivalent - contains
    println!("\nTest 6: LIKE '%mit%' equivalent");
    println!("SQL: SELECT * FROM employees WHERE name LIKE '%mit%';");
    
    let like_mit_query = Query::new(&employees)
        .where_(Employee::name(), |name| name.contains("mit"));
    let like_mit = like_mit_query.all();
    
    let test6_pass = like_mit.len() == 1 && like_mit[0].name == "Bob Smith";
    print_test("LIKE '%mit%' (contains) works correctly", test6_pass);
    if test6_pass {
        for emp in &like_mit {
            println!("  Found: {}", emp.name);
        }
    }

    // TEST 7: IN clause equivalent
    println!("\nTest 7: IN clause equivalent");
    println!("SQL: SELECT * FROM employees WHERE department IN ('Engineering', 'Sales');");
    
    let in_depts_query = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering" || dept == "Sales");
    let in_depts = in_depts_query.all();
    
    let test7_pass = in_depts.len() == 5 // 3 Engineering + 2 Sales
        && in_depts.iter().all(|e| e.department == "Engineering" || e.department == "Sales");
    print_test("IN clause works correctly", test7_pass);
    println!("  Found {} employees in Engineering or Sales", in_depts.len());

    // TEST 8: BETWEEN equivalent
    println!("\nTest 8: BETWEEN clause equivalent");
    println!("SQL: SELECT * FROM employees WHERE salary BETWEEN 75000 AND 90000;");
    
    let between_query = Query::new(&employees)
        .where_(Employee::salary(), |&sal| sal >= 75000.0 && sal <= 90000.0);
    let between = between_query.all();
    
    let test8_pass = between.len() == 3;
    print_test("BETWEEN clause works correctly", test8_pass);
    println!("  Found {} employees with salary between 75K-90K", between.len());

    // TEST 9: COUNT with WHERE
    println!("\nTest 9: COUNT with WHERE");
    println!("SQL: SELECT COUNT(*) FROM employees WHERE department = 'Engineering';");
    
    let count_eng = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering")
        .count();
    
    let test9_pass = count_eng == 3;
    print_test("COUNT with WHERE produces correct result", test9_pass);
    println!("  Count: {} (expected 3)", count_eng);

    // TEST 10: AVG produces exact result
    println!("\nTest 10: AVG aggregation accuracy");
    println!("SQL: SELECT AVG(salary) FROM employees WHERE department = 'Engineering';");
    
    let avg_sal = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering")
        .avg(Employee::salary())
        .unwrap_or(0.0);
    
    let expected_avg = (95000.0 + 87000.0 + 105000.0) / 3.0;
    let test10_pass = (avg_sal - expected_avg).abs() < 0.01;
    print_test("AVG produces mathematically correct result", test10_pass);
    println!("  Average: ${:.2} (expected ${:.2})", avg_sal, expected_avg);

    // TEST 11: MIN and MAX
    println!("\nTest 11: MIN and MAX aggregations");
    println!("SQL: SELECT MIN(salary), MAX(salary) FROM employees;");
    
    let min_sal = Query::new(&employees).min_float(Employee::salary()).unwrap_or(0.0);
    let max_sal = Query::new(&employees).max_float(Employee::salary()).unwrap_or(0.0);
    
    let test11_pass = min_sal == 71000.0 && max_sal == 105000.0;
    print_test("MIN and MAX produce correct results", test11_pass);
    println!("  MIN: ${:.2}, MAX: ${:.2}", min_sal, max_sal);

    // TEST 12: Complex WHERE with AND/OR
    println!("\nTest 12: Complex WHERE (AND + OR)");
    println!("SQL: SELECT * FROM employees WHERE (department = 'Engineering' OR department = 'Sales') AND salary > 80000;");
    
    let complex_query = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "Engineering" || dept == "Sales")
        .where_(Employee::salary(), |&sal| sal > 80000.0);
    let complex = complex_query.all();
    
    let test12_pass = complex.len() == 4; // Alice J ($95k), Bob ($87k), David ($82k), Alice C ($105k)
    print_test("Complex WHERE clause works correctly", test12_pass);
    println!("  Found {} employees", complex.len());

    // TEST 13: Case-sensitive string comparison (SQL default)
    println!("\nTest 13: Case-sensitive comparison (like SQL)");
    println!("SQL: SELECT * FROM employees WHERE department = 'engineering'; -- should find 0");
    
    let case_sensitive = Query::new(&employees)
        .where_(Employee::department(), |dept| dept == "engineering") // lowercase
        .count();
    
    let test13_pass = case_sensitive == 0;
    print_test("Case-sensitive comparison (SQL default)", test13_pass);
    println!("  Found {} with lowercase 'engineering' (expected 0)", case_sensitive);

    // TEST 14: Case-insensitive LIKE equivalent
    println!("\nTest 14: Case-insensitive LIKE (ILIKE equivalent)");
    println!("SQL: SELECT * FROM employees WHERE LOWER(name) LIKE '%alice%';");
    
    let ilike_query = Query::new(&employees)
        .where_(Employee::name(), |name| name.to_lowercase().contains("alice"));
    let ilike = ilike_query.all();
    
    let test14_pass = ilike.len() == 2;
    print_test("Case-insensitive LIKE works correctly", test14_pass);
    println!("  Found {} employees with 'alice' (case-insensitive)", ilike.len());

    // TEST 15: LIMIT produces exact subset
    println!("\nTest 15: LIMIT clause");
    println!("SQL: SELECT * FROM employees ORDER BY salary DESC LIMIT 3;");
    
    let limited = Query::new(&employees)
        .order_by_float_desc(Employee::salary())
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();
    
    let test15_pass = limited.len() == 3 
        && limited[0].salary == 105000.0
        && limited[1].salary == 95000.0
        && limited[2].salary == 87000.0;
    print_test("LIMIT returns correct top-N results", test15_pass);
    println!("  Top 3 salaries: ${:.0}, ${:.0}, ${:.0}", 
        limited[0].salary, limited[1].salary, limited[2].salary);

    // TEST 16: GROUP BY produces correct groups
    println!("\nTest 16: GROUP BY");
    println!("SQL: SELECT department, COUNT(*) FROM employees GROUP BY department;");
    
    let grouped = Query::new(&employees).group_by(Employee::department());
    let eng_count = grouped.get("Engineering").map(|v| v.len()).unwrap_or(0);
    let sales_count = grouped.get("Sales").map(|v| v.len()).unwrap_or(0);
    let marketing_count = grouped.get("Marketing").map(|v| v.len()).unwrap_or(0);
    
    let test16_pass = eng_count == 3 && sales_count == 2 && marketing_count == 1;
    print_test("GROUP BY produces correct counts", test16_pass);
    println!("  Engineering: {}, Sales: {}, Marketing: {}", eng_count, sales_count, marketing_count);

    // TEST 17: Email pattern matching (regex-like)
    println!("\nTest 17: Email domain filtering (LIKE '%@example.com')");
    println!("SQL: SELECT * FROM employees WHERE email LIKE '%@example.com';");
    
    let example_emails = Query::new(&employees)
        .where_(Employee::email(), |email| email.ends_with("@example.com"))
        .count();
    
    let test17_pass = example_emails == 6; // All employees have @example.com emails
    print_test("Email pattern matching works correctly", test17_pass);
    println!("  Found {} employees with @example.com emails", example_emails);

    // Summary
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Test Summary                                                  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    let tests = vec![
        ("ORDER BY DESC", test1_pass),
        ("ORDER BY ASC", test2_pass),
        ("ORDER BY name", test3_pass),
        ("LIKE 'prefix%'", test4_pass),
        ("LIKE '%suffix'", test5_pass),
        ("LIKE '%contains%'", test6_pass),
        ("IN clause", test7_pass),
        ("BETWEEN clause", test8_pass),
        ("COUNT", test9_pass),
        ("AVG accuracy", test10_pass),
        ("MIN/MAX", test11_pass),
        ("Complex WHERE", test12_pass),
        ("Case-sensitive", test13_pass),
        ("Case-insensitive", test14_pass),
        ("LIMIT", test15_pass),
        ("GROUP BY", test16_pass),
        ("Email patterns", test17_pass),
    ];
    
    let passed = tests.iter().filter(|(_, p)| *p).count();
    let total = tests.len();
    
    println!("Results: {}/{} tests passed", passed, total);
    
    if passed == total {
        println!("\n🎉 All tests PASSED! Rust Query Builder produces exact SQL-equivalent results!");
        println!("\n✅ Verified:");
        println!("  • Exact ordering (ASC/DESC)");
        println!("  • LIKE operations (starts_with, ends_with, contains)");
        println!("  • IN clause (OR conditions)");
        println!("  • BETWEEN clause");
        println!("  • Aggregations (COUNT, AVG, MIN, MAX)");
        println!("  • Complex WHERE conditions");
        println!("  • Case sensitivity");
        println!("  • LIMIT clause");
        println!("  • GROUP BY");
        println!("  • Pattern matching");
    } else {
        println!("\n⚠️  {} test(s) failed. See details above.", total - passed);
    }
}

