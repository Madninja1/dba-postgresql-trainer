# 4. General PostgreSQL Architecture

## 1. Topic

PostgreSQL's general architecture connects the client-server protocol, transactions, SQL processing, processes, memory, and data storage. A DBA should see not isolated commands, but the path of a request from the client to modified pages and WAL.

Core model:

```text
client
  ↓ protocol
backend process
  ↓
parser → rewriter → planner → executor
  ↓                 ↘
shared buffers       WAL
  ↓                   ↓
data files          pg_wal
```

---

# 2. Mechanism

## Client and server

The client and server must use the same communication protocol. Normally the client works through a driver that implements the PostgreSQL protocol; a driver may use libpq or implement the protocol itself. The protocol allows a client to connect to a database in the cluster; the server authenticates the client, then accepts SQL queries, executes them, and returns results. The client does not modify data files directly. ACID stands for atomicity, consistency, isolation, and durability. Backup itself is not one of the ACID properties.

As a rule, the client application determines which commands belong to a transaction and performs commit or rollback. On the server side, transactions can also be controlled by stored procedures. The system catalog contains database metadata needed, for example, to determine whether tables and other objects referenced by a query exist. After a statement is prepared, actual parameter values are supplied during the bind stage.

A connection pool should be considered when there are many clients or connections are frequently opened and closed. The pool manager accepts client connections and reuses a limited set of open PostgreSQL connections. Files are logically divided into pages (blocks); the normal page size is 8 KB. A different size can be selected only when building the server.

A page has an internal layout, contains a header and data, and may contain free space when not full. The page size is fixed for the built server, not selected per query.

## Transactions

Atomicity means "all or nothing": a transaction's changes are either committed as a whole or not committed. In the material, BEGIN marks the start of a transaction, COMMIT makes changes permanent, and ROLLBACK cancels them. The material names multiversion concurrency control as the main mechanism supporting the first three transaction properties: atomicity, consistency, and isolation.

## SQL query path

The material identifies parsing, rewriting, planning, and execution as query-processing stages. SQL is a declarative language: a query describes the required data, while the planner determines how to obtain it, for example whether an index should be used. Planning needs statistics containing information about table sizes and data distribution. A user's password is not relevant to plan selection.

A prepared statement allows parsing and rewriting to be performed in advance and the parse tree to be stored; actual parameter values are bound at execution time. The material notes that a plan can sometimes be reused and that prepared statements help protect against SQL injection. Replanning is not unconditionally excluded.

## Practical reference

```bash
BEGIN;
SAVEPOINT before_change;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
ROLLBACK TO SAVEPOINT before_change;
COMMIT;

PREPARE q(int) AS SELECT * FROM accounts WHERE id = $1;
EXECUTE q(1);
```

---

# 3. Important Details

## Prepared statements and cursors

A cursor allows a result set to be processed iteratively instead of retrieving the entire result at once. An open cursor is represented by a portal, and the query used by it is implicitly prepared.

## Processes and memory

When the server starts, the postmaster process is launched: it creates the other processes, monitors them, and listens for incoming connections. Backend local memory is used for the state of a specific serviced connection and for executing its queries. It can contain parsed queries and their plans, state of open cursors, the system-catalog cache, and working memory for sorts and joins. The shared buffer cache is located in shared memory. For each new client, postmaster starts a backend process; that process also performs authentication.

Short-lived locks are used for shared memory. For table data, PostgreSQL relies on MVCC and snapshots, trying to lock only conflicting changes. With pooling, multiple clients may share one backend over time, while the backend retains local state. This state must therefore be considered when designing applications. PostgreSQL's buffer cache is in shared memory so that different server processes can use cached pages.

After a failure, the contents of RAM can be lost together with pages that have not yet been written to disk. The journal allows lost changes to be recovered and data to be returned to a consistent state.

## Disk, WAL, and extensibility

In addition to PostgreSQL's shared buffer cache, there is an operating-system cache. Modified pages may be written to disk later, so durability requires an additional mechanism. PostgreSQL's architecture supports extensions that add types and operators, index access methods, background tasks, access to external sources, and other capabilities. The notes emphasize PostgreSQL's extensibility and point out that most extensions can be installed without restarting the server.

---

# What to Remember

```text
• each ordinary client connection is serviced by a separate backend process
• SQL passes through parsing, rewriting, planning, and execution
• a transaction defines an atomic unit of work; a savepoint creates an intermediate rollback point
• modified pages may first live in memory, while WAL provides durability
• PostgreSQL is extensible: types, functions, indexes, languages, and extensions can be added
```

Before the test, it is useful not just to recognize command names, but to be able to explain **what problem the mechanism solves, at what level it operates, and what constraints it has**.
