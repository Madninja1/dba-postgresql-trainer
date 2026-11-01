# Initial — psql basics

## 1. Topic

`psql` is the PostgreSQL terminal client. It connects to the server, sends SQL commands to it, and displays results, but it also has its own control language: **psql meta-commands**, which begin with a backslash `\`.

The main distinction at the first level is:

```text
SQL command
    ↓
psql sends it to the PostgreSQL server

\meta-command
    ↓
psql handles it itself
```

Therefore `SELECT`, `INSERT`, and `CREATE TABLE` are SQL, while `\dt`, `\conninfo`, and `\q` are client commands. These are different layers: the PostgreSQL server does not execute `\dt`, and psql does not implement the logic of `SELECT` itself.

---

# 2. Mechanism

## Connecting to PostgreSQL

General invocation form:

```text
psql [options...] [database_name [user_name]]
```

Main connection options:

```text
-d DBNAME    database
-h HOST      server
-p PORT      port
-U USER      PostgreSQL user
```

Example:

```bash
psql -h db.example.org -p 5432 -U app -d appdb
```

Here:

```text
-h db.example.org   where to connect
-p 5432             which port to use
-U app              which PostgreSQL role to use
-d appdb            which database to connect to
```

`-d` can accept not only a database name but also a connection string. For example, psql supports conninfo/URI forms:

```bash
psql "service=myservice sslmode=require"
psql postgresql://dbmaster:5433/mydb?sslmode=require
```

You do not have to specify every parameter. psql uses defaults and libpq connection parameters. Frequently used connection settings can also be supplied through environment variables:

```text
PGDATABASE   database
PGHOST       server
PGPORT       port
PGUSER       user
```

So the command

```bash
psql -h localhost -p 5432 -U dba -d postgres
```

and launching `psql` with `PGHOST`, `PGPORT`, `PGUSER`, and `PGDATABASE` already set solve the same task: they tell the client how to connect.

The password is not one of these four variables. A `.pgpass` file can be used to obtain a password automatically; `-W` forces a password prompt, while `-w` prevents psql from prompting interactively.

---

## What happens after connection

After a successful connection, psql shows a prompt that usually contains the name of the current database:

```text
testdb=>
```

You can now enter SQL:

```sql
SELECT current_database(), current_user;
```

An SQL command can span multiple lines. **A newline by itself normally does not terminate an SQL command.** psql waits for a terminating semicolon:

```sql
SELECT
    current_database(),
    current_user
;
```

Simplified model:

```text
enter SQL
   ↓
text accumulates in psql
   ↓
a terminating ; is encountered
   ↓
psql sends the command to the server
   ↓
PostgreSQL executes it
   ↓
psql displays the result
```

This is why queries can be formatted across multiple lines conveniently.

---

## psql meta-commands

Unquoted input that begins with `\` is treated by psql as one of its own meta-commands.

Examples:

```text
\conninfo
\dt
\d users
\q
```

These commands are useful for database navigation, administration, inspecting objects, configuring the client, and writing psql scripts.

### Current connection — `\conninfo`

```text
\conninfo
```

Displays information about the current connection.

This is the first command worth remembering when in doubt: **where exactly am I connected right now?**

---

## Reconnecting — `\c` / `\connect`

Inside a running psql session, you can establish a new connection:

```text
\c mydb
```

or:

```text
\connect mydb
```

The full positional form lets you specify the database, user, host, and port:

```text
\c mydb myuser host.example.org 5432
```

A connection string is also accepted:

```text
\c "host=localhost port=5432 dbname=mydb connect_timeout=10"
```

Do not confuse these forms:

```text
-d mydb     choose the database when starting psql
\c mydb     change the connection inside an existing psql session
```

---

## Basic object navigation

For everyday work, a small set of commands is enough to get started:

```text
\l     databases
\dn    schemas
\du    roles
\dt    tables
```

There is no placeholder in these commands: each letter has a specific meaning. For example, `t` in `\dt` stands for tables.

### Database list — `\l` / `\list`

```text
\l
```

or:

```text
\list
```

Displays databases on the server, including owners, encodings, and access privileges. `\l+` adds extra information.

### Tables — `\dt`

```text
\dt
```

Displays visible tables.

### Schemas — `\dn`

```text
\dn
```

Displays schemas.

### Roles — `\du`

```text
\du
```

Displays database roles. PostgreSQL unifies the concepts of users and groups into roles, so `\du` effectively works with the list of roles.

A convenient way to remember these commands is:

```text
\l     list databases
\dn    namespaces (schemas)
\du    users/roles
\dt    tables
```

---

## Describing an object — `\d`

`\d` is one of the most useful groups of psql commands.

For example:

```text
\d users
```

For a table, the command shows more than just column names. The description can include:

- columns and their types;
- `NOT NULL`;
- default values;
- related indexes;
- constraints;
- rules;
- triggers.

The `+` form provides more information:

```text
\d+ users
```

For many `\d*` commands, `+` means **expanded output**, although the exact additional fields depend on the object type.

If you run `\d` without a pattern, psql lists visible relations of several types instead of describing one specific table.

---

## Built-in help

psql has two different help systems.

### psql help — `\?`

```text
\?
```

Displays help for psql itself. The command accepts a topic:

```text
\? commands
\? options
\? variables
```

If no topic is specified, `commands` is used by default.

The same categories are available when starting the program:

```bash
psql --help=commands
psql --help=options
psql --help=variables
```

The main supported topics are:

```text
commands
options
variables
```

`indexes` is not a separate `--help` topic.

### SQL help — `\h` / `\help`

```text
\h SELECT
\h CREATE TABLE
```

This command displays the syntax of a PostgreSQL SQL command.

The distinction is:

```text
\?       how to use psql
\h       how an SQL command is written
```

---

## Executing a command with `-c`

A single command can be passed directly from the shell:

```bash
psql -d appdb -c 'SELECT now();'
```

`-c` / `--command`:

- passes a command to psql for execution;
- can be used multiple times;
- can be combined with `-f`;
- causes psql to exit after all `-c`/`-f` options have been processed.

The argument of one `-c` must be either a command string interpreted entirely by the server or **one** backslash meta-command. Therefore, you should not expect a single `-c` to freely mix SQL and psql meta-commands.

Valid:

```bash
psql -c '\x' -c 'SELECT * FROM users;'
```

Two separate `-c` options are used here.

An important detail: an SQL string passed through one `-c` is sent to the server as **one query**. The transactional consequences of this are covered in detail at the Strong level.

---

## Executing a file with `-f`

```bash
psql -d appdb -f script.sql
```

`-f` / `--file` reads commands from a file instead of standard input.

It can be:

```text
repeated
combined with -c
```

For example:

```bash
psql -f prepare.sql -c 'SELECT now();' -f finish.sql
```

Commands are processed in the order given.

`-f script.sql` is similar to:

```bash
psql < script.sql
```

but the two are **not exact equivalents in every detail**. For example, with `-f`, psql can include line numbers in error messages.

Inside an interactive psql session, a similar task is handled by:

```text
\i script.sql
```

The details of `\i` and script execution are covered at the Middle level.

---

## Exiting psql

```text
\q
```

or:

```text
\quit
```

exits psql.

Do not confuse it with:

```text
\r    clear the current query buffer
\p    display the query buffer
\g    execute the query buffer
```

These commands operate on the current query and are covered at the next level.

---

# 3. Important details

## Startup options and meta-commands are not the same thing

The same task sometimes has one form for the shell and another for an already running psql session:

```text
at startup        inside psql
--------------------------------
-d mydb           \c mydb
--help=commands   \? commands
-l                \l
-x                \x
```

The context determines the syntax: `-d` is processed when the program starts, while `\c` is processed inside psql.

---

## `\d` is not SQL

The command:

```text
\d users
```

is not sent to the server as the literal text `\d users`. psql implements the meta-command itself and queries PostgreSQL system catalogs when necessary.

At the advanced level, you can enable `ECHO_HIDDEN` to see the SQL that psql generates for such commands.

---

## Do not confuse database, user, host, and port

```text
-d   database
-h   host
-p   port
-U   user
```

A useful mnemonic:

```text
psql -h WHERE -p WHICH_PORT -U WHO -d WHICH_DATABASE
```

---

# What to remember

The core model:

```text
psql
 ├─ connects to PostgreSQL
 ├─ sends SQL to the server
 ├─ displays results
 └─ handles \meta-commands itself
```

Connection:

```text
-d DBNAME
-h HOST
-p PORT
-U USER
```

Equivalent environment variables:

```text
PGDATABASE
PGHOST
PGPORT
PGUSER
```

Navigation:

```text
\conninfo   current connection
\c          reconnect
\l          databases
\dn         schemas
\du         roles
\dt         tables
\d object   describe an object
\q          exit
```

Help:

```text
\?              psql help
\? commands     meta-commands
\? options      startup options
\? variables    psql variables
\h SELECT       SQL help
```

Non-interactive execution:

```text
-c command      execute a command
-f file.sql     execute a file
```

And two rules that must not be confused:

```text
newline               ≠ end of SQL command
terminating ;          → normally sends SQL to the server

SQL                    → executed by PostgreSQL
\meta-command          → handled by psql
```
