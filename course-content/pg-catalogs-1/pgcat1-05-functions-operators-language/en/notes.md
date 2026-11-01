# Functions, Aggregates, Operators, and Languages

## Mental model

In PostgreSQL, functions and related mechanisms are described by several catalogs:

```text
pg_proc      → general routine row
  ├─ pg_aggregate → additional aggregate properties
  └─ pg_language  → implementation language

pg_operator  → SQL operators
pg_cast      → allowed cast paths
pg_transform → type conversion at the procedural-language boundary
pg_conversion→ character-encoding conversion
pg_range     → additional range-type properties
```

`pg_proc` is the center of this model: functions, procedures, aggregate functions, and window functions are all represented there as routines.

---

## `pg_proc`: routines

`pg_proc` stores ordinary functions, procedures, aggregate functions, and window functions.

### Routine kind

`prokind`:

```text
f  ordinary function
p  procedure
a  aggregate function
w  window function
```

If `prokind = 'a'`, a corresponding additional row must exist in `pg_aggregate`.

### Identity and implementation

Useful fields:

- `proname` — name;
- `pronamespace` — schema;
- `proowner` — owner;
- `prolang` — language from `pg_language`;
- `prorettype` — return type;
- `proretset` — routine returns a set;
- `prosrc` — implementation-specific source/symbol;
- `probin` — additional implementation-specific information;
- `prosqlbody` — parsed SQL body for the SQL-standard body form;
- `proconfig` — routine-local runtime settings.

For interpreted procedural languages, `prosrc` normally contains source code. For an SQL routine declared with a standard SQL body, `prosqlbody` is used.

### Arguments

- `pronargs` — number of input arguments;
- `pronargdefaults` — number of input arguments with defaults;
- `proargtypes` — input argument types forming the call signature;
- `proallargtypes` — all arguments, including OUT/INOUT, when present;
- `proargmodes` — encoded `IN/OUT/INOUT/VARIADIC/TABLE` modes;
- `proargnames` — argument names;
- `proargdefaults` — internal representation of default expressions.

`proargtypes` and `proallargtypes` are not interchangeable: the former specifically describes the input call signature.

---

## Volatility: `provolatile`

`provolatile` tells the planner how strongly a result depends on external state.

```text
i  IMMUTABLE
s  STABLE
v  VOLATILE
```

`IMMUTABLE` means the same arguments always produce the same result.

`STABLE` permits dependence on state that does not change during a single scan.

`VOLATILE` allows the result to change at any time; this class is also used for functions with side effects so the planner does not optimize their calls as safely repeatable or removable.

---

## Parallel safety: `proparallel`

```text
s  safe
r  restricted: leader process of the parallel group only
u  unsafe
```

`u` does not mean "the function will raise a runtime error in a worker." It is planner metadata: the presence of an unsafe routine leads the planner to choose a non-parallel execution plan.

---

## Other important routine properties

- `prosecdef` — routine executes in a security-definer context;
- `proleakproof` — routine is declared leakproof;
- `proisstrict` — if any argument is `NULL`, the routine returns `NULL` without actually being called;
- `procost` — estimated execution cost;
- `prorows` — estimated output-row count for a set-returning function;
- `prosupport` — planner support function.

---

## `pg_aggregate`: an extension of `pg_proc`

An aggregate does not exist only in `pg_aggregate`. It also has an ordinary routine row in `pg_proc`; `pg_aggregate` stores additional properties of the aggregate machinery.

Relationship:

```text
pg_aggregate.aggfnoid → pg_proc.oid
```

### Aggregate kinds

`aggkind`:

```text
n  normal
o  ordered-set
h  hypothetical-set
```

Some other fields:

- `aggtransfn` — transition function;
- `aggfinalfn` — final function;
- `aggcombinefn` — combine function;
- `aggserialfn` / `aggdeserialfn` — serialization;
- `aggtranstype` — transition-state type;
- `agginitval` — initial state value.

These fields show that an aggregate is a state-accumulation protocol, not merely "an ordinary function that receives all rows at once."

---

## `pg_operator`: SQL operators

`pg_operator` stores operator definitions.

Useful fields:

- `oprname` — operator name;
- `oprnamespace` — schema;
- `oprowner` — owner;
- `oprkind` — binary infix (`b`) or prefix (`l`);
- `oprleft`, `oprright` — operand types;
- `oprresult` — result type;
- `oprcode` — implementing function;
- `oprcom` — commutator;
- `oprnegate` — negator;
- `oprcanmerge` — operator supports merge join;
- `oprcanhash` — operator supports hash join.

Operator metadata is closely tied to index infrastructure: operator families in `pg_amop` reference rows in `pg_operator`.

---

## `pg_cast`: type-conversion paths

`pg_cast` stores explicitly registered cast paths, but not every conversion PostgreSQL can perform. Some conversions are derived from general rules and do not need a catalog row.

Fields:

- `castsource` — source type;
- `casttarget` — target type;
- `castfunc` — implementing function or `0`;
- `castcontext` — contexts where the cast is allowed;
- `castmethod` — how the cast is performed.

### `castcontext`

```text
e  explicit only
a  assignment + explicit
i  implicit + assignment + explicit
```

The closer the value is to `i`, the more contexts permit the cast.

### `castmethod`

```text
f  function cast, using castfunc
i  input/output conversion
b  binary-coercible, no data conversion required
```

---

## `pg_language`: function and procedure languages

`pg_language` registers languages in which server-side routines can be created.

Fields:

- `lanname` — name;
- `lanowner` — owner;
- `lanispl` — procedural/user-language marker;
- `lanpltrusted` — trusted-language flag;
- `lanplcallfoid` — language handler;
- `laninline` — handler for `DO` blocks;
- `lanvalidator` — validator;
- `lanacl` — ACL.

### Trusted vs. untrusted

`lanpltrusted = true` means the language is considered unable to access resources outside the normal SQL execution environment. Therefore non-superusers may create functions in a trusted language when privileges allow it.

Only superusers may create functions in untrusted languages.

---

## `pg_transform`: adapting types for procedural languages

`pg_transform` associates a PostgreSQL type with a procedural language and conversion functions at the boundary between them.

- `trftype` — data type;
- `trflang` — language;
- `trffromsql` — converts an SQL value into the procedural language's representation;
- `trftosql` — converts it back to an SQL type.

A function OID of `0` means the default conversion is used.

---

## `pg_conversion`: character-encoding conversions

`pg_conversion` describes encoding-conversion functions.

- `conname` — conversion name;
- `connamespace` — schema;
- `conowner` — owner;
- `conforencoding` — source encoding ID;
- `contoencoding` — destination encoding ID;
- `conproc` — conversion function;
- `condefault` — whether this is the default conversion.

Encoding names can be resolved with catalog helper functions `pg_encoding_to_char()` and `pg_char_to_encoding()`.

---

## `pg_range`: additional range-type properties

The main row for a range type lives in `pg_type`; `pg_range` supplements it.

Fields:

- `rngtypid` — range type;
- `rngsubtype` — element/subtype;
- `rngmultitypid` — associated multirange type;
- `rngcollation` — collation for comparisons, if needed;
- `rngsubopc` — subtype operator class;
- `rngcanonical` — canonicalization function or `0`;
- `rngsubdiff` — subtype-difference function or `0`.

`rngcanonical` is especially relevant for discrete subtypes, where syntactically different ranges can be normalized to one canonical representation.

---

## Practical decompiler functions

To inspect a routine definition, you do not need to reconstruct `CREATE FUNCTION` manually from dozens of `pg_proc` fields.

PostgreSQL provides helper functions:

```sql
SELECT pg_get_functiondef('public.some_function(integer)'::regprocedure);
```

Similar functions exist for arguments, return types, and other catalog-backed objects.

This is a useful separation:

```text
system catalogs → structured metadata for analysis
pg_get_*        → human-readable SQL definition
```

---

## What to remember

```text
pg_proc       → routines
pg_aggregate  → aggregate-specific metadata
pg_operator   → operators
pg_cast       → cast paths
pg_language   → procedural languages
pg_transform  → type ↔ procedural language transforms
pg_conversion → encoding conversions
pg_range      → range-specific metadata

prokind:     f / p / a / w
provolatile: i / s / v
proparallel: s / r / u
castcontext: e / a / i
castmethod:  f / i / b
```

The main idea: `pg_proc` describes a callable object, while neighboring catalogs clarify how it participates in aggregation, operators, casts, language execution, and type conversion.
