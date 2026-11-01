# Middle — confident SQL

## 1. Topic

At this level SQL is no longer only about reading and changing rows. The focus is on expressing data rules and building queries with nested row sets, window calculations, set operations, and PostgreSQL-specific data types.

```text
data integrity
      ↓
subqueries and sets
      ↓
window calculations
      ↓
arrays and JSON
      ↓
convenient data modification
```

---

## 2. Integrity constraints

A column type defines the basic class of valid values. Business rules are expressed with constraints.

Main constraints:

```text
NOT NULL     value is required
CHECK        value must satisfy a condition
UNIQUE       values must be unique
PRIMARY KEY  unique row identifier
FOREIGN KEY  reference to an allowed row in another table
EXCLUDE      rejects rows that conflict under specified operators
```

Example:

```sql
CREATE TABLE products (
    id bigint PRIMARY KEY,
    name text NOT NULL,
    price numeric CHECK (price > 0),
    sku text UNIQUE
);
```

`PRIMARY KEY` combines uniqueness with non-nullability. A table has at most one primary key, but that key may contain multiple columns.

`EXCLUDE` expresses a more general rule that prevents conflicting rows. With an appropriate index method, for example, it can reject overlapping ranges. It is a distinct constraint type, not a sorting feature or a form of `UNIQUE`.

```sql
PRIMARY KEY (order_id, product_id)
```

### FOREIGN KEY

```sql
CREATE TABLE orders (
    id bigint PRIMARY KEY,
    product_id bigint REFERENCES products(id),
    quantity integer CHECK (quantity > 0)
);
```

A foreign key enforces referential integrity: a non-null `product_id` cannot reference a missing target row.

Actions for parent-row changes can be specified:

```text
NO ACTION / RESTRICT  reject the action while references exist
CASCADE               propagate delete/update
SET NULL              replace the reference with NULL
SET DEFAULT           replace it with the default value
```

For example:

```sql
order_id bigint REFERENCES orders(id) ON DELETE CASCADE
```

is appropriate when the child row has no meaning without its order.

---

## 3. Views

A view gives a name to a query and can be queried similarly to a table.

```sql
CREATE VIEW active_users AS
SELECT id, name
FROM users
WHERE active = true;
```

Then:

```sql
SELECT id, name
FROM active_users;
```

A view can act as a stable interface to data while underlying tables evolve.

---

## 4. Subqueries

A subquery is a `SELECT` nested inside another SQL statement.

### Scalar subquery

When used as a scalar value, the subquery must return one column and at most one row:

```sql
SELECT name,
       (SELECT max(price) FROM products) AS max_price
FROM categories;
```

No rows means `NULL`. More than one row is an error.

### EXISTS

`EXISTS` asks only whether the subquery returns at least one row.

```sql
SELECT c.id, c.name
FROM customers AS c
WHERE EXISTS (
    SELECT 1
    FROM orders AS o
    WHERE o.customer_id = c.id
);
```

The inner query is correlated with the current customer row.

For `EXISTS`, the select-list value normally does not matter, so `SELECT 1` is common.

### Subquery in FROM

A subquery may create a temporary row set for the outer query:

```sql
SELECT x.customer_id, x.total
FROM (
    SELECT customer_id, sum(amount) AS total
    FROM orders
    GROUP BY customer_id
) AS x
WHERE x.total > 1000;
```

It behaves as a table expression and needs a name so the outer query can reference its columns.

---

## 5. UNION, INTERSECT, EXCEPT

These operations combine **query results**, unlike `JOIN`, which combines related rows by a condition.

```text
UNION      rows from both queries
INTERSECT  rows present in both
EXCEPT     rows from the first not present in the second
```

Example:

```sql
SELECT email FROM customers
UNION
SELECT email FROM suppliers;
```

`UNION` removes duplicates by default. To keep them:

```sql
UNION ALL
```

Both sides must return compatible column counts and compatible types.

Remember:

```text
JOIN  → combines columns of related rows
UNION → stacks compatible row sets
```

---

## 6. Window functions

A normal aggregate with `GROUP BY` collapses multiple rows into one row per group.

A window function performs calculations across related rows **while keeping each result row separate**.

```sql
SELECT
    depname,
    empno,
    salary,
    avg(salary) OVER (PARTITION BY depname) AS dep_avg
FROM empsalary;
```

Every employee stays visible, while the department average is added beside the row.

### OVER

`OVER` marks a supported call as a window calculation.

```text
PARTITION BY  splits rows into independent partitions
ORDER BY      defines order inside a partition
```

Example:

```sql
SELECT
    depname,
    empno,
    salary,
    rank() OVER (
        PARTITION BY depname
        ORDER BY salary DESC
    ) AS salary_rank
FROM empsalary;
```

Ranking is calculated independently within each department.

`ORDER BY` inside `OVER (...)` controls window processing and does not have to match the final query `ORDER BY`.

Window functions are evaluated after `WHERE`, `GROUP BY`, and `HAVING`. They can therefore appear directly in the `SELECT` list and `ORDER BY`. To filter by a window result, add another query level:

```sql
SELECT *
FROM (
    SELECT
        empno,
        salary,
        rank() OVER (ORDER BY salary DESC) AS pos
    FROM empsalary
) AS ranked
WHERE pos <= 3;
```

---

## 7. FILTER on aggregates

`FILTER` limits the input rows of **one aggregate** without removing the row from the whole query.

```sql
SELECT
    count(*) AS all_orders,
    count(*) FILTER (WHERE status = 'paid') AS paid_orders,
    count(*) FILTER (WHERE status = 'cancelled') AS cancelled_orders
FROM orders;
```

Difference:

```text
WHERE   affects the whole query input
FILTER  affects only the aggregate it belongs to
```

---

## 8. NULL and conditional expressions

`NULL` means that a value is not known/present. It is neither an empty string nor zero.

Use:

```sql
IS NULL
IS NOT NULL
```

not `= NULL`.

### CASE

```sql
SELECT
    amount,
    CASE
        WHEN amount >= 1000 THEN 'large'
        WHEN amount >= 100 THEN 'medium'
        ELSE 'small'
    END AS size
FROM orders;
```

### COALESCE

Returns the first non-`NULL` expression:

```sql
SELECT COALESCE(nickname, name, 'unknown')
FROM users;
```

### NULLIF

Returns `NULL` when two expressions are equal:

```sql
SELECT NULLIF(value, 0);
```

`GREATEST` and `LEAST` return the greatest and least values from a list of expressions.

---

## 9. PostgreSQL arrays

PostgreSQL can store arrays of values of one element type:

```sql
CREATE TABLE articles (
    id bigint PRIMARY KEY,
    tags text[]
);
```

Create an array:

```sql
SELECT ARRAY['sql', 'postgresql', 'dba'];
```

Read an element:

```sql
SELECT tags[1]
FROM articles;
```

PostgreSQL array indexing normally starts at `1`.

Slices are also supported:

```sql
SELECT tags[1:2]
FROM articles;
```

Array elements and slices can be changed with `UPDATE`:

```sql
UPDATE articles
SET tags[1] = 'postgres'
WHERE id = 1;
```

Arrays are useful when the value is genuinely one multi-valued attribute. They do not replace a proper relation table when elements have their own attributes or participate in relational links.

---

## 10. JSON and JSONB

PostgreSQL provides `json` and `jsonb`.

`json` primarily preserves the original textual JSON representation.

`jsonb` stores JSON in a decomposed binary representation suitable for processing and supports indexing for a range of JSON operations.

```sql
CREATE TABLE events (
    id bigint PRIMARY KEY,
    payload jsonb
);
```

Example:

```sql
INSERT INTO events (id, payload)
VALUES (1, '{"type":"login","user_id":42}');
```

If JSON must be queried by structure, not only stored, `jsonb` is usually the important type to know.

---

## 11. Schemas and search_path

A schema is a namespace inside a database.

```text
public.users
audit.users
```

These are different objects even though both tables are named `users`.

A schema-qualified name is explicit:

```sql
SELECT *
FROM audit.events;
```

When the schema is omitted, PostgreSQL searches according to `search_path`.

Simplified model:

```text
unqualified name
      ↓
search_path
      ↓
first matching schema
```

Schema-qualified names are useful when object resolution must be unambiguous.

---

## 12. RETURNING

`INSERT`, `UPDATE`, and `DELETE` can return the affected rows directly.

```sql
INSERT INTO users (name)
VALUES ('Alice')
RETURNING id, name;
```

```sql
UPDATE users
SET active = false
WHERE id = 10
RETURNING id, active;
```

This avoids a separate `SELECT` when the generated identifier or new row state is needed immediately.

---

## 13. SAVEPOINT — partial rollback

`SAVEPOINT` works inside a transaction:

```sql
BEGIN;

UPDATE accounts SET balance = balance - 100 WHERE id = 1;
SAVEPOINT after_debit;

UPDATE accounts SET balance = balance + 100 WHERE id = 2;
ROLLBACK TO after_debit;

UPDATE accounts SET balance = balance + 100 WHERE id = 3;
COMMIT;
```

After `ROLLBACK TO`, the transaction remains open. The savepoint itself also continues to exist, so it can be rolled back to again until it is released or the transaction ends. Work can continue before a final `COMMIT`.

---

# What to remember

Integrity:

```text
NOT NULL     required value
CHECK        condition
UNIQUE       uniqueness
PRIMARY KEY  main row identifier
FOREIGN KEY  referential integrity
EXCLUDE      reject conflicting rows under specified operators
```

Query sets:

```text
EXISTS       at least one row exists
UNION        combine sets and remove duplicates
UNION ALL    combine sets and keep duplicates
INTERSECT    intersection
EXCEPT       difference
```

Windows:

```text
aggregate + GROUP BY → rows collapse
window + OVER        → rows remain

PARTITION BY → window partition
ORDER BY     → order inside the window
```

PostgreSQL features:

```text
jsonb       decomposed binary JSON representation
text[]      text array
search_path lookup path for unqualified object names
RETURNING   return modified rows immediately
```

Filter stages:

```text
WHERE   → query rows
HAVING  → groups
FILTER  → one aggregate's input
```
