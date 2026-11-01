# Strong — advanced SQL

## 1. Topic

At the Strong level, syntax alone is not enough. You need to understand **which rows are visible to each part of a query, how PostgreSQL can execute the query, and what that execution costs**.

Main areas:

```text
correlated data sources
      ↓
recursive queries and advanced grouping
      ↓
precise window frames
      ↓
indexes and execution plans
      ↓
concurrent transactions
```

---

## 2. LATERAL

A normal subquery in `FROM` is an independent source and cannot simply reference columns from a neighboring `FROM` item.

`LATERAL` allows dependence on items that appear to its left.

```sql
SELECT
    u.id,
    x.last_order_at
FROM users AS u
LEFT JOIN LATERAL (
    SELECT max(o.created_at) AS last_order_at
    FROM orders AS o
    WHERE o.user_id = u.id
) AS x ON true;
```

Conceptual model:

```text
next users row
      ↓
its u.id is visible to the LATERAL subquery
      ↓
the subquery calculates a result for that row
```

`LEFT JOIN LATERAL ... ON true` is useful when the left row must remain even when the right side produces no match.

Typical uses include top-N related rows per object, table functions, and calculations that depend on the current row of a previous source.

For table functions the `LATERAL` keyword may be omitted: function arguments can already reference columns from preceding `FROM` items. A subquery needs explicit `LATERAL` for that dependency.

---

## 3. Table functions and WITH ORDINALITY

A function may return a row set and appear in `FROM`.

```sql
SELECT *
FROM unnest(ARRAY['a', 'b', 'c']) AS t(value);
```

`WITH ORDINALITY` adds a `bigint` row number starting at `1`:

```sql
SELECT *
FROM unnest(ARRAY['a', 'b', 'c'])
     WITH ORDINALITY AS t(value, n);
```

Conceptual result:

```text
a  1
b  2
c  3
```

This is useful when unnesting a sequence while preserving element position.

---

## 4. GROUPING SETS, ROLLUP, and CUBE

A normal `GROUP BY` defines one grouping layout.

```sql
GROUP BY region, product
```

`GROUPING SETS` calculates several aggregation levels in one query:

```sql
SELECT region, product, sum(amount)
FROM sales
GROUP BY GROUPING SETS (
    (region, product),
    (region),
    ()
);
```

This produces:

```text
(region, product)  product totals inside each region
(region)           region totals
()                 grand total
```

`ROLLUP` is shorthand for hierarchical totals.

```sql
GROUP BY ROLLUP (region, product)
```

Conceptually it produces:

```text
(region, product)
(region)
()
```

`CUBE` produces combinations of grouping dimensions and is useful for multidimensional analysis.

---

## 5. Recursive CTEs

`WITH RECURSIVE` lets a query use the result produced by an earlier step of the same CTE.

Typical structure:

```sql
WITH RECURSIVE tree AS (
    -- seed part
    SELECT id, parent_id, 1 AS depth
    FROM nodes
    WHERE parent_id IS NULL

    UNION ALL

    -- recursive part
    SELECT n.id, n.parent_id, t.depth + 1
    FROM nodes AS n
    JOIN tree AS t
        ON n.parent_id = t.id
)
SELECT *
FROM tree;
```

Model:

```text
seed rows
   ↓
working set
   ↓
recursive term produces next rows
   ↓
new rows become input
   ↓
repeat until no new rows remain
```

Common uses:

- trees and hierarchies;
- graph-like relationships;
- dependency traversal;
- level-by-level expansion.

The main risk is a cycle or a rule that never converges. PostgreSQL separately supports search-order specification and cycle detection in recursive queries (`SEARCH` and `CYCLE`). Even when those clauses are unnecessary, termination and repeated visits must be considered explicitly.

`UNION ALL` is normally preferable when duplicate removal is not part of the logic because `UNION` adds duplicate-elimination work.

---

## 6. CTE materialization

A CTE should not be thought of as an unconditional temporary table.

PostgreSQL may inline suitable CTEs into the parent query so the optimizer can consider the whole expression together. In other cases, the CTE is materialized separately.

Intent can be stated explicitly:

```sql
WITH x AS MATERIALIZED (
    SELECT ...
)
SELECT ...;
```

or:

```sql
WITH x AS NOT MATERIALIZED (
    SELECT ...
)
SELECT ...;
```

Simplified model:

```text
MATERIALIZED
    → build a separate intermediate result first

NOT MATERIALIZED
    → allow inlining/joint optimization when valid
```

Materialization can help when an expensive result should be computed once and reused. Inlining can help when outer predicates can eliminate rows earlier.

Do not use a rule such as “CTEs are always faster” or “CTEs are always slower”. Check the execution plan.

---

## 7. Window frames

A window calculation has three distinct concepts:

```text
PARTITION BY  which rows belong to the partition
ORDER BY      their order inside the partition
frame         which part of that partition is used for the current row
```

Running total:

```sql
SELECT
    account_id,
    ts,
    amount,
    sum(amount) OVER (
        PARTITION BY account_id
        ORDER BY ts
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS running_total
FROM operations;
```

For each row, the frame starts at the first partition row and ends at the current row.

Frames can use `ROWS`, `RANGE`, or `GROUPS`:

```text
ROWS    physical rows relative to the current row
RANGE   ORDER BY values and their peers
GROUPS  peer groups defined by ORDER BY
```

The default frame matters. With `ORDER BY`, it may include rows from the beginning of the partition through the current row and its peers rather than the whole partition.

Therefore:

```sql
sum(amount) OVER (ORDER BY ts)
```

does not simply mean “sum over the whole table”. When exact semantics matter, specify the frame explicitly.

Window functions may appear directly in `SELECT` and `ORDER BY`. Filtering their result requires another query level.

---

## 8. Indexes

An index is a separate structure that can accelerate certain access patterns at the cost of space and maintenance during writes.

Core idea:

```text
an index does not make a table universally faster;
it helps specific ways of finding or ordering rows
```

The planner decides whether using an index is worthwhile.

### Multicolumn index

```sql
CREATE INDEX orders_customer_created_idx
ON orders (customer_id, created_at);
```

The index contains multiple key columns. Column order matters for which predicates and orderings can use it efficiently.

### Expression index

```sql
CREATE INDEX users_lower_email_idx
ON users (lower(email));
```

The indexed value is the result of `lower(email)`, so it is useful for matching expressions such as:

```sql
WHERE lower(email) = 'user@example.com'
```

### Partial index

```sql
CREATE INDEX orders_open_customer_idx
ON orders (customer_id)
WHERE status = 'open';
```

Only rows satisfying the predicate are stored in the index.

This is useful when queries repeatedly target a small important subset of a table.

### Covering index

An index may include extra columns so some queries can obtain required values from the index itself when other conditions allow it.

Index design should follow real query patterns and plans, not the rule “add an index to every column”.

---

## 9. EXPLAIN

`EXPLAIN` shows the plan selected by PostgreSQL.

```sql
EXPLAIN
SELECT *
FROM orders
WHERE customer_id = 42;
```

The plan contains execution nodes and planner estimates.

Common scan nodes:

```text
Seq Scan     sequential table scan
Index Scan   lookup through an index
Bitmap Scan  bitmap-based index/table access
```

`Filter` shows a condition applied at a plan node.

An existing index does not guarantee `Index Scan`. The planner compares costs and may choose a `Seq Scan`, `Index Scan`, or bitmap plan. A condition not directly supported by the chosen index can remain as a separate `Filter`.

The ordering of a B-tree index can sometimes satisfy `ORDER BY` without a separate `Sort` node when the requested order matches an available index scan order. Merely having an index does not guarantee this.

`EXPLAIN` cost values are planner estimates, not execution time in milliseconds.

---

## 10. EXPLAIN ANALYZE

```sql
EXPLAIN ANALYZE
SELECT *
FROM orders
WHERE customer_id = 42;
```

Unlike plain `EXPLAIN`, `EXPLAIN ANALYZE` **actually executes the query** and adds observed metrics such as real row counts and execution times for plan nodes.

Difference:

```text
EXPLAIN
→ plan + estimates

EXPLAIN ANALYZE
→ executes query + plan + actual measurements
```

Important consequence:

```sql
EXPLAIN ANALYZE DELETE FROM orders ...;
```

really executes the `DELETE`.

To inspect a modifying statement without keeping its changes, it can be run inside a transaction followed by rollback:

```sql
BEGIN;
EXPLAIN ANALYZE
UPDATE ...;
ROLLBACK;
```

The statement still executes while the transaction is open, so its runtime effects still matter, including applicable triggers. `EXPLAIN ANALYZE` does not automatically roll the command back.

---

## 11. MVCC, isolation, and locks

Concurrent transactions require rules for which committed states each transaction can see.

PostgreSQL's default isolation level is `READ COMMITTED`.

At this level, each SQL statement sees a snapshot that is current at the start of **that statement**. Two `SELECT` statements in one transaction can therefore observe different changes committed by other transactions between those statements.

Stronger levels include:

```text
READ COMMITTED
REPEATABLE READ
SERIALIZABLE
```

Stronger isolation can require retrying transactions when conflicts cannot be serialized safely.

MVCC often allows reads and ordinary writes to avoid blocking each other, but this does not mean PostgreSQL has no locks. PostgreSQL supports table-level and row-level locks; concurrent updates of the same rows, DDL, and explicit locking can still wait or conflict.

Waiting for a lock is not automatically a deadlock. A deadlock occurs when transactions form a cycle of resource waits; PostgreSQL detects the cycle and aborts one transaction.

Practical model:

```text
MVCC       → row versions and visibility
isolation  → which database states a transaction may observe
locks      → coordination of conflicting operations
```

These concepts are related but not interchangeable.

---

## 12. ONLY and inheritance

PostgreSQL supports table inheritance. A query against a parent table normally includes rows from child tables.

To target the parent table only:

```sql
SELECT *
FROM ONLY cities;
```

In the introductory material, `ONLY` is also used with `UPDATE` and `DELETE` in the inheritance context.

Meaning:

```text
cities       → parent + descendants
ONLY cities  → rows directly in cities only
```

---

# What to remember

Advanced row sources:

```text
LATERAL         right-side source may depend on rows to its left
WITH ORDINALITY adds row numbers starting at 1
WITH RECURSIVE  reuses results from the previous recursive step
```

Grouping:

```text
GROUPING SETS  several grouping layouts
ROLLUP         hierarchical subtotals
CUBE           combinations of dimensions
```

Windows:

```text
PARTITION BY  partition
ORDER BY      order inside partition
frame         rows used for the current calculation
```

Indexes:

```text
multicolumn index → several key columns
expression index  → result of an expression
partial index     → rows matching a predicate only
```

Diagnostics:

```text
EXPLAIN         estimates without executing the main query
EXPLAIN ANALYZE executes the query and shows actual measurements
```

Concurrency:

```text
READ COMMITTED → PostgreSQL default isolation level
MVCC           → row versions and visibility
locks          → waiting/protection for conflicting operations
```

Main practical rule: do not guess whether a query is “fast”. Inspect the plan, actual execution, data volume, and whether indexes match the real access pattern.
