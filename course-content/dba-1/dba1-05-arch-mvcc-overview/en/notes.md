# 5. MVCC and Isolation

## 1. Topic

MVCC allows multiple transactions to work with data concurrently by seeing appropriate row versions instead of globally locking reads and writes. A data snapshot and visibility rules determine which versions are visible to a particular statement or transaction.

Core model:

```text
UPDATE of a row
    ↓
old version + new version
    ↓             ↓
visibility is determined by snapshot
    ↓
VACUUM later removes versions
that are no longer needed by anyone
```

---

# 2. Mechanism

## Row versions and snapshots

The material defines multiversioning as the existence of multiple versions of the same row. The course associates the lifetime of a row version with transaction identifiers, which are assigned in increasing order. On `INSERT`, a version is marked with the identifier of the transaction that created it; on `DELETE`, with the identifier of the transaction that deleted it; conceptually, `UPDATE` consists of deleting the old version and creating a new one.

A snapshot is a consistent view of data that determines which row versions a transaction can see. In the simplified model used in the lesson, a snapshot stores the number of the last committed transaction at the time the snapshot is created and the list of transactions that were still active at that moment. This makes it possible to hide uncommitted changes and choose the appropriate version of each row without physically copying the entire database.

Isolation levels differ, among other things, in when a snapshot is created and how long it lives. At Read Committed, a new snapshot is built for each SQL statement. Because the snapshot is built at the start of each statement, a later query can see newly committed data.

The material describes this as creation of a snapshot at the start of a transaction, more precisely when its first statement is executed. Repeatable Read keeps one snapshot throughout the transaction, which makes it suitable, for example, for multi-step reports. MVCC leaves historical row versions behind; when no transaction can need them anymore, they should be cleaned up.

An old transaction may still need old versions, so the cleanup horizon cannot advance beyond it. Accumulation of obsolete versions increases the size of tables and indexes and makes work with current versions less efficient. One advantage of multiversioning is the reduced need for locking, which improves concurrency.

Ordinary reads under MVCC do not need to lock rows against reading or modification. The course highlights this as one of the key consequences of MVCC.

## Isolation levels

Read Uncommitted does not have separate semantics in PostgreSQL and behaves like Read Committed. The course explicitly identifies Read Committed as PostgreSQL's default isolation level. Serializable provides the strictest isolation, but the price is that transactions can be aborted and must be retried by the application.

## Practical reference

```bash
-- Session A
BEGIN ISOLATION LEVEL REPEATABLE READ;
SELECT * FROM accounts WHERE id = 1;

-- Session B modifies and commits the row
UPDATE accounts SET balance = balance + 10 WHERE id = 1;
COMMIT;

-- Session A continues to see data from its snapshot
SELECT * FROM accounts WHERE id = 1;
```

---

# 3. Important Details

## Concurrent changes

Modifying a row conflicts with other modifications of that same row, but does not prevent an ordinary read. A conflict occurs between concurrent changes to one row: the second writer must wait until the first writer's situation is resolved. In addition to row locks, there are higher-level table locks. They can, for example, prevent a table from being altered or dropped while an incompatible operation is in progress; the strongest modes can even prevent ordinary reads during some rebuild or physical-movement operations.

The material associates the normal lifetime of locks with a transaction; when rolling back to a savepoint, locks acquired after that savepoint can also be released.


## Transaction status

To determine whether a version is visible, the server needs to know whether the transaction that created or deleted it finished and how it finished. The material describes the **commit log (clog)**: only a small status needs to be stored for each transaction — whether it is active, committed, or aborted.

Statuses are stored in service files under `PGDATA/pg_xact` and cached in shared memory during operation. On `COMMIT` or `ROLLBACK`, the server changes the transaction status; it does not need to physically "undo" all row versions already written to pages. Versions created by an aborted transaction remain in pages until cleanup, but other transactions ignore them based on transaction status.

## Connection to VACUUM

The material mentions both automatic background cleanup and the possibility of running it manually with VACUUM.

---

# What to Remember

```text
• UPDATE usually creates a new row version rather than rewriting the row "in place" as the only version
• a snapshot determines version visibility and accounts for transactions active when the snapshot was created
• `READ COMMITTED` gets a new snapshot for each statement; `REPEATABLE READ` keeps a consistent transaction snapshot
• concurrent changes to the same row still require locking
• long-running transactions can retain old versions and prevent cleanup
• transaction status is stored separately; aborting a transaction does not require physically restoring pages to their previous state
```

Before the test, it is useful not just to recognize command names, but to be able to explain **what problem the mechanism solves, at what level it operates, and what constraints it has**.
