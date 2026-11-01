# 11. Low-Level Data Storage

## 1. Topic

A PostgreSQL object that stores data is not necessarily represented by a single file on disk. A relation has several **forks**, and a large fork is split into file segments.

The main model for this lesson:

```text
relation
├─ main   → primary data
├─ fsm    → free space map
├─ vm     → visibility map
└─ init   → initial state of an UNLOGGED relation

each fork
→ one or more file segments
```

A separate problem is values that make a row version too large to fit on a single page. PostgreSQL uses TOAST for them.

---

# 2. Relation Files and Segments

## A Fork and Its Files

For tables, indexes, sequences, and materialized views, PostgreSQL stores data in relations. A relation can have several forks, each containing a different kind of data.

At first, a fork is represented by one file. The name of the main file is the relation's numeric identifier; additional forks get a suffix.

When a fork file reaches its size limit, PostgreSQL creates the next segment:

```text
NNN
NNN.1
NNN.2
...
```

In a standard build, one segment is 1 GB. A different size can only be selected when building PostgreSQL from source.

The size of an individual fork is shown by:

```sql
SELECT pg_relation_size('relation_name');
```

Therefore, one database object can physically correspond to several files.

---

# 3. Relation Forks

## `main`

The main fork contains the object's actual data:

```text
table
→ row versions

index
→ index entries
```

The main fork exists for all objects considered here that physically store data.

## `fsm`

The free space map stores information about free space.

The file has the suffix:

```text
_fsm
```

FSM stores information about available space and helps find pages suitable for placing new data. This fork can exist for both tables and indexes.

## `vm`

The visibility map stores visibility information.

The file has the suffix:

```text
_vm
```

In the model used in this lesson, this fork belongs to tables. The purpose of the visibility map was already discussed in the VACUUM topic: it stores service flags describing page visibility.

## `init`

The initialization fork has the suffix:

```text
_init
```

It exists only for unlogged (`UNLOGGED`) tables and their indexes.

Changes to UNLOGGED relations are not protected by normal WAL. After a crash, PostgreSQL does not try to restore their previous contents: the main fork is replaced with a copy of the `init` fork, so the relation returns to its empty initial state.

```text
UNLOGGED relation
      ↓
main — working data
init — empty initial state

crash
      ↓
main is restored from init
```

---

# 4. TOAST

## Why TOAST Is Needed

A PostgreSQL row version must fit on one page. If individual values make the row too large, PostgreSQL can:

```text
compress the value
or
move the value out of the main row
or
compress first, then move it out if necessary
```

This mechanism is called **TOAST — The Oversized-Attribute Storage Technique**.

## TOAST Table

When necessary, a separate service TOAST table and its index are created for the main table.

A regular TOAST table is located in the schema:

```text
pg_toast
```

For temporary tables, the corresponding temporary schema `pg_toast_temp_N` is used.

A long value is stored in the TOAST table in chunks, each small enough to fit on a page. PostgreSQL reconstructs the value from these chunks transparently for the application.

```text
main row
      ↓ reference
TOAST table
├─ fragment 1
├─ fragment 2
└─ fragment ...
```

The TOAST table is read only when the out-of-line long field is actually needed.

## Independent Versioning

A TOAST table has its own versioning. If `UPDATE` creates a new version of the main row but the long field does not change, the new row version can continue referencing the existing TOAST value.

This avoids copying a large object unnecessarily.

---

# 5. Relation Sizes

The lesson distinguishes several levels of size measurement.

```text
pg_relation_size
→ size of an individual relation fork

pg_table_size
→ table + its TOAST part
→ excluding regular indexes of the main table

pg_indexes_size
→ total size of the table's regular indexes

pg_total_relation_size
→ full size of the table including indexes
```

`pg_table_size` can also be applied to an individual index because tables and indexes are both relations.

---

# 6. Practice

The main exercise connects the logical structure of a relation with actual files:

```text
create an UNLOGGED table
      ↓
find its init fork
      ↓
verify the presence of the _init file
```

For TOAST, the exercise suggests:

```text
create a table with text
      ↓
inspect its storage strategy
      ↓
change it to external
      ↓
insert short and long values
      ↓
inspect the contents of the TOAST table
```

The additional practice also compares the size of a database with the summed size of its tables and explores the available TOAST compression methods (`pglz` and `lz4`).

---

# What to Remember

```text
• a relation consists of one or more forks
• a large fork is split into file segments; the standard segment size is 1 GB
• main stores the object's data
• fsm stores free-space information
• vm stores the table's visibility map
• init is used by UNLOGGED relations to reset to their initial state after a crash
• a row version must fit on a page; large values are handled by TOAST
• TOAST can compress values and/or move them to a separate service table
• pg_relation_size, pg_table_size, pg_indexes_size, and pg_total_relation_size measure different levels of physical size
```
