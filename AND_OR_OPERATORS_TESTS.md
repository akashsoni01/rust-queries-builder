# AND/OR Operators Test Cases

This document describes the test cases for the AND/OR operators in `LazyQuery` and their expected outputs.

## Test Data

All tests use the following product data:

```rust
Product { id: 1, name: "Laptop", price: 999.99, category: "Electronics", stock: 5, rating: 4.5 }
Product { id: 2, name: "Mouse", price: 29.99, category: "Electronics", stock: 50, rating: 4.0 }
Product { id: 3, name: "Keyboard", price: 79.99, category: "Electronics", stock: 30, rating: 4.8 }
Product { id: 4, name: "Monitor", price: 299.99, category: "Electronics", stock: 12, rating: 4.2 }
Product { id: 5, name: "Desk Chair", price: 199.99, category: "Furniture", stock: 8, rating: 4.7 }
Product { id: 6, name: "Premium Laptop", price: 1999.99, category: "Electronics", stock: 3, rating: 4.9 }
```

## Test Cases

### Test 1: `test_where_implicit_and`
**Query:** `where_(price < 100).where_(stock > 10)`

**Expected Logic:** `price < 100 AND stock > 10`

**Expected Results:**
- Mouse (price: 29.99, stock: 50) ✓
- Keyboard (price: 79.99, stock: 30) ✓

**Expected Count:** 2

**Status:** ✅ PASS

---

### Test 2: `test_explicit_and`
**Query:** `where_(price < 100).and(stock > 10)`

**Expected Logic:** `price < 100 AND stock > 10`

**Expected Results:**
- Mouse (price: 29.99, stock: 50) ✓
- Keyboard (price: 79.99, stock: 30) ✓

**Expected Count:** 2

**Status:** ✅ PASS

---

### Test 3: `test_or_operator`
**Query:** `where_(price < 50).or(category == "Furniture")`

**Expected Logic:** `price < 50 OR category == "Furniture"`

**Expected Results:**
- Mouse (price: 29.99 < 50) ✓
- Desk Chair (category: "Furniture") ✓

**Expected Count:** 2

**Status:** ✅ PASS

---

### Test 4: `test_complex_and_or_composition`
**Query:** `where_(price < 100).and(stock > 10).or(category == "Furniture")`

**Expected Logic:** `(price < 100 AND stock > 10) OR (category == "Furniture")`

**Expected Results:**
- Mouse (price: 29.99 < 100, stock: 50 > 10) ✓
- Keyboard (price: 79.99 < 100, stock: 30 > 10) ✓
- Desk Chair (category: "Furniture") ✓

**Expected Count:** 3

**Status:** ✅ PASS

---

### Test 5: `test_multiple_and_conditions`
**Query:** `where_(price < 200).and(stock > 5).and(rating > 4.5)`

**Expected Logic:** `price < 200 AND stock > 5 AND rating > 4.5`

**Expected Results:**
- Keyboard (price: 79.99, stock: 30, rating: 4.8) ✓
- Desk Chair (price: 199.99, stock: 8, rating: 4.7) ✓

**Expected Count:** 2

**Status:** ✅ PASS

---

### Test 6: `test_multiple_or_conditions`
**Query:** `where_(price > 500).or(category == "Furniture").or(rating > 4.8)`

**Expected Logic:** `price > 500 OR category == "Furniture" OR rating > 4.8`

**Expected Results:**
- Laptop (price: 999.99 > 500) ✓
- Desk Chair (category: "Furniture") ✓
- Premium Laptop (rating: 4.9 > 4.8) ✓

**Expected Count:** 3

**Status:** ✅ PASS

---

### Test 7: `test_and_then_or_then_where`
**Query:** `where_(price < 100).and(stock > 10).or(category == "Furniture").where_(rating > 4.0)`

**Expected Logic:** `(price < 100 AND stock > 10) OR (category == "Furniture" OR rating > 4.0)`

**Note:** The second `where_()` adds to the OR group since the last group is OR.

**Expected Results:**
Since `rating > 4.0` is true for all products (all have rating >= 4.0), the OR group always passes, so all products match.

**Expected Count:** 6 (all products)

**Status:** ✅ PASS

---

### Test 8: `test_empty_results`
**Query:** `where_(price < 0)`

**Expected Logic:** `price < 0` (impossible condition)

**Expected Results:** None

**Expected Count:** 0

**Status:** ✅ PASS

---

### Test 9: `test_all_results`
**Query:** `where_(price > 0)`

**Expected Logic:** `price > 0` (matches all)

**Expected Results:** All 6 products

**Expected Count:** 6

**Status:** ✅ PASS

---

### Test 10: `test_or_with_no_previous_and`
**Query:** `or(category == "Furniture")`

**Expected Logic:** `category == "Furniture"` (only OR group)

**Expected Results:**
- Desk Chair (category: "Furniture") ✓

**Expected Count:** 1

**Status:** ✅ PASS

---

### Test 11: `test_count_with_and_or`
**Query:** `where_(price < 100).and(stock > 10).count()`

**Expected Logic:** `price < 100 AND stock > 10`

**Expected Count:** 2

**Status:** ✅ PASS

---

### Test 12: `test_first_with_and_or`
**Query:** `where_(price < 100).and(stock > 10).first()`

**Expected Logic:** `price < 100 AND stock > 10`

**Expected:** First product matching (Mouse or Keyboard)

**Status:** ✅ PASS

---

### Test 13: `test_any_with_and_or`
**Query:** `where_(price < 50).or(category == "Furniture").any()`

**Expected Logic:** `price < 50 OR category == "Furniture"`

**Expected:** `true` (at least one match exists)

**Status:** ✅ PASS

---

## Test Results Summary

**Total Tests:** 13  
**Passed:** 13 ✅  
**Failed:** 0  
**Success Rate:** 100%

## Filter Group Evaluation Logic

The filter groups are evaluated as follows:

1. **Only AND groups:** All AND groups must pass (all filters in each group must pass)
2. **Only OR groups:** At least one OR group must pass (at least one filter in any OR group must pass)
3. **Both AND and OR groups:** `(all AND groups pass) OR (any OR group passes)`
4. **No filters:** Everything passes

## Behavior Notes

- `where_()` adds to the current group type (AND or OR) if the last group exists
- `and()` explicitly adds to or creates an AND group
- `or()` explicitly adds to or creates an OR group
- When `where_()` is called after `or()`, it adds to the OR group (not creates a new AND group)
- This allows natural chaining: `where().and().or().where()` where the second `where()` continues the OR group

