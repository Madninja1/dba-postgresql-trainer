# 2. The psql Terminal Client

## 1. Topic

`psql` is PostgreSQL's primary terminal client. A DBA should know not only how to execute SQL, but also how to manage connections, run scripts, use meta-commands, and configure connection parameters safely.

Core model:

```text
shell / script
     ↓
   psql
  ↙    ↘
meta-commands   SQL
(client)        (server)
```

---

# 2. Mechanism

## Connections and startup options

In psql, -d specifies the database, -U the user, -h the host, and -p the server port. If no host is specified, libpq normally connects through a local Unix-domain socket; PostgreSQL's standard port is 5432. If no user is specified, psql uses the operating-system user name by default. After the user has been determined, that name is also used as the default database name. The \conninfo command displays information about the current connection. The \connect command establishes a new connection.

PGHOST corresponds to host, PGDATABASE to dbname, PGPORT to port, and PGUSER to user. These variables can supply default connection parameters. A password-file line has five fields in this order: host name, port, database, user name, and password. In the practical part of the material, \l+ is suggested for displaying a detailed list of databases.

The value interactive enables the described behavior only for interactive psql sessions. It does not apply when commands are being executed from scripts.

## SQL and meta-commands

Psql is PostgreSQL's terminal client. It can execute queries interactively, read commands from files or command-line arguments, and provides its own meta-commands. The `--help` option displays help for startup options; inside psql, \? shows help for meta-commands, \? variables shows help for psql variables, and \h or \help shows help for SQL commands. Psql can be terminated with the \q meta-command; the material also shows `exit` and `quit` as ways to leave psql. Psql's own commands begin with a backslash. They are processed by psql itself rather than sent to the server as SQL.

In normal mode, psql sends an SQL command to the server after the terminating semicolon. A newline does not necessarily terminate a statement, so SQL can conveniently be entered on multiple lines. Backslash meta-commands are processed by psql itself. PROMPT1 controls the primary psql prompt. PROMPT2 is displayed for continuation input, for example when a command has not yet been terminated with a semicolon or quotes remain open. The \timing on meta-command enables display of the execution time of each SQL statement. Calling \timing without an argument toggles the current state.

With ON_ERROR_ROLLBACK=on, psql establishes an implicit savepoint before every command in a transaction block. If a command fails, psql rolls back to that savepoint, allowing the transaction to continue.

## Scripts and non-interactive execution

The -c, or --command, option passes a command to psql for execution. When -c or -f is used, psql exits after processing the specified commands instead of reading commands from standard input. The -f option reads commands from a file, can be repeated and combined with -c, and is essentially similar to the \i meta-command. When -c or -f is present, psql exits after processing those options. The --single-transaction option wraps commands supplied through -c and -f in one transaction. With ON_ERROR_STOP enabled, an error causes a ROLLBACK instead of the final COMMIT.

## Practical reference

```bash
psql -h db.example.net -p 5432 -U app -d appdb

\conninfo
\?
\h SELECT
\dt
\d table_name
\q

psql -d appdb -f migration.sql
psql -d appdb -c "SELECT now();"
```

---

# 3. Important Details

## Passwords and connection parameters

Using PGPASSWORD is not recommended for security reasons because on some systems process environment variables may be visible to other users. The documentation recommends using a password file instead. The first four fields in .pgpass can contain the * wildcard, and lookup stops at the first matching entry. Therefore, when wildcards are used, more specific entries should be placed first. Colons and backslashes are escaped. On Unix, the password file must deny access to group and other users. The documentation gives mode 0600 as an appropriate setting. With less restrictive permissions, the file is ignored.

The connection service file `pg_service.conf` maps a service name to connection parameters. The default per-user file is named `~/.pg_service.conf`; a service can be selected through `PGSERVICE`, and the file uses INI format. One purpose of this mechanism is to allow connection parameters to be changed without recompiling the application.

## Configuring psql

A parameter from a service file overrides the corresponding environment variable, but can itself be overridden by a value specified directly in the connection string. Psql supports the PSQL_PAGER and PAGER variables and the \pset pager setting. PSQL_PAGER has priority over PAGER. The pager can be disabled. In the practical part, .psqlrc is used for persistent user-level psql settings, including through \setenv. The -X, or --no-psqlrc, option prevents reading the system and user psql startup files.

---

# What to Remember

```text
• `-d/-U/-h/-p` specify database/user/host/port
• meta-commands begin with `\` and are executed by psql itself
• `-c` and `-f` are useful for automation and cause psql to exit after processing
• `PGPASSWORD` is undesirable; prefer `.pgpass` with correct permissions
• `\conninfo`, `\?`, `\h`, and `\connect` are a core DBA toolkit
```

Before the test, it is useful not just to recognize command names, but to be able to explain **what problem the mechanism solves, at what level it operates, and what constraints it has**.
