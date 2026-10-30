# Статистика планировщика и полнотекстовый поиск

## 1. Тема

Тема посвящена системным каталогам PostgreSQL и связям между ними: где сервер хранит метаданные объектов и какие поля позволяют восстановить структуру и состояние базы.

```text
основной материал темы
        ↓
понимание механизма
        ↓
важные ограничения и связи
        ↓
факты, которые проверяются тестами
```

## 2. Механизм

Тема состоит из двух независимых частей: статистики, которую использует planner, и каталогов Full Text Search.

### pg_statistic

`ANALYZE` собирает статистику содержимого таблиц, а планировщик использует её для оценки селективности и выбора плана.

Базовые поля:

- `starelid` — таблица/индекс;
- `staattnum` — номер столбца;
- `stanullfrac` — доля `NULL`;
- `stawidth` — средний размер non-NULL значения;
- `stadistinct` — оценка числа различных non-NULL значений.

Также каталог содержит слоты статистики (`stakind*`, `staop*`, `stanumbers*`, `stavalues*`).

Прямое чтение ограничено, потому что статистические значения могут раскрывать конфиденциальные данные. Для обычных пользователей предназначено представление `pg_stats`, которое показывает данные только для доступных объектов.

### Расширенная статистика

#### pg_statistic_ext

Определение объектов `CREATE STATISTICS`.

`stxkind` включает:

```text
d  n-distinct
e  statistics по выражениям
f  functional dependencies
m  most-common-values list
```

#### pg_statistic_ext_data

Вычисленные `ANALYZE` значения расширенной статистики:

- `stxdndistinct`;
- `stxddependencies`;
- `stxdmcv`;
- `stxdexpr`.

Разделение важно: `pg_statistic_ext` — **определение**, `pg_statistic_ext_data` — **собранные данные**.

### Полнотекстовый поиск: общая схема

Конфигурация FTS связывает parser с наборами словарей для разных типов токенов.

Цепочка каталогов:

```text
pg_ts_config
    -> pg_ts_parser
    -> pg_ts_config_map
        -> pg_ts_dict
            -> pg_ts_template
```

### pg_ts_config

Описывает конфигурацию текстового поиска. `cfgparser` ссылается на parser.

### pg_ts_config_map

Для каждой конфигурации и типа токена задаёт порядок словарей:

- `mapcfg` → `pg_ts_config`;
- `maptokentype` — тип токена;
- `mapseqno` — порядок;
- `mapdict` → `pg_ts_dict`.

### pg_ts_parser

Описывает parser, разбивающий документ на токены и назначающий им типы. Каталог хранит ссылки на функции parser-а.

### pg_ts_dict

Словари текстового поиска:

- `dictname`;
- `dictnamespace`;
- `dictowner`;
- `dicttemplate` → `pg_ts_template`;
- `dictinitoption` — строка параметров инициализации.

### pg_ts_template

Шаблон — реализация класса словарей, задающая init/lexize functions. Создание новых templates требует суперпользовательских прав, поскольку они привязаны к функциям низкого уровня.

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.51. pg_statistic

- **Назначение pg_statistic.** pg_statistic хранит базовую статистику столбцов и выражений, которую создаёт ANALYZE и использует planner.
- **starelid, staattnum, stanullfrac, stawidth, stadistinct.** stanullfrac — доля NULL. Эти поля описывают отношение/столбец и ключевые общие статистики.
- **Ограничение доступа и представление pg_stats.** Даже статистика может раскрывать конфиденциальные сведения о значениях данных. Документация приводит пример, что даже min/max или другие статистики могут раскрывать чувствительные данные.
- **Представление pg_stats.** Представление pg_stats. pg_stats — безопаснее доступное view поверх pg_statistic с фильтрацией по правам текущего пользователя.

### 53.52. pg_statistic_ext

- **Назначение и связь с CREATE STATISTICS/ANALYZE.** pg_statistic_ext содержит definition объекта extended statistics; сами вычисленные данные появляются в pg_statistic_ext_data после ANALYZE.
- **Столбец stxkind.** `d` — n-distinct. stxkind перечисляет виды extended statistics: d/e/f/m.

### 53.53. pg_statistic_ext_data

- **Назначение pg_statistic_ext_data.** pg_statistic_ext_data хранит actual data для объектов, определённых в pg_statistic_ext.
- **stxdndistinct, stxddependencies, stxdmcv, stxdexpr.** stxdndistinct — n-distinct. Расширенная статистика сериализуется по отдельным полям для distinct, dependencies, MCV и expressions.

### 53.59. pg_ts_config

- **Назначение pg_ts_config.** Конфигурации текстового поиска, связывающие parser и набор словарей по типам фрагментов. Text search configuration определяет parser, а mapping tokens→dictionaries хранится в pg_ts_config_map.

### 53.60. pg_ts_config_map

- **Связи mapcfg и mapdict в цепочке text search.** pg_ts_config.cfgparser ссылается на pg_ts_parser. Конфигурация использует parser и mapping; mapping указывает dictionaries, а dictionary основан на template.

### 53.62. pg_ts_parser

- **Назначение pg_ts_parser.** pg_ts_parser хранит функции parser lifecycle: start, token, end и дополнительные callbacks.

### 53.63. pg_ts_template

- **Назначение pg_ts_template и ограничение на создание.** pg_ts_dict определяет словари и их параметры. Dictionary настраивает template, а template предоставляет реализационные функции; новые templates требуют superuser.

### 53.61. pg_ts_dict

- **Назначение; dictname; dicttemplate; dictinitoption.** Каталог содержит словари текстового поиска. pg_ts_dict описывает text search dictionaries, их namespace/owner, template и initialization options.

---

# Что запомнить

- **Назначение pg_statistic** → pg_statistic
- **starelid, staattnum, stanullfrac, stawidth, stadistinct** → stanullfrac — доля NULL
- **Ограничение доступа и представление pg_stats** → Даже статистика может раскрывать конфиденциальные сведения о значениях данных
- Представление pg_stats
- **Назначение и связь с CREATE STATISTICS/ANALYZE** → pg_statistic_ext
- **Столбец stxkind** → `d` — n-distinct
- **Назначение pg_statistic_ext_data** → pg_statistic_ext_data
- **stxdndistinct, stxddependencies, stxdmcv, stxdexpr** → stxdndistinct — n-distinct
- **Назначение pg_ts_config** → Конфигурации текстового поиска, связывающие parser и набор словарей по типам фрагментов
- **Связи mapcfg и mapdict в цепочке text search** → pg_ts_config.cfgparser ссылается на pg_ts_parser
- **Назначение pg_ts_parser** → pg_ts_parser
- **Назначение pg_ts_template и ограничение на создание** → pg_ts_dict определяет словари и их параметры
- **Назначение; dictname; dicttemplate; dictinitoption** → Каталог содержит словари текстового поиска
