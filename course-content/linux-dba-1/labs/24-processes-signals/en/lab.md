# Lab 24 — PostgreSQL processes + signals

**Time: 1.5 hours**

Goal:

> Learn how to find PostgreSQL processes in Linux, map a Linux PID to a PostgreSQL backend, and understand what happens when signals are sent.

This lab uses:

```text
ps
pgrep
pstree
kill
pg_backend_pid()
pg_stat_activity
pg_cancel_backend()
pg_terminate_backend()
```

## 1. What is a process

In Linux, every running process has a PID — Process ID.

Check the PID of the current shell:

```bash
echo $$
```

`$$` is the PID of the current shell. See `man bash`, then search for `/$$`.

## 2. `ps`

Start with:

```bash
ps
```

Then:

```bash
ps -ef
```

- `-e` — show all processes;
- `-f` — full-format listing.

You can verify this with `ps --help all` or `man ps`.

Find PostgreSQL:

```bash
ps -ef | grep postgres
```

You will usually see the main server process and PostgreSQL background processes.

## 3. `ps aux`

```bash
ps aux
```

Historically, `ps` supports several syntax styles:

```text
UNIX style       ps -ef
BSD style        ps aux
GNU long options ps --something
```

In practice, in `ps aux`:

```text
a → processes for all users
u → user-oriented format
x → processes without a controlling terminal
```

Compare:

```bash
ps aux | grep postgres
ps -ef | grep postgres
```

## 4. `pgrep`

Instead of `ps | grep`:

```bash
pgrep postgres
pgrep -a postgres
```

`-a` shows the PID together with the full command line.

## 5. Process hierarchy

```bash
pstree -p
```

`-p` shows PIDs next to processes. Find PostgreSQL and inspect how the main process relates to its child processes.

## 6. Map a PostgreSQL backend to a Linux PID

Open `psql`:

```bash
sudo -u postgres psql
```

Run:

```sql
SELECT pg_backend_pid();
```

Find the returned PID in Linux:

```bash
ps -fp PID
```

Here `-f` means full format, and `-p` selects a specific PID.

Model:

```text
PostgreSQL session
→ backend PID
→ Linux process
```

## 7. `pg_stat_activity`

```sql
SELECT pid, usename, datname, state, query
FROM pg_stat_activity;
```

Find your session and compare its PID with `pg_backend_pid()`.

Create a long-running query:

```sql
SELECT pg_sleep(300);
```

In another session:

```sql
SELECT pid, usename, state, query
FROM pg_stat_activity
WHERE query LIKE '%pg_sleep%';
```

## 8. `kill` and signals

`kill` does not necessarily "kill" a process. It sends a signal:

```bash
kill -SIGNAL PID
```

List available signals:

```bash
kill -l
```

### SIGINT

Start `pg_sleep(300)`, find the PID, and from Linux run:

```bash
kill -INT PID
```

Usually the current query is interrupted while the backend remains alive. Verify using the same connection:

```sql
SELECT 1;
```

### SIGTERM

Find the backend PID:

```sql
SELECT pg_backend_pid();
```

From Linux:

```bash
kill -TERM PID
```

For an individual PostgreSQL backend, this terminates the backend process; the client session loses its connection.

## 9. The same operations through PostgreSQL

Cancel the backend's current work:

```sql
SELECT pg_cancel_backend(PID);
```

Terminate the backend/session:

```sql
SELECT pg_terminate_backend(PID);
```

The key distinction:

```text
cancel ≠ terminate
```

## 10. About `kill -9`

```bash
kill -KILL PID
# or
kill -9 PID
```

SIGKILL cannot be handled or ignored. For PostgreSQL, it is not the standard way to manage processes and is not used as a normal operation in this lab.

## What you should understand

```text
PostgreSQL session
↓
pg_stat_activity.pid
↓
Linux PID
↓
signal
```

## Mini-exam 24

1. How do you show all PostgreSQL processes with `ps`?
2. How do you show PostgreSQL processes with `pgrep` together with their command lines?
3. How do you display the process tree with PIDs?
4. How do you find the PID of the current PostgreSQL session?
5. How do you find that PID in Linux?
6. How do you find the current query and PID through PostgreSQL?
7. How do you send SIGINT to a backend?
8. How do you send SIGTERM?
9. What is the difference between `pg_cancel_backend()` and `pg_terminate_backend()`?
10. Why should `kill -9` not be used as a normal way to work with PostgreSQL?
