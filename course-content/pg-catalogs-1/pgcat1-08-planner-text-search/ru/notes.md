# Статистика планировщика и полнотекстовый поиск

## Ментальная модель

Эта тема состоит из двух независимых подсистем.

### Статистика planner

```text
ANALYZE
  ├─ pg_statistic              → базовая статистика столбцов
  └─ pg_statistic_ext_data     → вычисленные данные extended statistics
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

Не смешивайте эти две части: первая помогает planner оценивать запросы, вторая описывает лингвистический pipeline text search.

---

## `pg_statistic`: базовая статистика planner

`pg_statistic` содержит статистику о содержимом базы. Записи создаёт `ANALYZE`, после чего planner использует их для оценки cardinality и стоимости планов.

Эти данные **по природе являются оценочными**, даже когда свежие.

### Что именно идентифицирует строка

- `starelid` — table или index;
- `staattnum` — номер описываемого column;
- `stainherit` — учитываются ли child tables/inheritance tree.

Обычно для column есть строка с `stainherit = false`. Для table с descendants может существовать дополнительная строка с `stainherit = true`.

Статистика для index expressions также хранится в `pg_statistic`; в таком случае `starelid` ссылается на index.

---

## Базовые численные показатели

- `stanullfrac` — доля rows, где value `NULL`;
- `stawidth` — средний storage size non-NULL value;
- `stadistinct` — оценка количества distinct non-NULL values.

### Как читать `stadistinct`

```text
> 0  → оценённое абсолютное число distinct values
< 0  → модуль — коэффициент относительно числа rows
= 0  → число distinct неизвестно
```

Например, `stadistinct = -0.4` означает, что planner оценивает число distinct values примерно как `0.4 * number_of_rows`.

Отрицательное значение — не ошибка и не «минус сорок процентов уникальных значений». Это специальное компактное представление масштабируемой оценки.

---

## Statistics slots

Разные data types требуют разных видов статистики, поэтому `pg_statistic` использует несколько slot groups:

- `stakindN` — вид статистики в слоте;
- `staopN` — связанный operator;
- `stacollN` — collation;
- `stanumbersN` — числовые значения;
- `stavaluesN` — значения data type.

Смысл конкретного slot определяется его `stakindN`.

Это объясняет, почему `pg_statistic` не имеет отдельной фиксированной колонки для каждого возможного вида histogram/MCV/type-specific statistics.

---

## Почему `pg_statistic` закрыт от всех пользователей

Статистика может раскрывать содержимое данных. Даже без прямого `SELECT` из пользовательской таблицы можно узнать, например, распространённые, минимальные или максимальные значения чувствительного столбца.

Поэтому `pg_statistic` не открыт всем для чтения.

Для обычного использования существует `pg_stats` — public view поверх `pg_statistic`, которое показывает статистику только для tables, доступных текущему пользователю.

Модель:

```text
pg_statistic → raw/sensitive catalog
pg_stats     → safer user-facing view
```

---

## Extended statistics: definition и data разделены

### `pg_statistic_ext`

Хранит **определение** объекта extended statistics, созданного `CREATE STATISTICS`.

Основные поля:

- `stxrelid` — table;
- `stxname` — имя;
- `stxnamespace` — schema;
- `stxowner` — владелец;
- `stxstattarget` — statistics target;
- `stxkeys` — номера columns;
- `stxkind` — включённые виды statistics;
- `stxexprs` — expression trees для expression entries.

Коды `stxkind`:

```text
d  n-distinct
e  expression statistics
f  functional dependencies
m  most-common-value lists (MCV)
```

`CREATE STATISTICS` создаёт definition, но не вычисляет сами values.

### `pg_statistic_ext_data`

Данные появляются после последующего `ANALYZE`.

- `stxoid` → definition в `pg_statistic_ext`;
- `stxdinherit` — inheritance-aware variant;
- `stxdndistinct` — n-distinct data;
- `stxddependencies` — functional dependencies;
- `stxdmcv` — MCV list;
- `stxdexpr` — expression statistics.

Как и `pg_statistic`, этот каталог может содержать чувствительную информацию и не предназначен для свободного чтения всеми. User-facing view — `pg_stats_ext`.

Ключевое различие:

```text
pg_statistic_ext      → что собирать
pg_statistic_ext_data → что ANALYZE реально собрал
```

---

# Full-text search catalogs

## Pipeline

Full-text search configuration отвечает на вопрос: **как из исходного текста получить нормализованные лексемы для поиска**.

Упрощённо:

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

## `pg_ts_parser`: разбор текста на tokens

Parser делит input text на lexemes/tokens и присваивает каждому token type.

Поля:

- `prsname` — имя parser;
- `prsnamespace` — schema;
- `prsstart` — start function;
- `prstoken` — функция выдачи следующего token;
- `prsend` — end function;
- `prsheadline` — headline function или `0`;
- `prslextype` — функция описания lexical token types.

Новый parser должен быть реализован C-level functions, поэтому создавать parsers могут только superusers.

---

## `pg_ts_config`: text search configuration

Configuration выбирает parser и определяет, какие dictionaries применять к token types.

В самой строке `pg_ts_config` находятся:

- `cfgname` — имя;
- `cfgnamespace` — schema;
- `cfgowner` — владелец;
- `cfgparser` → `pg_ts_parser`.

Mapping token types к dictionaries вынесен в отдельный каталог.

---

## `pg_ts_config_map`: token type → dictionaries

Поля:

- `mapcfg` → `pg_ts_config`;
- `maptokentype` — token type parser;
- `mapseqno` — порядок проверки dictionaries;
- `mapdict` → `pg_ts_dict`.

Для одного token type может быть несколько dictionary entries. Они рассматриваются в порядке увеличения `mapseqno`.

Поэтому configuration — это не просто ссылка «parser → dictionary». Между ними есть ordered mapping layer.

---

## `pg_ts_dict`: словари

Dictionary определяет конкретную конфигурацию normalization logic на базе template.

Поля:

- `dictname` — имя;
- `dictnamespace` — schema;
- `dictowner` — владелец;
- `dicttemplate` → `pg_ts_template`;
- `dictinitoption` — initialization options.

Template предоставляет implementation functions, а dictionary — конкретные parameter values для этой implementation.

---

## `pg_ts_template`: заготовка класса dictionaries

Template содержит implementation hooks:

- `tmplname` — имя;
- `tmplnamespace` — schema;
- `tmplinit` — init function или `0`;
- `tmpllexize` — lexize function.

Как и parser, template реализуется C-level functions. Создание новых templates требует superuser privileges.

---

## Связи full-text search одним взглядом

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

Если text search configuration ведёт себя неожиданно, диагностировать её нужно именно по этой цепочке: parser → token type → ordered dictionary mapping → dictionary → template.

---

## Что запомнить

```text
planner statistics:
pg_statistic          → базовая column statistics
pg_stats              → user-facing view
pg_statistic_ext      → extended statistics definition
pg_statistic_ext_data → collected extended statistics data
pg_stats_ext          → user-facing extended stats view

text search:
pg_ts_parser     → tokenization
pg_ts_config     → parser + configuration
pg_ts_config_map → token type → ordered dictionaries
pg_ts_dict       → configured dictionary
pg_ts_template   → dictionary implementation template
```

В статистике главное различать **definition** и **collected data**. В full-text search главное понимать **pipeline**, а не запоминать каталоги как несвязанный список.
