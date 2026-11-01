# Ограничения, зависимости, правила и триггеры

## Ментальная модель

Эта группа каталогов отвечает на три разных вопроса:

```text
какие правила целостности заданы объекту?   → pg_constraint
от чего объект зависит и что удалится?       → pg_depend / pg_shdepend
какие серверные механизмы реагируют на SQL?  → pg_rewrite / pg_trigger /
                                               pg_event_trigger
```

Комментарии объектов хранятся отдельно в `pg_description` и `pg_shdescription`.

---

## `pg_constraint`: ограничения

`pg_constraint` хранит:

- `CHECK`;
- `UNIQUE`;
- `PRIMARY KEY`;
- `FOREIGN KEY`;
- `EXCLUDE`;
- constraint triggers;
- domain constraints.

`NOT NULL` — важное исключение: оно представлено не строкой в `pg_constraint`, а флагом `pg_attribute.attnotnull`.

### Тип ограничения

`contype`:

```text
c  CHECK
f  FOREIGN KEY
p  PRIMARY KEY
u  UNIQUE
t  constraint trigger
x  EXCLUDE
```

### К какому объекту относится constraint

- `conrelid` — таблица, если это table constraint;
- `contypid` — domain, если это domain constraint;
- `connamespace` — namespace constraint;
- `conname` — имя, которое не обязано быть глобально уникальным.

### Состояние ограничения

- `condeferrable` — constraint можно откладывать;
- `condeferred` — deferred по умолчанию;
- `convalidated` — существующие данные были проверены.

`convalidated = false` в PostgreSQL 16 возможно для foreign key и `CHECK`.

---

## Индекс, поддерживающий constraint

`conindid` указывает на индекс, связанный с ограничением, когда это применимо: unique, primary key, foreign key или exclusion constraint.

Не следует читать это как «каждый constraint всегда имеет индекс». Например, обычный `CHECK` не требует supporting index.

---

## Как устроен FOREIGN KEY в `pg_constraint`

Для FK особенно важны четыре поля:

```text
conrelid  → таблица, где объявлен FK
conkey    → номера её столбцов
confrelid → referenced table
confkey   → номера referenced columns
```

Номера в `conkey/confkey` соответствуют `pg_attribute.attnum`.

Дополнительные свойства:

- `confupdtype` — действие при update referenced key;
- `confdeltype` — действие при delete referenced key;
- `confmatchtype` — `FULL`, `PARTIAL` или `SIMPLE` в кодированном виде;
- `conpfeqop`, `conppeqop`, `conffeqop` — операторы сравнения, используемые FK.

Коды update/delete action:

```text
a  NO ACTION
r  RESTRICT
c  CASCADE
n  SET NULL
d  SET DEFAULT
```

`confdelsetcols` может задавать конкретные referencing columns, которые меняются при `SET NULL` или `SET DEFAULT`.

### Выражение ограничения

Внутреннее выражение `CHECK` хранится в `conbin`. Для пользовательского вывода предпочтительно использовать:

```sql
pg_get_constraintdef(oid)
```

а не разбирать `pg_node_tree` вручную.

---

## `pg_depend`: граф зависимостей внутри базы

`pg_depend` записывает отношения зависимости между объектами текущей базы данных. Благодаря этим данным PostgreSQL понимает, что запрещать при `DROP RESTRICT` и что удалять при `DROP CASCADE` или автоматически.

Запись содержит две стороны:

```text
зависимый объект                          вышестоящий объект
classid, objid, objsubid   ───────────→   refclassid, refobjid, refobjsubid
```

`objsubid/refobjsubid` используются для подобъекта, например столбца; `0` означает объект целиком.

### `deptype`: не все зависимости одинаковы

#### `n` — NORMAL

Зависимый объект можно удалить отдельно. Вышестоящий объект удалить нельзя без `CASCADE`, если dependent object остаётся.

#### `a` — AUTO

Dependent object можно удалить отдельно, но при удалении referenced object он удаляется автоматически независимо от явного `CASCADE`.

#### `i` — INTERNAL

Dependent object считается внутренней частью другого объекта. Отдельный `DROP` для него запрещается; удалять нужно owning object.

#### `P` / `S` — partition dependencies

Используются для объектов секционирования с несколькими связанными parent objects. `P` — primary, `S` — secondary dependency. Они помогают корректно определять удаление partition-related объектов и формировать сообщения.

#### `e` — EXTENSION

Объект является членом extension. Удалять его следует через `DROP EXTENSION`.

#### `x` — AUTO_EXTENSION

Объект не является членом extension в смысле dump membership, но не может функционировать без extension и автоматически удаляется вместе с ним. При этом его можно удалить отдельно.

У одного объекта может быть несколько записей зависимостей разных типов. Итоговое поведение `DROP` получается из совокупности этих связей.

### Почему некоторых ожидаемых зависимостей нет

Многие объекты, созданные `initdb`, считаются pinned. Они не должны удаляться, поэтому `pg_depend` не обязан хранить обычные записи зависимости от каждого такого системного объекта.

---

## `pg_shdepend`: зависимости от shared objects

`pg_shdepend` решает похожую задачу, но для зависимостей от объектов, общих для кластера, прежде всего ролей и tablespaces.

Поля включают:

- `dbid` — база dependent object либо `0` для global object;
- `classid`, `objid`, `objsubid` — dependent object;
- `refclassid`, `refobjid` — shared referenced object;
- `deptype` — вид shared dependency.

Примеры типов:

```text
o  OWNER
 a ACL
 r POLICY
 t TABLESPACE
```

Так PostgreSQL, например, может определить, что роль ещё владеет объектами или упоминается в ACL и поэтому не может быть просто удалена без разрешения этих зависимостей.

---

## Комментарии: `pg_description` и `pg_shdescription`

Команда `COMMENT` хранит текст в системных каталогах.

### `pg_description`

Комментарии объектов текущей базы:

- `objoid` — объект;
- `classoid` — каталог объекта;
- `objsubid` — номер столбца или `0`;
- `description` — текст.

### `pg_shdescription`

Комментарии shared objects кластера. Каталог сам является shared.

Здесь нет `objsubid`, потому что каталог предназначен для глобальных объектов, описываемых как целые сущности.

---

## `pg_rewrite`: query rewrite rules

`pg_rewrite` хранит правила перезаписи для таблиц и представлений.

Основные поля:

- `rulename` — имя;
- `ev_class` — relation, к которому относится rule;
- `ev_type` — вид события;
- `ev_enabled` — режим `session_replication_role`;
- `is_instead` — `INSTEAD` rule;
- `ev_qual` — условие;
- `ev_action` — действие.

Коды `ev_type`:

```text
1  SELECT
2  UPDATE
3  INSERT
4  DELETE
```

Если для relation существуют rules, `pg_class.relhasrules` должен быть установлен.

---

## `pg_trigger`: триггеры таблиц и представлений

`pg_trigger` содержит trigger metadata.

Ключевые поля:

- `tgrelid` — relation;
- `tgname` — имя trigger, уникальное внутри одной таблицы;
- `tgfoid` — вызываемая функция;
- `tgtype` — bitmask условий срабатывания;
- `tgenabled` — режим `session_replication_role`;
- `tgisinternal` — внутренний trigger;
- `tgconstraint` — связанный `pg_constraint`, если есть;
- `tgnargs` — число аргументов;
- `tgattr` — column numbers для column-specific UPDATE trigger;
- `tgargs` — аргументы trigger function;
- `tgqual` — внутреннее представление `WHEN` condition;
- `tgoldtable`, `tgnewtable` — имена transition tables.

Коды `tgenabled`:

```text
O  origin + local
D  disabled
R  replica
A  always
```

Если у relation есть triggers, `pg_class.relhastriggers` должен быть установлен.

Для восстановления читаемого определения есть `pg_get_triggerdef()`.

---

## `pg_event_trigger`: event triggers

Event trigger реагирует не на изменение строки таблицы, а на события DDL-инфраструктуры.

`pg_event_trigger` содержит:

- `evtname` — имя;
- `evtevent` — event;
- `evtowner` — владелец;
- `evtfoid` — вызываемая функция;
- `evtenabled` — режим срабатывания;
- `evttags` — список command tags либо `NULL`, если фильтра нет.

`evtenabled` использует те же коды `O/D/R/A`, что и обычные triggers.

---

## Практический способ читать constraint

```sql
SELECT
    c.conname,
    c.contype,
    c.conrelid::regclass AS relation_name,
    pg_get_constraintdef(c.oid) AS definition
FROM pg_constraint AS c
WHERE c.conrelid = 'public.orders'::regclass;
```

Этот запрос показывает важный принцип: catalog columns нужны для машинного анализа и связей, а `pg_get_*def()` — для человекочитаемого DDL-представления.

---

## Что запомнить

```text
pg_constraint    → integrity constraints
pg_attribute     → NOT NULL
pg_depend        → dependencies внутри базы
pg_shdepend      → dependencies на shared objects
pg_description   → comments объектов базы
pg_shdescription → comments shared objects
pg_rewrite       → query rewrite rules
pg_trigger       → row/statement triggers
pg_event_trigger → event triggers
```

Главное различие: constraint описывает **правило целостности**, dependency — **отношение жизненного цикла между объектами**, а trigger/rule — **механизм реакции или переписывания SQL**.
