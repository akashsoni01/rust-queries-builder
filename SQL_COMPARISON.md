# SQL to Rust Query Builder Comparison

This document provides a comprehensive mapping between SQL queries (HSQLDB-style) and their equivalent Rust Query Builder implementations.

## Table of Contents

1. [Basic Queries](#basic-queries)
2. [Aggregations](#aggregations)
3. [Grouping and Ordering](#grouping-and-ordering)
4. [Joins](#joins)
5. [Advanced Patterns](#advanced-patterns)

## Basic Queries

### SELECT with WHERE

**SQL:**
```sql
SELECT * FROM employees WHERE department = 'Engineering';
```

**Rust Query Builder:**
```rust
let query = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Engineering");
let engineering = query.all();
```

### SELECT Specific Columns (Projection)

**SQL:**
```sql
SELECT name FROM employees WHERE salary > 80000;
```

**Rust Query Builder:**
```rust
let high_earners: Vec<String> = Query::new(&employees)
    .where_(Employee::salary_r(), |&salary| salary > 80000.0)
    .select(Employee::name_r());
```

### Multiple WHERE Conditions (AND)

**SQL:**
```sql
SELECT * FROM employees 
WHERE salary > 70000 AND age < 35;
```

**Rust Query Builder:**
```rust
let query = Query::new(&employees)
    .where_(Employee::salary_r(), |&salary| salary > 70000.0)
    .where_(Employee::age_r(), |&age| age < 35);
let results = query.all();
```

### Complex Conditions (BETWEEN, OR)

**SQL:**
```sql
SELECT * FROM employees
WHERE department IN ('Engineering', 'Sales')
  AND salary BETWEEN 80000 AND 100000
  AND age >= 30
ORDER BY salary DESC;
```

**Rust Query Builder:**
```rust
let results = Query::new(&employees)
    .where_(Employee::department_r(), |dept| 
        dept == "Engineering" || dept == "Sales")
    .where_(Employee::salary_r(), |&sal| 
        sal >= 80000.0 && sal <= 100000.0)
    .where_(Employee::age_r(), |&age| age >= 30)
    .order_by_float_desc(Employee::salary_r());
```

## Aggregations

### COUNT

**SQL:**
```sql
SELECT COUNT(*) FROM employees WHERE age < 30;
```

**Rust Query Builder:**
```rust
let count = Query::new(&employees)
    .where_(Employee::age_r(), |&age| age < 30)
    .count();
```

### SUM

**SQL:**
```sql
SELECT SUM(salary) FROM employees WHERE department = 'Engineering';
```

**Rust Query Builder:**
```rust
let total: f64 = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Engineering")
    .sum(Employee::salary_r());
```

### AVG, MIN, MAX

**SQL:**
```sql
SELECT 
    AVG(salary) as avg_salary,
    MIN(salary) as min_salary,
    MAX(salary) as max_salary
FROM employees 
WHERE department = 'Engineering';
```

**Rust Query Builder:**
```rust
let eng_query = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Engineering");

let avg = eng_query.avg(Employee::salary_r()).unwrap_or(0.0);
let min = eng_query.min_float(Employee::salary_r()).unwrap_or(0.0);
let max = eng_query.max_float(Employee::salary_r()).unwrap_or(0.0);
```

## Grouping and Ordering

### GROUP BY

**SQL:**
```sql
SELECT 
    department,
    COUNT(*) as emp_count,
    AVG(salary) as avg_salary
FROM employees
GROUP BY department;
```

**Rust Query Builder:**
```rust
let by_dept = Query::new(&employees).group_by(Employee::department_r());

for (dept, emps) in &by_dept {
    let dept_query = Query::new(emps);
    let count = emps.len();
    let avg = dept_query.avg(Employee::salary_r()).unwrap_or(0.0);
    println!("{}: {} employees, avg ${:.2}", dept, count, avg);
}
```

### GROUP BY with HAVING

**SQL:**
```sql
SELECT 
    city, 
    COUNT(*) as emp_count, 
    AVG(salary) as avg_salary
FROM employees
GROUP BY city
HAVING COUNT(*) > 1;
```

**Rust Query Builder:**
```rust
let by_city = Query::new(&employees).group_by(Employee::city_r());

let filtered: Vec<_> = by_city
    .iter()
    .filter(|(_, emps)| emps.len() > 1)  // HAVING equivalent
    .map(|(city, emps)| {
        let avg = Query::new(emps).avg(Employee::salary_r()).unwrap_or(0.0);
        (city.clone(), emps.len(), avg)
    })
    .collect();
```

### ORDER BY

**SQL:**
```sql
SELECT name, salary 
FROM employees
WHERE department = 'Sales'
ORDER BY salary DESC;
```

**Rust Query Builder:**
```rust
let sorted = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Sales")
    .order_by_float_desc(Employee::salary_r());
```

### ORDER BY Multiple Columns

While SQL supports `ORDER BY col1, col2`, you can achieve this in Rust by sorting after the query:

**SQL:**
```sql
SELECT * FROM employees ORDER BY department, salary DESC;
```

**Rust Query Builder:**
```rust
let mut results = Query::new(&employees).all().into_iter()
    .cloned()
    .collect::<Vec<_>>();

results.sort_by(|a, b| {
    a.department.cmp(&b.department)
        .then(b.salary.partial_cmp(&a.salary).unwrap())
});
```

## Joins

### INNER JOIN

**SQL:**
```sql
SELECT e.name, d.name as dept_name, d.budget
FROM employees e
INNER JOIN departments d ON e.department = d.name;
```

**Rust Query Builder:**
```rust
let results = JoinQuery::new(&employees, &departments).inner_join(
    Employee::department_r(),
    Department::name_r(),
    |emp, dept| (emp.name.clone(), dept.name.clone(), dept.budget)
);
```

### LEFT JOIN

**SQL:**
```sql
SELECT u.name, o.id as order_id, o.total
FROM users u
LEFT JOIN orders o ON u.id = o.user_id;
```

**Rust Query Builder:**
```rust
let results = JoinQuery::new(&users, &orders).left_join(
    User::id_r(),
    Order::user_id_r(),
    |user, order| match order {
        Some(o) => (user.name.clone(), Some(o.id), Some(o.total)),
        None => (user.name.clone(), None, None),
    }
);
```

### JOIN with WHERE

**SQL:**
```sql
SELECT e.name, o.total
FROM employees e
INNER JOIN orders o ON e.id = o.employee_id
WHERE o.total > 100;
```

**Rust Query Builder:**
```rust
let results = JoinQuery::new(&employees, &orders).inner_join_where(
    Employee::id_r(),
    Order::employee_id_r(),
    |_emp, order| order.total > 100.0,  // WHERE condition
    |emp, order| (emp.name.clone(), order.total)
);
```

### Three-Way JOIN

**SQL:**
```sql
SELECT e.name, p.name as project, d.name as department
FROM employees e
INNER JOIN projects p ON e.id = p.employee_id
INNER JOIN departments d ON p.department_id = d.id;
```

**Rust Query Builder:**
```rust
// First join
let emp_proj = JoinQuery::new(&employees, &projects).inner_join(
    Employee::id_r(),
    Project::employee_id_r(),
    |emp, proj| (emp.clone(), proj.clone())
);

// Second join
let results: Vec<_> = emp_proj
    .iter()
    .flat_map(|(emp, proj)| {
        departments.iter()
            .filter(|d| d.id == proj.department_id)
            .map(move |dept| {
                (emp.name.clone(), proj.name.clone(), dept.name.clone())
            })
    })
    .collect();
```

## Advanced Patterns

### LIMIT and OFFSET (Pagination)

**SQL:**
```sql
SELECT * FROM employees ORDER BY name LIMIT 10 OFFSET 20;
```

**Rust Query Builder:**
```rust
let page_3: Vec<_> = Query::new(&employees)
    .order_by(Employee::name_r())
    .into_iter()
    .skip(20)
    .take(10)
    .collect();
```

### TOP N

**SQL:**
```sql
SELECT TOP 5 name, salary 
FROM employees 
ORDER BY salary DESC;
```

**Rust Query Builder:**
```rust
let top_5: Vec<_> = Query::new(&employees)
    .order_by_float_desc(Employee::salary_r())
    .into_iter()
    .take(5)
    .collect();
```

### Subqueries

**SQL:**
```sql
SELECT * FROM employees
WHERE salary > (SELECT AVG(salary) FROM employees);
```

**Rust Query Builder:**
```rust
let avg_salary = Query::new(&employees)
    .avg(Employee::salary_r())
    .unwrap_or(0.0);

let query = Query::new(&employees)
    .where_(Employee::salary_r(), move |&sal| sal > avg_salary);
let above_avg = query.all();
```

### EXISTS

**SQL:**
```sql
SELECT EXISTS(SELECT 1 FROM employees WHERE department = 'Engineering');
```

**Rust Query Builder:**
```rust
let has_engineers = Query::new(&employees)
    .where_(Employee::department_r(), |dept| dept == "Engineering")
    .exists();
```

### DISTINCT (Approximate)

While there's no built-in DISTINCT, you can use HashSet:

**SQL:**
```sql
SELECT DISTINCT department FROM employees;
```

**Rust Query Builder:**
```rust
use std::collections::HashSet;

let departments: HashSet<String> = Query::new(&employees)
    .select(Employee::department_r())
    .into_iter()
    .collect();
```

### CASE/WHEN Equivalent

**SQL:**
```sql
SELECT 
    name,
    salary,
    CASE 
        WHEN salary > 100000 THEN 'High'
        WHEN salary > 70000 THEN 'Medium'
        ELSE 'Low'
    END as salary_grade
FROM employees;
```

**Rust Query Builder:**
```rust
let query = Query::new(&employees);
let all_employees = query.all();

let results: Vec<_> = all_employees
    .iter()
    .map(|emp| {
        let grade = if emp.salary > 100000.0 {
            "High"
        } else if emp.salary > 70000.0 {
            "Medium"
        } else {
            "Low"
        };
        (emp.name.clone(), emp.salary, grade)
    })
    .collect();
```

## Type Safety Advantages

Unlike SQL, the Rust Query Builder provides **compile-time type safety**:

```rust
// ✅ Compiles - correct types
query.where_(Employee::salary_r(), |&sal| sal > 80000.0)

// ❌ Won't compile - type mismatch
query.where_(Employee::salary_r(), |sal| sal == "80000")

// ❌ Won't compile - wrong field
query.where_(Employee::name_r(), |&name| name > 80000.0)

// ✅ Compiles - IDE autocomplete works
query.select(Employee::department_r())

// ❌ Won't compile - field doesn't exist
query.select(Employee::nonexistent_field_r())
```

## Performance Comparison

| Operation | SQL (Database) | Rust Query Builder |
|-----------|---------------|-------------------|
| Simple WHERE | Index-dependent | O(n) scan |
| JOIN | Index-dependent, O(n log n) - O(n²) | O(n + m) hash-based |
| ORDER BY | O(n log n) | O(n log n) |
| GROUP BY | Hash/Sort based | O(n) hash-based |
| Aggregation | O(n) | O(n) |
| Memory | Depends on result set | In-memory only |

**Advantages of Rust Query Builder:**
- Zero network latency
- Compile-time type safety
- No SQL injection vulnerabilities
- Works with any Rust struct
- No database setup required
- Direct memory access

**When to use SQL Database:**
- Large datasets (> memory)
- Persistent storage needed
- Concurrent access required
- Complex transactions
- Multi-user applications

## Running the Examples

```bash
# See complete SQL comparison with 15 examples
cargo run --example sql_comparison
```

## Conclusion

The Rust Query Builder provides a type-safe, compile-time checked alternative to SQL queries for in-memory data operations. While it doesn't replace databases for persistent storage, it excels at querying, filtering, and transforming data structures in Rust applications.

