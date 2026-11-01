# Каталоги логической репликации

## Ментальная модель

Логическая репликация разделяется на publisher-side и subscriber-side metadata:

```text
publisher database
  ├─ pg_publication
  ├─ pg_publication_namespace
  └─ pg_publication_rel

subscriber cluster
  ├─ pg_subscription
  ├─ pg_subscription_rel
  └─ pg_replication_origin
```

Publication описывает **что публиковать**. Subscription описывает **откуда получать и как применять**. `pg_subscription_rel` отслеживает состояние синхронизации отдельных relations.

---

## `pg_publication`: публикация текущей базы

`pg_publication` содержит publications, созданные в текущей базе данных.

Основные поля:

- `pubname` — имя;
- `pubowner` — владелец;
- `puballtables` — publication автоматически включает все tables базы;
- `pubinsert` — публиковать `INSERT`;
- `pubupdate` — `UPDATE`;
- `pubdelete` — `DELETE`;
- `pubtruncate` — `TRUNCATE`;
- `pubviaroot` — публиковать изменения partition через top-most published ancestor.

Если `puballtables = true`, publication включает не только существующие, но и будущие tables базы автоматически.

Флаги операций независимы: publication может, например, публиковать insert/update, но не delete.

---

## `pg_publication_namespace`: schema → publication

Этот каталог реализует many-to-many mapping между schemas и publications.

- `pnpubid` → publication;
- `pnnspid` → namespace.

Одна publication может включать несколько schemas, а одна schema может участвовать в нескольких publications.

---

## `pg_publication_rel`: relation → publication

`pg_publication_rel` — many-to-many mapping между relations и publications.

Поля:

- `prpubid` → publication;
- `prrelid` → relation;
- `prqual` — internal expression row filter или `NULL`;
- `prattrs` — номера публикуемых columns или `NULL` для всех columns.

Таким образом, publication metadata может ограничивать не только список tables, но и:

```text
строки   → prqual
столбцы  → prattrs
```

`prattrs` использует номера `pg_attribute.attnum`.

---

## `pg_subscription`: subscriptions кластера

`pg_subscription` содержит существующие subscriptions и является shared catalog: одна копия используется всеми базами кластера.

Основные поля:

- `subdbid` — база, в которой существует subscription;
- `subname` — имя;
- `subowner` — владелец;
- `subenabled` — subscription включена;
- `subconninfo` — connection string publisher;
- `subslotname` — replication slot на publisher;
- `subpublications` — массив publication names;
- `subsynccommit` — `synchronous_commit` apply workers;
- `suborigin` — filtering по replication origin.

### Почему `subconninfo` чувствителен

Connection string может содержать пароль в открытом виде. Поэтому обычным пользователям доступ к этому полю ограничен.

При разработке диагностического инструмента нельзя относиться к `subconninfo` как к безобидной metadata-строке.

---

## Дополнительные режимы subscription

### Binary transfer

`subbinary = true` означает запрос данных от publisher в binary representation.

### Streaming in-progress transactions: `substream`

```text
f  streaming in-progress transactions disabled

t  изменения stream-ятся/складываются на диск subscriber,
   затем применяются после получения commit publisher

p  пытаться применять напрямую parallel apply worker;
   если worker недоступен, поведение как t
```

### Two-phase

`subtwophasestate`:

```text
d  disabled
p  pending enable
e  enabled
```

### Ошибки и security context

- `subdisableonerr` — отключать subscription при ошибке worker;
- `subpasswordrequired` — требовать password authentication;
- `subrunasowner` — применять изменения с правами владельца subscription.

---

## `pg_subscription_rel`: состояние каждой relation

Этот каталог связывает subscription и relation и хранит progress/state её начальной синхронизации.

- `srsubid` → subscription;
- `srrelid` → relation;
- `srsubstate` — состояние;
- `srsublsn` — remote LSN для координации состояний `s`/`r`.

Коды:

```text
i  initialization
d  data copy
f  table copy finished
s  synchronized
r  ready / normal replication
```

`pg_subscription_rel` содержит только tables, которые известны subscription после `CREATE SUBSCRIPTION` или `ALTER SUBSCRIPTION ... REFRESH PUBLICATION`.

Поэтому отсутствие relation здесь может означать не «репликация сломана», а то, что subscriber ещё не обновил список publication relations.

---

## `pg_replication_origin`: происхождение replicated changes

`pg_replication_origin` содержит зарегистрированные replication origins и тоже является shared catalog.

Поля:

- `roident` — внутренний идентификатор, уникальный в кластере;
- `roname` — внешнее имя origin, задаваемое пользователем.

`roident` — внутренний ID, который не должен использоваться как переносимый внешний идентификатор между системами.

Replication origin нужен PostgreSQL для отслеживания происхождения изменений и координации replication state.

---

## Как читать publication как набор

Publication может собираться несколькими механизмами:

```text
puballtables = true            → все tables базы
pg_publication_namespace       → schemas
pg_publication_rel             → конкретные relations
                                 + row/column filters
```

Поэтому один только `pg_publication.puballtables` не описывает весь состав publication, если значение `false`.

---

## Практический запрос: relations subscription

```sql
SELECT
    s.subname,
    r.srrelid::regclass AS relation_name,
    r.srsubstate,
    r.srsublsn
FROM pg_subscription AS s
JOIN pg_subscription_rel AS r
  ON r.srsubid = s.oid
ORDER BY s.subname, relation_name;
```

Это даёт subscriber-side картину: какие relations известны каждой subscription и в каком состоянии они находятся.

---

## Что запомнить

```text
publisher:
pg_publication           → publication и operation flags
pg_publication_namespace → schemas in publication
pg_publication_rel       → relations + row/column filters

subscriber:
pg_subscription          → subscription connection/apply settings
pg_subscription_rel      → per-relation sync state
pg_replication_origin    → replication origins
```

Главное различие: publication определяет **набор публикуемых изменений**, subscription — **канал получения**, а `pg_subscription_rel` — **состояние конкретных relations внутри этого канала**.
