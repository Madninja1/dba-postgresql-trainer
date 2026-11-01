# Planner Statistics and Full-Text Search

## Mental model

This topic contains two independent subsystems.

### Planner statistics

```text
ANALYZE
  ├─ pg_statistic              → basic column statistics
  └─ pg_statistic_ext_data     → computed extended-statistics data
       ▲
       │ definition
  pg_statistic_ext
```

### Full-text search configuration

```text
text
 ↓
parser          → pg_ts_parser
 ↓ token type
configuration   → pg_ts_config
 ↓ mapping
pg_ts_config_map
 ↓
dictionary     → pg_ts_dict
 ↓
template       → pg_ts_template
```

Do not mix these two parts: the first helps the planner estimate queries, while the second describes the linguistic pipeline for text search.

---

## `pg_statistic`: basic planner statistics

`pg_statistic` contains statistics about database contents. `ANALYZE` creates these rows, and the planner then uses them to estimate cardinalities and plan costs.

These data are **inherently estimates**, even when they are fresh.

### What a row identifies

- `starelid` — table or index;
- `staattnum` — number of the described column;
- `stainherit` — whether child tables / the inheritance tree are included.

Normally a column has a row with `stainherit = false`. A table with descendants can also have another row with `stainherit = true`.

Statistics for index expressions are also stored in `pg_statistic`; in that case `starelid` points to the index.

---

## Basic numerical statistics

- `stanullfrac` — fraction of rows where the value is `NULL`;
- `stawidth` — average storage size of a non-NULL value;
- `stadistinct` — estimated number of distinct non-NULL values.

### How to read `stadistinct`

```text
> 0  → estimated absolute number of distinct values
< 0  → absolute value is a multiplier relative to row count
= 0  → number of distinct values is unknown
```

For example, `stadistinct = -0.4` means the planner estimates the number of distinct values as approximately `0.4 * number_of_rows`.

A negative value is not an error and does not mean "minus forty percent unique values." It is a compact representation of a scalable estimate.

---

## Statistics slots

Different data types need different forms of statistics, so `pg_statistic` uses several slot groups:

- `stakindN` — statistic kind stored in the slot;
- `staopN` — associated operator;
- `stacollN` — collation;
- `stanumbersN` — numeric values;
- `stavaluesN` — values of the data type.

The meaning of a particular slot is determined by its `stakindN`.

This explains why `pg_statistic` does not have one fixed column for every possible histogram, MCV, or type-specific statistic.

---

## Why `pg_statistic` is not readable by everyone

Statistics can reveal information about the underlying data. Even without direct `SELECT` access to a user table, a user might infer common, minimum, or maximum values of a sensitive column.

Therefore `pg_statistic` is not globally readable.

For normal use, PostgreSQL provides `pg_stats`, a public view over `pg_statistic` that exposes statistics only for tables readable by the current user.

Model:

```text
pg_statistic → raw/sensitive catalog
pg_stats     → safer user-facing view
```

---

## Extended statistics: definition and data are separate

### `pg_statistic_ext`

Stores the **definition** of an extended-statistics object created by `CREATE STATISTICS`.

Main fields:

- `stxrelid` — table;
- `stxname` — name;
- `stxnamespace` — schema;
- `stxowner` — owner;
- `stxstattarget` — statistics target;
- `stxkeys` — column numbers;
- `stxkind` — enabled statistics kinds;
- `stxexprs` — expression trees for expression entries.

`stxkind` codes:

```text
d  n-distinct
e  expression statistics
f  functional dependencies
m  most-common-value lists (MCV)
```

`CREATE STATISTICS` creates the definition but does not compute the actual values.

### `pg_statistic_ext_data`

The data appear after a subsequent `ANALYZE`.

- `stxoid` → definition in `pg_statistic_ext`;
- `stxdinherit` — inheritance-aware variant;
- `stxdndistinct` — n-distinct data;
- `stxddependencies` — functional dependencies;
- `stxdmcv` — MCV list;
- `stxdexpr` — expression statistics.

Like `pg_statistic`, this catalog can contain sensitive information and is not intended for unrestricted reading. The user-facing view is `pg_stats_ext`.

Key distinction:

```text
pg_statistic_ext      → what to collect
pg_statistic_ext_data → what ANALYZE actually collected
```

---

# Full-text search catalogs

## Pipeline

A full-text search configuration answers the question: **how does PostgreSQL turn source text into normalized lexemes for searching?**

Simplified:

```text
input text
   ↓
parser
   ↓  tokens + token types
configuration
   ↓
config map chooses dictionaries
   ↓
dictionaries normalize/reject tokens
```

---

## `pg_ts_parser`: splitting text into tokens

A parser splits input text into lexemes/tokens and assigns a token type to each one.

Fields:

- `prsname` — parser name;
- `prsnamespace` — schema;
- `prsstart` — start function;
- `prstoken` — function that returns the next token;
- `prsend` — end function;
- `prsheadline` — headline function or `0`;
- `prslextype` — function describing lexical token types.

A new parser must be implemented with C-level functions, so only superusers can create parsers.

---

## `pg_ts_config`: text search configuration

A configuration selects a parser and defines which dictionaries should be applied to token types.

The `pg_ts_config` row itself contains:

- `cfgname` — name;
- `cfgnamespace` — schema;
- `cfgowner` — owner;
- `cfgparser` → `pg_ts_parser`.

The mapping from token types to dictionaries is stored in a separate catalog.

---

## `pg_ts_config_map`: token type → dictionaries

Fields:

- `mapcfg` → `pg_ts_config`;
- `maptokentype` — parser token type;
- `mapseqno` — order in which dictionaries are tried;
- `mapdict` → `pg_ts_dict`.

A single token type can have multiple dictionary entries. They are considered in increasing `mapseqno` order.

So a configuration is not merely a direct "parser → dictionary" link. There is an ordered mapping layer between them.

---

## `pg_ts_dict`: dictionaries

A dictionary defines a concrete configuration of normalization logic based on a template.

Fields:

- `dictname` — name;
- `dictnamespace` — schema;
- `dictowner` — owner;
- `dicttemplate` → `pg_ts_template`;
- `dictinitoption` — initialization options.

The template provides implementation functions, while the dictionary supplies concrete parameter values for that implementation.

---

## `pg_ts_template`: a dictionary-class template

A template contains implementation hooks:

- `tmplname` — name;
- `tmplnamespace` — schema;
- `tmplinit` — initialization function or `0`;
- `tmpllexize` — lexize function.

Like parsers, templates are implemented with C-level functions. Creating new templates requires superuser privileges.

---

## Full-text search relationships at a glance

```text
pg_ts_config.cfgparser
        ↓
pg_ts_parser

pg_ts_config.oid
        ↓
pg_ts_config_map.mapcfg
        │
        └── mapdict → pg_ts_dict
                         │
                         └── dicttemplate → pg_ts_template
```

If a text search configuration behaves unexpectedly, diagnose it along this exact chain: parser → token type → ordered dictionary mapping → dictionary → template.

---

## What to remember

```text
planner statistics:
pg_statistic          → basic column statistics
pg_stats              → user-facing view
pg_statistic_ext      → extended-statistics definition
pg_statistic_ext_data → collected extended-statistics data
pg_stats_ext          → user-facing extended-stats view

text search:
pg_ts_parser     → tokenization
pg_ts_config     → parser + configuration
pg_ts_config_map → token type → ordered dictionaries
pg_ts_dict       → configured dictionary
pg_ts_template   → dictionary implementation template
```

For statistics, the key is distinguishing **definition** from **collected data**. For full-text search, the key is understanding the **pipeline**, not memorizing the catalogs as an unrelated list.
