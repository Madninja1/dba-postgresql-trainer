# Logical Replication Catalogs

## Mental model

Logical replication metadata is split between publisher-side and subscriber-side catalogs:

```text
publisher database
  ├─ pg_publication
  ├─ pg_publication_namespace
  └─ pg_publication_rel

subscriber cluster
  ├─ pg_subscription
  ├─ pg_subscription_rel
  └─ pg_replication_origin
```

A publication describes **what to publish**. A subscription describes **where to receive changes from and how to apply them**. `pg_subscription_rel` tracks synchronization state for individual relations.

---

## `pg_publication`: a publication in the current database

`pg_publication` contains publications created in the current database.

Main fields:

- `pubname` — name;
- `pubowner` — owner;
- `puballtables` — publication automatically includes all tables in the database;
- `pubinsert` — publish `INSERT`;
- `pubupdate` — publish `UPDATE`;
- `pubdelete` — publish `DELETE`;
- `pubtruncate` — publish `TRUNCATE`;
- `pubviaroot` — publish changes to a partition through the topmost published ancestor.

If `puballtables = true`, the publication automatically includes not only existing tables but also future tables created in the database.

The operation flags are independent: a publication can, for example, publish inserts and updates but not deletes.

---

## `pg_publication_namespace`: schema → publication

This catalog implements a many-to-many mapping between schemas and publications.

- `pnpubid` → publication;
- `pnnspid` → namespace.

One publication can include several schemas, and one schema can belong to several publications.

---

## `pg_publication_rel`: relation → publication

`pg_publication_rel` is the many-to-many mapping between relations and publications.

Fields:

- `prpubid` → publication;
- `prrelid` → relation;
- `prqual` — internal row-filter expression or `NULL`;
- `prattrs` — numbers of published columns, or `NULL` for all columns.

So publication metadata can restrict not only the table list, but also:

```text
rows     → prqual
columns  → prattrs
```

`prattrs` uses `pg_attribute.attnum` values.

---

## `pg_subscription`: cluster-wide subscriptions

`pg_subscription` contains existing subscriptions and is a shared catalog: one copy is used by all databases in the cluster.

Main fields:

- `subdbid` — database in which the subscription resides;
- `subname` — name;
- `subowner` — owner;
- `subenabled` — whether the subscription is enabled;
- `subconninfo` — publisher connection string;
- `subslotname` — replication slot on the publisher;
- `subpublications` — array of publication names;
- `subsynccommit` — `synchronous_commit` value for apply workers;
- `suborigin` — filtering by replication origin.

### Why `subconninfo` is sensitive

The connection string can contain a password in plain text. Therefore access to this field is restricted for ordinary users.

When building a diagnostic tool, do not treat `subconninfo` as harmless metadata.

---

## Additional subscription modes

### Binary transfer

`subbinary = true` means data is requested from the publisher in binary representation.

### Streaming in-progress transactions: `substream`

```text
f  streaming of in-progress transactions is disabled

t  changes are streamed/spilled to disk on the subscriber,
   then applied after the publisher commit is received

p  try to apply changes directly with a parallel apply worker;
   if no worker is available, behavior falls back to t
```

### Two-phase

`subtwophasestate`:

```text
d  disabled
p  pending enable
e  enabled
```

### Errors and security context

- `subdisableonerr` — disable the subscription when a worker encounters an error;
- `subpasswordrequired` — require password authentication;
- `subrunasowner` — apply changes with the subscription owner's privileges.

---

## `pg_subscription_rel`: state of each relation

This catalog links a subscription to a relation and stores the progress/state of its initial synchronization.

- `srsubid` → subscription;
- `srrelid` → relation;
- `srsubstate` — state;
- `srsublsn` — remote LSN used to coordinate `s`/`r` states.

Codes:

```text
i  initialization
d  data copy
f  table copy finished
s  synchronized
r  ready / normal replication
```

`pg_subscription_rel` contains only tables known to the subscription after `CREATE SUBSCRIPTION` or `ALTER SUBSCRIPTION ... REFRESH PUBLICATION`.

Therefore, a missing relation here does not necessarily mean replication is broken; it may mean the subscriber has not yet refreshed its list of publication relations.

---

## `pg_replication_origin`: origin of replicated changes

`pg_replication_origin` stores registered replication origins and is also a shared catalog.

Fields:

- `roident` — internal identifier, unique within the cluster;
- `roname` — user-defined external origin name.

`roident` is an internal ID and should not be treated as a portable external identifier between systems.

Replication origins allow PostgreSQL to track where changes originated and coordinate replication state.

---

## Reading a publication as a set

A publication can be assembled through several mechanisms:

```text
puballtables = true            → all tables in the database
pg_publication_namespace       → schemas
pg_publication_rel             → specific relations
                                 + row/column filters
```

Therefore, `pg_publication.puballtables` alone does not describe the full publication membership when its value is `false`.

---

## Practical query: subscription relations

```sql
SELECT
    s.subname,
    r.srrelid::regclass AS relation_name,
    r.srsubstate,
    r.srsublsn
FROM pg_subscription AS s
JOIN pg_subscription_rel AS r
  ON r.srsubid = s.oid
ORDER BY s.subname, relation_name;
```

This gives a subscriber-side view: which relations are known to each subscription and what state they are in.

---

## What to remember

```text
publisher:
pg_publication           → publication and operation flags
pg_publication_namespace → schemas in a publication
pg_publication_rel       → relations + row/column filters

subscriber:
pg_subscription          → subscription connection/apply settings
pg_subscription_rel      → per-relation synchronization state
pg_replication_origin    → replication origins
```

The key distinction: a publication defines the **set of published changes**, a subscription defines the **receiving channel**, and `pg_subscription_rel` stores the **state of individual relations inside that channel**.
