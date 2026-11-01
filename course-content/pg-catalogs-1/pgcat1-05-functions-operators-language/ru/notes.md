# Функции, агрегаты, операторы и языки

## Ментальная модель

В PostgreSQL функции и связанные с ними механизмы описываются несколькими каталогами:

```text
pg_proc      → общая запись routine
  ├─ pg_aggregate → дополнительные свойства aggregate
  └─ pg_language  → язык реализации

pg_operator  → SQL operators
pg_cast      → разрешённые пути cast
pg_transform → преобразование типов на границе procedural language
pg_conversion→ преобразование character encoding
pg_range     → дополнительные свойства range type
```

`pg_proc` — центр этой модели: функции, процедуры, aggregate functions и window functions представлены там как routines.

---

## `pg_proc`: routines

`pg_proc` хранит обычные функции, процедуры, aggregate functions и window functions.

### Тип routine

`prokind`:

```text
f  ordinary function
p  procedure
a  aggregate function
w  window function
```

Если `prokind = 'a'`, соответствующая дополнительная строка должна находиться в `pg_aggregate`.

### Идентичность и реализация

Полезные поля:

- `proname` — имя;
- `pronamespace` — schema;
- `proowner` — владелец;
- `prolang` — язык из `pg_language`;
- `prorettype` — return type;
- `proretset` — routine возвращает set;
- `prosrc` — implementation-specific source/symbol;
- `probin` — дополнительная implementation-specific информация;
- `prosqlbody` — parsed SQL body для SQL-standard body form;
- `proconfig` — локальные runtime settings функции.

Для interpreted procedural languages `prosrc` обычно содержит source code. Для SQL routine, объявленной стандартным SQL body, используется `prosqlbody`.

### Аргументы

- `pronargs` — число input arguments;
- `pronargdefaults` — число input arguments с defaults;
- `proargtypes` — типы input arguments, формирующие call signature;
- `proallargtypes` — все arguments, включая OUT/INOUT, если такие есть;
- `proargmodes` — `IN/OUT/INOUT/VARIADIC/TABLE` в кодированном виде;
- `proargnames` — имена;
- `proargdefaults` — internal representation default expressions.

`proargtypes` и `proallargtypes` не взаимозаменяемы: первое поле описывает именно входную сигнатуру вызова.

---

## Volatility: `provolatile`

`provolatile` сообщает planner, насколько результат зависит от внешнего состояния.

```text
i  IMMUTABLE
s  STABLE
v  VOLATILE
```

`IMMUTABLE` означает, что для одинаковых аргументов результат всегда одинаков.

`STABLE` допускает зависимость от состояния, которое не меняется в рамках одного scan.

`VOLATILE` допускает изменение результата в любой момент; этот класс также используется для функций с side effects, чтобы planner не оптимизировал их вызовы как безопасно повторяемые/устранимые.

---

## Parallel safety: `proparallel`

```text
s  safe
r  restricted: только leader процесса parallel group
u  unsafe
```

`u` не означает «функция выдаст runtime error в worker». Это metadata для planner: наличие unsafe routine приводит к выбору non-parallel execution plan.

---

## Другие важные свойства функции

- `prosecdef` — routine выполняется в security-definer context;
- `proleakproof` — routine объявлена leakproof;
- `proisstrict` — если любой аргумент `NULL`, routine возвращает `NULL` без фактического вызова;
- `procost` — оценка стоимости;
- `prorows` — ожидаемое число output rows для set-returning function;
- `prosupport` — planner support function.

---

## `pg_aggregate`: дополнение к `pg_proc`

Aggregate не существует только в `pg_aggregate`. У него есть обычная routine-запись в `pg_proc`, а `pg_aggregate` хранит дополнительные свойства aggregate machinery.

Связь:

```text
pg_aggregate.aggfnoid → pg_proc.oid
```

### Виды aggregate

`aggkind`:

```text
n  normal
o  ordered-set
h  hypothetical-set
```

Некоторые другие поля:

- `aggtransfn` — transition function;
- `aggfinalfn` — final function;
- `aggcombinefn` — combine function;
- `aggserialfn` / `aggdeserialfn` — serialization;
- `aggtranstype` — transition state type;
- `agginitval` — initial state value.

Эти поля показывают, что aggregate — это протокол накопления состояния, а не просто «обычная функция, получившая сразу все строки».

---

## `pg_operator`: SQL operators

`pg_operator` хранит определения operators.

Полезные поля:

- `oprname` — имя operator;
- `oprnamespace` — schema;
- `oprowner` — владелец;
- `oprkind` — binary infix (`b`) или prefix (`l`);
- `oprleft`, `oprright` — operand types;
- `oprresult` — result type;
- `oprcode` — implementing function;
- `oprcom` — commutator;
- `oprnegate` — negator;
- `oprcanmerge` — operator поддерживает merge join;
- `oprcanhash` — operator поддерживает hash join.

Operator metadata тесно связана с индексной инфраструктурой: operator families в `pg_amop` ссылаются именно на строки `pg_operator`.

---

## `pg_cast`: пути преобразования типов

`pg_cast` хранит явно зарегистрированные cast paths, но не абсолютно все преобразования, которые PostgreSQL может выполнить. Некоторые преобразования выводятся общими правилами и не требуют строки в каталоге.

Поля:

- `castsource` — source type;
- `casttarget` — target type;
- `castfunc` — implementing function либо `0`;
- `castcontext` — где cast разрешён;
- `castmethod` — как выполняется.

### `castcontext`

```text
e  explicit only
a  assignment + explicit
i  implicit + assignment + explicit
```

Чем ближе к `i`, тем больше контекстов разрешено.

### `castmethod`

```text
f  function cast, используется castfunc
i  input/output conversion
b  binary-coercible, преобразование данных не требуется
```

---

## `pg_language`: языки функций и процедур

`pg_language` регистрирует языки, на которых можно создавать server-side routines.

Поля:

- `lanname` — имя;
- `lanowner` — владелец;
- `lanispl` — procedural/user language marker;
- `lanpltrusted` — trusted language;
- `lanplcallfoid` — language handler;
- `laninline` — обработчик `DO` blocks;
- `lanvalidator` — validator;
- `lanacl` — ACL.

### Trusted vs untrusted

`lanpltrusted = true` означает, что язык считается не открывающим доступ за пределы обычной SQL execution environment. Поэтому создавать функции на trusted language могут не только superusers, если права это позволяют.

Для untrusted languages создание функций разрешено только superusers.

---

## `pg_transform`: адаптация типов для procedural languages

`pg_transform` связывает PostgreSQL type с procedural language и функциями преобразования на границе между ними.

- `trftype` — data type;
- `trflang` — language;
- `trffromsql` — преобразование SQL value во внутреннее значение procedural language;
- `trftosql` — преобразование обратно в SQL type.

`0` в function OID означает использование default conversion.

---

## `pg_conversion`: character encoding conversions

`pg_conversion` описывает функции преобразования кодировок.

- `conname` — имя conversion;
- `connamespace` — schema;
- `conowner` — владелец;
- `conforencoding` — source encoding ID;
- `contoencoding` — destination encoding ID;
- `conproc` — conversion function;
- `condefault` — conversion по умолчанию.

Имена кодировок можно получать через catalog helper functions `pg_encoding_to_char()` и `pg_char_to_encoding()`.

---

## `pg_range`: дополнительные свойства range types

Основная запись range type находится в `pg_type`, а `pg_range` дополняет её.

Поля:

- `rngtypid` — range type;
- `rngsubtype` — element/subtype;
- `rngmultitypid` — связанный multirange type;
- `rngcollation` — collation для сравнения, если нужна;
- `rngsubopc` — operator class subtype;
- `rngcanonical` — canonicalization function либо `0`;
- `rngsubdiff` — функция разницы subtype values либо `0`.

`rngcanonical` особенно относится к дискретным subtype, где разные синтаксические диапазоны могут приводиться к единой canonical representation.

---

## Практические функции декомпиляции

Для чтения определения routines не обязательно собирать `CREATE FUNCTION` вручную из десятков полей `pg_proc`.

PostgreSQL предоставляет helper functions:

```sql
SELECT pg_get_functiondef('public.some_function(integer)'::regprocedure);
```

Аналогично существуют функции для аргументов, return type и других catalog-backed объектов.

Это полезное разделение:

```text
system catalogs → структурированные metadata для анализа
pg_get_*        → человекочитаемое SQL definition
```

---

## Что запомнить

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

Главная идея: `pg_proc` описывает callable object, а соседние каталоги уточняют, как он участвует в агрегировании, operators, casts, language execution и type conversion.
