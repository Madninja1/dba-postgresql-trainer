# Ограничения, зависимости, правила и триггеры

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

Здесь важно понимать, где PostgreSQL хранит связи между объектами и метаданные механизмов, влияющих на DDL/DML.

### pg_constraint

Содержит `CHECK`, `UNIQUE`, `PRIMARY KEY`, `FOREIGN KEY` и `EXCLUDE` constraints.

`contype`:

```text
c  CHECK
f  FOREIGN KEY
p  PRIMARY KEY
u  UNIQUE
x  EXCLUDE
```

`NOT NULL` — особый случай: для обычного столбца он представлен `pg_attribute.attnotnull`, а не строкой `pg_constraint` с `contype='n'`.

Для внешнего ключа важны:

- `conrelid` — таблица с FK;
- `confrelid` — целевая таблица;
- `conkey` — номера ссылающихся столбцов;
- `confkey` — номера целевых столбцов.

### pg_depend

Хранит зависимости между объектами внутри базы.

`deptype` включает:

```text
n  NORMAL
a  AUTO
i  INTERNAL
e  EXTENSION
x  AUTO_EXTENSION
```

По этой информации PostgreSQL определяет, что можно удалить независимо, что должно удаляться автоматически и какие объекты являются внутренней частью других объектов.

### pg_shdepend

Аналог зависимостей для shared-объектов кластера, например ролей. Сам каталог также shared.

### Комментарии

- `pg_description` — комментарии локальных объектов базы;
- `pg_shdescription` — комментарии shared-объектов.

Поле `description` содержит текст комментария.

### pg_rewrite

Хранит правила системы переписывания запросов. Определения представлений также представлены правилами rewrite.

### pg_trigger

Основные поля:

- `tgrelid` — таблица;
- `tgname` — имя триггера;
- `tgfoid` — функция;
- `tgtype` — битовая маска характеристик триггера;
- `tgenabled` — режим включения относительно `session_replication_role`;
- `tgqual` — внутреннее представление `WHEN`-условия.

SQL-текст самой триггерной функции хранится не в `tgtype`, а через связанную функцию в `pg_proc`.

### pg_event_trigger

Event triggers реагируют на DDL-события.

Важные поля:

- `evtevent` — событие;
- `evtfoid` — вызываемая функция;
- `evtenabled` — режим включения;
- `evttags` — фильтр по command tags.

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.13. pg_constraint

- **Назначение каталога.** pg_constraint содержит основные табличные и доменные ограничения, включая CHECK, UNIQUE, PRIMARY KEY, FOREIGN KEY и EXCLUDE.
- **Столбец contype; примечание о NOT NULL.** `c` — CHECK. NOT NULL не представлен как contype в pg_constraint: это свойство хранится в pg_attribute.attnotnull.
- **NOT NULL хранится в pg_attribute.** В pg_attribute.attnotnull. Документация pg_constraint прямо отмечает, что ограничения NULL представлены не здесь, а в pg_attribute.
- **conrelid, confrelid, conkey, confkey.** conrelid — таблица с ограничением. Для FK pg_constraint хранит обе стороны связи и номера соответствующих столбцов.

### 53.18. pg_depend

- **Назначение каталога зависимостей.** pg_depend хранит зависимости объектов базы и позволяет DROP решать, что удалять с CASCADE и когда запрещать удаление при RESTRICT.
- **Поле deptype и виды зависимостей.** `n` — NORMAL. В тексте перечислены NORMAL, AUTO, INTERNAL, PARTITION_PRI/SEC, EXTENSION и AUTO_EXTENSION; `z` там не используется.

### 53.48. pg_shdepend

- **Назначение shared dependencies.** pg_shdepend хранит зависимости объектов баз данных от shared-объектов, таких как роли и tablespaces.

### 53.19. pg_description

- **Назначение и поле description.** pg_description — комментарии объектов базы данных. pg_description относится к объектам базы, а pg_shdescription — к глобальным объектам кластера.

### 53.45. pg_rewrite

- **Назначение каталога rewrite rules.** pg_rewrite содержит правила перезаписи; ev_class указывает отношение, а ev_action хранит дерево действия.

### 53.58. pg_trigger

- **Основные поля tgrelid, tgname, tgfoid, tgenabled, tgqual.** tgrelid указывает таблицу триггера. pg_trigger хранит связь триггера с таблицей и функцией, режим включения, аргументы и условие WHEN.

### 53.21. pg_event_trigger

- **Назначение каталога event triggers.** pg_event_trigger предназначен именно для событийных триггеров.
- **evtevent, evtfoid, evtenabled, evttags.** evtevent — событие. pg_event_trigger содержит событие, функцию, режим включения и список command tags.

### 53.49. pg_shdescription

- **Назначение; objoid; description; shared catalog.** Каталог хранит комментарии к глобальным объектам. pg_shdescription — shared-аналог pg_description для глобальных объектов кластера.

---

# Что запомнить

- **Назначение каталога** → pg_constraint
- **Столбец contype; примечание о NOT NULL** → `c` — CHECK
- **NOT NULL хранится в pg_attribute** → В pg_attribute.attnotnull
- **conrelid, confrelid, conkey, confkey** → conrelid — таблица с ограничением
- **Назначение каталога зависимостей** → pg_depend
- **Поле deptype и виды зависимостей** → `n` — NORMAL
- **Назначение shared dependencies** → pg_shdepend
- **Назначение и поле description** → pg_description — комментарии объектов базы данных
- **Назначение каталога rewrite rules** → pg_rewrite
- **Основные поля tgrelid, tgname, tgfoid, tgenabled, tgqual** → tgrelid указывает таблицу триггера
- **Назначение каталога event triggers** → pg_event_trigger
- **evtevent, evtfoid, evtenabled, evttags** → evtevent — событие
- **Назначение; objoid; description; shared catalog** → Каталог хранит комментарии к глобальным объектам
