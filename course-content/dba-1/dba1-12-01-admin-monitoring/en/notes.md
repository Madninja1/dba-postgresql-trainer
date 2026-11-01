# 12. PostgreSQL Monitoring

## 1. Topic

PostgreSQL monitoring starts with two levels of observation:

- the operating system shows processes, CPU, memory, I/O, and free space;
- PostgreSQL itself shows current sessions, waits, cumulative counters, and progress of long-running operations.

The main model:

```text
OS tools
ps / pgrep / top / iostat / vmstat / sar / df / du
                    ↕ PID
PostgreSQL
pg_stat_activity
pg_stat_* counters
pg_stat_progress_*
```

One source does not replace the other. High disk load is visible from outside, while the specific backend, query, or wait is visible from inside PostgreSQL.

---

# 2. Mechanism

## OS Processes and Resources

The material uses `ps` and `pgrep` to inspect processes. PostgreSQL can change its process title: with `update_process_title` enabled, `ps` output can show the purpose and current state of a server process. The `cluster_name` parameter helps distinguish PostgreSQL instances from one another.

For OS resources, the material lists:

```text
CPU / I/O / memory: iostat, vmstat, sar, top
used space:         du
free space:         df
OS quotas:          quota
```

PostgreSQL does not provide disk quotas for individual roles: server processes normally run under one OS user, so free space and quotas must be monitored at the OS and filesystem level.

## Statistics Collection Settings

The amount of information collected is controlled by server parameters:

| Parameter | What It Enables |
|---|---|
| `track_activities` | current commands and activity of server processes |
| `track_counts` | cumulative statistics for accesses to tables and indexes |
| `track_functions` | statistics for calls to user-defined functions |
| `track_io_timing` | timing of block I/O operations |
| `track_wal_io_timing` | timing of WAL I/O operations |

The material notes that `track_functions`, `track_io_timing`, and `track_wal_io_timing` are disabled by default: additional statistics have their own cost.

## `pg_stat_activity`

`pg_stat_activity` shows backend and background processes and their current activity. The following columns are especially useful for diagnostics:

```sql
SELECT pid,
       usename,
       datname,
       state,
       wait_event_type,
       wait_event,
       query_start,
       xact_start,
       query
FROM pg_stat_activity;
```

This is current activity, not cumulative counters. Information about the current query collected by `track_activities` differs from ordinary cumulative statistics because it must reflect the process's current state.

## Cumulative Statistics

Server processes do not send accumulated statistics to shared memory after every individual event. In PostgreSQL 16, updates are sent before a process goes idle and no more often than `PGSTAT_MIN_INTERVAL` — normally about one second.

During a clean shutdown, cumulative statistics are saved in `PGDATA/pg_stat/` and loaded at the next startup. After a crash, such counters can be lost.

For tables, you can analyze, for example:

```text
n_tup_ins   — inserted rows
n_tup_del   — deleted rows
n_live_tup  — estimated live rows
n_dead_tup  — estimated dead row versions
```

These are the values used in the lesson's practical exercise: perform `INSERT`, `DELETE`, then `VACUUM` and compare the statistics.

## Statistics Snapshot Consistency

`stats_fetch_consistency` controls caching of cumulative statistics within the current transaction:

```text
none      → do not cache previously fetched values
cache     → cache data for individual requested objects
snapshot  → use a single statistics snapshot for the current database
```

The default is `cache`. Cached values live until the end of the transaction. The function:

```sql
SELECT pg_stat_clear_snapshot();
```

clears the current statistics snapshot/cache, so the next access obtains fresh data according to the selected mode.

---

# 3. Progress of Long-Running Operations

For several potentially long-running commands, PostgreSQL provides dedicated `pg_stat_progress_*` views:

| Operation | View |
|---|---|
| `ANALYZE` | `pg_stat_progress_analyze` |
| `CREATE INDEX`, `REINDEX` | `pg_stat_progress_create_index` |
| `VACUUM` / autovacuum | `pg_stat_progress_vacuum` |
| `CLUSTER`, `VACUUM FULL` | `pg_stat_progress_cluster` |
| base backup | `pg_stat_progress_basebackup` |
| `COPY` | `pg_stat_progress_copy` |

An ordinary long-running `SELECT` does not automatically get a separate `pg_stat_progress_*` view.

## Additional Statistics

The DBA1 module also shows extensions that complement the built-in views:

```text
pg_stat_statements → SQL query statistics
pgstattuple         → row-version statistics
pg_buffercache      → buffer cache contents
pg_wait_sampling    → wait statistics
pg_stat_kcache      → CPU and I/O
pg_qualstats        → predicate statistics
```

`pg_stat_statements` is covered in detail later in a separate block of this topic.

---

# 4. Practical Reference

```sql
-- current activity
SELECT pid, usename, datname, state, wait_event_type, wait_event, query
FROM pg_stat_activity
WHERE pid <> pg_backend_pid();

-- database statistics
SELECT *
FROM pg_stat_database
WHERE datname = current_database();

-- table statistics
SELECT relname, n_tup_ins, n_tup_del, n_live_tup, n_dead_tup
FROM pg_stat_user_tables;

-- VACUUM progress
SELECT * FROM pg_stat_progress_vacuum;
```

---

# What to Remember

```text
• PostgreSQL monitoring always combines the OS level with internal DBMS views
• `track_*` parameters determine which statistics the server collects at all
• `pg_stat_activity` shows current activity and waits
• cumulative statistics are updated with a delay and can be cached within a transaction
• `pg_stat_clear_snapshot()` lets you discard the current statistics cache
• `pg_stat_progress_*` exists only for a supported set of long-running operations
• statistics should be compared over time; one snapshot is not the whole picture
```
