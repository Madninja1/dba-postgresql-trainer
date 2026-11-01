# Middle — confident PL/pgSQL

## 1. Topic

At the basic PL/pgSQL level, variables, `SELECT INTO`, conditions, and loops are enough. In practice, more complex questions quickly appear:

- how to avoid duplicating a column type in a variable declaration;
- how to store an entire table row;
- what to do if a query returns zero or multiple rows;
- how to build SQL when the table or column name is known only at runtime;
- how to get the exact number of processed rows;
- how to catch errors;
- how to read a large result through a cursor;
- how to produce diagnostic messages.

This level is devoted to exactly these mechanisms.

---

## 2. Mechanism

### 2.1. `%TYPE`: inherit the type of an existing value

Instead of hard-coding the column type again:

```sql
DECLARE
    v_user_id bigint;
```

you can tie the variable type to the table definition:

```sql
DECLARE
    v_user_id users.user_id%TYPE;
```

`%TYPE` takes the type of the specified variable or column.

The advantage is less duplication. If the type of `users.user_id` changes later, the function declaration does not contain separate knowledge that it used to be `bigint`.

Example:

```sql
DECLARE
    v_price products.price%TYPE;
    v_login users.login%TYPE;
```

`%TYPE` inherits the **type**, not the value.

### 2.2. `%ROWTYPE`: a variable with row structure

When you need an entire row from a table or view:

```sql
DECLARE
    v_user users%ROWTYPE;
BEGIN
    SELECT *
    INTO v_user
    FROM users
    WHERE id = p_id;

    RAISE NOTICE 'login = %', v_user.login;
END;
```

`users%ROWTYPE` has the structure of a `users` row.

Fields are accessed with dot notation:

```sql
v_user.id
v_user.login
v_user.created_at
```

This is convenient when the function genuinely works with a row as one object.

### 2.3. `RECORD`: a row without a predefined structure

`RECORD` differs from `%ROWTYPE`:

```sql
DECLARE
    rec record;
```

It has no predefined set of fields. Its structure appears after a row is assigned by a query or loop:

```sql
SELECT id, login
INTO rec
FROM users
WHERE id = p_id;
```

After that, these are available:

```sql
rec.id
rec.login
```

But before the first assignment, the structure of a `record` is unknown, and trying to access a field causes a runtime error.

Mental model:

```text
%TYPE     → type of one value
%ROWTYPE  → predefined row structure of a table/view
RECORD    → structure is determined by the row received at runtime
```

### 2.4. `SELECT ... INTO` without `STRICT`

Recall the basic behavior:

```sql
SELECT id, login
INTO v_id, v_login
FROM users
WHERE email = p_email;
```

Without `STRICT`:

```text
0 rows   → targets receive NULL, FOUND = false
1 row    → the row is assigned, FOUND = true
>1 rows  → the first row is assigned
```

If multiple rows are possible and choosing the first one matters, specify `ORDER BY`. Otherwise, the "first" row is not reliably defined.

### 2.5. `INTO STRICT`: exactly one row is required

If a query must return exactly one row:

```sql
SELECT id, login
INTO STRICT v_id, v_login
FROM users
WHERE email = p_email;
```

`STRICT` changes the semantics:

```text
0 rows   → NO_DATA_FOUND
1 row    → success, FOUND = true
>1 rows  → TOO_MANY_ROWS
```

The errors can be handled in `EXCEPTION`:

```sql
BEGIN
    SELECT *
    INTO STRICT v_user
    FROM users
    WHERE email = p_email;
EXCEPTION
    WHEN NO_DATA_FOUND THEN
        RAISE EXCEPTION 'user not found';
    WHEN TOO_MANY_ROWS THEN
        RAISE EXCEPTION 'email is not unique';
END;
```

For `INSERT/UPDATE/DELETE ... RETURNING ... INTO`, PL/pgSQL cannot simply choose an arbitrary row if several are returned: multiple rows cause an error even without `STRICT`.

### 2.6. When dynamic `EXECUTE` is needed

Ordinary SQL inside PL/pgSQL works well when the command structure is constant:

```sql
SELECT count(*)
INTO v_count
FROM users
WHERE status = p_status;
```

But a parameter value and an object name are different things.

You can pass a **value** like this:

```sql
WHERE status = p_status
```

But an ordinary query parameter cannot replace a table name:

```text
FROM <table name from a variable>
```

If the SQL text itself must change — for example, a different table or column is selected — use PL/pgSQL `EXECUTE`:

```sql
EXECUTE command_text;
```

`command_text` is an expression that must produce a `text` string containing the command to execute. The full form is:

```sql
EXECUTE command_text
    [ INTO [STRICT] target ]
    [ USING expression [, ...] ];
```

### 2.7. `EXECUTE ... USING`: values separate from SQL text

A safe form for data values:

```sql
EXECUTE
    'SELECT count(*) FROM users WHERE status = $1 AND created_at >= $2'
INTO v_count
USING p_status, p_since;
```

Here `$1` and `$2` refer to the expressions after `USING`.

Advantages:

- values do not have to be manually converted into SQL text;
- original data types are preserved;
- quotes inside values do not have to be escaped manually;
- the risk of SQL injection is substantially reduced.

But these parameters substitute **data**, not SQL identifiers.

This does not work:

```sql
EXECUTE 'SELECT * FROM $1' USING table_name;
```

The table name must be safely inserted into the command text.

### 2.8. Dynamic identifiers: `format('%I', ...)`

For a table or column name, use `%I`:

```sql
EXECUTE format(
    'SELECT count(*) FROM %I WHERE status = $1',
    table_name
)
INTO v_count
USING p_status;
```

`%I` is intended for SQL identifiers and is equivalent to safe quoting through `quote_ident`.

For a textual representation of a value, `%L` is available and is equivalent to `quote_nullable`:

```sql
EXECUTE format(
    'SELECT * FROM %I WHERE category = %L',
    table_name,
    category_value
);
```

But when possible, it is still better to pass values through `USING`:

```sql
EXECUTE format(
    'SELECT * FROM %I WHERE category = $1',
    table_name
)
USING category_value;
```

This preserves the value's type and avoids converting it to text.

Useful mapping:

```text
%I              → identifier → quote_ident(...)
%L              → SQL literal → quote_nullable(...)
USING $1, $2...  → data values without textual insertion
```

### 2.9. Planning dynamic `EXECUTE`

Unlike ordinary static SQL in PL/pgSQL, a command inside `EXECUTE` is planned again every time it is executed.

At the Middle level, it is enough to understand the consequence:

```text
ordinary static SQL → can reuse a prepared/cached plan
EXECUTE text        → builds a plan on every execution
```

At the Strong level, this will be connected to generic/custom plan selection and performance.

### 2.10. `EXECUTE ... INTO`

A dynamic query can return data into a variable:

```sql
EXECUTE format('SELECT * FROM %I WHERE id = $1', table_name)
INTO rec
USING p_id;
```

`STRICT` is also supported:

```sql
EXECUTE format('SELECT * FROM %I WHERE id = $1', table_name)
INTO STRICT rec
USING p_id;
```

`STRICT` has the same meaning: exactly one row is required.

### 2.11. `GET DIAGNOSTICS`: obtain execution state

`FOUND` answers a boolean question: was there a row, iteration, or modification?

When you need the exact number of rows, use diagnostics:

```sql
DECLARE
    v_rows bigint;
BEGIN
    UPDATE jobs
    SET status = 'expired'
    WHERE expires_at < now();

    GET DIAGNOSTICS v_rows = ROW_COUNT;
END;
```

`ROW_COUNT` is the number of rows processed by the last SQL command.

Compare:

```text
FOUND     → was at least one row affected?
ROW_COUNT → how many rows were processed?
```

`GET DIAGNOSTICS` can also retrieve other pieces of current execution state; `PG_CONTEXT` is covered in more detail at the next level.

### 2.12. Error handling: `EXCEPTION`

By default, an error stops function execution and the current transaction. A block can add a handler:

```sql
BEGIN
    risky_operation();
EXCEPTION
    WHEN division_by_zero THEN
        RAISE NOTICE 'division by zero';
END;
```

Block logic:

```text
BEGIN
    statements
    ↓ error
remaining statements in the block are skipped
    ↓
find the first matching WHEN
    ↓
execute the handler
    ↓
continue after the block's END
```

Conditions can be combined:

```sql
EXCEPTION
    WHEN unique_violation OR foreign_key_violation THEN
        ...
```

`OTHERS` catches all errors except `QUERY_CANCELED` and `ASSERT_FAILURE`. Using such a broad handler without a specific need is usually a bad idea.

### 2.13. What is rolled back after an error inside an `EXCEPTION` block

This is a fundamental PL/pgSQL behavior.

If an error is caught by a handler:

- data changes performed **inside the failing part of the block** are rolled back;
- local PL/pgSQL variables retain the values they had acquired before the error;
- changes made before entering that nested block are not rolled back merely because of its handler.

Example:

```sql
x := 10;

BEGIN
    UPDATE accounts SET balance = balance - 100 WHERE id = 1;
    x := 20;
    y := 1 / 0;
EXCEPTION
    WHEN division_by_zero THEN
        RAISE NOTICE 'x = %', x;
END;
```

After the error is handled, `x` remains `20`, but the `UPDATE` inside the block is undone.

This makes `EXCEPTION` similar in effect to an internal rollback point, although you do not write a manual `SAVEPOINT` here.

### 2.14. `RETURN QUERY EXECUTE`

A set-returning function can add rows from a dynamic query:

```sql
RETURN QUERY EXECUTE
    format('SELECT * FROM %I WHERE active = $1', table_name)
USING true;
```

Like ordinary `RETURN QUERY`, this command **adds** rows to the result and does not finish the function immediately.

It also affects `FOUND`: `true` if the query returned at least one row.

### 2.15. Cursors and `refcursor`

A cursor lets you work with a query result not as one fully materialized value in a variable, but through a position maintained within the row set.

A cursor variable has type:

```sql
refcursor
```

For example:

```sql
DECLARE
    cur refcursor;
```

Or a cursor can be bound to a query in its declaration:

```sql
DECLARE
    cur CURSOR FOR
        SELECT id, login FROM users ORDER BY id;
```

### 2.16. `OPEN`, `FETCH`, `MOVE`, `CLOSE`

The main operations are:

```text
OPEN   → open the cursor
FETCH  → retrieve a row and move the position
MOVE   → move the position without retrieving a row
CLOSE  → close the cursor and release the portal
```

Example:

```sql
OPEN cur FOR
    SELECT id, login FROM users ORDER BY id;

FETCH cur INTO v_id, v_login;
MOVE cur;
CLOSE cur;
```

After `FETCH` and `MOVE`, you can check `FOUND`.

A cursor is associated with a server-side portal. Open portals exist within a transaction and are automatically closed at its end.

### 2.17. Looping over a bound cursor

A bound cursor can be used directly in `FOR`:

```sql
DECLARE
    cur CURSOR FOR SELECT id, login FROM users ORDER BY id;
BEGIN
    FOR rec IN cur LOOP
        RAISE NOTICE '%', rec.login;
    END LOOP;
END;
```

Such a `FOR` opens the cursor itself and closes it when the loop finishes.

For ordinary traversal of a result, `FOR rec IN SELECT ... LOOP` is often simpler than explicit cursor management. A cursor is useful when you need a separate position, `FETCH/MOVE`, passing a `refcursor` to the caller, or other special semantics.

### 2.18. `RAISE`: messages and errors

`RAISE` is used both for diagnostic messages and for exceptions.

Levels:

```text
DEBUG
LOG
INFO
NOTICE
WARNING
EXCEPTION
```

`EXCEPTION` creates an error by default and interrupts normal control flow. The other levels generate messages with different severities.

Example:

```sql
RAISE NOTICE 'processing user %', p_user_id;
```

`%` substitutes the next argument. A literal percent sign is written as `%%`.

Structured information can be added:

```sql
RAISE EXCEPTION 'user % not found', p_user_id
    USING ERRCODE = 'P0002',
          DETAIL = 'lookup failed',
          HINT = 'check the identifier';
```

Supported fields include:

- `MESSAGE`;
- `DETAIL`;
- `HINT`;
- `ERRCODE`;
- names of related objects (`TABLE`, `COLUMN`, `CONSTRAINT`, ...).

`RAISE;` without arguments inside a handler rethrows the current error.

### 2.19. `ASSERT`: verify developer assumptions

For debugging invariants there is:

```sql
ASSERT condition [, message];
```

If the condition is false or `NULL`, `ASSERT_FAILURE` is raised.

`ASSERT` is intended to detect defects in the program, not for normal business validation. Use `RAISE` for user-facing errors.

---

## 3. How to choose the mechanism

```text
Need the type of a specific column?        → %TYPE
Need a row of a specific table?             → %ROWTYPE
Row structure is known only later?          → RECORD
Need at most the first row?                  → INTO
Need exactly one row?                        → INTO STRICT
Only a value changes in SQL?                 → ordinary variable / parameter
Table/column name changes?                   → EXECUTE + format(%I)
Need to pass a data value into EXECUTE?      → USING
Need the number of affected rows?            → GET DIAGNOSTICS ... ROW_COUNT
Need to handle an error?                     → EXCEPTION ... WHEN
Need positional work with a row set?         → cursor / refcursor
Need a message/error?                        → RAISE
```

---

# What to remember

- `%TYPE` inherits the type of a column or variable.
- `%ROWTYPE` provides the predefined row structure of a table/view.
- `RECORD` obtains its structure at runtime and has no fields before the first assignment.
- `INTO` without `STRICT` takes the first row or produces `NULL` if there are no rows.
- `INTO STRICT` requires exactly one row: `NO_DATA_FOUND` for zero, `TOO_MANY_ROWS` for several.
- `EXECUTE` is needed when the command text itself changes, especially object identifiers.
- In dynamic SQL, values should preferably be passed through `$1`, `$2`, ... and `USING`.
- Identifiers are formed through `format('%I', ...)`/`quote_ident`; `%L` corresponds to quoting a value as an SQL literal.
- Dynamic `EXECUTE` is planned on every execution.
- `GET DIAGNOSTICS ... ROW_COUNT` gives the exact number of processed rows; `FOUND` is only boolean state.
- `EXCEPTION ... WHEN` catches errors; data changes inside the failing part of the block are rolled back, while local variables retain their values.
- A cursor variable has type `refcursor`; the core operations are `OPEN`, `FETCH`, `MOVE`, and `CLOSE`.
- `RETURN QUERY EXECUTE` adds the result of a dynamic query to a set-returning function.
- `RAISE` generates messages at different levels, while `RAISE EXCEPTION` raises an error.
