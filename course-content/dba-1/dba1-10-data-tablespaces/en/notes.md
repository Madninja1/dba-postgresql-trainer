# 10. Tablespaces

## 1. Topic

Tablespaces let PostgreSQL map logical object placement to filesystem directories. They are useful for distributing data across storage devices, but they are not a backup mechanism or an automatic balancing mechanism.

Main model:

```text
tablespace name
      ↓ pg_tablespace
filesystem directory
      ↓
pg_tblspc/<oid> → symlink
      ↓
relation files
```

---

# 2. Mechanism

## Creation and placement

A tablespace defines the filesystem location of data and allows different objects to be placed on different storage devices. Tablespaces make it possible to choose physical storage for objects and therefore manage available disk space and I/O characteristics. Tablespaces are cluster-level objects. One tablespace can be used by multiple databases, and one database can place its objects in multiple tablespaces.

A user-defined tablespace allows data to be placed outside the main PGDATA directory. The path specified in `CREATE TABLESPACE ... LOCATION` must be absolute; the directory must already exist, be empty, and belong to the operating-system user running PostgreSQL. For user-defined tablespaces, PostgreSQL creates symbolic links in `PGDATA/pg_tblspc/<tsoid>` that point to the actual storage directories; inside the target directory PostgreSQL uses a version-specific subdirectory and then a subdirectory named with the OID of the particular database. In psql, `\db` lists tablespaces. The same information can be obtained from the `pg_tablespace` system catalog.

The documentation explicitly allows tablespaces to be selected for tables, indexes, and entire databases. The additional practice notes that the smaller the cost ratio of random reads to sequential reads, the more often the planner may prefer index access to a sequential scan. The full lifecycle of a user-defined tablespace includes creation, template inheritance by a new database, inspection of the physical link in PGDATA, and later removal.

## Standard tablespaces

Cluster initialization creates the standard tablespaces `pg_default` and `pg_global`. `pg_global` contains system objects that belong to the whole cluster rather than to a particular database. `pg_default` contains normal database objects and its local system catalogs; the `template0` and `template1` databases initially use `pg_default` as well. Every database has a default tablespace. Initially this is usually `pg_default`, but a different tablespace can be assigned to a database.

If TABLESPACE is not specified, a new database inherits the tablespace of the template database being copied. Therefore the result in a normal configuration is often `pg_default`. The standard tablespaces are located directly inside PGDATA: `pg_global` uses `global`, while `pg_default` uses `base`. User-defined tablespaces use links in `pg_tblspc`.

## Assigning tablespaces to objects

A nonempty `default_tablespace` value is used as the implicit TABLESPACE when creating tables and indexes if no tablespace is specified directly by the command. `DROP TABLESPACE` applies to an empty tablespace. A tablespace cannot be dropped while objects still use it. PostgreSQL 16 syntax for `ALTER TABLESPACE` includes RENAME, OWNER, SET, and RESET. The statement does not provide a LOCATION clause for moving an existing tablespace.

Per-tablespace planner and I/O parameters are configured with `ALTER TABLESPACE ... SET (...)`. This is exactly the method proposed in the additional practice. PostgreSQL 16 documentation for `ALTER TABLESPACE` lists `seq_page_cost`, `random_page_cost`, `effective_io_concurrency`, and `maintenance_io_concurrency`.

## Practical orientation

The course exercise connects the logical and physical levels: create a user-defined tablespace, make it the default tablespace for `template1`, create a new database and verify inheritance, inspect the symbolic link in `PGDATA/pg_tblspc`, then restore the settings and remove the created objects.

```bash
CREATE TABLESPACE fastspace LOCATION '/mnt/nvme/pg_ts';

CREATE TABLE events(
    id bigint,
    payload jsonb
) TABLESPACE fastspace;

CREATE INDEX events_id_idx ON events(id) TABLESPACE fastspace;

ALTER TABLE events SET TABLESPACE pg_default;
```

---

# 3. Important details

## Temporary objects

`temp_tablespaces` defines tablespaces used for temporary tables and their indexes, as well as temporary files created, for example, by sorts. The parameter can contain a list of several tablespaces, allowing the corresponding workload to be distributed.

## Limitations and portability

A tablespace remains part of a particular cluster: it cannot be treated as a self-contained set of files that can be attached to another cluster or backed up independently of `PGDATA`. Before `DROP TABLESPACE`, objects from all databases must be removed from it. `ALTER TABLESPACE` can change the name, owner, and parameters, but it does not provide a `LOCATION` clause for moving an existing tablespace.

---

# What to remember

```text
• a tablespace is a logical name pointing to a physical directory
• `pg_default` stores ordinary objects; `pg_global` stores cluster-wide system relations
• a table and its index can be placed in different tablespaces
• the tablespace directory must exist and have the correct owner
• a tablespace does not make its data independent from the rest of the cluster
```

Before the test, it is useful not only to recognize command names, but to be able to explain **what problem a mechanism solves, at what level it works, and what limitations it has**.
