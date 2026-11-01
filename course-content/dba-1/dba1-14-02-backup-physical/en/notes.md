# 18. Physical Backup and Recovery

## 1. Topic

A physical backup preserves the files of the entire cluster. It can be restored quickly and, together with a WAL archive, can reproduce the cluster state at a required point in time. However, it cannot restore a single database independently and requires a compatible PostgreSQL major version and architecture.

Core model:

```text
primary cluster
   ├─ pg_basebackup → base backup
   └─ WAL → file archive / pg_receivewal

restore:
base backup + WAL
       ↓
recovery
       ↓
consistent cluster
```

---

# 2. Mechanism

## Cold and hot backup

Physical backup copies cluster files.

A **cold backup** is created while the server is stopped. After a clean shutdown, the files are already consistent, so no separate set of WAL is required to bring the backup to a consistent state. If the snapshot was taken after a crash state, the WAL required for crash recovery must also be preserved.

A **hot backup** is created while the server is running. A plain `cp` is not enough: while different files are being copied, the server continues to modify them. A special mechanism is therefore required to guarantee that WAL covering the entire backup interval is available.

## `pg_basebackup`

`pg_basebackup` creates a base physical backup of an entire running cluster. It cannot be used to copy only one database or one table.

The utility connects through the replication protocol. The connection requires:

- a superuser or a role with the `REPLICATION` attribute;
- permission for a replication connection in `pg_hba.conf`;
- `wal_level = replica` or higher;
- a sufficient `max_wal_senders` value.

For a self-contained backup, the required WAL can be received while the cluster files are being copied, so that after the backup is deployed the server can restore consistency by itself.

## Replication protocol and slot

The replication protocol is used not only by replicas but also by `pg_basebackup` and `pg_receivewal`. A replication connection is served by a `wal sender` process.

A replication slot stores the position up to which the client is guaranteed to have received WAL. Until the required records are acknowledged through the slot, the server must not remove the corresponding WAL. This protects the client from a gap in the WAL sequence, but an abandoned or stopped slot can retain a large amount of WAL on the primary server.

## File-based WAL archive

Continuous archiving stores completed WAL segments separately from the cluster.

```conf
wal_level = replica
archive_mode = on
archive_command = 'cp %p /archive/%f'
```

`archive_command` is invoked for a completed segment. Exit code zero means successful archiving; on failure PostgreSQL retains the segment and retries later. Therefore, a broken archive can fill `pg_wal` if the problem remains unresolved for too long.

File-based archiving inevitably has some delay: the current incomplete WAL segment has not yet been sent to the archive.

## Streaming archive: `pg_receivewal`

`pg_receivewal` receives WAL through the streaming replication protocol and writes it to a separate directory without waiting for a segment switch. A replication slot is normally used for reliability.

An incomplete file has the `.partial` suffix; after the segment is completed, it gets its normal name. During recovery, the latest available `.partial` file may contain records that are not yet present in a completed archived segment.

Each `pg_receivewal` connection occupies a `wal sender`, so it must be taken into account when choosing `max_wal_senders`.

---

# 3. Recovery

## Base backup + archive

With continuous archiving, the base backup is the starting point and subsequent WAL advances it to the required state.

Typical procedure:

1. Restore the base backup.
2. Configure `restore_command` to retrieve WAL from the archive.
3. If necessary, set a recovery target.
4. Create `recovery.signal`.
5. Start the server.

```conf
restore_command = 'cp /archive/%f %p'
```

During recovery, the server first uses WAL already available in `pg_wal`, and requests missing segments through `restore_command`. If no recovery target is specified, all available records are applied. PITR makes it possible to stop replay at a selected time or another supported target.

After managed recovery is complete, the server switches to normal operation and can become a new primary server.

## Practical reference

```bash
# base backup
pg_basebackup -h primary -D /backup/base -Fp -Xs -P

# streaming archive
pg_receivewal -h primary -D /archive -S archive_slot

# recovery
# deploy /backup/base
# configure restore_command
touch recovery.signal
```

---

# What to remember

```text
• a physical backup covers the entire cluster and depends on server compatibility
• a hot base backup requires WAL covering the backup interval
• pg_basebackup and pg_receivewal use the replication protocol and wal sender processes
• a slot prevents removal of WAL that a client still needs
• archive_command sends completed WAL to the archive and must be retried after failure
• pg_receivewal reduces delay and writes the current segment as .partial
• restore_command retrieves WAL during recovery, while recovery.signal enables managed recovery
• base backup + continuous WAL enables PITR
```

Before the test, be able to reconstruct the complete path mentally: **base backup → WAL preservation → recovery → target point**.
