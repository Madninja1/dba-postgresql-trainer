# Расширения, FDW и большие объекты

## Ментальная модель

Тема объединяет три относительно независимых механизма:

```text
extension metadata     → pg_extension
foreign data access    → pg_foreign_data_wrapper
                          ↓
                        pg_foreign_server
                          ↓
                        pg_foreign_table
                          +
                        pg_user_mapping
large objects          → pg_largeobject_metadata + pg_largeobject
```

Ключ к пониманию — не смешивать **контейнер метаданных расширения**, **цепочку FDW** и **страничное хранение large object**.

---

## `pg_extension`: установленное расширение

`pg_extension` содержит информацию об установленных extensions.

Основные поля:

- `extname` — имя extension;
- `extowner` — владелец;
- `extnamespace` — schema, содержащая экспортируемые extension objects;
- `extrelocatable` — можно ли переместить extension в другую schema;
- `extversion` — установленная версия;
- `extconfig` — OID configuration tables extension;
- `extcondition` — filters для этих configuration tables.

### Что на самом деле означает `extnamespace`

Extension **не принадлежит schema как schema-qualified object**. Имена extensions не квалифицируются schema.

`extnamespace` означает schema, в которой находятся все или большинство экспортируемых extension objects. Если `extrelocatable = true`, эта schema должна содержать все schema-aware objects extension.

Поэтому неверная модель:

```text
schema.extension_name
```

Правильнее:

```text
extension
  └─ exported objects live primarily in extnamespace
```

---

## Foreign Data Wrapper: четыре уровня

### 1. `pg_foreign_data_wrapper`

Это описание самого FDW mechanism.

Поля:

- `fdwname` — имя wrapper;
- `fdwowner` — владелец;
- `fdwhandler` — handler function или `0`;
- `fdwvalidator` — options validator или `0`;
- `fdwacl` — ACL;
- `fdwoptions` — wrapper-level options.

### 2. `pg_foreign_server`

Foreign server описывает конкретный внешний источник, доступный через FDW.

- `srvname` — имя server;
- `srvowner` — владелец;
- `srvfdw` → `pg_foreign_data_wrapper.oid`;
- `srvtype`, `srvversion` — необязательные metadata;
- `srvacl` — ACL;
- `srvoptions` — server-level options.

Связь:

```text
foreign server --uses--> foreign data wrapper
```

### 3. `pg_foreign_table`

Foreign table уже имеет обычную строку в `pg_class`, как relation с `relkind = 'f'`. `pg_foreign_table` хранит только дополнительные свойства, специфичные для foreign table.

- `ftrelid` → `pg_class.oid` foreign table;
- `ftserver` → `pg_foreign_server.oid`;
- `ftoptions` — table-level options.

То есть:

```text
pg_class        → общее описание relation
pg_foreign_table→ foreign-specific properties
```

### 4. `pg_user_mapping`

User mapping связывает локальную роль с foreign server.

- `umuser` — local role или `0` для mapping «для всех»;
- `umserver` → `pg_foreign_server`;
- `umoptions` — mapping-specific options.

Каталог может содержать чувствительные connection options, поэтому обычным пользователям прямой доступ к нему не предоставляется. Для них предназначено представление `pg_user_mappings`.

---

## Где живут FDW options

У каждого уровня свои options:

```text
pg_foreign_data_wrapper.fdwoptions  → wrapper
pg_foreign_server.srvoptions        → server
pg_foreign_table.ftoptions          → foreign table
pg_user_mapping.umoptions           → user mapping
```

Это не четыре копии одного и того же поля, а четыре уровня конфигурации.

При диагностике полезно идти сверху вниз:

```text
wrapper
  ↓
server
  ↓
foreign table
  +
user mapping
```

---

## `pg_largeobject`: данные large object

Large object в PostgreSQL идентифицируется OID и физически разбивается на страницы, которые помещаются в строки `pg_largeobject`.

Поля:

- `loid` — OID large object;
- `pageno` — номер страницы, начиная с `0`;
- `data` — `bytea` с данными этой страницы.

Размер страницы large object задаётся `LOBLKSIZE`, который в обычной конфигурации равен `BLCKSZ/4`.

Смещение данных страницы:

```text
pageno * LOBLKSIZE
```

Хранилище может быть sparse: страницы могут отсутствовать или быть короче обычного размера. Пропущенные области читаются как нулевые байты.

---

## `pg_largeobject_metadata`: владелец и ACL

Данные и metadata large object разделены:

```text
pg_largeobject          → pages/data
pg_largeobject_metadata → owner + ACL
```

`pg_largeobject_metadata` содержит:

- `oid` — идентификатор large object;
- `lomowner` — владелец;
- `lomacl` — права доступа.

Такое разделение связано в том числе с безопасностью: прямое чтение содержимого large objects не должно автоматически давать возможность перечислять/читать все объекты без проверки прав.

---

## Практическая навигация по FDW

```sql
SELECT
    ft.ftrelid::regclass AS foreign_table,
    s.srvname,
    w.fdwname
FROM pg_foreign_table AS ft
JOIN pg_foreign_server AS s
  ON s.oid = ft.ftserver
JOIN pg_foreign_data_wrapper AS w
  ON w.oid = s.srvfdw;
```

Этот запрос показывает естественное направление связей:

```text
foreign table → server → wrapper
```

User mappings присоединяются отдельно по `umserver` и роли.

---

## Что запомнить

```text
pg_extension → installed extension metadata
extnamespace → schema экспортируемых объектов, а не schema-qualified extension name

FDW chain:
pg_foreign_data_wrapper
        ↓
pg_foreign_server
        ↓
pg_foreign_table
        +
pg_user_mapping

large object:
pg_largeobject_metadata → owner/ACL
pg_largeobject          → data pages
```

При диагностике FDW сначала выясните, на каком уровне находится свойство: wrapper, server, table или user mapping. При работе с large objects отдельно рассматривайте metadata и сами binary pages.
