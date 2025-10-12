// Compares SQL queries (HSQLDB-style) with Rust Query Builder
// This example demonstrates that the Rust query builder produces
// the same results as equivalent SQL queries would in a database.
// cargo run --example sql_comparison

use rust_queries_builder::{Query, JoinQuery};
use key_paths_derive::Keypaths;
use rust_queries_core::QueryExt;
use std::collections::HashMap;

#[derive(Debug, Clone, Keypaths)]
struct Employee {
    id: u32,
    name: String,
    department: String,
    salary: f64,
    age: u32,
    city: String,
}

#[derive(Debug, Clone, Keypaths)]
struct Department {
    id: u32,
    name: String,
    budget: f64,
    location: String,
}

#[derive(Debug, Clone, Keypaths)]
struct Project {
    id: u32,
    name: String,
    department_id: u32,
    budget: f64,
}

// Sample data
fn create_employees() -> Vec<Employee> {
    vec![
        Employee { id: 1, name: "Alice Johnson".to_string(), department: "Engineering".to_string(), salary: 95000.0, age: 32, city: "San Francisco".to_string() },
        Employee { id: 2, name: "Bob Smith".to_string(), department: "Engineering".to_string(), salary: 87000.0, age: 28, city: "San Francisco".to_string() },
        Employee { id: 3, name: "Carol White".to_string(), department: "Sales".to_string(), salary: 75000.0, age: 35, city: "New York".to_string() },
        Employee { id: 4, name: "David Brown".to_string(), department: "Sales".to_string(), salary: 82000.0, age: 41, city: "New York".to_string() },
        Employee { id: 5, name: "Eve Davis".to_string(), department: "Marketing".to_string(), salary: 71000.0, age: 29, city: "Chicago".to_string() },
        Employee { id: 6, name: "Frank Wilson".to_string(), department: "Engineering".to_string(), salary: 105000.0, age: 38, city: "Seattle".to_string() },
        Employee { id: 7, name: "Grace Lee".to_string(), department: "Marketing".to_string(), salary: 68000.0, age: 26, city: "Chicago".to_string() },
        Employee { id: 8, name: "Henry Taylor".to_string(), department: "HR".to_string(), salary: 62000.0, age: 33, city: "Boston".to_string() },
        Employee { id: 9, name: "Iris Moore".to_string(), department: "Engineering".to_string(), salary: 92000.0, age: 30, city: "San Francisco".to_string() },
        Employee { id: 10, name: "Jack Anderson".to_string(), department: "Sales".to_string(), salary: 79000.0, age: 45, city: "New York".to_string() },
    ]
}

fn create_departments() -> Vec<Department> {
    vec![
        Department { id: 1, name: "Engineering".to_string(), budget: 1500000.0, location: "San Francisco".to_string() },
        Department { id: 2, name: "Sales".to_string(), budget: 800000.0, location: "New York".to_string() },
        Department { id: 3, name: "Marketing".to_string(), budget: 600000.0, location: "Chicago".to_string() },
        Department { id: 4, name: "HR".to_string(), budget: 400000.0, location: "Boston".to_string() },
    ]
}

fn create_projects() -> Vec<Project> {
    vec![
        Project { id: 1, name: "Cloud Migration".to_string(), department_id: 1, budget: 250000.0 },
        Project { id: 2, name: "Mobile App".to_string(), department_id: 1, budget: 180000.0 },
        Project { id: 3, name: "Q4 Campaign".to_string(), department_id: 2, budget: 120000.0 },
        Project { id: 4, name: "Brand Refresh".to_string(), department_id: 3, budget: 95000.0 },
        Project { id: 5, name: "Employee Portal".to_string(), department_id: 4, budget: 65000.0 },
    ]
}

fn print_separator(title: &str) {
    println!("\n{}", "=".repeat(80));
    println!("  {}", title);
    println!("{}", "=".repeat(80));
}

fn print_query(sql: &str, description: &str) {
    println!("\n--- {} ---", description);
    println!("SQL Query:");
    println!("{}", sql);
    println!("\nResults:");
}

fn main() {
    let employees = create_employees();
    let departments = create_departments();
    let projects = create_projects();

    println!("\n╔════════════════════════════════════════════════════════════════════════╗");
    println!("║     SQL vs Rust Query Builder Comparison                              ║");
    println!("║     Demonstrating equivalent operations                               ║");
    println!("╚════════════════════════════════════════════════════════════════════════╝");

    // ============================================================================
    // EXAMPLE 1: Simple SELECT with WHERE
    // ============================================================================
    print_separator("Example 1: SELECT with WHERE clause");
    print_query(
        "SELECT * FROM employees WHERE department = 'Engineering';",
        "Filter employees by department"
    );

    let eng_query = Query::new(&employees)
        .where_(Employee::department_r(), |dept| dept == "Engineering");
    let engineering_employees = eng_query.all();

    for emp in &engineering_employees {
        println!("  ID: {}, Name: {}, Salary: ${:.2}", emp.id, emp.name, emp.salary);
    }
    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::department_r(), |dept| dept == \"Engineering\")");
    println!("    .all()");
    println!("✓ Found {} employees", engineering_employees.len());

    // ============================================================================
    // EXAMPLE 2: SELECT specific columns
    // ============================================================================
    print_separator("Example 2: SELECT specific columns (Projection)");
    print_query(
        "SELECT name FROM employees WHERE salary > 80000;",
        "Get names of high-earning employees"
    );

    let high_earners = Query::new(&employees)
        .where_(Employee::salary_r(), |&salary| salary > 80000.0)
        .select(Employee::name_r());

    for name in &high_earners {
        println!("  {}", name);
    }
    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::salary_r(), |&salary| salary > 80000.0)");
    println!("    .select(Employee::name_r())");
    println!("✓ Found {} employees", high_earners.len());

    // ============================================================================
    // EXAMPLE 3: COUNT
    // ============================================================================
    print_separator("Example 3: COUNT aggregation");
    print_query(
        "SELECT COUNT(*) FROM employees WHERE age < 30;",
        "Count young employees"
    );

    let young_count = Query::new(&employees)
        .where_(Employee::age_r(), |&age| age < 30)
        .count();

    println!("  Count: {}", young_count);
    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::age_r(), |&age| age < 30)");
    println!("    .count()");
    println!("✓ Result: {}", young_count);

    // ============================================================================
    // EXAMPLE 4: SUM, AVG, MIN, MAX
    // ============================================================================
    print_separator("Example 4: Aggregate functions (SUM, AVG, MIN, MAX)");
    print_query(
        "SELECT \n\
         SUM(salary) as total_salary,\n\
         AVG(salary) as avg_salary,\n\
         MIN(salary) as min_salary,\n\
         MAX(salary) as max_salary\n\
         FROM employees WHERE department = 'Engineering';",
        "Engineering department salary statistics"
    );

    let eng_query = Query::new(&employees)
        .where_(Employee::department_r(), |dept| dept == "Engineering");

    let total = eng_query.sum(Employee::salary_r());
    let avg = eng_query.avg(Employee::salary_r()).unwrap_or(0.0);
    let min = eng_query.min_float(Employee::salary_r()).unwrap_or(0.0);
    let max = eng_query.max_float(Employee::salary_r()).unwrap_or(0.0);

    println!("  Total Salary: ${:.2}", total);
    println!("  Avg Salary:   ${:.2}", avg);
    println!("  Min Salary:   ${:.2}", min);
    println!("  Max Salary:   ${:.2}", max);

    println!("\nRust Query Builder:");
    println!("let query = Query::new(&employees)");
    println!("    .where_(Employee::department_r(), |dept| dept == \"Engineering\");");
    println!("query.sum(Employee::salary_r())  // ${:.2}", total);
    println!("query.avg(Employee::salary_r())  // ${:.2}", avg);
    println!("query.min_float(Employee::salary_r())  // ${:.2}", min);
    println!("query.max_float(Employee::salary_r())  // ${:.2}", max);

    // ============================================================================
    // EXAMPLE 5: GROUP BY with aggregation
    // ============================================================================
    print_separator("Example 5: GROUP BY with aggregation");
    print_query(
        "SELECT \n\
         department,\n\
         COUNT(*) as emp_count,\n\
         AVG(salary) as avg_salary\n\
         FROM employees\n\
         GROUP BY department;",
        "Statistics by department"
    );

    let by_dept = Query::new(&employees).group_by(Employee::department_r());

    for (dept, emps) in &by_dept {
        let dept_query = Query::new(emps);
        let count = emps.len();
        let avg_sal = dept_query.avg(Employee::salary_r()).unwrap_or(0.0);
        println!("  {}: {} employees, avg salary ${:.2}", dept, count, avg_sal);
    }

    println!("\nRust Query Builder:");
    println!("let by_dept = Query::new(&employees).group_by(Employee::department_r());");
    println!("for (dept, emps) in &by_dept {{");
    println!("    let dept_query = Query::new(emps);");
    println!("    dept_query.avg(Employee::salary_r())");
    println!("}}");

    // ============================================================================
    // EXAMPLE 6: ORDER BY
    // ============================================================================
    print_separator("Example 6: ORDER BY");
    print_query(
        "SELECT name, salary FROM employees\n\
         WHERE department = 'Sales'\n\
         ORDER BY salary DESC;",
        "Sales employees ordered by salary (descending)"
    );

    let sales_sorted = Query::new(&employees)
        .where_(Employee::department_r(), |dept| dept == "Sales")
        .order_by_float_desc(Employee::salary_r());

    for emp in &sales_sorted {
        println!("  {}: ${:.2}", emp.name, emp.salary);
    }

    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::department_r(), |dept| dept == \"Sales\")");
    println!("    .order_by_float_desc(Employee::salary_r())");

    // ============================================================================
    // EXAMPLE 7: Multiple WHERE conditions (AND)
    // ============================================================================
    print_separator("Example 7: Multiple WHERE conditions (AND)");
    print_query(
        "SELECT * FROM employees\n\
         WHERE salary > 70000 AND age < 35;",
        "High-earning young employees"
    );

    let filter_query = Query::new(&employees)
        .where_(Employee::salary_r(), |&salary| salary > 70000.0)
        .where_(Employee::age_r(), |&age| age < 35);
    let filtered = filter_query.all();

    for emp in &filtered {
        println!("  {}: Age {}, Salary ${:.2}", emp.name, emp.age, emp.salary);
    }

    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::salary_r(), |&salary| salary > 70000.0)");
    println!("    .where_(Employee::age_r(), |&age| age < 35)");
    println!("    .all()");
    println!("✓ Found {} employees", filtered.len());

    // ============================================================================
    // EXAMPLE 8: LIMIT (TOP N)
    // ============================================================================
    print_separator("Example 8: LIMIT / TOP N");
    print_query(
        "SELECT TOP 3 name, salary FROM employees\n\
         ORDER BY salary DESC;",
        "Top 3 highest paid employees"
    );

    let top_earners = Query::new(&employees)
        .order_by_float_desc(Employee::salary_r());

    for (i, emp) in top_earners.iter().take(3).enumerate() {
        println!("  {}. {}: ${:.2}", i + 1, emp.name, emp.salary);
    }

    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .order_by_float_desc(Employee::salary_r())");
    println!("    .into_iter().take(3)");

    // ============================================================================
    // EXAMPLE 9: OFFSET and LIMIT (Pagination)
    // ============================================================================
    print_separator("Example 9: OFFSET and LIMIT (Pagination)");
    print_query(
        "SELECT name FROM employees\n\
         ORDER BY name\n\
         LIMIT 3 OFFSET 3;",
        "Page 2 of employee names (3 per page)"
    );

    let page_2 = Query::new(&employees)
        .order_by(Employee::name_r())
        .into_iter()
        .skip(3)
        .take(3)
        .collect::<Vec<_>>();

    for emp in &page_2 {
        println!("  {}", emp.name);
    }

    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .order_by(Employee::name_r())");
    println!("    .into_iter().skip(3).take(3)");

    // ============================================================================
    // EXAMPLE 10: INNER JOIN
    // ============================================================================
    print_separator("Example 10: INNER JOIN");
    print_query(
        "SELECT e.name, d.name as dept_name, d.budget\n\
         FROM employees e\n\
         INNER JOIN departments d ON e.department = d.name;",
        "Employees with their department info"
    );

    let emp_dept = JoinQuery::new(&employees, &departments).inner_join(
        Employee::department_r(),
        Department::name_r(),
        |emp, dept| (emp.name.clone(), dept.name.clone(), dept.budget),
    );

    for (emp_name, dept_name, budget) in emp_dept.iter().take(5) {
        println!("  {} works in {} (Budget: ${:.0})", emp_name, dept_name, budget);
    }
    println!("  ... (showing first 5)");

    println!("\nRust Query Builder:");
    println!("JoinQuery::new(&employees, &departments).inner_join(");
    println!("    Employee::department_r(),");
    println!("    Department::name_r(),");
    println!("    |emp, dept| (emp.name.clone(), dept.name.clone(), dept.budget)");
    println!(")");
    println!("✓ Found {} matches", emp_dept.len());

    // ============================================================================
    // EXAMPLE 11: GROUP BY with HAVING equivalent
    // ============================================================================
    print_separator("Example 11: GROUP BY with filtering (HAVING equivalent)");
    print_query(
        "SELECT city, COUNT(*) as emp_count, AVG(salary) as avg_salary\n\
         FROM employees\n\
         GROUP BY city\n\
         HAVING COUNT(*) > 1;",
        "Cities with multiple employees"
    );

    let by_city = Query::new(&employees).group_by(Employee::city_r());
    let mut city_stats: Vec<_> = by_city
        .iter()
        .filter(|(_, emps)| emps.len() > 1) // HAVING equivalent
        .map(|(city, emps)| {
            let avg_sal = Query::new(emps).avg(Employee::salary_r()).unwrap_or(0.0);
            (city.clone(), emps.len(), avg_sal)
        })
        .collect();
    
    city_stats.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count

    for (city, count, avg_sal) in &city_stats {
        println!("  {}: {} employees, avg salary ${:.2}", city, count, avg_sal);
    }

    println!("\nRust Query Builder:");
    println!("let by_city = Query::new(&employees).group_by(Employee::city_r());");
    println!("by_city.iter()");
    println!("    .filter(|(_, emps)| emps.len() > 1)  // HAVING equivalent");
    println!("    .map(|(city, emps)| {{");
    println!("        let avg = Query::new(emps).avg(Employee::salary_r());");
    println!("        (city, emps.len(), avg)");
    println!("    }})");

    // ============================================================================
    // EXAMPLE 12: Complex query with multiple operations
    // ============================================================================
    print_separator("Example 12: Complex multi-operation query");
    print_query(
        "SELECT name, salary, age\n\
         FROM employees\n\
         WHERE department IN ('Engineering', 'Sales')\n\
         AND salary BETWEEN 80000 AND 100000\n\
         AND age >= 30\n\
         ORDER BY salary DESC;",
        "Experienced mid-to-senior level employees in core departments"
    );

    let complex_query = Query::new(&employees)
        .where_(Employee::department_r(), |dept| dept == "Engineering" || dept == "Sales")
        .where_(Employee::salary_r(), |&sal| sal >= 80000.0 && sal <= 100000.0)
        .where_(Employee::age_r(), |&age| age >= 30)
        .order_by_float_desc(Employee::salary_r());

    for emp in &complex_query {
        println!("  {}: Age {}, {} dept, ${:.2}", emp.name, emp.age, emp.department, emp.salary);
    }

    println!("\nRust Query Builder:");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::department_r(), |dept| dept == \"Engineering\" || dept == \"Sales\")");
    println!("    .where_(Employee::salary_r(), |&sal| sal >= 80000.0 && sal <= 100000.0)");
    println!("    .where_(Employee::age_r(), |&age| age >= 30)");
    println!("    .order_by_float_desc(Employee::salary_r())");
    println!("✓ Found {} employees", complex_query.len());

    // ============================================================================
    // EXAMPLE 13: Three-table JOIN
    // ============================================================================
    print_separator("Example 13: Three-table JOIN");
    print_query(
        "SELECT p.name as project, d.name as department, d.location\n\
         FROM projects p\n\
         INNER JOIN departments d ON p.department_id = d.id;",
        "Projects with their department details"
    );

    let proj_dept = JoinQuery::new(&projects, &departments).inner_join(
        Project::department_id_r(),
        Department::id_r(),
        |proj, dept| {
            (proj.name.clone(), dept.name.clone(), dept.location.clone(), proj.budget)
        },
    );

    for (proj_name, dept_name, location, budget) in &proj_dept {
        println!("  {}: {} dept in {} (${:.0})", proj_name, dept_name, location, budget);
    }

    println!("\nRust Query Builder:");
    println!("JoinQuery::new(&projects, &departments).inner_join(");
    println!("    Project::department_id_r(),");
    println!("    Department::id_r(),");
    println!("    |proj, dept| (proj.name, dept.name, dept.location, proj.budget)");
    println!(")");

    // ============================================================================
    // EXAMPLE 14: Subquery equivalent (using intermediate results)
    // ============================================================================
    print_separator("Example 14: Subquery equivalent");
    print_query(
        "SELECT * FROM employees\n\
         WHERE salary > (SELECT AVG(salary) FROM employees);",
        "Employees earning above average"
    );

    let avg_salary = Query::new(&employees)
        .avg(Employee::salary_r())
        .unwrap_or(0.0);

    let above_avg_query = Query::new(&employees)
        .where_(Employee::salary_r(), move |&sal| sal > avg_salary);
    let above_avg = above_avg_query.all();

    println!("  Average salary: ${:.2}", avg_salary);
    for emp in &above_avg {
        println!("  {}: ${:.2} ({:.1}% above average)", 
            emp.name, emp.salary, ((emp.salary - avg_salary) / avg_salary * 100.0));
    }

    println!("\nRust Query Builder:");
    println!("let avg = Query::new(&employees).avg(Employee::salary_r()).unwrap_or(0.0);");
    println!("Query::new(&employees)");
    println!("    .where_(Employee::salary_r(), |&sal| sal > avg)");
    println!("    .all()");
    println!("✓ Found {} employees above average", above_avg.len());

    // ============================================================================
    // EXAMPLE 15: Advanced aggregation (revenue calculation)
    // ============================================================================
    print_separator("Example 15: Advanced aggregation");
    print_query(
        "SELECT \n\
         d.name,\n\
         d.budget as dept_budget,\n\
         SUM(p.budget) as total_project_budget,\n\
         (d.budget - SUM(p.budget)) as remaining_budget\n\
         FROM departments d\n\
         LEFT JOIN projects p ON d.id = p.department_id\n\
         GROUP BY d.name, d.budget;",
        "Department budget utilization"
    );

    // Join departments with projects
    let dept_projects = JoinQuery::new(&departments, &projects).left_join(
        Department::id_r(),
        Project::department_id_r(),
        |dept, proj| (dept.clone(), proj.map(|p| p.clone())),
    );

    // Aggregate by department
    let mut dept_budget_map: HashMap<String, (f64, Vec<f64>)> = HashMap::new();
    for (dept, proj) in dept_projects {
        let entry = dept_budget_map
            .entry(dept.name.clone())
            .or_insert((dept.budget, Vec::new()));
        if let Some(p) = proj {
            entry.1.push(p.budget);
        }
    }

    for (dept_name, (total_budget, project_budgets)) in dept_budget_map.iter() {
        let used: f64 = project_budgets.iter().sum();
        let remaining = total_budget - used;
        println!("  {}: Budget ${:.0}, Used ${:.0}, Remaining ${:.0} ({:.1}% utilized)",
            dept_name, total_budget, used, remaining, (used / total_budget * 100.0));
    }

    println!("\nRust Query Builder:");
    println!("let dept_projects = JoinQuery::new(&departments, &projects)");
    println!("    .left_join(Department::id_r(), Project::department_id_r(), ...)");
    println!("// Then aggregate using HashMap");

    // ============================================================================
    // Summary
    // ============================================================================
    print_separator("Summary");
    println!("\n✓ All 15 SQL queries successfully replicated with Rust Query Builder!");
    println!("\nDemonstrated equivalents for:");
    println!("  • SELECT with WHERE");
    println!("  • Projection (SELECT specific columns)");
    println!("  • Aggregations (COUNT, SUM, AVG, MIN, MAX)");
    println!("  • GROUP BY");
    println!("  • ORDER BY");
    println!("  • LIMIT and OFFSET");
    println!("  • INNER JOIN and LEFT JOIN");
    println!("  • Complex conditions (AND, OR, BETWEEN)");
    println!("  • Subqueries");
    println!("  • Multi-table operations");
    println!("\n🎯 Type-safe, compile-time checked, and zero runtime overhead!");
    println!();
}

