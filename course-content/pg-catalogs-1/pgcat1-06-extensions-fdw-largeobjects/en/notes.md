# Extensions, FDWs, and Large Objects

## Mental model

This topic combines three relatively independent mechanisms:

```text
extension metadata     → pg_extension
foreign data access    → pg_foreign_data_wrapper
                          ↓
                        pg_foreign_server
                          ↓
                        pg_foreign_table
                          +
                        pg_user_mapping
large objects          → pg_largeobject_metadata + pg_largeobject
```

The key is not to mix up the **extension metadata container**, the **FDW chain**, and **paged large-object storage**.

---

## `pg_extension`: an installed extension

`pg_extension` contains information about installed extensions.

Main fields:

- `extname` — extension name;
- `extowner` — owner;
- `extnamespace` — schema containing exported extension objects;
- `extrelocatable` — whether the extension can be moved to another schema;
- `extversion` — installed version;
- `extconfig` — OIDs of extension configuration tables;
- `extcondition` — filters for those configuration tables.

### What `extnamespace` actually means

An extension does **not belong to a schema as a schema-qualified object**. Extension names are never schema-qualified.

`extnamespace` identifies the schema containing all or most exported extension objects. If `extrelocatable = true`, that schema must contain all schema-aware objects belonging to the extension.

So this is the wrong model:

```text
schema.extension_name
```

A better model is:

```text
extension
  └─ exported objects live primarily in extnamespace
```

---

## Foreign Data Wrapper: four levels

### 1. `pg_foreign_data_wrapper`

This describes the FDW mechanism itself.

Fields:

- `fdwname` — wrapper name;
- `fdwowner` — owner;
- `fdwhandler` — handler function or `0`;
- `fdwvalidator` — option validator or `0`;
- `fdwacl` — ACL;
- `fdwoptions` — wrapper-level options.

### 2. `pg_foreign_server`

A foreign server describes a concrete external data source accessed through an FDW.

- `srvname` — server name;
- `srvowner` — owner;
- `srvfdw` → `pg_foreign_data_wrapper.oid`;
- `srvtype`, `srvversion` — optional metadata;
- `srvacl` — ACL;
- `srvoptions` — server-level options.

Relationship:

```text
foreign server --uses--> foreign data wrapper
```

### 3. `pg_foreign_table`

A foreign table already has an ordinary row in `pg_class`, as a relation with `relkind = 'f'`. `pg_foreign_table` stores only the additional properties specific to foreign tables.

- `ftrelid` → `pg_class.oid` of the foreign table;
- `ftserver` → `pg_foreign_server.oid`;
- `ftoptions` — table-level options.

So:

```text
pg_class         → general relation description
pg_foreign_table → foreign-specific properties
```

### 4. `pg_user_mapping`

A user mapping associates a local role with a foreign server.

- `umuser` — local role, or `0` for a mapping "for everyone";
- `umserver` → `pg_foreign_server`;
- `umoptions` — mapping-specific options.

The catalog can contain sensitive connection options, so ordinary users do not get direct access to it. The `pg_user_mappings` view is intended for them instead.

---

## Where FDW options live

Each level has its own options:

```text
pg_foreign_data_wrapper.fdwoptions  → wrapper
pg_foreign_server.srvoptions        → server
pg_foreign_table.ftoptions          → foreign table
pg_user_mapping.umoptions           → user mapping
```

These are not four copies of the same field; they are four configuration layers.

A useful diagnostic path is top-down:

```text
wrapper
  ↓
server
  ↓
foreign table
  +
user mapping
```

---

## `pg_largeobject`: large-object data

A PostgreSQL large object is identified by an OID and physically split into pages stored as rows in `pg_largeobject`.

Fields:

- `loid` — large-object OID;
- `pageno` — page number, starting at `0`;
- `data` — `bytea` containing that page's data.

The large-object page size is `LOBLKSIZE`, which in a normal configuration is `BLCKSZ/4`.

Page-data offset:

```text
pageno * LOBLKSIZE
```

Storage can be sparse: pages can be missing or shorter than the normal page size. Missing regions are read as zero bytes.

---

## `pg_largeobject_metadata`: owner and ACL

Large-object data and metadata are separated:

```text
pg_largeobject          → pages/data
pg_largeobject_metadata → owner + ACL
```

`pg_largeobject_metadata` contains:

- `oid` — large-object identifier;
- `lomowner` — owner;
- `lomacl` — access privileges.

This separation is related in part to security: direct access to large-object contents should not automatically make it possible to enumerate or read every object without privilege checks.

---

## Practical FDW navigation

```sql
SELECT
    ft.ftrelid::regclass AS foreign_table,
    s.srvname,
    w.fdwname
FROM pg_foreign_table AS ft
JOIN pg_foreign_server AS s
  ON s.oid = ft.ftserver
JOIN pg_foreign_data_wrapper AS w
  ON w.oid = s.srvfdw;
```

This query shows the natural direction of relationships:

```text
foreign table → server → wrapper
```

User mappings are joined separately through `umserver` and the role.

---

## What to remember

```text
pg_extension → installed extension metadata
extnamespace → schema of exported objects, not a schema-qualified extension name

FDW chain:
pg_foreign_data_wrapper
        ↓
pg_foreign_server
        ↓
pg_foreign_table
        +
pg_user_mapping

large object:
pg_largeobject_metadata → owner/ACL
pg_largeobject          → data pages
```

When diagnosing an FDW, first determine which layer owns the property: wrapper, server, table, or user mapping. For large objects, treat metadata and the binary pages as separate concerns.
