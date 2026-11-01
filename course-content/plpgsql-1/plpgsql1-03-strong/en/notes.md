# Strong — advanced PL/pgSQL

## 1. Topic

At the Strong level, it is important not merely to know PL/pgSQL syntax, but to understand its **runtime semantics**:

- when SQL is parameterized automatically and when dynamic `EXECUTE` is required;
- why an identifier cannot be passed through `$1`;
- how PL/pgSQL caches static plans;
- why `EXECUTE` is sometimes used deliberately to obtain a new plan;
- how rollback inside `EXCEPTION` works;
- how current diagnostics differ from diagnostics for a caught error;
- what data is available to a trigger function and what its return value means;
- where `COMMIT`/`ROLLBACK` are allowed;
- how to avoid conflicts between variable names and column names.

This is no longer a collection of commands, but a model of how server-side code executes.

---

## 2. Mechanism

### 2.1. Static SQL: values become parameters

Consider this function:

```sql
CREATE FUNCTION count_users(p_status text)
RETURNS bigint
LANGUAGE plpgsql
AS $$
DECLARE
    v_count bigint;
BEGIN
    SELECT count(*)
    INTO v_count
    FROM users
    WHERE status = p_status;

    RETURN v_count;
END;
$$;
```

`p_status` in an optimizable SQL command is used as a parameter value. PL/pgSQL does not insert it as an arbitrary fragment of SQL text.

This works for commands where the SQL engine accepts query parameters, including `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `MERGE`, and certain commands that contain them.

But a query parameter can represent a **value**, not an SQL syntactic object.

Therefore a parameter is suitable for:

```sql
WHERE id = p_id
```

but cannot directly replace:

```text
table name
column name
ORDER BY <arbitrary identifier>
```

If the command structure changes, `EXECUTE` is required.

### 2.2. Dynamic `EXECUTE`: a new plan every time

Form:

```sql
EXECUTE command_text
    [ INTO [STRICT] target ]
    [ USING expression [, ...] ];
```

A command run through PL/pgSQL `EXECUTE` does not have a cached plan reused across executions. The text is parsed and the plan is built again each time.

This is simultaneously:

- the cost of dynamic SQL;
- an important property of dynamic SQL.

If the optimal plan depends heavily on current parameter values, explicit `EXECUTE` is sometimes used even when the command text is unchanged, in order to make PostgreSQL replan the query using the current values.

For example:

```sql
EXECUTE
    'SELECT * FROM events WHERE tenant_id = $1 AND created_at >= $2'
USING p_tenant_id, p_since;
```

### 2.3. `USING`: data values only

Parameters `$1`, `$2`, ... inside `EXECUTE` correspond to expressions in `USING`:

```sql
EXECUTE
    'UPDATE accounts SET balance = balance + $1 WHERE id = $2'
USING p_delta, p_id;
```

They:

- pass data values;
- preserve their types without requiring conversion to text;
- remove the need to quote values manually;
- work only where the executed SQL command accepts query parameters.

But they do not parameterize identifiers:

```sql
-- wrong idea
EXECUTE 'SELECT * FROM $1' USING table_name;
```

`$1` cannot become a table name here.

### 2.4. `format()` for dynamic identifiers

The correct combination is:

```sql
EXECUTE format(
    'SELECT count(*) FROM %I WHERE tenant_id = $1',
    table_name
)
INTO v_count
USING p_tenant_id;
```

The roles are separated here:

```text
%I     → SQL identifier
$1     → data value
```

`%I` is equivalent to `quote_ident`.

For a value that really must be inserted into the text as an SQL literal, `%L` is available and is equivalent to `quote_nullable`:

```sql
EXECUTE format(
    'SELECT * FROM %I WHERE category = %L',
    table_name,
    category
);
```

If `USING` can be used, it is generally preferable to `%L` for data values.

### 2.5. `quote_literal` and `quote_nullable`

`quote_literal(value)` produces a textual SQL literal, but the function itself is `STRICT`: for `NULL` it returns `NULL`.

This is dangerous during concatenation:

```sql
'... ' || quote_literal(nullable_value)
```

If `nullable_value IS NULL`, the whole concatenation result may become `NULL`, and `EXECUTE` will not receive a valid command string.

For values that may be `NULL`, use:

```sql
quote_nullable(value)
```

or `%L` in `format()`.

But remember SQL `NULL` semantics: the condition

```sql
WHERE key = NULL
```

never becomes true. If `NULL` must be treated as an ordinary comparable value, `IS NOT DISTINCT FROM` may be needed.

### 2.6. Dollar quoting does not escape unknown dynamic values

Dollar quoting is excellent for **fixed function-body text**:

```sql
CREATE FUNCTION f()
RETURNS void
LANGUAGE plpgsql
AS $func$
BEGIN
    ...
END;
$func$;
```

But this idea is not safe:

```text
take unknown user-supplied text
→ wrap it in $$...$$
→ insert it into dynamic SQL
```

The unknown value itself may contain the chosen dollar delimiter.

For dynamic values, use:

- `USING` — preferred for data;
- `%I` / `quote_ident` — for identifiers;
- `%L` / `quote_nullable` — if the value really is inserted as text.

Dollar quotes solve the quoting problem for **fixed source PL/pgSQL code**, not the SQL-injection problem.

### 2.7. `FOUND` and `EXECUTE`

Ordinary dynamic `EXECUTE` **does not change `FOUND`**.

This is an important difference from several static operations.

```sql
EXECUTE 'UPDATE jobs SET status = $1 WHERE id = $2'
USING 'done', p_id;
```

After this command, you cannot assume that `FOUND` automatically reflects the result of the `UPDATE`.

For the number of rows, use diagnostics:

```sql
GET DIAGNOSTICS v_rows = ROW_COUNT;
```

Dynamic `EXECUTE` updates the information available through `GET DIAGNOSTICS`, but not the special `FOUND` variable.

### 2.8. `GET DIAGNOSTICS`: current state

Example:

```sql
UPDATE jobs
SET status = 'done'
WHERE worker_id = p_worker_id;

GET DIAGNOSTICS v_rows = ROW_COUNT;
```

Useful current diagnostic items include:

```text
ROW_COUNT       → number of rows processed by the last SQL command
PG_CONTEXT      → current call stack
PG_ROUTINE_OID  → OID of the current function
```

`PG_CONTEXT` shows the place where `GET DIAGNOSTICS` itself is executing and the calling functions higher in the stack.

### 2.9. `GET STACKED DIAGNOSTICS`: state of a caught error

Inside an `EXCEPTION` handler, a different command is needed:

```sql
GET STACKED DIAGNOSTICS
    v_message = MESSAGE_TEXT,
    v_detail  = PG_EXCEPTION_DETAIL,
    v_hint    = PG_EXCEPTION_HINT;
```

It retrieves data about **the error that led to the current handler**, not the current normal execution state.

Available items include:

```text
RETURNED_SQLSTATE
MESSAGE_TEXT
PG_EXCEPTION_DETAIL
PG_EXCEPTION_HINT
PG_EXCEPTION_CONTEXT
TABLE_NAME
SCHEMA_NAME
COLUMN_NAME
CONSTRAINT_NAME
PG_DATATYPE_NAME
```

The handler also has these special variables:

```text
SQLSTATE → code of the caught error
SQLERRM  → error message text
```

The difference:

```text
GET DIAGNOSTICS          → current execution state
GET STACKED DIAGNOSTICS  → information about the caught error
```

### 2.10. `EXCEPTION` semantics: data is rolled back, variables are not

A block with an error handler has special semantics:

```sql
BEGIN
    statements
EXCEPTION
    WHEN condition THEN
        handler
END;
```

If an error occurs in the executable part of the block:

1. the remaining statements in that part are not executed;
2. changes to persistent database state made inside the block are rolled back;
3. local PL/pgSQL variables retain the values they had acquired by the time of the error;
4. the first matching `WHEN` is selected;
5. after the handler finishes, execution continues after the block's `END`.

Example:

```sql
x := 1;

BEGIN
    UPDATE t SET value = 100 WHERE id = 1;
    x := 2;
    y := 10 / 0;
EXCEPTION
    WHEN division_by_zero THEN
        RAISE NOTICE 'x = %', x;
END;
```

Inside the handler, `x = 2`, but the table change made inside the block has been undone.

This is not a `COMMIT` of the inner work — quite the opposite: the database returns to the state it had when the failing block was entered.

### 2.11. The cost of `EXCEPTION`

Entering and leaving a block with `EXCEPTION` is more expensive than using an ordinary block. Therefore, do not mechanically wrap every fragment of a function in a handler "just in case."

Use `EXCEPTION` when you genuinely intend to handle an expected error category or add useful context.

### 2.12. Trigger function: contract with the trigger manager

A data-change trigger function is declared as:

```sql
CREATE FUNCTION audit_row()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    ...
END;
$$;
```

It has no ordinary function arguments. Arguments from `CREATE TRIGGER` are available through `TG_ARGV`.

PostgreSQL automatically creates special context variables.

The main ones are:

```text
NEW             new row for an INSERT/UPDATE row-level trigger
OLD             old row for an UPDATE/DELETE row-level trigger
TG_NAME         trigger name
TG_WHEN         BEFORE / AFTER / INSTEAD OF
TG_LEVEL        ROW / STATEMENT
TG_OP           INSERT / UPDATE / DELETE / TRUNCATE
TG_TABLE_NAME   table name
TG_TABLE_SCHEMA table schema
TG_NARGS        number of trigger arguments
TG_ARGV         array of trigger arguments, indexed from 0
```

`NEW` and `OLD` exist as `record` variables, but their contents depend on the operation:

```text
INSERT → NEW exists, OLD = NULL
UPDATE → both NEW and OLD exist
DELETE → OLD exists, NEW = NULL
```

For a statement-level trigger there is no individual mutable row value in `NEW`/`OLD`.

### 2.13. Return value of a BEFORE row trigger

For a row-level `BEFORE` trigger, the function result affects the subsequent operation.

If it returns `NULL`:

```sql
RETURN NULL;
```

processing of that row stops: the row operation itself and subsequent row triggers for that row do not continue.

If it returns a row:

```sql
RETURN NEW;
```

the operation continues with the returned row.

This allows values to be changed before they are written:

```sql
NEW.updated_at := now();
RETURN NEW;
```

For `BEFORE DELETE`, `OLD` is normally returned if the deletion should continue.

### 2.14. AFTER and statement-level triggers

For a row-level `AFTER` trigger, the return value is ignored. It is likewise ignored for statement-level `BEFORE`/`AFTER` triggers.

Therefore code often uses:

```sql
RETURN NULL;
```

not to cancel the action, but because the trigger manager no longer uses the result.

Do not confuse this with `BEFORE ROW`, where `RETURN NULL` specifically means "skip this row."

### 2.15. INSTEAD OF trigger

`INSTEAD OF` triggers apply to views and are row-level triggers.

Their contract is different:

- `NULL` means that the trigger did not perform the change for the row;
- a non-NULL result reports that the requested change was handled;
- for `INSERT/UPDATE`, `NEW` is normally returned;
- for `DELETE`, `OLD` is returned.

### 2.16. Transaction control: not every function can execute `COMMIT`

An ordinary PL/pgSQL function called through `SELECT` is not a place where `COMMIT` or `ROLLBACK` can be executed arbitrarily.

Transaction control is allowed in the appropriate context:

- a procedure invoked by `CALL`;
- an anonymous `DO` block;
- nested `CALL`/`DO` invocations when there is no intervening command that breaks the permitted call chain.

Procedure example:

```sql
CREATE PROCEDURE process_batch()
LANGUAGE plpgsql
AS $$
BEGIN
    ...
    COMMIT;
    ...
END;
$$;
```

After `COMMIT` or `ROLLBACK`, a new transaction starts automatically — a separate `START TRANSACTION` is not needed inside the procedure.

### 2.17. `COMMIT AND CHAIN` / `ROLLBACK AND CHAIN`

Ordinary `COMMIT` ends the current transaction, after which a new one starts with the default transaction characteristics.

If the next transaction should continue with the characteristics of the previous one, use:

```sql
COMMIT AND CHAIN;
ROLLBACK AND CHAIN;
```

`AND CHAIN` starts the next transaction with the previous transaction's characteristics, for example the same isolation level.

### 2.18. Transaction-control restrictions

There are important restrictions.

Transaction control is allowed only in the appropriate call stack. For example:

```text
CALL p1() → CALL p2() → CALL p3()
```

can preserve a valid context.

But an intervening call through an SQL function changes the situation:

```text
CALL p1() → SELECT f2() → CALL p3()
```

and the final procedure no longer has the same freedom to control transactions.

In addition, a transaction cannot be ended inside a block that has an `EXCEPTION` handler.

### 2.19. Cursor loop and `COMMIT`

Cursors are normally closed when a transaction ends. However, a cursor created by a `FOR ... IN SELECT` loop can be converted to a holdable cursor on the first `COMMIT`/`ROLLBACK` inside such a procedural loop: the result is fully evaluated so that the loop can continue across the transaction boundary.

This matters for large data sets: memory behavior and the moment when the result is actually evaluated change.

Transaction control is not allowed in a cursor loop driven by a query that itself modifies data, such as `UPDATE ... RETURNING`.

### 2.20. Conflicts between variable names and column names

Consider:

```sql
DECLARE
    id integer;
BEGIN
    SELECT id
    INTO id
    FROM users
    WHERE id = id;
END;
```

It is already difficult for a human reader to tell which `id` is a column and which is a variable. PL/pgSQL also has to resolve the ambiguity.

By default, if a reference can syntactically mean either a PL/pgSQL variable or a table column, PL/pgSQL raises an error.

This is a good reason not to rely on an implicit choice.

### 2.21. How to remove ambiguity

#### Method 1: naming convention

For example:

```text
p_*  → parameters
v_*  → local variables
```

```sql
WHERE users.id = p_user_id
```

#### Method 2: qualify the column

```sql
SELECT u.id
FROM users AS u
WHERE u.id = p_user_id;
```

#### Method 3: qualify the variable

A block variable:

```sql
<<fn>>
DECLARE
    id integer;
BEGIN
    SELECT u.id
    FROM users AS u
    WHERE u.id = fn.id;
END;
```

A function parameter can be qualified with the function name:

```sql
WHERE users.id = get_user.id
```

if `id` is a parameter of the `get_user` function.

### 2.22. `plpgsql.variable_conflict`

The behavior can be changed with this setting:

```text
error        → ambiguity = error (default)
use_variable → prefer the PL/pgSQL variable
use_column   → prefer the column
```

For an individual function, a compiler directive can be used:

```sql
#variable_conflict error
```

or the corresponding `use_variable` / `use_column` values.

However, more reliable code normally removes ambiguity explicitly instead of making meaning depend on a setting.

### 2.23. How PL/pgSQL caches static SQL

When a particular SQL statement or expression is executed for the first time, PL/pgSQL prepares it through the server-side SPI mechanism. On later executions, the prepared statement can be reused.

The planner can use current parameter values to create a custom plan. After repeated execution, PostgreSQL may also decide that it is more beneficial to reuse a generic plan that does not depend on particular values.

Simplified model:

```text
static SQL
    ↓
prepared statement
    ↓
custom plan for current values
    or
generic plan for reuse
```

You do not need to choose this mode manually for every ordinary PL/pgSQL query: PostgreSQL makes the decision itself as part of the prepared-statement mechanism.

### 2.24. Why object names must remain constant for cacheable SQL

For static SQL to be prepared and reused, its tables and columns must be determined by the command text.

Therefore this is natural:

```sql
SELECT * FROM users WHERE id = p_id;
```

while this meaning requires dynamic SQL:

```text
SELECT * FROM <table_chosen_at_runtime>
```

Dynamic `EXECUTE` buys flexibility at the cost of parsing and planning on every execution.

### 2.25. When a fresh plan through `EXECUTE` can be useful

Suppose the data distribution is highly uneven:

```text
tenant A → 10 rows
tenant B → 100,000,000 rows
```

The optimal plan for the same parameterized query can depend heavily on the tenant.

In such cases, explicit `EXECUTE ... USING` can be a tool that guarantees replanning with the current parameters.

This does not mean "dynamic EXECUTE is always faster." On the contrary, it adds planning cost. Using it for replanning makes sense only when there is a real reason.

### 2.26. Trigger functions and the plan cache

If one trigger function is used for several tables, PL/pgSQL prepares and caches statements separately for each trigger-function + table combination.

This matters because columns with the same name in different tables can have different types.

Polymorphic functions similarly have separate caches for different combinations of actual argument types.

### 2.27. Time-sensitive values and caching

Do not write a time-sensitive value as a text literal in a place where the analyzer can turn it into a constant while preparing SQL.

Prefer:

```sql
now()
current_timestamp
```

instead of trying to use the text `'now'` as a dynamically changing value.

### 2.28. Additional PL/pgSQL checks

Useful settings for development are:

```text
plpgsql.extra_warnings
plpgsql.extra_errors
```

They can be set to `all` or to individual checks.

Examples:

```text
shadowed_variables       → a new variable hides an existing variable
strict_multi_assignment  → source and target field counts do not match
too_many_rows            → INTO receives more than one row
```

In test/development environments, these checks help detect mistakes earlier.

### 2.29. `ASSERT` and `RAISE`: different purposes

`ASSERT`:

```sql
ASSERT total >= 0, 'total must not be negative';
```

checks a developer assumption and is intended to find programming defects.

`RAISE EXCEPTION`:

```sql
RAISE EXCEPTION 'user % does not exist', p_id;
```

is used for an ordinary application/domain error or to stop execution explicitly.

Do not turn `ASSERT` into the main mechanism for validating user data.

---

## 3. Practical patterns

### Safe dynamic SQL

```sql
EXECUTE format(
    'UPDATE %I SET %I = $1 WHERE id = $2',
    table_name,
    column_name
)
USING new_value, row_id;
```

Model:

```text
SQL syntax / identifiers → format(%I)
data values              → USING
```

### Error diagnostics

```sql
BEGIN
    ...
EXCEPTION
    WHEN OTHERS THEN
        GET STACKED DIAGNOSTICS
            v_state   = RETURNED_SQLSTATE,
            v_message = MESSAGE_TEXT,
            v_detail  = PG_EXCEPTION_DETAIL,
            v_context = PG_EXCEPTION_CONTEXT;

        RAISE WARNING 'state=%, message=%', v_state, v_message;
        RAISE;
END;
```

### BEFORE trigger that modifies a row

```sql
CREATE FUNCTION set_updated_at()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$;
```

### BEFORE trigger that skips a row

```sql
IF NEW.disabled THEN
    RETURN NULL;
END IF;

RETURN NEW;
```

### Explicit names instead of conflicts

```sql
CREATE FUNCTION get_user(p_id bigint)
RETURNS users
LANGUAGE plpgsql
AS $$
DECLARE
    v_user users%ROWTYPE;
BEGIN
    SELECT u.*
    INTO STRICT v_user
    FROM users AS u
    WHERE u.id = p_id;

    RETURN v_user;
END;
$$;
```

The role of each identifier is immediately clear from the names.

---

# What to remember

- In static optimizable SQL, PL/pgSQL can substitute variables as query parameters and reuse a prepared plan.
- Dynamic `EXECUTE` builds a new plan on every execution.
- `$1`, `$2`, ... in `EXECUTE ... USING` are for **data values**, not table/column names.
- Identifiers are safely inserted through `format('%I', ...)` or `quote_ident`.
- `%L` is equivalent to `quote_nullable`; for data values, prefer `USING` when possible.
- Dollar quoting is useful for a fixed function body, but it does not make an unknown dynamic value safe.
- Ordinary `EXECUTE` does not change `FOUND`; obtain the row count through `GET DIAGNOSTICS ... ROW_COUNT`.
- `GET DIAGNOSTICS` describes current execution; `GET STACKED DIAGNOSTICS` describes a caught error.
- In `EXCEPTION`, database changes in the failing part of the block are rolled back, while PL/pgSQL variables retain the values reached by the time of the error.
- Blocks with `EXCEPTION` have additional cost, so they should not be used without a reason.
- A trigger function receives `NEW`, `OLD`, `TG_OP`, `TG_WHEN`, `TG_LEVEL`, and other special variables.
- A `BEFORE ROW` trigger can return `NULL` to skip the operation for a row; returning `NEW`/`OLD` continues processing according to the relevant semantics.
- For an `AFTER` row trigger and statement-level triggers, the return value is ignored.
- `COMMIT`/`ROLLBACK` are allowed in appropriate `CALL` and `DO` procedure contexts, not in an ordinary function invoked through `SELECT`.
- After `COMMIT`/`ROLLBACK` in a procedure, a new transaction starts automatically; `AND CHAIN` preserves the previous transaction's characteristics.
- By default, ambiguity between "variable or column" raises an error. The best solution is qualification and clear naming conventions.
- Static SQL can use a custom or generic plan; `EXECUTE` is useful when a genuinely fresh plan is needed for current parameter values.
