# 13. Logging and Log Analysis

## 1. Topic

The server log is the second major source of information about PostgreSQL operation after statistics views. It is needed to investigate errors, connections, locks, slow queries, checkpoints, and other events.

The main model:

```text
server processes
      ↓
what to log
      ↓
log_destination
stderr / csvlog / jsonlog / syslog / eventlog
      ↓
logging_collector   (for stderr/csvlog/jsonlog files)
      ↓
log_directory / log_filename
      ↓
rotation
      ↓
grep / awk / pgBadger
```

---

# 2. Where Messages Go

## `log_destination`

`log_destination` accepts one or more comma-separated destinations:

```text
stderr   — standard error stream, the default
csvlog   — structured CSV
jsonlog  — structured JSON
syslog   — Unix system syslog
 eventlog — Windows event log
```

`csvlog` and `jsonlog` are intended to be used with the logging collector enabled.

## `logging_collector`

`logging_collector = on` starts a separate process that collects messages from server processes and writes `stderr`, `csvlog`, and `jsonlog` to files.

Key properties from the material:

- messages should not be lost;
- therefore, under very heavy load the collector itself can become a bottleneck;
- files are created in `log_directory` with names determined by `log_filename`.

`syslog` and `eventlog` are separate destinations and do not require writing through the collector into PostgreSQL-managed files.

---

# 3. What to Log

The material emphasizes that "log everything" is expensive: logs consume disk space and create I/O. Required events should be selected deliberately.

The main parameters from the lesson:

| Parameter | Purpose |
|---|---|
| `log_min_messages` | minimum server message level |
| `log_min_duration_statement` | log statements whose duration is at least the specified threshold |
| `log_duration` | log the duration of executed statements |
| `log_checkpoints` | checkpoint events |
| `log_connections`, `log_disconnections` | connections and disconnections |
| `log_lock_waits` | long lock waits |
| `log_statement` | selected classes of SQL statements |
| `log_temp_files` | temporary-file usage |

`log_statement` supports `none`, `ddl`, `mod`, and `all`. For finding slow queries, targeted `log_min_duration_statement` is usually more useful than unconditional logging of every SQL statement.

## Log-Line Context

`log_line_prefix` adds context: time, PID/session identifier, user, database, application, and other fields. For example:

```conf
log_line_prefix = '%m [%p] %q%u@%d/%a '
```

A PID or session identifier is especially useful for correlating a log entry with `pg_stat_activity` and nearby messages from the same backend.

---

# 4. Files and Rotation

`log_directory` defines the directory, while `log_filename` defines the file name or name pattern. Built-in rotation uses:

```text
log_filename
log_rotation_age
log_rotation_size
log_truncate_on_rotation
```

Examples from the lesson:

```text
postgresql-%H.log + 1h → 24 file names by hour
postgresql-%a.log + 1d → 7 file names by day of week
```

`log_truncate_on_rotation = on` allows an existing file to be overwritten when rotation is triggered by time and the same name appears again.

An alternative to built-in rotation is the system `logrotate` utility. For the Ubuntu package installation, the material points to `/etc/logrotate.d/postgresql-common`.

A log must not be treated as a harmless technical file: it can contain SQL text, parameters, and other sensitive data, so access permissions and retention policy are part of operations.

---

# 5. Log Analysis

Basic tools:

```text
grep
awk
```

For specialized analysis, the material uses **pgBadger**. It builds reports from PostgreSQL logs, but the quality of the result depends on logging configuration.

The original lesson separately notes that messages for pgBadger should be in English. The pgBadger documentation also requires a recognizable `log_line_prefix`; for stderr it must include at least the time and process/session identifier. For a useful report, the relevant events are normally enabled (`log_checkpoints`, connections, lock waits, temporary files, and so on).

Do not blindly enable maximum logging with every parameter at once: this inflates the log and can distort analysis.

---

# 6. Practical Reference

```sql
SHOW log_destination;
SHOW logging_collector;
SHOW log_directory;
SHOW log_filename;
SHOW log_line_prefix;
SHOW log_min_duration_statement;

ALTER SYSTEM SET log_min_duration_statement = '500ms';
SELECT pg_reload_conf();
```

For a lab diagnostic exercise, it is useful to create a controlled deadlock and inspect the messages written by the server to the log.

---

# What to Remember

```text
• `log_destination` selects message destinations
• `logging_collector` collects stderr/csvlog/jsonlog into managed files
• `log_*` parameters determine not only where the log goes, but also what it contains
• `log_line_prefix` provides context needed to correlate events
• rotation limits file growth; an external alternative is logrotate
• logs can contain sensitive data and must be protected
• pgBadger requires a suitable log format and English-language messages
```
