# Initial — PL/pgSQL basics

## 1. Topic

PL/pgSQL is PostgreSQL's procedural language. It is useful when a single SQL statement is not enough and it is more convenient to execute the logic **inside the server**: store an intermediate value in a variable, branch, run a loop, execute several SQL statements, and return a result.

PL/pgSQL is used to create:

- functions;
- procedures;
- trigger functions.

The main model:

```text
SQL                         PL/pgSQL
-------------------------   ----------------------------------
describe required data      add a sequence of actions
work with sets of rows       keep local state
JOIN / GROUP BY / UPDATE    IF / CASE / LOOP / FOR / WHILE
one SQL statement           a block of SQL and procedural statements
```

PL/pgSQL does not replace SQL. Inside PL/pgSQL, ordinary `SELECT`, `INSERT`, `UPDATE`, and `DELETE` statements are still executed by PostgreSQL's SQL engine. The procedural language only connects these operations with control-flow logic.

This is especially useful when several actions should be performed close to the data on the server: intermediate results do not have to travel between the client and server, while SQL can be combined with conditions and loops.

---

## 2. Mechanism

### 2.1. What a PL/pgSQL function looks like

A minimal function:

```sql
CREATE FUNCTION add_tax(subtotal numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

The function body is passed to `CREATE FUNCTION` as a string. In practice it is usually wrapped in dollar quotes (`$$ ... $$` or a named delimiter such as `$func$ ... $func$`) so that every single quote inside the body does not have to be escaped.

PL/pgSQL itself is a block-structured language. The general block form is:

```sql
[ <<label>> ]
[ DECLARE
    declarations ]
BEGIN
    statements
END [ label ];
```

`DECLARE` is optional. The executable `BEGIN ... END` section is required.

Important: `BEGIN` and `END` here **group PL/pgSQL statements**. They are not the SQL `BEGIN`/`COMMIT` commands used for transaction control.

### 2.2. Semicolons and nested blocks

Declarations and statements end with `;`:

```sql
DECLARE
    total integer := 0;
BEGIN
    total := total + 1;
    RETURN total;
END;
```

Do not put a semicolon immediately after `BEGIN`:

```sql
BEGIN;
    -- wrong
```

Blocks can be nested:

```sql
<<outer_block>>
DECLARE
    value integer := 10;
BEGIN
    DECLARE
        value integer := 20;
    BEGIN
        RAISE NOTICE 'inner = %', value;
        RAISE NOTICE 'outer = %', outer_block.value;
    END;
END;
```

An inner variable with the same name hides the outer one. A block label lets you explicitly refer to the outer variable.

### 2.3. Declaring variables

Ordinary local variables are declared in `DECLARE`:

```sql
DECLARE
    counter integer;
    price numeric(10, 2) := 0;
    title text DEFAULT 'unknown';
    started_at CONSTANT timestamptz := now();
BEGIN
    ...
END;
```

The general meaning of a declaration is:

```text
name [CONSTANT] type [COLLATE ...] [NOT NULL]
     [DEFAULT expression | := expression | = expression]
```

If no initial value is specified, the variable receives the SQL value `NULL`, not `0` and not an empty string.

Example:

```sql
DECLARE
    attempts integer;       -- NULL
    enabled boolean := true;
    code text NOT NULL := 'A';
BEGIN
    ...
END;
```

`CONSTANT` prevents subsequent modification of the value. `NOT NULL` prevents assigning `NULL` and therefore requires a non-NULL initial value. `COLLATE` lets you specify a collation for a collatable type.

The initial expression is evaluated **each time the block is entered**. For example:

```sql
DECLARE
    started_at timestamptz := now();
```

gets a value for the current execution of the block.

### 2.4. Assignment

PL/pgSQL accepts both forms:

```sql
counter := counter + 1;
counter = counter + 1;
```

`:=` is more commonly used because it is visually easier to distinguish assignment from SQL comparison.

You can assign not only to a simple variable:

```sql
row_value.id := 10;
items[1] := 'first';
```

The expression on the right-hand side is evaluated by PostgreSQL's SQL engine.

### 2.5. Function parameters

Parameters can be named directly in `CREATE FUNCTION`:

```sql
CREATE FUNCTION add_tax(subtotal numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

Parameters are also available as `$1`, `$2`, ...:

```sql
CREATE FUNCTION add_two(integer, integer)
RETURNS integer
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN $1 + $2;
END;
$$;
```

An older but still supported way to name a positional parameter is `ALIAS`:

```sql
CREATE FUNCTION add_tax(numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
DECLARE
    subtotal ALIAS FOR $1;
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

For new code it is usually clearer to name parameters directly in the function header.

### 2.6. Ordinary SQL statements inside PL/pgSQL

SQL statements that do not return a row set to the calling PL/pgSQL code are written almost as usual:

```sql
INSERT INTO audit_log(message)
VALUES ('started');

UPDATE jobs
SET status = 'done'
WHERE id = job_id;
```

PL/pgSQL variables can be used as values inside ordinary optimizable SQL statements.

### 2.7. `PERFORM`: execute a query and discard the result

In ordinary SQL you can write:

```sql
SELECT refresh_cache();
```

But inside PL/pgSQL, a query that returns rows cannot simply be left without handling its result. If the result is not needed, use `PERFORM`:

```sql
PERFORM refresh_cache();
```

This means:

```text
execute the query
→ do not store the result
→ preserve side effects
```

`PERFORM` uses query syntax similar to `SELECT`, but the keyword `SELECT` is replaced with `PERFORM`.

Example:

```sql
PERFORM 1
FROM jobs
WHERE status = 'failed';
```

After `PERFORM`, the `FOUND` variable tells you whether the query returned at least one row.

### 2.8. `SELECT ... INTO`: put a row into variables

To save a `SELECT` result into variables:

```sql
DECLARE
    v_name text;
    v_email text;
BEGIN
    SELECT name, email
    INTO v_name, v_email
    FROM users
    WHERE id = 42;
END;
```

This is **PL/pgSQL `INTO`**, meaning capture the result into variables. Do not confuse it with the SQL `SELECT INTO` command that creates a table.

Without `STRICT`, the behavior is:

```text
0 rows     → targets receive NULL, FOUND = false
1 row      → it is assigned to the targets, FOUND = true
>1 rows    → the first row is used, the rest are discarded
```

If it matters which row is considered first, use an explicit `ORDER BY`.

The result can be stored in:

- a list of scalar variables;
- a row variable;
- a `record`.

`STRICT`, which requires exactly one row, is covered in detail at the next level.

### 2.9. `FOUND`: status of the last relevant operation

`FOUND` is a special local variable of type `boolean`. It is `false` when the function is entered.

Example:

```sql
SELECT *
INTO user_row
FROM users
WHERE id = p_id;

IF NOT FOUND THEN
    RAISE EXCEPTION 'user % not found', p_id;
END IF;
```

Several different statements modify `FOUND`.

For the basic level, remember:

```text
SELECT ... INTO      true if a row was obtained
PERFORM              true if the query returned at least one row
INSERT/UPDATE/DELETE true if at least one row was affected
MERGE                true if at least one row was affected
FETCH                true if a cursor row was obtained
MOVE                 true if cursor movement succeeded
FOR / FOREACH        true after the loop if at least one iteration occurred
RETURN QUERY         true if the query added at least one row
```

`FOUND` is not a row count. For the exact number there is `GET DIAGNOSTICS ... ROW_COUNT`, which is covered later.

### 2.10. Conditions: `IF` and `CASE`

A simple `IF`:

```sql
IF balance < 0 THEN
    status := 'debt';
END IF;
```

With an alternative:

```sql
IF balance < 0 THEN
    status := 'debt';
ELSE
    status := 'ok';
END IF;
```

Several conditions:

```sql
IF score >= 90 THEN
    grade := 'A';
ELSIF score >= 75 THEN
    grade := 'B';
ELSE
    grade := 'C';
END IF;
```

`CASE` is also supported:

```sql
CASE status
    WHEN 'new' THEN priority := 1;
    WHEN 'retry' THEN priority := 2;
    ELSE priority := 3;
END CASE;
```

There is also the `CASE WHEN condition THEN ...` form.

### 2.11. Loops

#### Unconditional `LOOP`

```sql
LOOP
    counter := counter + 1;
    EXIT WHEN counter >= 10;
END LOOP;
```

`EXIT` terminates the loop, while `CONTINUE` proceeds to the next iteration.

#### `WHILE`

```sql
WHILE counter < 10 LOOP
    counter := counter + 1;
END LOOP;
```

#### Integer `FOR`

```sql
FOR i IN 1..10 LOOP
    total := total + i;
END LOOP;
```

The variable `i` is created automatically as an `integer` for the duration of the loop.

`REVERSE` and `BY` are also available:

```sql
FOR i IN REVERSE 10..1 BY 2 LOOP
    ...
END LOOP;
```

#### `FOR` over query rows

```sql
FOR r IN
    SELECT id, name FROM users WHERE active
LOOP
    RAISE NOTICE '%: %', r.id, r.name;
END LOOP;
```

The body executes once for each result row.

#### `FOREACH` over an array

```sql
FOREACH item IN ARRAY items LOOP
    total := total + item;
END LOOP;
```

More advanced loop forms and cursor work are covered at later levels.

### 2.12. `RETURN`: finish a function

For a function that returns one value:

```sql
RETURN expression;
```

For example:

```sql
RETURN subtotal * 1.20;
```

`RETURN expression` stops function execution and gives the value to the caller.

For a `void` function or a function with output parameters, you can simply use:

```sql
RETURN;
```

### 2.13. Set-returning functions: `RETURN NEXT` and `RETURN QUERY`

If a function returns a set of rows, it can add results in parts.

`RETURN NEXT` adds one row:

```sql
RETURN NEXT value;
```

`RETURN QUERY` adds all rows from a query:

```sql
RETURN QUERY
SELECT id
FROM users
WHERE active;
```

The key difference from ordinary `RETURN`:

```text
RETURN value       → finish the function now
RETURN NEXT value  → add a row and continue
RETURN QUERY query → add query rows and continue
```

`RETURN NEXT` and `RETURN QUERY` can be mixed in the same set-returning function. The final `RETURN;` only ends further execution.

---

## 3. Practical model

When writing a simple function, it is useful to work from top to bottom:

```text
1. What result should the function return?
2. What input parameters does it need?
3. What local values need to be stored?
4. What SQL statements are executed?
5. Is branching needed?
6. Is a loop needed?
7. What should happen if SELECT finds nothing?
8. Where should the function finish?
```

Example:

```sql
CREATE FUNCTION user_label(p_id bigint)
RETURNS text
LANGUAGE plpgsql
AS $$
DECLARE
    v_name text;
    v_active boolean;
BEGIN
    SELECT name, active
    INTO v_name, v_active
    FROM users
    WHERE id = p_id;

    IF NOT FOUND THEN
        RETURN 'missing';
    END IF;

    IF v_active THEN
        RETURN v_name || ' [active]';
    END IF;

    RETURN v_name || ' [inactive]';
END;
$$;
```

This example combines several core mechanisms at once: parameters, `DECLARE`, `SELECT INTO`, `FOUND`, `IF`, and `RETURN`.

---

# What to remember

- PL/pgSQL is a server-side procedural layer on top of SQL, not a replacement for SQL.
- It is used for functions, procedures, and triggers and adds variables, conditions, loops, and other control-flow logic.
- A block has an optional `DECLARE` section and an executable `BEGIN ... END` section.
- PL/pgSQL `BEGIN/END` group statements and do not control a transaction by themselves.
- Ordinary local variables are declared in `DECLARE`; without an initial value they receive `NULL`.
- Declarations can use `DEFAULT`/`:=`/`=`, `CONSTANT`, `NOT NULL`, and `COLLATE`.
- Both `:=` and `=` are valid for assignment.
- A function parameter can be referenced by name, as `$1`, `$2`, ... or through `ALIAS FOR $n`.
- `PERFORM` executes a query when its result is not needed.
- `SELECT ... INTO` stores one selected row in variables; without `STRICT`, no row means `NULL`.
- `FOUND` is a boolean status flag for several operations; it is `false` when the function is entered.
- `IF`, `CASE`, `LOOP`, `WHILE`, `FOR`, and `FOREACH` provide procedural control flow.
- `RETURN value` finishes the function; `RETURN NEXT` and `RETURN QUERY` accumulate rows and continue execution.
