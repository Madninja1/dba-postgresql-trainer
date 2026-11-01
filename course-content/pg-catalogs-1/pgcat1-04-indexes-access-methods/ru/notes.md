# Индексы и методы доступа

## Ментальная модель

Описание индекса в PostgreSQL распределено по нескольким каталогам:

```text
pg_class
   │  общая запись relation
   ▼
pg_index
   │  свойства конкретного индекса и связь с таблицей
   │
   ├─ pg_opclass ── pg_opfamily
   │       │            │
   │       │            ├─ pg_amop   operators
   │       │            └─ pg_amproc support procedures
   │       ▼
   └───── pg_am         access method
```

Секционирование и наследование добавляют ещё два каталога:

```text
pg_partitioned_table → ключ секционирования
pg_inherits          → parent/child edges
```

---

## `pg_am`: методы доступа

`pg_am` содержит методы доступа relations. Каждая строка описывает один access method.

Поля:

- `amname` — имя;
- `amhandler` — handler function;
- `amtype` — вид метода.

`amtype`:

```text
t  table access method
i  index access method
```

Свойства индексных методов больше не кодируются множеством колонок `pg_am`, как в старых версиях PostgreSQL. Для проверки свойств существуют специальные функции `pg_indexam_has_property()`, `pg_index_has_property()` и `pg_index_column_has_property()`.

---

## Семейство операторов и класс операторов

### `pg_opfamily`

Operator family — набор совместимых operators и support procedures для одного index access method.

Поля:

- `opfmethod` → `pg_am`;
- `opfname` — имя family;
- `opfnamespace` — schema;
- `opfowner` — владелец.

Само содержимое family находится преимущественно в связанных строках `pg_amop`, `pg_amproc` и `pg_opclass`.

### `pg_opclass`

Operator class связывает:

```text
index access method
        +
индексируемый тип данных
        +
operator family
```

Поля:

- `opcmethod` → `pg_am`;
- `opcname` — имя opclass;
- `opcfamily` → `pg_opfamily`;
- `opcintype` — индексируемый тип;
- `opcdefault` — default opclass для пары access method + type;
- `opckeytype` — тип, реально хранимый в индексе, либо `0`, если совпадает с `opcintype`.

Для каждой комбинации `opcmethod + opcintype` может быть не больше одного default opclass.

---

## `pg_amop`: operators семейства

`pg_amop` содержит operators, входящие в operator families.

Основные поля:

- `amopfamily` — family;
- `amoplefttype`, `amoprighttype` — типы operands;
- `amopstrategy` — strategy number;
- `amoppurpose` — назначение;
- `amopopr` — operator;
- `amopmethod` — access method;
- `amopsortfamily` — B-tree family для ordering operator.

`amoppurpose`:

```text
s  search
o  ordering
```

Search operator означает, что index family умеет искать строки для условия вида:

```sql
indexed_column operator constant
```

Ordering operator позволяет index scan выдавать строки в порядке, определяемом выражением `ORDER BY indexed_column operator constant`.

---

## `pg_amproc`: support procedures

`pg_amproc` хранит опорные функции operator families.

Поля:

- `amprocfamily` — family;
- `amproclefttype`, `amprocrighttype` — связанные types;
- `amprocnum` — номер support function;
- `amproc` — OID функции.

Конкретная семантика номера support procedure зависит от access method. Поэтому `amprocnum` нельзя интерпретировать без знания соответствующего метода доступа.

---

## `pg_index`: свойства конкретного индекса

Общая запись индекса находится в `pg_class`, а `pg_index` добавляет индекс-специфичные свойства.

Основные связи:

```text
indexrelid → pg_class.oid индекса
indrelid   → pg_class.oid таблицы
```

### Число атрибутов

- `indnatts` — все attributes индекса, включая key и non-key INCLUDE columns;
- `indnkeyatts` — только key attributes.

### Назначение и состояние

- `indisunique` — unique index;
- `indisprimary` — индекс представляет PRIMARY KEY;
- `indisexclusion` — поддерживает exclusion constraint;
- `indisclustered` — таблица в последний раз была cluster-ована по этому index;
- `indisreplident` — индекс выбран как replica identity.

Состояние жизненного цикла:

- `indisvalid` — индекс допустимо использовать в запросах;
- `indisready` — индекс готов принимать новые данные;
- `indislive` — индекс считается живым; `false` означает процесс удаления.

Эти флаги особенно важны при online/concurrent операциях: индекс может физически существовать, но ещё не быть пригодным для planner или не участвовать в maintenance всех новых строк.

### NULL в unique index

`indnullsnotdistinct` относится к unique index:

```text
false → NULL считаются различными, можно несколько NULL
true  → NULL считаются равными для uniqueness
```

---

## `indkey`: какие столбцы индексируются

`indkey` содержит номера столбцов таблицы (`pg_attribute.attnum`).

Пример:

```text
1 3
```

означает первый и третий столбцы таблицы.

**Ноль** в `indkey` означает не «нулевой столбец», а expression key: соответствующий атрибут индекса задаётся выражением.

Само дерево expression хранится в `indexprs`. В нём есть один expression для каждого нуля среди key attributes.

Partial index определяется через `indpred`: если поле не `NULL`, оно содержит внутреннее дерево predicate.

Практически:

```text
indkey       → column numbers / 0 for expressions
indexprs     → expressions для нулей indkey
indpred      → WHERE predicate partial index
```

---

## `pg_partitioned_table`: ключ секционирования

`pg_partitioned_table` дополняет запись partitioned table из `pg_class`.

Поля:

- `partrelid` — partitioned table;
- `partstrat` — стратегия;
- `partnatts` — число частей partition key;
- `partdefid` — default partition или `0`;
- `partattrs` — column numbers либо `0` для expression;
- `partclass` — operator classes;
- `partcollation` — collations;
- `partexprs` — выражения partition key для нулей в `partattrs`.

Коды стратегии:

```text
h  HASH
l  LIST
r  RANGE
```

Как и в `pg_index.indkey`, `0` в `partattrs` означает expression, а не обычный столбец.

---

## `pg_inherits`: дерево наследования и секционирования

`pg_inherits` хранит **непосредственные** parent/child связи таблиц и индексов.

- `inhrelid` — child relation;
- `inhparent` — parent relation;
- `inhseqno` — порядок inherited columns при multiple inheritance;
- `inhdetachpending` — partition сейчас отсоединяется.

Каждый edge хранится отдельной строкой. Косвенную иерархию нужно получать обходом цепочки записей.

Для declarative partitioning эта же структура представляет parent/partition relationships.

---

## Практические запросы

### Индексы таблицы

```sql
SELECT
    ic.relname AS index_name,
    i.indisunique,
    i.indisprimary,
    i.indisvalid,
    i.indisready,
    i.indislive
FROM pg_index AS i
JOIN pg_class AS ic
  ON ic.oid = i.indexrelid
WHERE i.indrelid = 'public.orders'::regclass;
```

### Восстановление DDL индекса

Для человекочитаемого определения лучше использовать:

```sql
SELECT pg_get_indexdef(indexrelid)
FROM pg_index
WHERE indrelid = 'public.orders'::regclass;
```

Это безопаснее, чем пытаться вручную декодировать `indexprs`, `indpred`, opclass и option fields.

---

## Что запомнить

```text
pg_am       → access methods
pg_opfamily → совместимое семейство operators/procedures
pg_opclass  → type + access method + family
pg_amop     → operators family
pg_amproc   → support procedures family
pg_index    → свойства конкретного индекса

indkey = N  → table column attnum N
indkey = 0  → expression key

pg_partitioned_table → partition key
pg_inherits          → parent/child edges
```

Индекс в PostgreSQL — не одна строка каталога. Его полное описание получается из общей relation metadata, `pg_index`, access method и operator infrastructure.
