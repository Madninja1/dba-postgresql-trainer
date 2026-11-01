# PostgreSQL Processes and Signals

## 1. Topic

Each PostgreSQL client connection is handled by a separate backend process. Therefore, a DBA must be able to connect three views of the same activity:

```text
PostgreSQL client session
        ↓
backend PID
        ↓
Linux process
```

This relationship is the basis for diagnosing stuck queries, locating the responsible process, and stopping work safely.

Main tools:

```text
ps / pgrep / pstree
pg_backend_pid()
pg_stat_activity
kill
pg_cancel_backend()
pg_terminate_backend()
```

---

## 2. Mechanism

### Process PID

A process is a running instance of a program to which the kernel assigns a PID.

You can see the PID of the current shell with:

```bash
echo $$
```

`$$` is a special shell parameter containing the PID of the shell process itself.

---

### Viewing processes with `ps`

Classic form:

```bash
ps -ef
```

In this form:

```text
-e → all processes
-f → full/extended format
```

`ps` historically supports multiple option styles. That is why this form is also common:

```bash
ps aux
```

It belongs to the BSD-style syntax, so the lack of a leading `-` before `aux` is expected.

PostgreSQL processes can be found through a pipeline:

```bash
ps -ef | grep postgres
```

However, this often includes the `grep` process itself. A more direct tool is:

```bash
pgrep -a postgres
```

`pgrep` searches processes by name, while `-a` prints the PID together with the full command line of each matching process.

---

### Process tree

PostgreSQL is not a single isolated PID. The server process has child processes with different responsibilities.

View the hierarchy:

```bash
pstree -p
```

`-p` adds PIDs. This helps you see parent-child relationships and understand how an individual backend relates to the main server.

---

### PID of the current PostgreSQL session

Inside SQL:

```sql
SELECT pg_backend_pid();
```

The function returns the PID of the backend process serving the current connection.

You can then locate that PID at the Linux level:

```bash
ps -fp <PID>
```

And inside the database:

```sql
SELECT pid, usename, datname, state, query
FROM pg_stat_activity
WHERE pid = <PID>;
```

The relationship is:

```text
pg_backend_pid()
      ↓
pg_stat_activity.pid
      ↓
Linux PID
```

`pg_stat_activity` lets you see who is connected, to which database, what state the backend is in, and what query it is currently executing.

---

### Observing a long-running query

A convenient training target is:

```sql
SELECT pg_sleep(300);
```

The query remains active for a long time, so it is easy to find through `pg_stat_activity`, map to a Linux process, and then cancel.

---

### Linux signals

The command:

```bash
kill PID
```

does not necessarily mean “kill the process immediately.” `kill` sends a signal to the process.

List signals:

```bash
kill -l
```

Three signals matter in this topic:

```text
SIGINT  → interrupt the current work
SIGTERM → orderly process termination
SIGKILL → forced termination by the kernel
```

#### SIGINT

For a PostgreSQL backend during an active query:

```bash
kill -INT <PID>
```

In the lab, this is used to cancel the current query. After the cancellation error, the connection normally remains alive and can be checked with:

```sql
SELECT 1;
```

Model:

```text
SIGINT
→ cancel current work
→ backend/session remains alive
```

#### SIGTERM

```bash
kill -TERM <PID>
```

For an individual backend, this terminates the process and causes the corresponding client connection to be lost.

```text
SIGTERM
→ terminate backend
→ connection disappears
```

#### SIGKILL

```bash
kill -KILL <PID>
# or
kill -9 <PID>
```

SIGKILL cannot be caught or handled by a user-space process. It is an emergency mechanism, not the normal way to manage PostgreSQL. For routine DBA work, use PostgreSQL's own tools or orderly signals first.

---

### The same operations through PostgreSQL

To cancel the current work of a backend:

```sql
SELECT pg_cancel_backend(<PID>);
```

To terminate a session:

```sql
SELECT pg_terminate_backend(<PID>);
```

The distinction is fundamental:

```text
pg_cancel_backend
→ cancels the current query
→ the connection may continue working

pg_terminate_backend
→ terminates the backend
→ the connection is lost
```

These are SQL-level equivalents of two different administrative intentions: “stop the query” and “close the session.”

---

## 3. Important details

### Do not confuse a query with a session

A long-running or bad query does not always require killing the connection. If the problem is only the current work, cancellation is preferable.

```text
bad query
→ pg_cancel_backend()

unwanted/stuck session
→ pg_terminate_backend()
```

### Verify the PID before acting

Operating systems reuse PIDs over time. Before sending a signal, make sure the PID still belongs to the intended backend:

```bash
ps -fp <PID>
```

and compare it with `pg_stat_activity`.

### Linux and PostgreSQL provide different context

Linux sees the process and its resources. PostgreSQL sees the database user, database, state, and SQL query. Good diagnostics use both levels.

### `pgrep -a` is cleaner than plain `grep`

`ps | grep` is useful as a general technique, but `pgrep -a postgres` gives a cleaner list of PIDs and command lines without an extra `grep` line.

---

## What to remember

```bash
echo $$
```

→ PID of the current shell.

```bash
ps -ef
ps aux
pgrep -a postgres
pstree -p
```

→ different ways to inspect processes and their hierarchy.

```sql
SELECT pg_backend_pid();
SELECT pid, usename, datname, state, query
FROM pg_stat_activity;
```

→ map a PostgreSQL session to a Linux PID.

```text
SIGINT  → cancel current work
SIGTERM → terminate backend
SIGKILL → hard emergency termination
```

```text
pg_cancel_backend()    → cancel query
pg_terminate_backend() → terminate session
```

Main model:

```text
session → backend PID → Linux process
```
