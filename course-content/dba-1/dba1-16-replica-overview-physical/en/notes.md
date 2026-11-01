# 20. Logical Replication

## 1. Topic

Logical replication decodes WAL into row changes and transfers selected data from a publication to a subscription. Unlike physical replication, it does not copy the entire cluster, does not require binary compatibility, and allows selective and bidirectional topologies.

Main model:

```text
PUBLISHER
WAL → logical decoding → wal sender
                      ===== protocol =====>
SUBSCRIBER
logical replication worker → apply row changes
```

---

# 2. Mechanism

## `wal_level = logical`

The regular `replica` level is sufficient for physical replication, but logical decoding requires `wal_level = logical` on the publishing server. This level adds information to WAL that is required to reconstruct the logical meaning of row changes and relation structure.

## Publications

A publication exists within a single database and describes a set of changes that the server is prepared to send to subscribers.

It can define:

- tables;
- a set of operations: `INSERT`, `UPDATE`, `DELETE`, `TRUNCATE`;
- published columns;
- row filters.

A table can belong to multiple publications. A publication is not a schema and does not itself change table privileges.

## Subscriptions

A subscription connects to one or more publications and usually creates a logical replication slot on the publisher. Changes for one subscription arrive through its slot, while initial synchronization may temporarily require additional slots.

When a subscription is created, existing data is copied by default (`copy_data = true`), after which new changes are streamed. Target tables and the required schema must already exist: DDL is not transferred automatically.

## What is transferred

Row changes are replicated, not the original SQL text and not physical pages. The subscriber applies the received operations to its own tables.

Standard built-in logical replication does not automatically transfer:

- DDL and schema changes;
- sequence values;
- large objects (`lo`);
- changes to views, materialized views, and foreign tables.

Regular and partitioned tables can be replicated directly.

---

# 3. Replica Identity and Conflicts

## Replica identity

For `UPDATE` and `DELETE`, the subscriber must determine which row to modify. `REPLICA IDENTITY` is used for this purpose:

- `DEFAULT` — primary key;
- `USING INDEX` — a suitable unique, non-partial, non-deferrable index on `NOT NULL` columns;
- `FULL` — old values of all columns;
- `NOTHING` — do not store a row identifier.

If a publication sends `UPDATE`/`DELETE` but there is no suitable replica identity, those changes cannot be published correctly.

## Conflicts

A subscriber is a regular PostgreSQL database, so local changes can conflict with incoming ones. If applying an incoming operation violates a constraint or required privileges are missing, the logical replication worker stops with an error until the cause is resolved manually.

Built-in logical replication does not provide universal automatic conflict resolution.

Sequences are replicated separately from tables, and their current values are not synchronized automatically. This is especially important in bidirectional topologies: different servers should use non-overlapping ranges or globally unique identifiers such as UUIDs.

---

# 4. Operational Limitations

## Slots and connection loss

A slot ensures that the publisher retains WAL still needed by the subscriber. If a subscription remains inactive for a long time, the slot can retain WAL and the cleanup horizon, so a lost connection cannot be ignored indefinitely.

## `TRUNCATE` and foreign keys

For `TRUNCATE` replication to work correctly, all tables related by foreign keys to the truncated table and requiring a coordinated `TRUNCATE` must be included in a compatible replication topology. Otherwise, applying the operation on the subscriber may fail.

## Long transactions and `streaming`

Without streaming, changes from a long-running transaction are fully decoded on the publisher and sent only after it commits. The subscription parameter `streaming` allows changes from an in-progress transaction to be sent earlier; the subscriber can temporarily store them or apply them with a parallel worker in a supported mode.

---

# 5. Use Cases

## Consolidation

Several branches can publish selected tables, while a central server subscribes to them and consolidates the data.

## Major version upgrade

Logical replication does not require binary compatibility between major versions. Therefore, a new server can be prepared, synchronized, and then clients can be switched over, reducing the downtime window.

## Bidirectional replication

PostgreSQL 16 allows bidirectional topologies, but the application must avoid conflicts and ensure correct identifier generation itself. The built-in mechanism does not provide global distributed transactions or automatic cluster management.

For the practical subscription between two servers used in the course:

```text
copy_data = false
origin = none
```

`origin = none` requests that changes which already have a replication origin not be forwarded, helping prevent immediate propagation of received changes back to their source.

## Subscription within the same cluster

When subscribing to a publication in the same cluster, a regular `CREATE SUBSCRIPTION` that also tries to create the remote slot can hang. A practical workaround is to create the logical slot separately in advance and then create the subscription with `create_slot = false`.

---

# 6. Practical Reference

```sql
-- publisher
CREATE PUBLICATION app_pub FOR TABLE app.orders;

-- subscriber: table app.orders must already exist
CREATE SUBSCRIPTION app_sub
CONNECTION 'host=publisher dbname=appdb user=repl'
PUBLICATION app_pub;

-- if there is no suitable PK/unique identity
ALTER TABLE app.orders REPLICA IDENTITY FULL;
```

---

# What to Remember

```text
• publisher for built-in logical replication uses wal_level = logical
• a publication selects tables/operations and can restrict rows and columns
• a subscription usually creates a logical replication slot and can perform an initial copy
• DDL, sequences, and large objects are not synchronized automatically
• UPDATE/DELETE require a replica identity
• an inactive slot retains required WAL and must be monitored
• streaming helps with long-running transactions
• a bidirectional topology requires explicit conflict handling and key uniqueness design
• a subscription within the same cluster requires creating the slot separately and using create_slot = false
```

Before the test, it is important to see logical replication as **a stream of changes for selected tables with explicit limitations around schema, row identification, and conflicts**.
