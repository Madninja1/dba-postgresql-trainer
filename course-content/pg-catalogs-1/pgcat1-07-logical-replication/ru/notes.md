# Каталоги логической репликации

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

Логическая репликация PostgreSQL построена вокруг публикаций, подписок и состояния синхронизации отдельных отношений.

### pg_publication

Публикации текущей базы.

Флаги операций:

- `pubinsert`;
- `pubupdate`;
- `pubdelete`;
- `pubtruncate`.

`puballtables=true` означает публикацию всех таблиц, включая подходящие таблицы, созданные позже.

### pg_publication_namespace

Связь многие-ко-многим между схемами и публикациями. Используется, когда публикация охватывает таблицы схемы.

### pg_publication_rel

Связь многие-ко-многим между отношениями и публикациями.

Дополнительно может хранить:

- `prqual` — row filter;
- `prattrs` — ограничение набора публикуемых столбцов.

### pg_subscription

Содержит подписки и является shared catalog, потому что subscription worker должен находить подписки всего кластера.

Полезные поля:

- `subenabled` — подписка включена;
- `subconninfo` — строка подключения к publisher;
- `subslotname` — replication slot;
- `subpublications` — массив имён публикаций;
- `substream` — режим streaming больших транзакций.

Доступ к `subconninfo` ограничен, потому что строка подключения может содержать пароль в открытом виде.

#### substream

Режимы включают:

```text
f  не stream-ить изменения выполняющейся транзакции
t  сохранять stream на диск и применять после commit publisher
p  применять через parallel apply worker, если доступен
```

### pg_subscription_rel

Хранит состояние каждого отношения в каждой подписке.

`srsubstate` проходит стадии:

```text
i  initialize
d  copying data
f  table copy finished
s  synchronized
r  ready / normal replication
```

### pg_replication_origin

Содержит зарегистрированные replication origins и является shared catalog.

- `roident` — внутренний идентификатор, уникальный в кластере;
- `roname` — внешнее пользовательское имя.

`roident` — внутренний ID, не предназначенный как стабильный внешний идентификатор между системами.

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.40. pg_publication

- **Назначение pg_publication.** pg_publication хранит публикации логической репликации.
- **pubinsert, pubupdate, pubdelete, pubtruncate.** pubinsert/pubupdate/pubdelete/pubtruncate отвечают за типы DML; puballtables отдельно означает автоматическое включение всех таблиц.
- **Столбец puballtables.** Публикация автоматически включает все таблицы базы, включая созданные в будущем. puballtables=true включает все таблицы базы и будущие таблицы.

### 53.42. pg_publication_rel

- **Назначение; prqual; prattrs.** pg_publication_namespace — many-to-many связь схем и публикаций. Эти каталоги связывают публикации со схемами и отношениями; pg_publication_rel также хранит row/column filters.

### 53.54. pg_subscription

- **Назначение pg_subscription.** pg_subscription хранит subscriptions и является shared catalog.
- **Shared catalog; subenabled; subslotname; subpublications; subconninfo.** Каталог разделяется всеми базами кластера. pg_subscription shared между базами. subconninfo может содержать пароль, поэтому обычным пользователям доступ к нему ограничен.
- **Ограничение доступа к subconninfo.** Строка подключения может содержать пароль в открытом виде. Документация прямо предупреждает, что subconninfo может содержать открытый пароль.
- **Столбец substream.** `f` — не stream-ить выполняющиеся транзакции. substream управляет streaming in-progress transactions с кодами f/t/p.

### 53.55. pg_subscription_rel

- **Назначение pg_subscription_rel.** Состояние каждого реплицируемого отношения в каждой подписке. pg_subscription_rel — many-to-many связь subscription/relations с состоянием синхронизации.
- **Столбец srsubstate.** `i` — initialization. srsubstate кодирует стадии подготовки relation к обычной logical replication.

### 53.44. pg_replication_origin

- **Назначение pg_replication_origin.** pg_replication_origin содержит зарегистрированные replication origins и разделяется всеми базами кластера.
- **roident и roname.** roident — уникальный в кластере внутренний идентификатор. roident уникален внутри кластера и, по документации, не должен покидать пределы системы; roname — внешнее имя.

### 53.41. pg_publication_namespace

- **Назначение; pnpubid; pnnspid.** Сопоставление схем с публикациями вида многие-ко-многим. pg_publication_namespace связывает публикации и схемы через pnpubid и pnnspid.

---

# Что запомнить

- **Назначение pg_publication** → pg_publication
- **pubinsert, pubupdate, pubdelete, pubtruncate** → pubinsert
- **Столбец puballtables** → Публикация автоматически включает все таблицы базы, включая созданные в будущем
- **Назначение; prqual; prattrs** → pg_publication_namespace — many-to-many связь схем и публикаций
- **Назначение pg_subscription** → pg_subscription
- **Shared catalog; subenabled; subslotname; subpublications; subconninfo** → Каталог разделяется всеми базами кластера
- **Ограничение доступа к subconninfo** → Строка подключения может содержать пароль в открытом виде
- **Столбец substream** → `f` — не stream-ить выполняющиеся транзакции
- **Назначение pg_subscription_rel** → Состояние каждого реплицируемого отношения в каждой подписке
- **Столбец srsubstate** → `i` — initialization
- **Назначение pg_replication_origin** → pg_replication_origin
- **roident и roname** → roident — уникальный в кластере внутренний идентификатор
- **Назначение; pnpubid; pnnspid** → Сопоставление схем с публикациями вида многие-ко-многим
