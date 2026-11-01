# Strong — advanced psql

## 1. Topic

At the Strong level, psql is treated as a tool for **reliable automation**: a script must stop correctly on errors, return a meaningful exit code, avoid committing only part of a change set, and substitute values safely.

Core model:

```text
shell / CI
    ↓
psql process
    ├─ startup options
    ├─ psql variables
    ├─ error handling
    ├─ query buffer
    └─ meta-commands
          ↓
      PostgreSQL
          ↓
 SQLSTATE / result / transaction
          ↓
      psql exit code
          ↓
       shell / CI
```

The important part here is not individual commands, but understanding **where the boundary between psql and the server lies** and what happens when an error occurs.

---

# 2. Mechanism

## By default, psql does not stop a script on every SQL error

This is a fundamental automation detail.

If an SQL command in a psql script fails, the client's normal behavior is to continue processing subsequent commands.

That is often convenient interactively. For a migration or deployment script, it can be dangerous.

### `ON_ERROR_STOP`

```text
\set ON_ERROR_STOP on
```

or at startup:

```bash
psql -v ON_ERROR_STOP=1 -f deploy.sql
```

changes the behavior: when an error occurs, psql stops processing commands.

In an interactive session, the client returns to the prompt. In non-interactive mode, psql exits with a dedicated return code.

Model:

```text
ON_ERROR_STOP off (default)
error
  ↓
psql normally continues the script

ON_ERROR_STOP on
error
  ↓
psql stops processing
```

This is a **client-side** psql setting. It does not stop the PostgreSQL server itself.

---

## psql exit codes

For shell, cron, and CI, the important signal is not terminal text but the process exit code.

psql defines:

```text
0   normal completion
1   fatal error in psql itself
2   connection failure in a non-interactive session
3   script error with ON_ERROR_STOP enabled
```

Example shell check:

```bash
psql -v ON_ERROR_STOP=1 -f deploy.sql
status=$?

if [ "$status" -ne 0 ]; then
    echo "psql failed: $status" >&2
    exit "$status"
fi
```

It is especially important to distinguish `1`, `2`, and `3`: they represent different classes of failure.

---

## `-1` / `--single-transaction`

For multiple `-c`/`-f` options, psql can wrap all processing in one transaction:

```bash
psql -1 -v ON_ERROR_STOP=1 -f migration.sql
```

`-1` can be used only together with one or more `-c` and/or `-f` options.

Mechanism:

```text
psql
  ↓
BEGIN
  ↓
first -c/-f
  ↓
next -c/-f
  ↓
...
  ↓
COMMIT
```

If one of the commands fails **and** `ON_ERROR_STOP` is enabled, psql sends `ROLLBACK` instead of the final `COMMIT`.

Thus the common combination:

```bash
psql -1 -v ON_ERROR_STOP=1 -f migration.sql
```

gives the model:

```text
everything succeeds → COMMIT
error                → stop + ROLLBACK
```

### Limitations of `-1`

`-1` is not magic that can be layered over any script.

If the `-c`/`-f` content itself contains explicit:

```sql
BEGIN;
COMMIT;
ROLLBACK;
```

those commands can break the expected `--single-transaction` semantics.

The whole transaction also fails if the script contains a command that cannot run inside a transaction block.

Therefore `-1` works best for scripts that **do not manage transaction boundaries themselves**.

---

## Transaction behavior of a single `-c`

Consider:

```bash
psql -c 'INSERT INTO a VALUES (1); INSERT INTO b VALUES (2);'
```

One SQL string passed through one `-c` is sent to the server as **one query**.

If it does not contain explicit `BEGIN`/`COMMIT` commands that divide the work, the server executes the multiple SQL commands in that string in one transaction.

This differs from:

```bash
psql \
  -c 'INSERT INTO a VALUES (1);' \
  -c 'INSERT INTO b VALUES (2);'
```

Here psql sends two separate queries.

Therefore, a semicolon inside one `-c` argument **does not create a separate connection and does not automatically mean a separate client-side transaction for each command**.

If multiple SQL commands should not be grouped into one query string, use multiple `-c` options or feed the commands separately through standard input.

---

## `AUTOCOMMIT`

`AUTOCOMMIT` is a special psql variable.

By default:

```text
AUTOCOMMIT = on
```

### With `on`

After each successfully executed command, changes are normally committed automatically unless you explicitly started a transaction block yourself:

```sql
BEGIN;
...
COMMIT;
```

### With `off`

```text
\set AUTOCOMMIT off
```

psql does not commit changes until you explicitly execute:

```sql
COMMIT;
```

or:

```sql
END;
```

Before an appropriate command, psql can implicitly issue `BEGIN` if a transaction has not already started.

Simplified model:

```text
AUTOCOMMIT on
SQL succeeds
  ↓
normally committed immediately

AUTOCOMMIT off
SQL
  ↓
implicit BEGIN when needed
  ↓
changes remain uncommitted
  ↓
explicit COMMIT / END
```

If you leave the session with uncommitted changes, they are **lost**, not committed automatically.

After an error with autocommit disabled, the failed transaction must be explicitly rolled back with `ROLLBACK`/`ABORT`.

---

## `ON_ERROR_ROLLBACK`: not the same as `ON_ERROR_STOP`

Another special variable is:

```text
ON_ERROR_ROLLBACK
```

When it is `on`, psql creates an implicit `SAVEPOINT` before each command inside a transaction block and rolls back to it after an error, allowing the transaction to continue.

Comparison:

```text
ON_ERROR_STOP       what psql does with the script after an error
ON_ERROR_ROLLBACK   how psql helps a transaction block continue after an error
```

These are different mechanisms. For migration/deployment work, the `ON_ERROR_STOP + -1` combination is usually more important.

---

## Safe variable interpolation

psql supports four important forms:

```text
:name       literal substitution of the value
:'name'     value as an SQL literal
:"name"     value as an SQL identifier
:{?name}    TRUE/FALSE: whether the variable is defined
```

### Plain `:name`

```text
\set condition 'id > 10'
SELECT * FROM users WHERE :condition;
```

psql copies the value literally.

This is powerful, but dangerous if the value comes from external input: it can contain quotes or other SQL text.

### SQL literal — `:'name'`

For a data value:

```text
\set login 'Alice'
```

use:

```sql
SELECT *
FROM users
WHERE name = :'login';
```

psql formats the value correctly as an SQL literal, including the necessary quoting.

### SQL identifier — `:"name"`

For a table or column name:

```text
\set table_name 'User Data'
```

use:

```sql
SELECT * FROM :"table_name";
```

psql formats the value correctly as an SQL identifier.

### Existence check — `:{?name}`

```text
\if :{?table_name}
    \echo 'table_name is set'
\endif
```

The expression is replaced with `TRUE` or `FALSE` depending on whether the variable exists.

Rule:

```text
data                    → :'name'
table/column names      → :"name"
ready-made SQL fragment → :name, only when you fully control its contents
```

---

## `\gset`: a strict one-row contract

At the Middle level, we used `\gset` as a bridge from SQL to variables. Here all details of that contract matter.

```sql
SELECT current_database() AS db,
       current_user AS usr
\gset session_
```

Requirements and behavior:

- the query must return **exactly one row**;
- each column becomes a separate psql variable;
- an optional prefix is added to every column name;
- if a column value is `NULL`, the corresponding variable is unset;
- if the query fails or does not return exactly one row, variables are left unchanged.

So psql does not choose the “last row” from a multi-row result.

---

## `\gexec`: query result becomes SQL

`\gexec` first executes the current query buffer, then treats every non-null result field as an SQL command and sends it to the server.

Example:

```sql
SELECT format(
    'ANALYZE %I.%I;',
    schemaname,
    tablename
)
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY tablename
\gexec
```

Model:

```text
SELECT generates SQL text
          ↓
        \gexec
          ↓
each non-null field
          ↓
separate SQL query to the server
```

### Order

Generated queries are executed:

```text
row by row from top to bottom
and within each row, column by column from left to right
```

### `NULL`

Fields whose value is `NULL` are ignored.

### Error in one generated command

By default, `\gexec` continues executing the remaining commands after an error.

If:

```text
ON_ERROR_STOP
```

is enabled, processing stops.

### What exactly is executed

Result values are sent to the server literally as SQL. They are **not interpreted as psql meta-commands** and do not undergo another round of psql variable interpolation.

Therefore, when generating SQL, safe formatting is especially important, for example `%I` for SQL identifiers and `%L` for literals in `format()`.

---

## `\bind`: parameters for the next query

`\bind` sets values for `$1`, `$2`, ... for the **next** query execution.

Example:

```text
INSERT INTO audit_log(message, severity)
VALUES ($1, $2)
\bind 'started' 'INFO'
\g
```

The key feature is that psql uses the **extended query protocol** for that execution instead of the normal simple query protocol.

`\bind` affects only the next execution. Later queries return to normal behavior unless `\bind` is issued again.

This does not create a permanent server-side prepared statement and has nothing to do with binding a role to a user.

---

## `\gexec`, `\gset`, and `\bind`: three different data flows

They are easy to confuse, so one diagram is useful:

```text
\gset
SQL result → psql variables

\gexec
SQL result → new SQL → server

\bind
psql values → $1, $2 parameters → next query execution
```

---

## `\errverbose`

After a server error:

```text
\errverbose
```

repeats the **most recent server error message** with maximum detail — as if `VERBOSITY=verbose` and `SHOW_CONTEXT=always` were enabled.

This is useful when the normal message was already displayed in a shorter form and more diagnostic detail is now needed.

The command does not run `EXPLAIN`, does not display the entire history, and does not change client encoding.

---

## Error-state variables

psql also supports special variables:

```text
ERROR               true/false for the last SQL query
SQLSTATE            SQLSTATE of the last query, or 00000 on success
LAST_ERROR_MESSAGE  text of the most recent error
LAST_ERROR_SQLSTATE SQLSTATE of the most recent error
ROW_COUNT           number of rows from the last command when known
```

They are useful in automation together with `\if`.

---

## `ECHO_HIDDEN`: see the SQL behind a meta-command

Many psql commands such as `\d` query the system catalogs internally.

```text
\set ECHO_HIDDEN on
```

makes psql show the underlying SQL query before such a meta-command executes.

This is useful for:

- learning the system catalogs;
- understanding the implementation of `\d*`;
- building your own diagnostic queries.

There is also a value:

```text
ECHO_HIDDEN = noexec
```

in which psql displays those queries but does not send them to the server.

Ordinary `on`, however, **does not disable execution** — it only displays the SQL in addition to executing it.

---

## `FETCH_COUNT`: large results in batches

By default, psql normally receives the complete result set before displaying it.

If you set a positive integer:

```text
\set FETCH_COUNT 500
```

`SELECT` results are fetched from the database and displayed in groups of the specified number of rows.

Meaning:

```text
large SELECT
     ↓
FETCH_COUNT = 500
     ↓
display results in batches of 500 rows
     ↓
limit client memory consumption
```

Important points:

- `FETCH_COUNT` does not add `LIMIT` to your SQL;
- it does not mean the query as a whole will return at most `FETCH_COUNT` rows;
- it is a psql mechanism for processing a result set in batches.

In this mode, a query can fail after some rows have already been displayed.

---

## `\watch`: interval and count

PostgreSQL 16 syntax:

```text
\watch [interval=seconds] [count=number]
```

or the compatible short form with a number of seconds.

For example:

```sql
SELECT now(), count(*) FROM pg_stat_activity;
\watch interval=1 count=20
```

Behavior:

- executes the current query buffer like `\g`;
- if the buffer is empty, repeats the last submitted query;
- the default interval is **2 seconds**;
- `count` limits the number of executions;
- without a limit, repetition continues until interrupted or an error occurs.

---

## Advanced `\d*` patterns

psql patterns are more than just `*` and `?`.

### Schema and object

```text
\dt foo*.*bar*
```

The part before the dot is the schema pattern; the part after it is the object-name pattern.

Therefore, the claim “a dot never refers to a schema” is false: the dot specifically separates these components.

### All schemas

```text
\dt *.*
```

allows objects to be found regardless of their visibility through the current `search_path`.

### Regular-expression elements

Advanced users can use regexp elements such as character classes:

```text
\dt log_[0-9]
```

At the same time, psql applies its own pattern rules: `*` means any number of characters, `?` means one character, and matching applies to the full name.

### Quotes

Inside double quotes, special pattern characters lose their special meaning and match themselves literally.

---

# 3. Important details

## A reliable migration script combines several mechanisms

One setting alone does not solve everything.

A practical combination:

```bash
psql \
  -1 \
  -v ON_ERROR_STOP=1 \
  -f migration.sql
```

provides:

```text
ON_ERROR_STOP   stop on error
-1              one outer transaction
exit code       report the result to the shell
```

But only if the script itself is compatible with a transaction block and does not break the outer transaction management with its own `BEGIN/COMMIT/ROLLBACK`.

---

## Do not confuse a client error with SQLSTATE

```text
psql exit code   process result for the operating system
SQLSTATE         SQL execution state on the PostgreSQL side
```

For example, process exit code `3` means a script error with `ON_ERROR_STOP`, while the specific cause of the server-side error is described by SQLSTATE.

---

## psql interpolation happens before PostgreSQL parses the SQL

psql substitutes variables first, and only then does PostgreSQL receive the completed text.

Therefore:

```text
:name
```

must not be treated as a prepared-statement parameter. It is client-side text interpolation.

If you need `$1`, `$2` parameters for the next execution using the extended protocol, use `\bind`.

---

# What to remember

Errors and automation:

```text
ON_ERROR_STOP off   after an error, psql normally continues
ON_ERROR_STOP on    stop processing

exit 0              normal
exit 1              fatal psql error
exit 2              connection failure in non-interactive session
exit 3              script error with ON_ERROR_STOP
```

One transaction for `-c`/`-f`:

```text
-1 / --single-transaction
        ↓
BEGIN
        ↓
all -c / -f
        ↓
COMMIT

error + ON_ERROR_STOP
        ↓
ROLLBACK
```

But explicit `BEGIN/COMMIT/ROLLBACK` inside the script can break the expected semantics.

Variables:

```text
:name       raw substitution
:'name'     SQL literal
:"name"     SQL identifier
:{?name}    variable exists? → TRUE/FALSE
```

Advanced commands:

```text
\gset        one SQL row → psql variables
\gexec       result fields → SQL commands
\bind        values → $1/$2 of next execution
\errverbose  last server error in detail
```

`\gset`:

```text
exactly 1 row
column → variable
prefix → variable-name prefix
NULL   → unset variable
```

`\gexec`:

```text
NULL                ignored
order               rows top to bottom, columns left to right
error               normally continue
ON_ERROR_STOP       stop
psql meta-commands  are not executed from the result
```

Client state:

```text
AUTOCOMMIT = on       default
AUTOCOMMIT = off      explicit COMMIT/END required
ECHO_HIDDEN = on      show SQL behind \d-like commands
FETCH_COUNT = N       display SELECT results in batches of N rows
```

Monitoring and patterns:

```text
\watch                repeat query buffer
default interval       2 seconds
count=N                limit number of executions

schema.object          schema + object
*.*                    objects regardless of search_path visibility
[0-9]                  example regexp character class in a pattern
```

And one separate rule for `-c`:

```text
one -c with multiple SQL commands
        ↓
one query sent to the server
        ↓
normally one transaction,
unless explicit BEGIN/COMMIT inside divide it
```
