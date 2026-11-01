# Middle — confident psql

## 1. Topic

At the Initial level, psql was an interactive client: connect, execute SQL, inspect objects. At the Middle level, it is important to understand **the state of the client itself** — the query buffer, variables, output formatting, file input/output, and conditional logic in scripts.

Level model:

```text
input / file / variables
          ↓
       psql client
          ↓
     query buffer
          ↓
        \g / ;
          ↓
 PostgreSQL server
          ↓
result → format / file / pipe / variables
```

This is no longer just a “set of convenient commands”. psql becomes a small scripting environment around SQL.

---

# 2. Mechanism

## Query buffer: the query before it is sent to the server

Until SQL is sent to the server, psql keeps its text in the **query buffer**.

For example, after entering:

```sql
SELECT
    pid,
    usename
FROM pg_stat_activity
```

without a terminating `;`, the text remains in the buffer.

Main operations:

```text
\p                 show buffer
\r                 clear buffer
\g                 execute buffer
\gx                execute with expanded output
\w file.sql        write buffer to a file
\w |command        send buffer to a program
```

### `\p` — inspect

```text
\p
```

Prints the contents of the current query buffer. If the buffer is empty, psql shows the last executed query.

### `\r` — reset

```text
\r
```

Clears the query buffer. This is useful when the query you entered is wrong and it is easier to discard it.

### `\g` — execute

```text
\g
```

Sends the current query buffer to the server. Without arguments, it is conceptually close to a terminating `;`.

If the buffer is empty, `\g` re-executes the previous query.

`\g` can redirect the result for one execution:

```text
\g result.txt
\g |some_command
```

Unlike `\o`, this redirection applies only to the current execution.

`\g` can also temporarily change formatting parameters for a single query:

```text
\g (format=aligned tuples_only=off expanded=on)
```

### `\gx` — `\g` + expanded output

```text
\gx
```

Works like `\g`, but forcibly enables expanded output for the current execution.

In other words:

```text
\g     execute as configured
\gx    execute and display the result vertically
```

### `\w` — save the query text

```text
\w query.sql
```

Writes **the query buffer itself**, not the query result.

It can also be sent to a program:

```text
\w |command
```

This differs from `\o`, which redirects **query results**.

---

## Expanded output — `\x`

In normal mode, a result row is displayed horizontally:

```text
id | name | created_at | ...
```

This is inconvenient for wide results. The command:

```text
\x
```

toggles expanded mode, where one row is displayed vertically as a set of `field | value` pairs.

You can set the state explicitly:

```text
\x on
\x off
\x auto
```

`\x` is equivalent to the `\pset expanded` setting.

When starting psql, the same mode can be enabled with:

```bash
psql -x
```

---

## Execution time — `\timing`

```text
\timing on
```

enables display of the execution time of each SQL statement.

```text
\timing off
```

disables it.

Without an argument, the state is toggled:

```text
\timing
```

`\timing` does not change the timezone and does not run `EXPLAIN`: it is specifically a client-side display of command execution time.

---

## Output formats and CLI equivalents

Output parameters are configured through:

```text
\pset parameter value
```

Several frequently used modes have short commands and CLI options.

### Unaligned — `-A`

```bash
psql -A
```

switches output to `unaligned` mode. The same can be done with:

```text
\pset format unaligned
```

In this mode, values are separated by the active field separator, which makes the output easier for other programs to process.

### CSV — `--csv`

```bash
psql --csv
```

enables CSV format. Equivalent command:

```text
\pset format csv
```

### Expanded — `-x`

```bash
psql -x
```

is equivalent to enabling `\x` / `\pset expanded`.

### Tuples only — `-t`

```bash
psql -t
```

removes column headings and the final row-count footer. Inside psql:

```text
\t
```

or:

```text
\pset tuples_only
```

Remember the distinction:

```text
-A       unaligned format
--csv    CSV format
-x       expanded output
-t       data only, without presentation metadata
-U       user; unrelated to formatting
```

---

## Redirecting results — `\o`

```text
\o result.txt
```

sends the results of subsequent queries to a file.

Restore normal output with:

```text
\o
```

Output can also be redirected to a program:

```text
\o |command
```

The important distinction is:

```text
\w file     saves query-buffer text
\o file     redirects query results
\g file     sends the result of one execution to a file
```

---

## Executing files — `\i`

```text
\i script.sql
```

or:

```text
\include script.sql
```

reads commands from a file and processes them as if they had been entered into psql.

For scripts that include other files, there is also `\ir` / `\include_relative`. In interactive mode it behaves similarly to `\i`, but when called from a file, a relative path is resolved relative to the directory of the including script itself.

The distinction between the two levels is:

```text
psql -f file.sql    choose a file when starting psql
\i file.sql         include a file from a running psql session/script
```

---

## `\copy` and server-side `COPY`

This is one of the most important practical distinctions in psql.

### Server-side `COPY`

An SQL command such as:

```sql
COPY users TO '/var/lib/postgresql/users.csv' CSV;
```

works with a file from the perspective of the **PostgreSQL server process**.

Therefore, the path refers to the server's filesystem, and access is controlled by server-side privileges.

### psql `\copy`

```text
\copy users TO 'users.csv' CSV HEADER
```

psql arranges transfer of data between the server and the **client's local filesystem**.

Model:

```text
COPY ... '/server/path'
        ↓
PostgreSQL server opens the file

\copy ... 'local/path'
        ↓
psql client opens the file
        ↓
data is transferred between client and server
```

Consequences of `\copy`:

- the file is read/written by psql;
- the permissions of the local OS user running psql are used;
- server superuser file privileges are not required to access the local file;
- data **does travel between client and server**, so the claim “the data does not go over the network” is false.

This is why `\copy` is often more convenient from a DBA workstation, while server-side `COPY` can be more efficient for large volumes if the server can directly access the required file.

---

## psql variables

A psql variable is a **name/value** pair. Its value can be any string length supported by psql.

Create or change a variable:

```text
\set foo bar
```

Inspect its value:

```text
\echo :foo
```

Remove an ordinary variable:

```text
\unset foo
```

Show all defined variables:

```text
\set
```

If `\set` is called with only a name:

```text
\set foo
```

the variable receives an empty string (some special control variables have additional rules).

Variable names may contain letters, digits, and underscores and are case-sensitive. They do not have to begin with `PG_`.

Do not confuse:

```text
\set NAME value    psql variable
SET name = value   PostgreSQL SQL configuration command
```

---

## Variable substitution — `:name`

After:

```text
\set foo 42
```

the expression:

```sql
SELECT :foo;
```

substitutes the variable's text value.

For an initial understanding, three forms are enough:

```text
:name       substitute value as text
:'name'     quote correctly as an SQL literal
:"name"     quote correctly as an SQL identifier
```

For example:

```text
\set table_name users
SELECT * FROM :"table_name";
```

and:

```text
\set login alice
SELECT * FROM users WHERE name = :'login';
```

Without quoting, `:name` is **literal text substitution**. The safety properties of these forms and checking whether a variable exists are covered in detail at the Strong level.

---

## `\gset`: query result → variables

`\gset` connects SQL results with psql variables.

Example:

```sql
SELECT current_database() AS db,
       current_user AS usr
\gset
```

After that:

```text
\echo :db :usr
```

prints the column values.

Critical rule: the query used with `\gset` must return **exactly one row**.

```text
0 rows       → variables are unchanged
1 row        → columns are stored in variables
>1 rows      → variables are unchanged
```

Each column becomes a separate variable named after the column.

You can add a prefix:

```sql
SELECT current_database() AS db, current_user AS usr
\gset conn_
```

which produces:

```text
:conn_db
:conn_usr
```

If a column value is SQL `NULL`, the corresponding psql variable is **unset**, rather than receiving the text `NULL`.

---

## `\watch`: repeat a query

`\watch` repeats the current query buffer. If the buffer is empty, the last submitted query is used.

For example:

```sql
SELECT now(), count(*) FROM pg_stat_activity;
\watch 5
```

re-runs the query at the specified interval.

If no interval is specified, the default is **2 seconds**.

The modern form can also limit the number of executions:

```text
\watch interval=1 count=10
```

`\watch` is not limited to parameterless `SELECT` statements: it executes the current query as `\g` would, repeating it until `count` is reached, an error occurs, or execution is interrupted.

---

## Conditions in psql scripts

psql has its own conditional blocks:

```text
\if expression
    ...
\elif expression
    ...
\else
    ...
\endif
```

Example together with `\gset`:

```sql
SELECT EXISTS (
    SELECT 1
    FROM pg_roles
    WHERE rolname = 'app'
) AS role_exists
\gset

\if :role_exists
    \echo 'role exists'
\else
    \echo 'role does not exist'
\endif
```

This is **psql client** logic, not SQL `CASE` and not PL/pgSQL `IF`.

Supported block components:

```text
\if
\elif
\else
\endif
```

There is no command such as `\switch` in this mechanism.

---

## Patterns for `\d*` object commands

Commands such as:

```text
\dt
\di
\df
\dn
```

accept name patterns.

### `*` — any number of characters

```text
\dt app*
```

matches tables whose names begin with `app`.

### `?` — one character

```text
\dt log_?
```

`?` matches exactly one character.

### `schema.object`

A dot separates the schema pattern from the object pattern:

```text
\dt public.*
```

### `*.*` — independent of current visibility

Without a pattern, `\d*` commands usually show objects visible through the current `search_path`.

```text
\dt *.*
```

allows searching across all schemas, rather than limiting output to objects currently visible through the search path.

### Case and double quotes

As with SQL identifiers, unquoted parts of a pattern are normally folded to lowercase.

Double quotes preserve case and make special pattern characters inside the quotes ordinary characters. Therefore `*` and `?` inside double quotes are no longer wildcards.

---

## `\d+`: expanded description

The `+` suffix adds details to many object meta-commands.

For example:

```text
\d users
\d+ users
```

Both commands describe the object, but `\d+` displays additional information. Depending on the object, this can include comments, size, privileges, storage properties, or other object-specific fields.

`+` **does not mean** that psql automatically includes every system object. Several commands use a separate `S` modifier or an explicit pattern for system objects.

---

# 3. Important details

## Do not confuse “query text” with “query result”

```text
\p      show buffer text
\r      remove buffer text
\w      save buffer text

\g      execute buffer
\o      change the result output channel
\g file execute once and write the result
```

These are different entities: the query buffer exists before execution, while the result appears only after the server responds.

---

## `\copy` is defined by where the file is opened

The most useful diagnostic question is:

```text
who opens the file?
```

If the PostgreSQL server opens it, this is SQL `COPY` using a server-side file.

If the psql process opens it, this is `\copy`.

---

## `\gset` does not “pick one of the rows”

psql does not automatically take the first or last row. Its contract is strict:

```text
exactly one result row
```

If the SQL can return multiple rows, first rewrite the query so that it is guaranteed to return one.

---

# What to remember

Query buffer:

```text
\p          show
\r          clear
\g          execute
\gx         execute in expanded mode
\w file     save query text
```

Output:

```text
\x          expanded output
\timing     SQL command timing
\o file     results to a file

-A          unaligned
--csv       CSV
-x          expanded
-t          tuples only
```

Files:

```text
\i file.sql           execute a psql script
COPY ... server path  server opens the file
\copy ... local path  psql client opens the file
```

Variables:

```text
\set name value   set
\unset name       remove
\set              show all
:name             ordinary substitution
:'name'           SQL literal
:"name"            SQL identifier
```

SQL → variables:

```text
query
  ↓
exactly 1 row
  ↓
\gset [prefix]
  ↓
each column → separate psql variable
NULL        → variable is unset
```

Automation:

```text
\watch                  repeat query buffer
default interval          2 seconds
count=N                   limit number of executions

\if / \elif / \else / \endif
```

Patterns:

```text
*       any number of characters
?       one character
schema.object
*.*     search independently of search_path visibility
"..."   preserves case and disables wildcards inside quotes
```
