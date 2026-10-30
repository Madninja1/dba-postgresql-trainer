# Расширения, FDW и большие объекты

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

Тема объединяет три механизма: extensions, Foreign Data Wrappers и large objects.

### pg_extension

Содержит установленные расширения.

Основные поля:

- `extname` — имя;
- `extowner` — владелец;
- `extnamespace` — схема, содержащая все или большинство экспортируемых объектов;
- `extrelocatable` — можно ли переместить расширение;
- `extversion` — версия.

Важно: `extnamespace` **не означает**, что само имя расширения schema-qualified. Расширение — объект уровня базы, а поле лишь указывает связанную схему объектов.

### Цепочка FDW

Модель состоит из нескольких каталогов:

1. `pg_foreign_data_wrapper` — реализация wrapper;
2. `pg_foreign_server` — конкретный внешний сервер;
3. `pg_foreign_table` — свойства foreign table;
4. `pg_user_mapping` — сопоставление локальной роли с внешним сервером.

#### pg_foreign_server

`srvfdw` указывает используемый FDW. Также есть `srvname`, `srvowner`, `srvoptions`.

#### pg_foreign_table

- `ftrelid` — OID записи foreign table в `pg_class`;
- `ftserver` — внешний сервер;
- `ftoptions` — параметры таблицы.

#### pg_user_mapping

- `umuser` — локальная роль или 0 для PUBLIC;
- `umserver` — foreign server;
- `umoptions` — параметры, потенциально содержащие чувствительные данные.

Из-за возможных секретов прямой доступ к каталогу ограничен; для пользователей есть представление `pg_user_mappings`, скрывающее недоступные options.

### Где лежат options

```text
pg_foreign_data_wrapper.fdwoptions
pg_foreign_server.srvoptions
pg_foreign_table.ftoptions
pg_user_mapping.umoptions
```

Они не хранятся централизованно в `postgresql.conf`.

### Large Objects

#### pg_largeobject

Содержит **данные** large object, разбитые на страницы:

- `loid` — идентификатор объекта;
- `pageno` — номер страницы, начиная с 0;
- `data` — `bytea`.

#### pg_largeobject_metadata

Содержит метаданные large object:

- OID объекта;
- владелец;
- ACL.

Обычные пользователи не должны обходить SQL/large-object API прямым изменением этих каталогов.

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.22. pg_extension

- **Назначение pg_extension.** pg_extension содержит установленные extensions и их свойства.
- **extname, extowner, extnamespace, extrelocatable, extversion.** extname — имя расширения. pg_extension хранит имя, владельца, namespace, relocatability, version и config-related fields.
- **Примечание о extnamespace.** Он не означает, что само расширение принадлежит схеме; имена расширений не schema-qualified. Документация отдельно отмечает: extnamespace показывает место большинства/всех schema-bound объектов, но extension name не дополняется схемой.

### 53.25. pg_foreign_table

- **Связь foreign table с pg_class и foreign server.** pg_foreign_data_wrapper описывает FDW-механизм. FDW-объекты распределены по каталогам wrapper → server → foreign table/user mapping.
- **ftrelid, ftserver, ftoptions.** ftrelid — OID записи pg_class для foreign table. pg_foreign_table хранит специфические свойства foreign table, дополняя основную запись в pg_class.

### 53.24. pg_foreign_server

- **Столбец srvfdw.** srvfdw — OID pg_foreign_data_wrapper для данного сервера.

### 53.65. pg_user_mapping

- **Назначение pg_user_mapping.** Сопоставления локальных пользователей/ролей с удалёнными через foreign server. pg_user_mapping содержит local-to-remote user mappings для foreign servers.
- **Ограничение доступа и pg_user_mappings.** Обычные пользователи не имеют прямого доступа к каталогу. Документация направляет обычных пользователей к view pg_user_mappings.

### 53.30. pg_largeobject

- **Назначение pg_largeobject.** pg_largeobject хранит страницы данных large objects; metadata вынесены в отдельный каталог.
- **loid, pageno, data.** loid — идентификатор large object. Строка pg_largeobject представляет страницу large object через loid, pageno и data.

### 53.31. pg_largeobject_metadata

- **lomowner и lomacl.** pg_largeobject_metadata содержит lomowner и lomacl; собственные bytes находятся в pg_largeobject.

### 53.23. pg_foreign_data_wrapper

- **fdwoptions и структура FDW.** pg_foreign_data_wrapper.fdwoptions хранит параметры wrapper. Каждый уровень FDW имеет собственный массив options в соответствующем каталоге.

---

# Что запомнить

- **Назначение pg_extension** → pg_extension
- **extname, extowner, extnamespace, extrelocatable, extversion** → extname — имя расширения
- **Примечание о extnamespace** → Он не означает, что само расширение принадлежит схеме; имена расширений не schema-qualified
- **Связь foreign table с pg_class и foreign server** → pg_foreign_data_wrapper описывает FDW-механизм
- **Столбец srvfdw** → srvfdw
- **ftrelid, ftserver, ftoptions** → ftrelid — OID записи pg_class для foreign table
- **Назначение pg_user_mapping** → Сопоставления локальных пользователей/ролей с удалёнными через foreign server
- **Ограничение доступа и pg_user_mappings** → Обычные пользователи не имеют прямого доступа к каталогу
- **Назначение pg_largeobject** → pg_largeobject
- **loid, pageno, data** → loid — идентификатор large object
- **lomowner и lomacl** → pg_largeobject_metadata
- **fdwoptions и структура FDW** → pg_foreign_data_wrapper.fdwoptions хранит параметры wrapper
