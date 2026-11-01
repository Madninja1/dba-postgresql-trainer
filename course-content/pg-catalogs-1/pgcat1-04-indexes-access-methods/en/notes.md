# Indexes and Access Methods

## Mental model

An index description in PostgreSQL is distributed across several catalogs:

```text
pg_class
   │  general relation row
   ▼
pg_index
   │  properties of a specific index and its table link
   │
   ├─ pg_opclass ── pg_opfamily
   │       │            │
   │       │            ├─ pg_amop   operators
   │       │            └─ pg_amproc support procedures
   │       ▼
   └───── pg_am         access method
```

Partitioning and inheritance add two more catalogs:

```text
pg_partitioned_table → partition key
pg_inherits          → parent/child edges
```

---

## `pg_am`: access methods

`pg_am` contains relation access methods. Each row describes one access method.

Fields:

- `amname` — name;
- `amhandler` — handler function;
- `amtype` — method kind.

`amtype`:

```text
t  table access method
i  index access method
```

Index access-method properties are no longer encoded in many `pg_am` columns as they were in older PostgreSQL versions. Dedicated functions such as `pg_indexam_has_property()`, `pg_index_has_property()`, and `pg_index_column_has_property()` are used to inspect those properties.

---

## Operator families and operator classes

### `pg_opfamily`

An operator family is a set of compatible operators and support procedures for one index access method.

Fields:

- `opfmethod` → `pg_am`;
- `opfname` — family name;
- `opfnamespace` — schema;
- `opfowner` — owner.

Most of the family's actual contents are represented by related rows in `pg_amop`, `pg_amproc`, and `pg_opclass`.

### `pg_opclass`

An operator class connects:

```text
index access method
        +
indexed data type
        +
operator family
```

Fields:

- `opcmethod` → `pg_am`;
- `opcname` — opclass name;
- `opcfamily` → `pg_opfamily`;
- `opcintype` — indexed type;
- `opcdefault` — default opclass for an access-method + type pair;
- `opckeytype` — type actually stored in the index, or `0` if it matches `opcintype`.

For each `opcmethod + opcintype` combination, at most one opclass can be the default.

---

## `pg_amop`: operators in a family

`pg_amop` contains operators that belong to operator families.

Main fields:

- `amopfamily` — family;
- `amoplefttype`, `amoprighttype` — operand types;
- `amopstrategy` — strategy number;
- `amoppurpose` — purpose;
- `amopopr` — operator;
- `amopmethod` — access method;
- `amopsortfamily` — B-tree family for an ordering operator.

`amoppurpose`:

```text
s  search
o  ordering
```

A search operator means an index from that family can search for rows satisfying a condition of the form:

```sql
indexed_column operator constant
```

An ordering operator allows an index scan to produce rows in the order defined by an expression such as `ORDER BY indexed_column operator constant`.

---

## `pg_amproc`: support procedures

`pg_amproc` stores support functions for operator families.

Fields:

- `amprocfamily` — family;
- `amproclefttype`, `amprocrighttype` — related types;
- `amprocnum` — support-function number;
- `amproc` — function OID.

The exact meaning of a support-procedure number depends on the access method. Therefore `amprocnum` cannot be interpreted correctly without knowing the corresponding access method.

---

## `pg_index`: properties of a specific index

The general index row lives in `pg_class`, while `pg_index` adds index-specific properties.

Main relationships:

```text
indexrelid → pg_class.oid of the index
indrelid   → pg_class.oid of the table
```

### Number of attributes

- `indnatts` — all index attributes, including key and non-key `INCLUDE` columns;
- `indnkeyatts` — key attributes only.

### Purpose and state

- `indisunique` — unique index;
- `indisprimary` — index represents a `PRIMARY KEY`;
- `indisexclusion` — supports an exclusion constraint;
- `indisclustered` — the table was last clustered on this index;
- `indisreplident` — index is selected as replica identity.

Lifecycle state:

- `indisvalid` — the index may be used in queries;
- `indisready` — the index is ready to receive new data;
- `indislive` — the index is considered live; `false` means it is being dropped.

These flags matter especially during online/concurrent operations: an index may physically exist but not yet be usable by the planner or not yet participate in maintenance for every new row.

### NULL in a unique index

`indnullsnotdistinct` applies to unique indexes:

```text
false → NULL values are distinct; multiple NULLs are allowed
true  → NULL values are equal for uniqueness purposes
```

---

## `indkey`: which table columns are indexed

`indkey` stores table column numbers (`pg_attribute.attnum`).

Example:

```text
1 3
```

means the first and third table columns.

A **zero** in `indkey` does not mean "column zero". It means an expression key: the corresponding index attribute is defined by an expression.

The expression tree itself is stored in `indexprs`. It contains one expression for each zero among the key attributes.

A partial index is described by `indpred`: when it is not `NULL`, it contains the internal predicate tree.

In practice:

```text
indkey       → column numbers / 0 for expressions
indexprs     → expressions for zero entries in indkey
indpred      → WHERE predicate of a partial index
```

---

## `pg_partitioned_table`: the partition key

`pg_partitioned_table` supplements the partitioned-table row in `pg_class`.

Fields:

- `partrelid` — partitioned table;
- `partstrat` — strategy;
- `partnatts` — number of partition-key elements;
- `partdefid` — default partition or `0`;
- `partattrs` — column numbers or `0` for an expression;
- `partclass` — operator classes;
- `partcollation` — collations;
- `partexprs` — partition-key expressions for zero entries in `partattrs`.

Strategy codes:

```text
h  HASH
l  LIST
r  RANGE
```

As with `pg_index.indkey`, `0` in `partattrs` means an expression, not an ordinary column.

---

## `pg_inherits`: inheritance and partitioning tree

`pg_inherits` stores **direct** parent/child relationships between tables and indexes.

- `inhrelid` — child relation;
- `inhparent` — parent relation;
- `inhseqno` — order of inherited columns for multiple inheritance;
- `inhdetachpending` — the partition is currently being detached.

Each edge is stored as a separate row. An indirect hierarchy must be obtained by traversing the chain of rows.

For declarative partitioning, this same structure represents parent/partition relationships.

---

## Practical queries

### Indexes of a table

```sql
SELECT
    ic.relname AS index_name,
    i.indisunique,
    i.indisprimary,
    i.indisvalid,
    i.indisready,
    i.indislive
FROM pg_index AS i
JOIN pg_class AS ic
  ON ic.oid = i.indexrelid
WHERE i.indrelid = 'public.orders'::regclass;
```

### Reconstructing index DDL

For a human-readable definition, prefer:

```sql
SELECT pg_get_indexdef(indexrelid)
FROM pg_index
WHERE indrelid = 'public.orders'::regclass;
```

This is safer than trying to decode `indexprs`, `indpred`, opclass information, and option fields manually.

---

## What to remember

```text
pg_am       → access methods
pg_opfamily → compatible family of operators/procedures
pg_opclass  → type + access method + family
pg_amop     → family operators
pg_amproc   → family support procedures
pg_index    → properties of a specific index

indkey = N  → table column attnum N
indkey = 0  → expression key

pg_partitioned_table → partition key
pg_inherits          → parent/child edges
```

A PostgreSQL index is not described by one catalog row. Its full description comes from general relation metadata, `pg_index`, the access method, and the operator infrastructure.
