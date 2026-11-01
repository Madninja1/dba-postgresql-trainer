# Initial — SQL basics

## 1. Topic

SQL is a language for working with relational data. At the first level, the goal is not to memorize isolated commands but to understand the basic workflow:

```text
define a table
      ↓
insert data
      ↓
select rows and columns
      ↓
join related tables
      ↓
group and aggregate
      ↓
update or delete data
```

PostgreSQL is a relational database management system. In the basic model, a **relation** corresponds to a table: a named set of columns with types and rows containing data.

SQL does not guarantee row order by itself. If order matters, request it explicitly with `ORDER BY`.

---

## 2. Tables and data types

A table is created with `CREATE TABLE`:

```sql
CREATE TABLE users (
    id integer,
    name text,
    created_at timestamp
);
```

Here:

```text
users       table name
id          integer column
name        text column
created_at  timestamp column
```

A data type limits valid values and defines which operations make sense for a column.

To remove the table itself:

```sql
DROP TABLE users;
```

`DROP TABLE` removes the table object and its data. This is different from `DELETE`, which removes rows while leaving the table in place.

---

## 3. INSERT — adding rows

Rows are added with `INSERT`:

```sql
INSERT INTO users (id, name, created_at)
VALUES (1, 'Alice', '2026-01-10 12:00:00');
```

Prefer an explicit column list:

```sql
INSERT INTO users (name, id)
VALUES ('Bob', 2);
```

This makes the statement independent of the physical column order and clearly shows where every value goes.

A shorter form also exists:

```sql
INSERT INTO users
VALUES (3, 'Carol', '2026-01-11 09:30:00');
```

It depends on the exact table column order and is therefore more fragile when the schema changes.

For bulk loading, PostgreSQL provides `COPY`. In the server-side form:

```sql
COPY users FROM '/path/users.txt';
```

the file is read by the **PostgreSQL server**, not by the client that sent the command. `COPY` is optimized for bulk transfer and is generally a better fit than many individual `INSERT` statements.

---

## 4. SELECT — reading data

Basic form:

```sql
SELECT id, name
FROM users;
```

`SELECT` defines the result and `FROM` defines the data source.

```sql
SELECT *
FROM users;
```

`*` means “all columns”.

This is convenient for ad-hoc inspection. In application code, listing the required columns explicitly is safer because the result does not silently change when new columns are added.

Expressions may also appear in the select list:

```sql
SELECT
    city,
    (temp_hi + temp_lo) / 2 AS temp_avg
FROM weather;
```

`AS` gives a name to an output column.

---

## 5. WHERE — filtering rows

`WHERE` keeps rows for which the condition is true:

```sql
SELECT city, temp_hi
FROM weather
WHERE temp_hi > 25;
```

Conditions can be combined:

```sql
SELECT city, temp_hi
FROM weather
WHERE city = 'San Francisco'
  AND temp_hi > 20;
```

Basic boolean operators:

```text
AND  both conditions must be true
OR   at least one condition must be true
NOT  logical negation
```

`WHERE` filters rows **before** grouping and aggregate calculation.

---

## 6. ORDER BY and DISTINCT

Without `ORDER BY`, result row order is not defined.

```sql
SELECT city, temp_lo
FROM weather
ORDER BY city, temp_lo;
```

Rows are ordered by `city`, then by `temp_lo` within equal cities.

To remove duplicate result rows, use `DISTINCT`:

```sql
SELECT DISTINCT city
FROM weather
ORDER BY city;
```

`DISTINCT` controls uniqueness, not ordering. If stable order matters, still use `ORDER BY`.

---

## 7. JOIN — combining tables

Relational data is usually split across multiple tables. `JOIN` combines related rows into one result.

```sql
SELECT
    w.city,
    w.temp_hi,
    c.location
FROM weather AS w
JOIN cities AS c
    ON c.name = w.city;
```

`w` and `c` are table aliases. They shorten expressions and make column ownership explicit:

```text
w.city      city from weather
c.location  location from cities
```

Plain `JOIN` is an inner join: only rows that match the `ON` condition are returned.

### LEFT JOIN

```sql
SELECT
    w.city,
    c.location
FROM weather AS w
LEFT JOIN cities AS c
    ON c.name = w.city;
```

`LEFT JOIN` keeps **all rows from the left table**. If there is no match on the right, right-side columns become `NULL`.

Simplified model:

```text
INNER JOIN → matching pairs only
LEFT JOIN  → all left rows + matching right rows
```

### Self join

A table may be joined to itself. Aliases are then essential because the two occurrences represent different roles:

```sql
SELECT w1.city, w2.city
FROM weather AS w1
JOIN weather AS w2
    ON w1.temp_lo < w2.temp_lo;
```

---

## 8. Aggregates and GROUP BY

An aggregate function reduces a set of rows to one value.

Common aggregates:

```text
count  number of rows/values
sum    total
avg    average
min    minimum
max    maximum
```

Example:

```sql
SELECT count(*)
FROM weather;
```

To calculate aggregates separately for different groups, use `GROUP BY`:

```sql
SELECT
    city,
    count(*) AS measurements,
    max(temp_hi) AS max_temp
FROM weather
GROUP BY city;
```

After `GROUP BY city`, each group contains rows for one city and aggregates are calculated independently for each group.

### WHERE vs HAVING

They filter at different stages:

```text
WHERE    filters source rows
GROUP BY creates groups
HAVING   filters resulting groups
```

```sql
SELECT city, count(*)
FROM weather
WHERE temp_hi > 0
GROUP BY city
HAVING count(*) > 1;
```

PostgreSQL also supports `FILTER` for a single aggregate:

```sql
SELECT
    count(*) AS all_rows,
    count(*) FILTER (WHERE temp_hi > 20) AS warm_rows
FROM weather;
```

The second aggregate sees only rows accepted by its own filter.

---

## 9. UPDATE and DELETE

Update existing rows:

```sql
UPDATE users
SET name = 'Robert'
WHERE id = 2;
```

Delete rows:

```sql
DELETE FROM users
WHERE id = 2;
```

Safety rule:

```sql
DELETE FROM users;
```

without `WHERE` deletes **all rows in the table**.

The same mindset applies to `UPDATE`: without `WHERE`, every row is updated.

---

## 10. Transactions

A transaction combines several SQL statements into one logical operation.

```sql
BEGIN;

UPDATE accounts
SET balance = balance - 100
WHERE name = 'Alice';

UPDATE accounts
SET balance = balance + 100
WHERE name = 'Bob';

COMMIT;
```

Model:

```text
BEGIN
  ↓
related changes
  ↓
COMMIT   keep all changes
or
ROLLBACK discard all changes
```

To cancel the transaction:

```sql
ROLLBACK;
```

PostgreSQL also supports savepoints:

```sql
BEGIN;

UPDATE accounts
SET balance = balance - 100
WHERE name = 'Alice';

SAVEPOINT before_receiver;

UPDATE accounts
SET balance = balance + 100
WHERE name = 'Bob';

ROLLBACK TO before_receiver;

UPDATE accounts
SET balance = balance + 100
WHERE name = 'Wally';

COMMIT;
```

`ROLLBACK TO` cancels changes made after the savepoint without cancelling the whole transaction.

---

## 11. Identifiers and case

SQL keywords are commonly written in uppercase for readability:

```sql
SELECT name FROM users;
```

PostgreSQL folds **unquoted identifiers** to lowercase.

```sql
CREATE TABLE Users (...);
```

therefore creates the name `users`.

Double quotes preserve identifier case:

```sql
CREATE TABLE "Users" (...);
```

Now `"Users"` and `users` are different identifiers. Quoted mixed-case object names therefore add unnecessary friction unless they are explicitly required.

String values use **single quotes**:

```sql
SELECT 'Alice';
```

---

# What to remember

Main sequence:

```text
CREATE TABLE  define structure
INSERT        add rows
SELECT        read data
WHERE         filter rows
ORDER BY      order rows
DISTINCT      remove result duplicates
JOIN          combine tables
GROUP BY      create groups
HAVING        filter groups
UPDATE        change rows
DELETE        remove rows
```

Key distinctions:

```text
INNER JOIN → matching rows only
LEFT JOIN  → all left rows, missing right side = NULL

WHERE      → before grouping
HAVING     → after grouping

DELETE ... WHERE ... → selected rows
DELETE without WHERE → all rows

COMMIT      → keep transaction changes
ROLLBACK    → cancel transaction
ROLLBACK TO → return to a SAVEPOINT
```

Three practical rules:

1. Need deterministic order — use `ORDER BY`.
2. Prefer explicit column lists in `INSERT` and application `SELECT` statements.
3. Before `UPDATE` or `DELETE`, verify the `WHERE` condition.
