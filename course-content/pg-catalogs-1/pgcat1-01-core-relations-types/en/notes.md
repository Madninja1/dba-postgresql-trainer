# Catalog Core: Relations, Columns, Types, and Databases

## Mental model

PostgreSQL system catalogs are ordinary tables where the server stores schema metadata and internal state. You can query them with regular SQL, but you normally should not modify them manually: database objects should be changed with DDL commands such as `CREATE`, `ALTER`, and `DROP`.

It is more useful to think in terms of an object graph than isolated catalog tables:

```text
database
  ├─ schema / namespace         → pg_namespace
  │    └─ relation              → pg_class
  │         ├─ columns          → pg_attribute
  │         ├─ defaults         → pg_attrdef
  │         └─ row type         → pg_type
  ├─ enum values                → pg_enum
  ├─ collations                 → pg_collation
  └─ sequences                  → pg_sequence + pg_class

cluster-wide objects
  ├─ databases                  → pg_database
  └─ tablespaces                → pg_tablespace
```

Most catalogs belong to a single database: when a database is created, they are copied from the template database and then belong to that database. Some catalogs are shared by every database in the cluster. This distinction matters: a row in `pg_class` belongs to the current database, while a row in `pg_database` describes a database at cluster scope.

---

## `pg_class`: the central relation catalog

In PostgreSQL, a **relation** is not only an ordinary table. `pg_class` contains tables, indexes, sequences, views, materialized views, composite types, foreign tables, TOAST tables, and partitioned objects.

Main fields:

- `oid` — catalog-row identifier;
- `relname` — relation name;
- `relnamespace` — namespace OID from `pg_namespace`;
- `relowner` — owner OID from `pg_authid`;
- `relam` — access method for a table or index;
- `reltablespace` — tablespace, or `0` for the default;
- `relkind` — relation kind.

Common `relkind` values:

```text
r  ordinary table
i  index
S  sequence
t  TOAST table
v  view
m  materialized view
c  composite type
f  foreign table
p  partitioned table
I  partitioned index
```

Treat `relkind` as the object's classification. For example, the mere existence of a row in `pg_class` does not mean the object is a user table.

### Estimates, not exact counters

`pg_class` also stores data used by the planner:

- `relpages` — estimated number of relation pages;
- `reltuples` — estimated number of live rows;
- `relallvisible` — estimated number of all-visible pages.

These values are updated by `VACUUM`, `ANALYZE`, and some DDL commands. Therefore:

```text
pg_class.reltuples ≠ SELECT count(*)
```

`reltuples = -1` means the row-count estimate is not yet known.

### Useful flags

`pg_class` also contains state and property flags, for example:

- `relpersistence`: `p` permanent, `u` unlogged, `t` temporary;
- `relhasindex` — the table has or recently had indexes;
- `relhastriggers` — the relation has or previously had triggers;
- `relrowsecurity` — RLS is enabled for the table;
- `relispartition` — the object is a partition.

Some flags are not maintained as an instantly exact snapshot. For example, `relhasindex` can remain `true` after the last index is dropped and be cleared later by `VACUUM`. Do not always interpret such fields as perfectly synchronized state.

---

## `pg_attribute`: relation columns

`pg_attribute` stores column metadata. There is one row for every column of every relation. Entries exist not only for user tables but also for other objects from `pg_class`, such as indexes.

Main fields:

- `attrelid` — relation OID from `pg_class`;
- `attname` — column name;
- `atttypid` — type OID from `pg_type`;
- `attnum` — attribute number;
- `attnotnull` — whether `NOT NULL` is present;
- `atthasdef` — whether a default or generated expression exists;
- `attidentity` — identity-column marker;
- `attgenerated` — generated-column marker;
- `attisdropped` — whether the column was dropped.

Ordinary user columns have `attnum` values starting at `1`. System columns such as `ctid` usually have negative numbers.

A dropped column can remain physically present in table rows. Therefore its `pg_attribute` row does not necessarily disappear: it can remain with `attisdropped = true`, while its `atttypid` is reset to `0`.

### Type and type modifier

`atttypid` identifies the column's base type, while `atttypmod` stores additional type-specific information, such as the length limit in `varchar(n)`.

`attndims` is nonzero for an array declaration, but PostgreSQL currently does not enforce the actual number of array dimensions from this value; in practice, a nonzero value means "the column was declared as an array."

---

## `pg_attrdef`: DEFAULT and generated expressions

`pg_attribute.atthasdef` only tells you that an expression exists. Its internal representation is stored in `pg_attrdef`.

Key fields:

- `adrelid` — relation;
- `adnum` — column number;
- `adbin` — internal expression representation.

To get readable SQL, use `pg_get_expr`:

```sql
SELECT
    a.attname,
    pg_get_expr(d.adbin, d.adrelid) AS default_expression
FROM pg_attribute AS a
JOIN pg_attrdef AS d
  ON d.adrelid = a.attrelid
 AND d.adnum = a.attnum
WHERE a.attrelid = 'public.orders'::regclass;
```

This illustrates an important general principle of PostgreSQL catalogs: some expressions are stored in the internal `pg_node_tree` format, while PostgreSQL provides decompiler functions for human-readable output.

---

## `pg_namespace`: the foundation of SQL schemas

`pg_namespace` stores namespaces, which are the underlying mechanism for SQL schemas.

Main fields:

- `oid` — namespace identifier;
- `nspname` — name;
- `nspowner` — owner;
- `nspacl` — access privileges.

Relation link:

```text
pg_class.relnamespace → pg_namespace.oid
```

So a normal "table + schema" query joins these two catalogs:

```sql
SELECT n.nspname, c.relname, c.relkind
FROM pg_class AS c
JOIN pg_namespace AS n
  ON n.oid = c.relnamespace
WHERE c.relkind IN ('r', 'p');
```

---

## `pg_type`: data types

`pg_type` describes PostgreSQL data types. Creating a table automatically creates a composite row type for that table, so the `pg_class` ↔ `pg_type` relationship is part of the normal catalog model.

The main classification field is `typtype`:

```text
b  base type
c  composite type
d  domain
e  enum
p  pseudo-type
r  range
m  multirange
```

Other useful links:

- `typnamespace` → `pg_namespace`;
- `typowner` → `pg_authid`;
- `typrelid` → `pg_class` for a composite type;
- `typelem` — element type;
- `typarray` — corresponding true array type;
- `typbasetype` — base type of a domain.

For a fixed-length type, `typlen` stores the size of its internal representation. Variable-length types have negative values; `-1` means varlena.

---

## `pg_database`: cluster-wide databases

`pg_database` differs from most catalogs: it is **shared by every database in the cluster**. There is one common copy of this catalog for the whole cluster.

Useful fields:

- `datname` — database name;
- `datdba` — owner;
- `encoding` — encoding;
- `datistemplate` — whether the database can be used as a template;
- `datallowconn` — whether connections are allowed;
- `datconnlimit` — connection limit;
- `dattablespace` — default tablespace;
- database locale/collation fields.

Limit values:

```text
datconnlimit = -1  → no connection limit
datconnlimit = -2  → connections are not allowed
```

`datallowconn = false` also prohibits connections. This is used, for example, to protect `template0` from modification.

---

## `pg_sequence`: sequence parameters

A sequence is represented in more than one place:

```text
name and schema  → pg_class
parameters       → pg_sequence
```

`pg_sequence` contains:

- `seqrelid` — OID of the sequence in `pg_class`;
- `seqtypid` — data type;
- `seqstart` — start value;
- `seqincrement` — increment;
- `seqmax`, `seqmin` — bounds;
- `seqcache` — cache size;
- `seqcycle` — whether the sequence cycles.

---

## `pg_enum`: enumeration values

Each enum value is represented by a separate row in `pg_enum`.

- `enumtypid` — OID of the enum type from `pg_type`;
- `enumlabel` — textual label;
- `enumsortorder` — position in enum ordering.

For values created together with an enum type, the sort order normally starts at `1..n`. Values added later can receive fractional or negative `enumsortorder` values; the only requirement is that ordering remains correct and values are unique within the enum.

---

## `pg_collation`: collations

`pg_collation` describes available collations.

Main fields:

- `collname` — name;
- `collnamespace` — schema;
- `collowner` — owner;
- `collprovider` — provider;
- `collisdeterministic` — whether the collation is deterministic;
- `collencoding` — applicable encoding;
- `collversion` — provider-defined version.

Important provider codes:

```text
c  libc
i  ICU
d  database default
```

`collencoding = -1` means the collation applies to any encoding.

`collversion` is used to detect changes in a provider's collation definition that could make previously built data inconsistent with the new sort order.

---

## `pg_tablespace`: tablespaces

`pg_tablespace` stores available tablespaces and, like `pg_database`, is a shared catalog.

Main fields:

- `spcname` — name;
- `spcowner` — owner;
- `spcacl` — ACL;
- `spcoptions` — options.

Use the dedicated `pg_tablespace_location()` function to obtain the physical location instead of trying to infer it from PostgreSQL's internal cluster file layout.

---

## Practical catalog-reading workflow

When investigating an object, move from general to specific:

```text
1. find the object          → pg_class
2. determine the schema     → pg_namespace
3. obtain the columns       → pg_attribute
4. resolve the types        → pg_type
5. obtain the default       → pg_attrdef + pg_get_expr
6. move to the object-specific catalog when needed
```

Example: describing table columns:

```sql
SELECT
    n.nspname AS schema_name,
    c.relname AS table_name,
    a.attnum,
    a.attname,
    format_type(a.atttypid, a.atttypmod) AS data_type,
    a.attnotnull
FROM pg_class AS c
JOIN pg_namespace AS n
  ON n.oid = c.relnamespace
JOIN pg_attribute AS a
  ON a.attrelid = c.oid
WHERE c.oid = 'public.orders'::regclass
  AND a.attnum > 0
  AND NOT a.attisdropped
ORDER BY a.attnum;
```

---

## What to remember

```text
pg_class      → what kind of relation this is
pg_namespace  → which schema it belongs to
pg_attribute  → which columns the relation has
pg_attrdef    → its default/generated expression
pg_type       → its data type

pg_database   → cluster databases, shared catalog
pg_tablespace → cluster tablespaces, shared catalog

reltuples     → estimate, not count(*)
attnum > 0    → ordinary user columns
attnum < 0    → system columns
```

The point of system catalogs is not memorizing hundreds of fields. The important skill is knowing which catalog is the source of truth for a property and which OID links it to neighboring catalogs.
