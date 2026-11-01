# Constraints, Dependencies, Rules, and Triggers

## Mental model

This group of catalogs answers three different questions:

```text
which integrity rules apply to an object?   → pg_constraint
what does an object depend on and what drops?→ pg_depend / pg_shdepend
which server mechanisms react to SQL?        → pg_rewrite / pg_trigger /
                                               pg_event_trigger
```

Object comments are stored separately in `pg_description` and `pg_shdescription`.

---

## `pg_constraint`: constraints

`pg_constraint` stores:

- `CHECK`;
- `UNIQUE`;
- `PRIMARY KEY`;
- `FOREIGN KEY`;
- `EXCLUDE`;
- constraint triggers;
- domain constraints.

`NOT NULL` is an important exception: it is represented not by a row in `pg_constraint`, but by the `pg_attribute.attnotnull` flag.

### Constraint type

`contype`:

```text
c  CHECK
f  FOREIGN KEY
p  PRIMARY KEY
u  UNIQUE
t  constraint trigger
x  EXCLUDE
```

### Which object the constraint belongs to

- `conrelid` — table, if this is a table constraint;
- `contypid` — domain, if this is a domain constraint;
- `connamespace` — constraint namespace;
- `conname` — name, which does not have to be globally unique.

### Constraint state

- `condeferrable` — the constraint can be deferred;
- `condeferred` — deferred by default;
- `convalidated` — existing data has been validated.

In PostgreSQL 16, `convalidated = false` is possible for foreign keys and `CHECK` constraints.

---

## The index supporting a constraint

`conindid` points to the index associated with a constraint when applicable: unique, primary key, foreign key, or exclusion constraints.

Do not read this as "every constraint always has an index." For example, an ordinary `CHECK` constraint does not require a supporting index.

---

## How a FOREIGN KEY is represented in `pg_constraint`

Four fields are especially important for a foreign key:

```text
conrelid  → table where the FK is declared
conkey    → numbers of its columns
confrelid → referenced table
confkey   → numbers of referenced columns
```

The numbers in `conkey/confkey` correspond to `pg_attribute.attnum`.

Additional properties:

- `confupdtype` — action when the referenced key is updated;
- `confdeltype` — action when the referenced key is deleted;
- `confmatchtype` — encoded `FULL`, `PARTIAL`, or `SIMPLE` match type;
- `conpfeqop`, `conppeqop`, `conffeqop` — comparison operators used by the FK.

Update/delete action codes:

```text
a  NO ACTION
r  RESTRICT
c  CASCADE
n  SET NULL
d  SET DEFAULT
```

`confdelsetcols` can specify the particular referencing columns changed by `SET NULL` or `SET DEFAULT`.

### Constraint expression

The internal expression for a `CHECK` constraint is stored in `conbin`. For user-facing output, prefer:

```sql
pg_get_constraintdef(oid)
```

instead of parsing `pg_node_tree` manually.

---

## `pg_depend`: the dependency graph inside a database

`pg_depend` records dependency relationships between objects in the current database. PostgreSQL uses this information to know what to reject under `DROP RESTRICT` and what to remove under `DROP CASCADE` or automatically.

A dependency row has two sides:

```text
dependent object                           referenced object
classid, objid, objsubid   ───────────→   refclassid, refobjid, refobjsubid
```

`objsubid/refobjsubid` identify subobjects such as columns; `0` means the whole object.

### `deptype`: dependencies are not all the same

#### `n` — NORMAL

The dependent object may be dropped separately. The referenced object cannot be dropped without `CASCADE` while the dependent object remains.

#### `a` — AUTO

The dependent object may be dropped separately, but it is dropped automatically when the referenced object is dropped, regardless of an explicit `CASCADE` clause.

#### `i` — INTERNAL

The dependent object is considered an internal part of another object. A direct `DROP` of it is rejected; the owning object must be dropped instead.

#### `P` / `S` — partition dependencies

Used for partitioning objects with multiple related parent objects. `P` is the primary and `S` the secondary dependency. These relationships help PostgreSQL decide how partition-related objects are dropped and how errors are reported.

#### `e` — EXTENSION

The object is a member of an extension. It should be removed through `DROP EXTENSION`.

#### `x` — AUTO_EXTENSION

The object is not an extension member in dump-membership terms, but cannot function without the extension and is automatically dropped with it. It can still be dropped separately.

An object can have multiple dependency rows of different kinds. Final `DROP` behavior is determined by the combined set of relationships.

### Why some expected dependencies are absent

Many objects created by `initdb` are considered pinned. They are not supposed to be dropped, so `pg_depend` does not need to store ordinary dependency rows from every object to those pinned system objects.

---

## `pg_shdepend`: dependencies on shared objects

`pg_shdepend` solves a similar problem for dependencies on cluster-wide shared objects, especially roles and tablespaces.

Fields include:

- `dbid` — database containing the dependent object, or `0` for a global object;
- `classid`, `objid`, `objsubid` — dependent object;
- `refclassid`, `refobjid` — shared referenced object;
- `deptype` — shared dependency type.

Examples of dependency types:

```text
o  OWNER
a  ACL
r  POLICY
t  TABLESPACE
```

This is how PostgreSQL can determine, for example, that a role still owns objects or is mentioned in ACLs and therefore cannot simply be dropped before those dependencies are resolved.

---

## Comments: `pg_description` and `pg_shdescription`

The `COMMENT` command stores text in system catalogs.

### `pg_description`

Comments for objects in the current database:

- `objoid` — object;
- `classoid` — object's catalog;
- `objsubid` — column number or `0`;
- `description` — text.

### `pg_shdescription`

Comments for shared cluster objects. The catalog itself is shared.

There is no `objsubid` here because the catalog is intended for global objects described as whole entities.

---

## `pg_rewrite`: query rewrite rules

`pg_rewrite` stores rewrite rules for tables and views.

Main fields:

- `rulename` — name;
- `ev_class` — relation to which the rule belongs;
- `ev_type` — event type;
- `ev_enabled` — `session_replication_role` mode;
- `is_instead` — whether this is an `INSTEAD` rule;
- `ev_qual` — condition;
- `ev_action` — action.

`ev_type` codes:

```text
1  SELECT
2  UPDATE
3  INSERT
4  DELETE
```

If rules exist for a relation, `pg_class.relhasrules` should be set.

---

## `pg_trigger`: table and view triggers

`pg_trigger` stores trigger metadata.

Key fields:

- `tgrelid` — relation;
- `tgname` — trigger name, unique within one table;
- `tgfoid` — called function;
- `tgtype` — bitmask describing firing conditions;
- `tgenabled` — `session_replication_role` firing mode;
- `tgisinternal` — internal trigger;
- `tgconstraint` — associated `pg_constraint`, if any;
- `tgnargs` — number of arguments;
- `tgattr` — column numbers for a column-specific UPDATE trigger;
- `tgargs` — trigger-function arguments;
- `tgqual` — internal representation of the `WHEN` condition;
- `tgoldtable`, `tgnewtable` — transition-table names.

`tgenabled` codes:

```text
O  origin + local
D  disabled
R  replica
A  always
```

If a relation has triggers, `pg_class.relhastriggers` should be set.

Use `pg_get_triggerdef()` to reconstruct a readable definition.

---

## `pg_event_trigger`: event triggers

An event trigger reacts not to a table-row change, but to DDL infrastructure events.

`pg_event_trigger` contains:

- `evtname` — name;
- `evtevent` — event;
- `evtowner` — owner;
- `evtfoid` — called function;
- `evtenabled` — firing mode;
- `evttags` — list of command tags, or `NULL` when there is no tag filter.

`evtenabled` uses the same `O/D/R/A` codes as ordinary triggers.

---

## Practical way to inspect a constraint

```sql
SELECT
    c.conname,
    c.contype,
    c.conrelid::regclass AS relation_name,
    pg_get_constraintdef(c.oid) AS definition
FROM pg_constraint AS c
WHERE c.conrelid = 'public.orders'::regclass;
```

This query demonstrates an important principle: catalog columns are useful for machine analysis and joins, while `pg_get_*def()` functions are useful for human-readable DDL representations.

---

## What to remember

```text
pg_constraint    → integrity constraints
pg_attribute     → NOT NULL
pg_depend        → dependencies inside a database
pg_shdepend      → dependencies on shared objects
pg_description   → comments on database-local objects
pg_shdescription → comments on shared objects
pg_rewrite       → query rewrite rules
pg_trigger       → row/statement triggers
pg_event_trigger → event triggers
```

The key distinction is that a constraint describes an **integrity rule**, a dependency describes a **lifecycle relationship between objects**, and a trigger/rule describes a **mechanism that reacts to or rewrites SQL**.
