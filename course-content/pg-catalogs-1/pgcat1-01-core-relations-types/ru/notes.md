# Ядро каталогов: отношения, столбцы, типы и базы

## Ментальная модель

Системные каталоги PostgreSQL — это обычные таблицы, в которых сервер хранит метаданные схемы и служебное состояние. Их можно читать обычным SQL, но вручную менять их обычно не следует: для изменения объектов базы предназначены DDL-команды вроде `CREATE`, `ALTER` и `DROP`.

Удобно мыслить не отдельными таблицами каталога, а графом объектов:

```text
база данных
  ├─ schema / namespace         → pg_namespace
  │    └─ relation             → pg_class
  │         ├─ columns         → pg_attribute
  │         ├─ defaults        → pg_attrdef
  │         └─ row type        → pg_type
  ├─ enum values               → pg_enum
  ├─ collations                → pg_collation
  └─ sequences                → pg_sequence + pg_class

cluster-wide objects
  ├─ databases                 → pg_database
  └─ tablespaces               → pg_tablespace
```

Большинство каталогов относится к одной базе данных: при создании базы они копируются из базы-шаблона и дальше принадлежат этой базе. Некоторые каталоги разделяются всеми базами кластера. Это важно: строка из `pg_class` относится к текущей базе, а строка из `pg_database` описывает базу на уровне всего кластера.

---

## `pg_class`: центральный каталог отношений

В PostgreSQL словом **relation** называют не только обычную таблицу. В `pg_class` представлены таблицы, индексы, последовательности, представления, materialized views, composite types, foreign tables, TOAST-таблицы и секционированные объекты.

Главные поля:

- `oid` — идентификатор строки каталога;
- `relname` — имя отношения;
- `relnamespace` — OID пространства имён из `pg_namespace`;
- `relowner` — OID владельца из `pg_authid`;
- `relam` — метод доступа для таблицы или индекса;
- `reltablespace` — tablespace, либо `0` для значения по умолчанию;
- `relkind` — вид отношения.

Часто встречающиеся значения `relkind`:

```text
r  ordinary table
i  index
S  sequence
t  TOAST table
v  view
m  materialized view
c  composite type
f  foreign table
p  partitioned table
I  partitioned index
```

`relkind` нужно читать как классификацию объекта. Например, наличие строки в `pg_class` ещё не означает, что объект является таблицей пользователя.

### Оценки, а не точные счётчики

`pg_class` также содержит данные, которыми пользуется планировщик:

- `relpages` — приблизительное число страниц отношения;
- `reltuples` — приблизительное число актуальных строк;
- `relallvisible` — приблизительное число полностью видимых страниц.

Эти значения обновляются `VACUUM`, `ANALYZE` и некоторыми DDL-командами. Поэтому:

```text
pg_class.reltuples ≠ SELECT count(*)
```

`reltuples = -1` означает, что оценка количества строк пока неизвестна.

### Полезные флаги

В `pg_class` есть признаки состояния и свойств отношения, например:

- `relpersistence`: `p` permanent, `u` unlogged, `t` temporary;
- `relhasindex` — таблица имеет или недавно имела индексы;
- `relhastriggers` — для отношения есть или были триггеры;
- `relrowsecurity` — для таблицы включён RLS;
- `relispartition` — объект является секцией.

Некоторые флаги поддерживаются не строго мгновенно. Например, `relhasindex` может остаться `true` после удаления последнего индекса и быть сброшен позднее `VACUUM`. Поэтому такие поля нельзя всегда трактовать как идеально синхронный снимок состояния.

---

## `pg_attribute`: столбцы отношений

`pg_attribute` хранит сведения о столбцах. Для каждого столбца каждого отношения существует отдельная строка. Записи есть не только для пользовательских таблиц, но и для других объектов из `pg_class`, например индексов.

Основные поля:

- `attrelid` — OID отношения из `pg_class`;
- `attname` — имя столбца;
- `atttypid` — OID типа из `pg_type`;
- `attnum` — номер атрибута;
- `attnotnull` — наличие `NOT NULL`;
- `atthasdef` — есть default или generated expression;
- `attidentity` — identity column;
- `attgenerated` — generated column;
- `attisdropped` — столбец был удалён.

Обычные пользовательские столбцы имеют `attnum`, начиная с `1`. Системные столбцы, например `ctid`, обычно имеют отрицательные номера.

Удалённый столбец может продолжать физически присутствовать в строке таблицы. Поэтому запись в `pg_attribute` не обязательно исчезает: она может остаться с `attisdropped = true`, а `atttypid` для неё сбрасывается в `0`.

### Тип и модификатор типа

`atttypid` указывает базовый тип столбца, а `atttypmod` хранит дополнительную информацию, связанную с типом, например ограничение длины `varchar(n)`.

`attndims` ненулевой для массива, но PostgreSQL в настоящее время не контролирует фактическое число измерений массива по этому значению; практически ненулевое значение означает «столбец объявлен как массив».

---

## `pg_attrdef`: выражения DEFAULT и generated expression

`pg_attribute.atthasdef` говорит только о наличии выражения. Само внутреннее представление находится в `pg_attrdef`.

Ключевые поля:

- `adrelid` — отношение;
- `adnum` — номер столбца;
- `adbin` — внутреннее представление выражения.

Для получения читаемого SQL обычно используют `pg_get_expr`:

```sql
SELECT
    a.attname,
    pg_get_expr(d.adbin, d.adrelid) AS default_expression
FROM pg_attribute AS a
JOIN pg_attrdef AS d
  ON d.adrelid = a.attrelid
 AND d.adnum = a.attnum
WHERE a.attrelid = 'public.orders'::regclass;
```

Это важный общий принцип системных каталогов: некоторые выражения хранятся во внутреннем формате `pg_node_tree`, а для человека PostgreSQL предоставляет функции декомпиляции.

---

## `pg_namespace`: основа SQL-схем

`pg_namespace` хранит пространства имён, на которых построены SQL-схемы.

Основные поля:

- `oid` — идентификатор namespace;
- `nspname` — имя;
- `nspowner` — владелец;
- `nspacl` — права доступа.

Связь с отношениями:

```text
pg_class.relnamespace → pg_namespace.oid
```

Поэтому обычный запрос «таблица + схема» строится соединением этих двух каталогов:

```sql
SELECT n.nspname, c.relname, c.relkind
FROM pg_class AS c
JOIN pg_namespace AS n
  ON n.oid = c.relnamespace
WHERE c.relkind IN ('r', 'p');
```

---

## `pg_type`: типы данных

`pg_type` описывает типы данных PostgreSQL. При создании таблицы автоматически создаётся составной тип строки этой таблицы, поэтому связь `pg_class` ↔ `pg_type` является частью обычной модели каталога.

Главное поле классификации — `typtype`:

```text
b  base type
c  composite type
d  domain
e  enum
p  pseudo-type
r  range
m  multirange
```

Другие полезные связи:

- `typnamespace` → `pg_namespace`;
- `typowner` → `pg_authid`;
- `typrelid` → `pg_class` для composite type;
- `typelem` — тип элемента;
- `typarray` — соответствующий настоящий array type;
- `typbasetype` — базовый тип домена.

Для типа фиксированной длины `typlen` содержит размер во внутреннем представлении. Для переменной длины значение отрицательное; `-1` обозначает varlena.

---

## `pg_database`: базы всего кластера

`pg_database` отличается от большинства каталогов: он **разделяется всеми базами кластера**. В кластере существует одна общая копия этого каталога.

Полезные поля:

- `datname` — имя базы;
- `datdba` — владелец;
- `encoding` — кодировка;
- `datistemplate` — можно ли использовать базу как template;
- `datallowconn` — разрешены ли подключения;
- `datconnlimit` — лимит соединений;
- `dattablespace` — tablespace по умолчанию;
- locale/collation-поля базы.

Значения лимита:

```text
datconnlimit = -1  → лимита нет
datconnlimit = -2  → подключаться нельзя
```

`datallowconn = false` также запрещает подключения. Это, например, используется для защиты `template0` от модификаций.

---

## `pg_sequence`: параметры последовательностей

Последовательность представлена сразу в нескольких местах:

```text
имя и schema     → pg_class
параметры        → pg_sequence
```

`pg_sequence` содержит:

- `seqrelid` — OID последовательности в `pg_class`;
- `seqtypid` — тип данных;
- `seqstart` — старт;
- `seqincrement` — шаг;
- `seqmax`, `seqmin` — границы;
- `seqcache` — размер cache;
- `seqcycle` — зацикливается ли sequence.

---

## `pg_enum`: значения перечислений

Каждое значение enum представлено отдельной строкой в `pg_enum`.

- `enumtypid` — OID enum-типа из `pg_type`;
- `enumlabel` — текстовая метка;
- `enumsortorder` — положение значения в порядке перечисления.

Для значений, созданных вместе с enum-типом, порядок обычно начинается с `1..n`. Значения, добавленные позднее, могут получить дробные или отрицательные `enumsortorder`; важно только, чтобы порядок оставался корректным и значения были уникальны внутри enum.

---

## `pg_collation`: правила сортировки

`pg_collation` описывает доступные collations.

Основные поля:

- `collname` — имя;
- `collnamespace` — schema;
- `collowner` — владелец;
- `collprovider` — provider;
- `collisdeterministic` — детерминированность;
- `collencoding` — применимая кодировка;
- `collversion` — версия определения от provider.

Важные коды provider:

```text
c  libc
i  ICU
d  database default
```

`collencoding = -1` означает, что правило применимо к любой кодировке.

`collversion` нужен для обнаружения изменений определения collation у provider, которые потенциально могут сделать ранее построенные данные несогласованными с новым порядком сортировки.

---

## `pg_tablespace`: табличные пространства

`pg_tablespace` хранит доступные tablespaces и, как `pg_database`, является shared catalog.

Основные поля:

- `spcname` — имя;
- `spcowner` — владелец;
- `spcacl` — ACL;
- `spcoptions` — параметры.

Физическое местоположение можно получать специальной функцией `pg_tablespace_location()` вместо попытки выводить его из внутреннего устройства файлов кластера.

---

## Практическая схема чтения каталогов

Когда нужно исследовать объект, двигайтесь от общего к частному:

```text
1. найти объект         → pg_class
2. определить schema    → pg_namespace
3. получить столбцы     → pg_attribute
4. получить типы        → pg_type
5. получить default     → pg_attrdef + pg_get_expr
6. перейти к каталогу конкретного вида объекта при необходимости
```

Пример описания столбцов таблицы:

```sql
SELECT
    n.nspname AS schema_name,
    c.relname AS table_name,
    a.attnum,
    a.attname,
    format_type(a.atttypid, a.atttypmod) AS data_type,
    a.attnotnull
FROM pg_class AS c
JOIN pg_namespace AS n
  ON n.oid = c.relnamespace
JOIN pg_attribute AS a
  ON a.attrelid = c.oid
WHERE c.oid = 'public.orders'::regclass
  AND a.attnum > 0
  AND NOT a.attisdropped
ORDER BY a.attnum;
```

---

## Что запомнить

```text
pg_class      → что это за relation
pg_namespace  → в какой schema
pg_attribute  → какие у relation столбцы
pg_attrdef    → какое default/generated expression
pg_type       → какой тип данных

pg_database   → базы кластера, shared catalog
pg_tablespace → tablespaces кластера, shared catalog

reltuples     → оценка, не count(*)
attnum > 0    → обычные пользовательские столбцы
attnum < 0    → системные столбцы
```

Смысл системных каталогов не в запоминании сотен полей. Главное — понимать, какой каталог является источником истины для нужного свойства и через какой OID он связан с соседними каталогами.
