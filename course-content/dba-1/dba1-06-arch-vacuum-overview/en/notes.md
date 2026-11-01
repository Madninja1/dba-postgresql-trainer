# 6. Vacuuming and Maintenance

## 1. Topic

MVCC leaves old row versions behind after changes. While transaction snapshots still need them, they cannot be removed; later they become dead. If they are not maintained, tables and indexes will bloat, statistics will become stale, and old transaction identifiers will create a wraparound risk.

Therefore VACUUM is not a single operation but part of a set of periodic maintenance tasks:

```text
removing dead row versions
        +
visibility map
        +
free space map
        +
ANALYZE
        +
freezing old XIDs
```

---

# 2. Mechanism

## Regular `VACUUM`

Vacuuming removes dead row versions from tables and obsolete index entries. Without a table list, the command processes all tables in the current database that the user has permission to vacuum; specific tables can be named when needed. The freed space becomes available for reuse, but the table file usually **does not physically shrink**. Regular VACUUM can run concurrently with ordinary reads and writes, but `VACUUM` cannot be executed inside a transaction block.

```sql
VACUUM table_name;
```

For routine maintenance this is normal behavior: the goal is to make internal space reusable for new row versions, not to continuously rewrite the file.

## Visibility map

The visibility map marks table pages on which all row versions are visible to all snapshots.

It is used for two important optimizations:

```text
VACUUM
→ can skip pages that are known to be fully visible

index-only scan
→ can avoid reading the table page to check visibility
  if the required page is marked all-visible
```

The visibility map exists for tables, not indexes.

## Free space map

The free space map stores information about free space inside pages. When inserting a new row version, PostgreSQL can quickly find a page where it will fit.

```text
vacuuming
→ more free space
→ FSM is updated
→ the next insert finds a suitable page faster
```

FSM exists for both tables and indexes.

## `ANALYZE`

The planner needs statistics about the data: row counts and value distributions. `ANALYZE` collects them from a sample of rows and updates planner statistics.

```sql
ANALYZE table_name;
VACUUM ANALYZE table_name;
```

`VACUUM ANALYZE` first vacuums and then runs `ANALYZE`. Planner statistics are stored in the `pg_statistic` system catalog; `ANALYZE` can be limited to specific tables and even individual columns.

## XID freezing

A regular PostgreSQL XID has a 32-bit range and wraps around cyclically. Very old row versions must be **frozen** so that their visibility no longer depends on the original transaction number and they are treated as visible to all relevant snapshots instead of appearing to come from "future" transactions after wraparound.

VACUUM freezes old row versions; the visibility map can also mark pages as all-frozen so later freeze work can skip them when possible.

If freezing is neglected for too long, PostgreSQL eventually has to restrict normal work to prevent loss of data correctness.

## Autovacuum

Routine maintenance is normally handled by the autovacuum subsystem.

```text
autovacuum launcher
→ chooses databases to maintain

        ↓

autovacuum worker
→ services a particular database / objects
```

Autovacuum reacts to the volume of changes and automatically launches VACUUM/ANALYZE. The `autovacuum` and `track_counts` settings are important for it; disabling either one disables the normal automatic vacuuming mechanism.

Manual commands remain useful:

```sql
VACUUM;
ANALYZE;
VACUUM ANALYZE;
```

---

# 3. Bloat

## Why bloat appears

Bloat is a situation where the physical size of a table or index is significantly larger than the space occupied by useful current data.

Typical causes from the course material:

```text
poor autovacuum configuration
large-scale data changes
long-running transactions
```

Consequences:

```text
wasted disk space
larger backups
slower sequential reads
less efficient index access
```

Regular VACUUM frees space **inside** the file, so the operating-system file size usually does not shrink. The material includes a query from the PostgreSQL Wiki for approximate bloat estimation, while the `pgstattuple` extension can measure the physical state of a relation directly. `pgstattuple` returns, among other things, the relation length, the number and share of live and dead tuples, and the amount and share of free space; useful diagnostic fields include `dead_tuple_count`, `dead_tuple_percent`, `free_space`, and `free_percent`. The `pgstattuple_approx` variant avoids a full scan of the table and obtains some values approximately.

## `VACUUM FULL`

To physically compact a bloated table, you can use:

```sql
VACUUM FULL table_name;
```

It rewrites the table and requires an exclusive lock, so it is much heavier than regular VACUUM and is used as a special operation rather than daily maintenance. Extra disk space is required while the rewrite is in progress.

## Indexes: `REINDEX`

An index may need to be rebuilt after bloat, corruption, or changes to certain storage parameters:

```sql
REINDEX INDEX index_name;
REINDEX TABLE table_name;
```

`REINDEX TABLE` rebuilds all indexes of the specified table. Regular `REINDEX` blocks use of the index being rebuilt and can delay table modifications. PostgreSQL also provides `REINDEX ... CONCURRENTLY`, which allows ordinary modifications to continue at the cost of a longer and more complex operation; the course material separately notes its limitations and the possibility of failure.

`pg_repack` is a third-party extension for rebuilding tables and indexes while minimizing long exclusive locks; the course presents it as a practical alternative to a heavy blocking rewrite.

For estimating object sizes in practice, functions such as these are used:

```sql
pg_table_size(...)
pg_indexes_size(...)
```

---

# What to remember

```text
• VACUUM removes dead row versions and makes space reusable
• regular VACUUM usually does not shrink the file at the OS level
• the visibility map speeds up VACUUM and index-only scans
• the free space map helps find room for new row versions
• ANALYZE updates planner statistics
• freezing protects against XID wraparound
• the autovacuum launcher starts worker processes and should run continuously
• VACUUM FULL physically rewrites the table and blocks it heavily
• REINDEX maintains bloated indexes
```
