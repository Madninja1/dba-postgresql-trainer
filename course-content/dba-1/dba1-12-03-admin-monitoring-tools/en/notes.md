# 14. Monitoring Tools and Query Statistics

## 1. Topic

A one-time look at `pg_stat_activity` answers the question "what is happening now." Full monitoring must continuously collect metrics, retain history, build graphs, and alert when values leave acceptable ranges.

For SQL workload analysis inside PostgreSQL, `pg_stat_statements` is used: the extension aggregates statistics from many executions of structurally identical queries.

The main model:

```text
PostgreSQL + OS metrics
        ↓
external monitoring
        ↓
history → dashboards → alerts

SQL executions
        ↓
pg_stat_statements
        ↓
queryid / calls / time / rows / I/O
```

---

# 2. External Monitoring

The material divides solutions into several groups.

General-purpose systems:

```text
Zabbix
Munin
Cacti
```

Cloud services:

```text
Okmeter
NewRelic
Datadog
```

PostgreSQL-oriented tools:

```text
pg_profile
pgpro_pwr
PGObserver
PoWA
OPM
```

PostgreSQL exposes metrics, but by itself it is not a complete monitoring system. A practical system should:

- collect PostgreSQL and OS metrics;
- retain history;
- show graphs and trends;
- generate alerts;
- allow time intervals to be compared rather than showing only the current snapshot.

`pg_profile` builds snapshots of cumulative statistics and allows intervals to be compared. `pgpro_pwr` is an advanced Postgres Pro Enterprise solution for workload reporting.

---

# 3. `pg_stat_statements`

## Enabling the Module

The module requires additional shared memory, so it must be added to `shared_preload_libraries` and the server must be restarted:

```conf
shared_preload_libraries = 'pg_stat_statements'
```

After the module is loaded, the extension object is created in the required database:

```sql
CREATE EXTENSION pg_stat_statements;
```

In PostgreSQL 16, built-in query identifier calculation is controlled by `compute_query_id`; with a normal `auto`/`on` configuration it makes `queryid` available.

## Normalization and `queryid`

`pg_stat_statements` does not create a separate row for every SQL execution. Queries with the same internal structure are combined into one entry. Constant values normally do not participate in grouping and are replaced with parameters in the displayed text:

```sql
SELECT * FROM orders WHERE id = 10;
SELECT * FROM orders WHERE id = 20;

-- the aggregated representation can look like
SELECT * FROM orders WHERE id = $1;
```

`queryid` is a hash of the query's internal structure. It is a useful identifier within a particular installation and version, but it must not be treated as a universal permanent ID across arbitrary servers and major versions.

## Main Metrics

Useful columns include:

```text
queryid
query
calls
plans
total_plan_time / mean_plan_time
total_exec_time / mean_exec_time
rows
shared_blks_hit / shared_blks_read
...
```

When looking for queries that are expensive in total, `total_exec_time` is often more important than average duration alone: a frequent moderately slow query can create more load than a rare very slow one.

Example:

```sql
SELECT queryid,
       calls,
       total_exec_time,
       mean_exec_time,
       rows,
       query
FROM pg_stat_statements
ORDER BY total_exec_time DESC
LIMIT 20;
```

---

# 4. Collection Scope and Statistics Reset

`pg_stat_statements.track` determines which statements are tracked:

```text
top   → only top-level statements (default)
all   → also nested statements, for example inside functions
none  → statistics collection disabled
```

`pg_stat_statements.track_utility` controls tracking of utility commands, while `pg_stat_statements.track_planning` controls planning statistics. Enabling additional collection can have a cost.

Statistics can be reset with:

```sql
SELECT pg_stat_statements_reset();
```

The function also allows the reset to be limited to a particular user, database, and `queryid`. Therefore, when analyzing data it is always important to know **from what point in time statistics have been collected**. In `pg_stat_statements_info`, the `stats_reset` field shows the time of the last global reset.

---

# 5. Practical Reference

```conf
# postgresql.conf
shared_preload_libraries = 'pg_stat_statements'
```

```sql
CREATE EXTENSION pg_stat_statements;

-- execute several queries, then:
SELECT queryid, calls, total_exec_time, mean_exec_time, rows, query
FROM pg_stat_statements
ORDER BY total_exec_time DESC;
```

---

# What to Remember

```text
• full monitoring means history, visualization, and alerts, not just a current snapshot
• external monitoring combines PostgreSQL and OS metrics
• `pg_stat_statements` aggregates statistics for structurally identical SQL queries
• the module requires `shared_preload_libraries` and a server restart
• `queryid` identifies query structure, while constants are normally normalized
• `top`, `all`, and `none` define the scope of `pg_stat_statements.track`
• after `pg_stat_statements_reset()`, a new analysis period starts from zero
```
