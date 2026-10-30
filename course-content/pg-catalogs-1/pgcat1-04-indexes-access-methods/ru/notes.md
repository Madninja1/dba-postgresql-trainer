# Индексы и методы доступа

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

Метаданные индексов распределены между несколькими каталогами: общий объект живёт в `pg_class`, свойства индекса — в `pg_index`, а семантика метода доступа — в `pg_am`, opclass/opfamily и связанных каталогах.

### pg_am

Описывает методы доступа отношений.

`amtype`:

```text
t  table access method
i  index access method
```

### pg_opfamily и pg_opclass

`pg_opfamily` — семейство совместимых операторов и support procedures для метода доступа.

`pg_opclass` связывает индексируемый тип данных с методом доступа и семейством операторов.

Ключевые ссылки:

- `opcmethod` → `pg_am`;
- `opcfamily` → `pg_opfamily`.

### pg_amop

Содержит операторы семейства метода доступа.

`amoppurpose`:

```text
s  search
o  ordering
```

Search-оператор используется для условий поиска по индексу; ordering-оператор описывает индексное упорядочивание.

### pg_amproc

Содержит опорные функции семейств операторов. `amprocnum` задаёт номер support function, `amproc` — OID функции.

### pg_index

Дополняет запись индекса из `pg_class`.

Полезные флаги:

- `indisunique` — unique;
- `indisprimary` — PRIMARY KEY;
- `indisvalid` — индекс разрешено использовать в запросах;
- `indisready` — индекс готов принимать новые записи;
- `indislive` — индекс не находится в процессе удаления.

`indkey` хранит номера столбцов таблицы. Ноль в позиции означает, что соответствующий атрибут индекса задан **выражением**, а не простой ссылкой на столбец.

### pg_partitioned_table

Хранит сведения о ключе секционирования:

- `partstrat`: `h` hash, `l` list, `r` range;
- `partnatts` — число элементов ключа;
- `partattrs` — номера столбцов, `0` для выражения.

### pg_inherits

Непосредственные связи parent-child для наследования таблиц и индексов:

- `inhrelid` — потомок;
- `inhparent` — родитель;
- `inhseqno` — порядок родителя при множественном наследовании;
- `inhdetachpending` — секция в процессе отсоединения.

#### Практический запрос

```sql
SELECT c.relname,
       i.indisunique,
       i.indisprimary,
       i.indisvalid
FROM pg_index AS i
JOIN pg_class AS c ON c.oid = i.indexrelid;
```

---

## 3. Важные детали и покрытие тестов

Ниже собраны детали, которые проверяются вопросами этой темы. Они сгруппированы по разделам исходной документации, чтобы их можно было связывать с механизмом, а не запоминать как отдельный список ответов.

### 53.3. pg_am

- **Назначение pg_am.** Методы доступа отношений. pg_am содержит по строке на метод доступа, поддерживаемый системой; в текущем описании методы задаются для таблиц и индексов.
- **Столбец amtype.** `t` — табличный метод доступа. amtype различает table (`t`) и index (`i`) access methods.

### 53.4. pg_amop

- **Назначение pg_amop.** Операторы, являющиеся членами семейств операторов методов доступа. pg_amop хранит операторы семейств методов доступа; оператор может быть поисковым или упорядочивающим.
- **Столбец amoppurpose.** `s` — search. amoppurpose использует `s` для поиска и `o` для упорядочивания.

### 53.5. pg_amproc

- **Назначение pg_amproc.** pg_amproc описывает support functions, принадлежащие семействам операторов методов доступа.

### 53.33. pg_opclass

- **Назначение pg_opclass и ссылки opcmethod/opcfamily.** pg_opclass связывает индексируемый тип данных с методом доступа и семейством операторов. Operator class задаёт применение семейства для типа/метода доступа, а operator family объединяет совместимые операторы и support procedures.

### 53.26. pg_index

- **Назначение pg_index.** pg_index содержит специфические свойства индекса, тогда как общая информация об отношении находится в pg_class.
- **indisunique, indisprimary, indisvalid, indisready, indislive.** indisunique — уникальный индекс. Эти поля позволяют различать уникальность/primary key и фазы готовности, валидности и удаления индекса.
- **Столбец indkey.** Соответствующий атрибут индекса задаётся выражением, а не простой ссылкой на столбец. В indkey обычные номера соответствуют pg_attribute.attnum, а 0 указывает на expression index attribute.

### 53.37. pg_partitioned_table

- **partstrat, partnatts, partattrs.** Информация о ключах секционирования. pg_partitioned_table описывает стратегию и состав partition key.

### 53.27. pg_inherits

- **Назначение pg_inherits.** pg_inherits содержит по записи на каждую непосредственную связь parent-child.
- **inhrelid, inhparent, inhseqno, inhdetachpending.** inhrelid — OID дочерней таблицы/индекса. pg_inherits хранит child, parent, sequence number и флаг pending detach.

### 53.35. pg_opfamily

- **Назначение и связь с pg_amop/pg_amproc/pg_opclass.** Семейство совместимых операторов и support procedures для метода доступа. pg_opfamily определяет operator families; основная информация семейства также находится в связанных строках pg_amop, pg_amproc и pg_opclass.

---

# Что запомнить

- **Назначение pg_am** → Методы доступа отношений
- **Столбец amtype** → `t` — табличный метод доступа
- **Назначение pg_amop** → Операторы, являющиеся членами семейств операторов методов доступа
- **Столбец amoppurpose** → `s` — search
- **Назначение pg_amproc** → pg_amproc
- **Назначение pg_opclass и ссылки opcmethod/opcfamily** → pg_opclass связывает индексируемый тип данных с методом доступа и семейством операторов
- **Назначение pg_index** → pg_index
- **indisunique, indisprimary, indisvalid, indisready, indislive** → indisunique — уникальный индекс
- **Столбец indkey** → Соответствующий атрибут индекса задаётся выражением, а не простой ссылкой на столбец
- **partstrat, partnatts, partattrs** → Информация о ключах секционирования
- **Назначение pg_inherits** → pg_inherits
- **inhrelid, inhparent, inhseqno, inhdetachpending** → inhrelid — OID дочерней таблицы/индекса
- **Назначение и связь с pg_amop/pg_amproc/pg_opclass** → Семейство совместимых операторов и support procedures для метода доступа
