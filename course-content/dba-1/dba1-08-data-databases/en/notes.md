# 8. Databases and Schemas

## 1. Topic

A PostgreSQL instance serves multiple databases. Inside each database, objects are organized into schemas; template databases define the initial contents of new databases, while `search_path` controls resolution of unqualified names.

Main model:

```text
cluster
├─ template0
├─ template1
├─ postgres
└─ appdb
    ├─ pg_catalog
    ├─ public
    ├─ app
    └─ pg_temp_*
```

---

# 2. Mechanism

## Databases and templates

Cluster initialization creates three databases: postgres, template0, and template1. Other databases are subsequently created by cloning an existing database. Template1 is the standard template for CREATE DATABASE, template0 is preserved as a clean original template and is useful, among other things, when restoring a `pg_dump` into a database with the original locale/encoding settings, while postgres is intended, among other uses, as a normal database for connecting to the cluster. CREATE DATABASE is implemented as a copy of an existing database. The standard template is template1.

Template0 preserves the original contents and allows a clean database to be created. This is useful for dump restoration and for creating a database with different encoding and locale settings. To copy a template database, existing connections to it must be closed. If other sessions are connected, CREATE DATABASE reports an error. `postgres.bki` is the bootstrap file from which the server obtains commands and initial data used to build the system catalogs during initialization.

In `pg_database`, the `datistemplate` flag means that the database may be used as a public template by users with the `CREATEDB` privilege; `datallowconn` controls whether new connections are allowed. Clearing `datallowconn` prevents new connections but does not terminate sessions that already exist.

## Schemas and `search_path`

A schema is a namespace inside one database. It is used to organize objects logically and avoid name conflicts. A client connects to one database but can access objects in different schemas within it. `search_path` applies to schemas of the current database. For an unqualified name, PostgreSQL searches available schemas in the effective path from left to right.

If a creation command does not contain a qualified name, PostgreSQL uses the first schema in `search_path` that is suitable for object creation. The default `search_path` is `"$user", public`. The special `$user` component substitutes the current user name when a matching accessible schema exists. A name of the form schema.object is qualified: the schema is selected directly by that qualifier.

`pg_catalog` contains PostgreSQL system catalogs, while `information_schema` provides another, standardized SQL view of metadata. `pg_catalog` always participates in name lookup. If it is not written explicitly, PostgreSQL searches it before ordinary path elements. The material separately notes that the configured `search_path` value and the path actually used can differ. `current_schemas(true)` shows the effective path including implicit schemas.

Schema privileges and privileges on objects inside the schema are separate. `USAGE` allows use of the namespace, while `CREATE` allows objects to be created there; privileges on individual tables and other objects are granted separately. For an unqualified name, schemas are examined from left to right. Therefore a custom schema must precede `app` if it is intended to have higher lookup priority.

## Temporary objects

Temporary tables have session or transaction lifetime. They do not need crash recovery and use local temporary buffers of the backend process instead of the shared buffer cache. The actual temporary schema has a name such as `pg_temp_N`. The universal name `pg_temp` refers to the temporary schema of the current session. Each session gets its own temporary schema. Even when `pg_temp` is not explicitly listed in `search_path`, the temporary schema is implicitly searched before the other schemas for relations and data types; the `pg_temp` alias resolves to it. Objects are removed at the end of the session, and the schema itself may later be reused.

`temp_buffers` sets the maximum amount of local buffer memory for temporary tables in one session. These buffers are not used for ordinary permanent tables. In PostgreSQL 16 the default value is 8 MB. Four times the default is 32 MB; this can be set for a database with ALTER DATABASE ... SET.

## Practical orientation

```bash
CREATE DATABASE appdb TEMPLATE template0;

CREATE SCHEMA app;
SET search_path = app, public;
CREATE TABLE orders(id bigint);

CREATE TEMP TABLE stage(id bigint);
SHOW search_path;
```

---

# 3. Important details

## Database-level parameters

ALTER DATABASE ... SET defines a default session value for a parameter at the level of a specific database. It takes effect for new connections.

## Database boundary

A normal client session is connected to exactly one database. Schemas and `search_path` work inside that database; a table in another database of the same cluster cannot be addressed by an ordinary SQL name.

---

# What to remember

```text
• a PostgreSQL connection is always established to a specific database
• `template1` is normally the CREATE DATABASE template; `template0` is kept as a clean baseline
• schemas are namespaces inside one database
• `search_path` affects both convenience and name-resolution security
• temporary objects belong to a session and live in a temporary schema
```

Before the test, it is useful not only to recognize command names, but to be able to explain **what problem a mechanism solves, at what level it works, and what limitations it has**.
