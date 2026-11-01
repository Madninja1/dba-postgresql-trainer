# 7. Buffer Cache and WAL

## 1. Topic

PostgreSQL does not have to write every modified table page to disk immediately. Changes first reach the buffer cache, while transaction durability is provided by WAL: the log record must be safely written before the corresponding data page.

Main model:

```text
UPDATE
  ↓
shared_buffers: dirty page
  ↓
WAL record → WAL buffers → pg_wal
                   ↓ fsync at COMMIT

later checkpoint/background writer
        ↓
     data files
```

---

# 2. Mechanism

## Buffer cache

The buffer cache smooths out the speed difference between RAM and disk. After a page is loaded into the cache, PostgreSQL can access it repeatedly without reading it again through the operating system. The lecture describes the buffer cache as an array of buffers containing data pages and service information. The normal page size is 8 KB; it can be changed when PostgreSQL is built. If the data in a cached page has been modified, the corresponding buffer is dirty and must eventually be written to disk.

The lecture describes an LRU-like principle: a page that has not been used for a long time is selected; if it is dirty, it is written first, after which the buffer can be reused for another page. Buffering improves performance, but RAM contents can be lost during a crash. WAL makes it possible to replay the required changes and restore a consistent state. A checkpoint writes the relevant dirty buffers and creates a reference point from which later recovery can begin.

A checkpoint establishes a recovery boundary: all buffers that were dirty at the start of the checkpoint must be written to disk, and after the checkpoint completes, changes up to that boundary are guaranteed to be present in the data files. Therefore crash recovery only needs to replay WAL beginning with the last completed checkpoint. The checkpoint position corresponds to the start of writing accumulated dirty buffers, but the checkpoint is considered complete only after those writes finish. The lecture separates responsibilities: walwriter services WAL, checkpointer performs checkpoints, and the background writer proactively cleans some dirty buffers that are likely to be evicted. Bgwriter tries to write buffers that are likely to be replaced soon, reducing latency for backend processes.

## WAL rule

The lecture states that WAL protects tables, indexes, other WAL-logged objects, and transaction status. Changes to temporary tables are not written to WAL. The ordering "WAL first, changed data later" is what makes recovery after a crash possible. The documentation defines this as the core idea of WAL. In the lecture, WAL is presented as a stream of information used to replay lost operations; its files are stored in `pg_wal`. Temporary and unlogged tables are excluded from normal WAL protection.

Unlogged tables can be faster because they do not have normal WAL protection, but after a crash their contents are not recovered: the tables are truncated. After an abnormal shutdown, the server restores consistency starting from the last completed checkpoint and replaying the necessary WAL records. The lecture explicitly states that if a commit record did not make it into WAL, that transaction is treated as aborted after the crash.

The documentation emphasizes the sequential nature of WAL writes and the lower synchronization cost compared with writing many changed data files: a WAL record is usually smaller than a whole page, writes are sequential, and one WAL synchronization can confirm several small transactions at once. The lecture explains the use of fsync as a way to ensure that a WAL record is not left only in the OS cache and has reached nonvolatile storage. With synchronous commit, the backend waits for the relevant WAL to be safely written before confirming `COMMIT`. With asynchronous commit, confirmation can be sent earlier; walwriter handles background flushing, reducing latency at the cost of possibly losing a few recently acknowledged transactions after an OS or power failure. WAL is still used to restore database consistency after a crash.

Even if WAL records were previously flushed asynchronously, before PostgreSQL writes the corresponding data page it must ensure that the required WAL has reached disk. The amount of information recorded in WAL is controlled by `wal_level`: `minimal` keeps enough for crash recovery, `replica` adds information required for physical backup and replication, and `logical` additionally supports logical decoding of row changes. The slide on WAL levels identifies `replica` as the default level.

The immediate shutdown mode stops the server without a checkpoint. The on-disk state is similar to a crash, and at the next startup PostgreSQL restores consistency from WAL. The documentation connects WAL not only with REDO after a crash but also with physical backup and point-in-time recovery. The lecture additionally emphasizes WAL as the basis for replication.

## Checkpoints

In fast shutdown mode, the server terminates connections and performs a checkpoint. Shutdown can take longer, but crash recovery is not required at the next startup.

---

# What to remember

```text
• WAL must reach durable storage before the data page it describes
• COMMIT does not mean every modified table page is already on disk; durable WAL is enough
• a checkpoint limits how much WAL must be replayed after a crash
• `pg_wal` is a critical part of the cluster, not an ordinary human-readable log
```
