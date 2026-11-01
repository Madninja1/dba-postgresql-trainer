# 17. Logical Backup

## 1. Topic

A logical backup stores not the files in `PGDATA`, but a logical description of objects and data. In the simplest case, it is a set of SQL commands that can be executed on another server to recreate the required state.

The main model is:

```text
database objects/data
       ↓
COPY / pg_dump / pg_dumpall
       ↓
SQL script or archive
       ↓
psql / pg_restore
       ↓
restored objects
```

The logical approach is flexible: you can copy individual objects or databases, move data to a different hardware architecture, and usually restore a dump on a newer major PostgreSQL version. The cost of this flexibility is comparatively lower speed on large data sets and the absence of arbitrary point-in-time recovery: a dump reflects the state captured when it was created.

---

# 2. Mechanism

## `COPY` and `\copy`

`COPY` is designed for fast exchange of table contents with a file, standard input/output stream, or program. It can:

- export a table;
- export a query result;
- load rows into an existing table;
- work with text, CSV, and binary formats.

When loading with `COPY FROM`, existing rows are not removed — new rows are added to the table.

The key difference between the two variants is:

```text
SQL COPY       → the PostgreSQL server opens the file
psql \copy     → the psql client opens the file
```

Therefore, with server-side `COPY`, the file must be accessible to the operating-system user running PostgreSQL; with `\copy`, it must be accessible to the user on the client machine who started `psql`.

## `pg_dump`

`pg_dump` is the primary tool for logical backup of a single database. It works as a regular PostgreSQL client and creates a consistent snapshot of the database as of the start of the dump without blocking normal data activity.

The main output forms are:

- a plain SQL script;
- an archive in custom or directory format;
- a selective dump of particular schemas, tables, DDL only, or data only.

A SQL dump is restored with `psql`, while archive formats are restored with `pg_restore`. A format with a table of contents is convenient because the set of objects can be selected during restore, and suitable formats support parallel work.

## Restoring a Dump

For a full restore of a single-database dump, the target database is usually created from `template0` so that local changes in `template1` are not duplicated with objects from the dump.

Before restoring, cluster-level dependencies that a single-database `pg_dump` does not save must already exist, primarily:

- owner roles and roles to which privileges were granted;
- required tablespaces.

A text dump is conveniently executed with `psql -X`, which prevents the user's `.psqlrc` from affecting the restore. For controlled restoration, it is useful to stop on the first error (`ON_ERROR_STOP`) or execute the entire dump in one transaction if that mode fits the task.

After a large restore, running `ANALYZE` is useful so that the optimizer gets up-to-date statistics.

## `pg_dumpall`

`pg_dumpall` is intended for a logical copy of an entire cluster. It invokes `pg_dump` for the cluster databases in sequence and additionally saves cluster-wide global objects:

- roles;
- tablespaces;
- privileges on configuration parameters.

The output of `pg_dumpall` is always a SQL script, so parallel restore of an archive format is not available here. For large clusters, it is often more convenient to split the task:

```bash
pg_dumpall --globals-only > globals.sql
pg_dump -Fc -f appdb.dump appdb
```

`pg_dumpall` must be able to access all objects being saved; for a complete backup it is usually run with superuser privileges. Restoring global objects also requires such privileges.

---

# 3. Practical Pattern

```bash
# SQL dump of one database
pg_dump appdb > appdb.sql
createdb -T template0 appdb_restore
psql -X --set ON_ERROR_STOP=on -d appdb_restore -f appdb.sql

# custom archive
pg_dump -Fc -f appdb.dump appdb
pg_restore -d appdb_restore -j 4 appdb.dump

# only cluster-wide global objects
pg_dumpall --globals-only > globals.sql
```

---

# What to Remember

```text
• a logical copy is portable and can be selective, but is usually slower than a physical backup
• `COPY` opens the file on the server, while `\copy` opens it on the psql client side
• `pg_dump` backs up one database and creates a consistent snapshot without stopping normal work
• SQL dumps are restored with psql; archive formats are restored with pg_restore
• for a full restore, the target database is usually created from template0
• roles and tablespaces are global objects; back them up separately or with pg_dumpall
• pg_dumpall outputs an SQL script and does not provide parallel archive restore
```

Before the test, it is important to understand not only the commands, but also the **backup level, portability, limitations, and restore requirements**.
