# 19. Physical Replication

## 1. Topic

Physical replication creates a binary-compatible copy of the entire cluster by transmitting WAL from the primary server to a standby. The standby remains continuously in recovery and applies the same page and transaction-state changes that occurred on the primary.

Core model:

```text
PRIMARY
 WAL → wal sender =====stream=====> wal receiver → WAL
                                      ↓
                                   startup
                                      ↓
                                   STANDBY
                              hot standby: read
```

Physical replication provides the basic mechanisms for fault tolerance, high availability, and read scaling, but by itself it is not a complete automatic failover or load-balancing system.

---

# 2. Mechanism

## Standby foundation

A standby server is created from a physical backup of the primary. It then remains in continuous recovery and keeps applying new WAL.

Properties of physical replication:

- data flows from primary to standby;
- the entire cluster is replicated;
- the servers must be binary-compatible;
- changes are transmitted at the WAL level rather than as SQL commands.

## Streaming and file-based delivery

In streaming mode, the standby `wal receiver` connects to the primary `wal sender` using the replication protocol and receives WAL almost immediately after it is generated.

File-based replication reads completed WAL segments from an archive and therefore usually has greater lag. In practice, a WAL archive is useful as an additional source: if streaming is temporarily unavailable, the standby can obtain the required segment from the archive.

## `wal_level`

Physical replication requires `wal_level = replica` or higher. The `replica` level also contains the information required for hot physical backups.

## Hot standby

With `hot_standby = on`, clients can connect to a standby during recovery and execute read-only queries. This includes, for example, `SELECT`, read-only transactions, and session parameter changes.

Actions that modify database state are not allowed:

- `INSERT`, `UPDATE`, `DELETE`, `TRUNCATE`, `nextval`;
- DDL;
- `SELECT FOR UPDATE` and similar locking operations;
- `VACUUM`, `ANALYZE`, `REINDEX`;
- access-control commands.

With `hot_standby = off`, recovery continues but client connections are not allowed. This is a warm standby.

---

# 3. Synchronization, reads, and conflicts

## Synchronous replication

In synchronous mode, `COMMIT` can wait for acknowledgment from a selected synchronous standby. This reduces the risk of losing already committed transactions if the primary fails, but increases commit latency.

Which standby is considered synchronous is configured with `synchronous_standby_names` together with the corresponding `synchronous_commit` setting.

Important: synchronous commit does not mean that arbitrary reads performed simultaneously on different servers form one shared consistent snapshot.

## Hot standby conflicts

A standby must simultaneously:

1. serve a read query;
2. apply WAL from the primary.

A conflict can occur, for example, when the primary has already removed row versions still required by a long-running query on the standby, or when WAL requires a lock that is incompatible with the query.

`max_standby_streaming_delay` limits how long streamed WAL application may be delayed for conflicting queries. After the allowed delay is exceeded, the query can be canceled.

`hot_standby_feedback = on` sends the primary information about the visibility horizon required by standby queries. This reduces cancellations caused by cleanup, but can increase bloat on the primary because removal of old row versions is postponed.

---

# 4. Topologies and operations

## Multiple standbys

Several standbys can connect to one primary and serve read workloads. PostgreSQL itself provides the replication mechanism, but automatic distribution of client connections requires an external load balancer.

Even with synchronous replication, reads from different nodes do not guarantee one shared consistent state: different standbys can be at different WAL positions.

## Cascading replication

To reduce load on the primary and on the network, a standby can forward WAL to another standby. The longer the chain, the greater the potential lag. Synchronous acknowledgment by the primary applies to the directly connected synchronous standby; a cascade does not automatically become a chain of synchronous acknowledgments.

## Delayed standby

WAL application can be intentionally delayed. Such a standby acts as a “time machine”: after an erroneous data change, there is a window in which the standby has not yet applied the destructive operation.

This does not replace a complete backup strategy, but it provides an additional protection mechanism against user errors.

## Switching

A **planned switchover** is performed for maintenance: the primary is shut down cleanly or taken out of service, and the application is moved to a prepared standby.

A **failover** is performed after a primary failure. Core PostgreSQL does not fully handle failure detection, selection of a new primary, and client reconfiguration; external cluster software is usually required for automation.

---

# 5. Practical reference

```bash
# primary
wal_level = replica
max_wal_senders = 10
synchronous_commit = on
synchronous_standby_names = 'standby1'

# create the standby foundation
pg_basebackup -h primary -D /var/lib/postgresql/standby -R -Xs -P

# standby conflict settings
max_standby_streaming_delay = '30s'
hot_standby_feedback = on

# manual promotion
pg_ctl -D /var/lib/postgresql/standby promote
```

---

# What to remember

```text
• physical replication copies the entire cluster and requires binary compatibility
• streaming reduces lag compared with waiting for a completed WAL segment
• synchronous mode improves durability at the cost of COMMIT latency
• hot standby allows reads during recovery but not writes
• max_standby_streaming_delay and hot_standby_feedback define the trade-off between standby freshness, long queries, and primary bloat
• multiple and cascading standbys distribute load, but balancing requires external tools
• a delayed standby can help recover from erroneous changes
• automatic failover does not appear merely by configuring streaming replication
```

Before the test, understand physical replication as the **basic mechanism for delivering and applying WAL**, on top of which high-availability operational solutions are built.
