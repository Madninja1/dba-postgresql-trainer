# Strong — продвинутый PL/pgSQL

## 1. Тема

На Strong-уровне важно не просто знать синтаксис PL/pgSQL, а понимать его **runtime-семантику**:

- когда SQL параметризуется автоматически, а когда нужен dynamic `EXECUTE`;
- почему идентификатор нельзя передать через `$1`;
- как PL/pgSQL кеширует статические планы;
- почему `EXECUTE` иногда сознательно используют ради нового плана;
- как устроен rollback внутри `EXCEPTION`;
- чем текущая диагностика отличается от диагностики перехваченной ошибки;
- какие данные доступны trigger function и что означает её возвращаемое значение;
- где разрешены `COMMIT`/`ROLLBACK`;
- как избежать конфликтов имён переменных и столбцов.

Это уже не набор команд, а модель исполнения серверного кода.

---

## 2. Механизм

### 2.1. Статический SQL: значения становятся параметрами

Рассмотрим функцию:

```sql
CREATE FUNCTION count_users(p_status text)
RETURNS bigint
LANGUAGE plpgsql
AS $$
DECLARE
    v_count bigint;
BEGIN
    SELECT count(*)
    INTO v_count
    FROM users
    WHERE status = p_status;

    RETURN v_count;
END;
$$;
```

`p_status` в оптимизируемой SQL-команде используется как значение параметра. PL/pgSQL не вставляет его как произвольный фрагмент SQL-текста.

Это работает для команд, где SQL-движок допускает query parameters, в частности для `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `MERGE` и некоторых команд, содержащих их.

Но query parameter может представлять **значение**, а не синтаксический объект SQL.

Поэтому параметр подходит для:

```sql
WHERE id = p_id
```

но не может напрямую заменить:

```text
имя таблицы
имя столбца
ORDER BY <произвольный идентификатор>
```

Если меняется структура команды, нужен `EXECUTE`.

### 2.2. Dynamic `EXECUTE`: план каждый раз новый

Форма:

```sql
EXECUTE command_text
    [ INTO [STRICT] target ]
    [ USING expression [, ...] ];
```

Для команды через PL/pgSQL `EXECUTE` нет кешированного плана между выполнениями. Текст разбирается и план строится заново каждый раз.

Это одновременно:

- цена dynamic SQL;
- его важное свойство.

Если оптимальный план сильно зависит от текущих значений параметров, явный `EXECUTE` иногда применяют даже при неизменном тексте, чтобы заставить PostgreSQL перепланировать запрос с учётом текущих значений.

Например:

```sql
EXECUTE
    'SELECT * FROM events WHERE tenant_id = $1 AND created_at >= $2'
USING p_tenant_id, p_since;
```

### 2.3. `USING`: только значения данных

Параметры `$1`, `$2`, ... внутри `EXECUTE` соответствуют выражениям `USING`:

```sql
EXECUTE
    'UPDATE accounts SET balance = balance + $1 WHERE id = $2'
USING p_delta, p_id;
```

Они:

- передают значения данных;
- сохраняют их типы без обязательного преобразования в текст;
- избавляют от ручного quoting значений;
- работают только там, где выполняемая SQL-команда принимает query parameters.

Но они не параметризуют идентификаторы:

```sql
-- неверная идея
EXECUTE 'SELECT * FROM $1' USING table_name;
```

`$1` здесь не может стать именем таблицы.

### 2.4. `format()` для динамических идентификаторов

Правильная комбинация:

```sql
EXECUTE format(
    'SELECT count(*) FROM %I WHERE tenant_id = $1',
    table_name
)
INTO v_count
USING p_tenant_id;
```

Здесь роли разделены:

```text
%I     → SQL identifier
$1     → data value
```

`%I` эквивалентен `quote_ident`.

Для значения, которое всё же нужно вставить как SQL literal в текст, есть `%L`, эквивалентный `quote_nullable`:

```sql
EXECUTE format(
    'SELECT * FROM %I WHERE category = %L',
    table_name,
    category
);
```

Если можно использовать `USING`, обычно это предпочтительнее `%L` для data values.

### 2.5. `quote_literal` и `quote_nullable`

`quote_literal(value)` создаёт строковый SQL literal, но сама функция `STRICT`: для `NULL` она возвращает `NULL`.

При конкатенации это опасно:

```sql
'... ' || quote_literal(nullable_value)
```

Если `nullable_value IS NULL`, весь результат конкатенации может стать `NULL`, и `EXECUTE` не получит корректной command string.

Для значений, допускающих `NULL`, используется:

```sql
quote_nullable(value)
```

или `%L` в `format()`.

Но помните SQL-семантику `NULL`: строка

```sql
WHERE key = NULL
```

не становится истинной. Если `NULL` должен считаться обычным сравниваемым значением, может понадобиться `IS NOT DISTINCT FROM`.

### 2.6. Dollar quoting не экранирует неизвестные dynamic values

Dollar quoting отлично подходит для **фиксированного текста тела функции**:

```sql
CREATE FUNCTION f()
RETURNS void
LANGUAGE plpgsql
AS $func$
BEGIN
    ...
END;
$func$;
```

Но нельзя считать безопасной такую идею:

```text
взять неизвестный пользовательский текст
→ обрамить его $$...$$
→ вставить в dynamic SQL
```

Неизвестное значение само может содержать выбранный dollar delimiter.

Для dynamic values используйте:

- `USING` — предпочтительно для данных;
- `%I` / `quote_ident` — для идентификаторов;
- `%L` / `quote_nullable` — если значение действительно вставляется текстом.

Dollar quotes решают проблему quoting **исходного фиксированного PL/pgSQL-кода**, а не проблему SQL injection.

### 2.7. `FOUND` и `EXECUTE`

Обычный dynamic `EXECUTE` **не меняет `FOUND`**.

Это важное отличие от ряда статических операций.

```sql
EXECUTE 'UPDATE jobs SET status = $1 WHERE id = $2'
USING 'done', p_id;
```

После этой команды нельзя считать, что `FOUND` автоматически отражает результат `UPDATE`.

Для числа строк используйте диагностику:

```sql
GET DIAGNOSTICS v_rows = ROW_COUNT;
```

Dynamic `EXECUTE` обновляет данные, доступные через `GET DIAGNOSTICS`, но не специальную переменную `FOUND`.

### 2.8. `GET DIAGNOSTICS`: текущее состояние

Пример:

```sql
UPDATE jobs
SET status = 'done'
WHERE worker_id = p_worker_id;

GET DIAGNOSTICS v_rows = ROW_COUNT;
```

Полезные элементы текущей диагностики включают:

```text
ROW_COUNT       → число строк, обработанных последней SQL-командой
PG_CONTEXT      → текущий стек вызовов
PG_ROUTINE_OID  → OID текущей функции
```

`PG_CONTEXT` показывает место, где выполняется сама `GET DIAGNOSTICS`, и вызывающие функции выше по стеку.

### 2.9. `GET STACKED DIAGNOSTICS`: состояние перехваченной ошибки

Внутри обработчика `EXCEPTION` нужна другая команда:

```sql
GET STACKED DIAGNOSTICS
    v_message = MESSAGE_TEXT,
    v_detail  = PG_EXCEPTION_DETAIL,
    v_hint    = PG_EXCEPTION_HINT;
```

Она получает данные **ошибки, которая привела в текущий обработчик**, а не текущее нормальное состояние выполнения.

Доступны, в частности:

```text
RETURNED_SQLSTATE
MESSAGE_TEXT
PG_EXCEPTION_DETAIL
PG_EXCEPTION_HINT
PG_EXCEPTION_CONTEXT
TABLE_NAME
SCHEMA_NAME
COLUMN_NAME
CONSTRAINT_NAME
PG_DATATYPE_NAME
```

В обработчике также доступны специальные переменные:

```text
SQLSTATE → код перехваченной ошибки
SQLERRM  → текст сообщения
```

Разница:

```text
GET DIAGNOSTICS          → текущее состояние выполнения
GET STACKED DIAGNOSTICS  → информация о перехваченной ошибке
```

### 2.10. Семантика `EXCEPTION`: данные откатываются, переменные нет

Блок с обработчиком ошибок имеет специальную семантику:

```sql
BEGIN
    statements
EXCEPTION
    WHEN condition THEN
        handler
END;
```

Если внутри выполняемой части блока возникает ошибка:

1. дальнейшие операторы этой части не выполняются;
2. изменения persistent database state, сделанные внутри блока, откатываются;
3. локальные переменные PL/pgSQL сохраняют значения, полученные к моменту ошибки;
4. выбирается первый подходящий `WHEN`;
5. после обработчика выполнение продолжается после `END` блока.

Пример:

```sql
x := 1;

BEGIN
    UPDATE t SET value = 100 WHERE id = 1;
    x := 2;
    y := 10 / 0;
EXCEPTION
    WHEN division_by_zero THEN
        RAISE NOTICE 'x = %', x;
END;
```

В обработчике `x = 2`, но изменение таблицы внутри блока отменено.

Это не `COMMIT` внутренней работы — наоборот, база возвращается к состоянию на входе в ошибочный блок.

### 2.11. Цена `EXCEPTION`

Вход и выход из блока с `EXCEPTION` дороже обычного блока. Поэтому не стоит механически оборачивать каждый фрагмент функции в обработчик «на всякий случай».

Используйте `EXCEPTION`, когда действительно собираетесь обработать ожидаемую категорию ошибки или добавить полезный контекст.

### 2.12. Trigger function: контракт с trigger manager

Триггерная функция изменения данных объявляется:

```sql
CREATE FUNCTION audit_row()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    ...
END;
$$;
```

У неё нет обычных аргументов функции. Аргументы `CREATE TRIGGER` доступны через `TG_ARGV`.

PostgreSQL автоматически создаёт специальные переменные контекста.

Основные:

```text
NEW             новая строка для INSERT/UPDATE row-level trigger
OLD             старая строка для UPDATE/DELETE row-level trigger
TG_NAME         имя триггера
TG_WHEN         BEFORE / AFTER / INSTEAD OF
TG_LEVEL        ROW / STATEMENT
TG_OP           INSERT / UPDATE / DELETE / TRUNCATE
TG_TABLE_NAME   имя таблицы
TG_TABLE_SCHEMA схема таблицы
TG_NARGS        число аргументов trigger
TG_ARGV         массив аргументов trigger, индексируется с 0
```

`NEW` и `OLD` существуют как `record`, но их содержимое зависит от операции:

```text
INSERT → NEW есть, OLD = NULL
UPDATE → NEW и OLD есть
DELETE → OLD есть, NEW = NULL
```

Для statement-level trigger строкового значения `NEW`/`OLD` как отдельной изменяемой строки нет.

### 2.13. Возвращаемое значение BEFORE row trigger

Для row-level `BEFORE` trigger результат функции влияет на дальнейшую операцию.

Если вернуть `NULL`:

```sql
RETURN NULL;
```

обработка этой строки прекращается: сама операция для строки и последующие row triggers для неё не продолжаются.

Если вернуть строку:

```sql
RETURN NEW;
```

операция продолжается с возвращённой строкой.

Это позволяет изменить значения до записи:

```sql
NEW.updated_at := now();
RETURN NEW;
```

Для `BEFORE DELETE` обычно возвращают `OLD`, если удаление должно продолжиться.

### 2.14. AFTER и statement-level triggers

Для row-level `AFTER` trigger возвращаемое значение игнорируется. Аналогично оно игнорируется для statement-level `BEFORE`/`AFTER` triggers.

Поэтому часто пишут:

```sql
RETURN NULL;
```

не для отмены действия, а потому что результат уже не используется trigger manager.

Не путайте это с `BEFORE ROW`, где `RETURN NULL` имеет специальный смысл «пропустить строку».

### 2.15. INSTEAD OF trigger

`INSTEAD OF` triggers применяются к представлениям и являются row-level.

Их контракт отличается:

- `NULL` означает, что trigger не выполнил изменение для строки;
- ненулевой результат сообщает, что запрошенное изменение обработано;
- для `INSERT/UPDATE` обычно возвращается `NEW`;
- для `DELETE` — `OLD`.

### 2.16. Управление транзакциями: не каждая функция может делать `COMMIT`

Обычная PL/pgSQL-функция, вызванная через `SELECT`, не является местом, где можно произвольно выполнять `COMMIT` или `ROLLBACK`.

Управление транзакциями разрешено в подходящем контексте:

- процедура, вызванная `CALL`;
- анонимный блок `DO`;
- вложенные `CALL`/`DO`, если между ними нет команды, нарушающей допустимый call chain.

Пример процедуры:

```sql
CREATE PROCEDURE process_batch()
LANGUAGE plpgsql
AS $$
BEGIN
    ...
    COMMIT;
    ...
END;
$$;
```

После `COMMIT` или `ROLLBACK` новая транзакция начинается автоматически — отдельный `START TRANSACTION` в процедуре не нужен.

### 2.17. `COMMIT AND CHAIN` / `ROLLBACK AND CHAIN`

Обычный `COMMIT` завершает текущую транзакцию, после чего новая начинается с характеристиками транзакций по умолчанию.

Если нужно продолжить со свойствами предыдущей транзакции, используются:

```sql
COMMIT AND CHAIN;
ROLLBACK AND CHAIN;
```

`AND CHAIN` начинает следующую транзакцию с характеристиками предыдущей, например с тем же уровнем изоляции.

### 2.18. Ограничения transaction control

Есть важные ограничения.

Управление транзакциями разрешено только в соответствующем call stack. Например:

```text
CALL p1() → CALL p2() → CALL p3()
```

может сохранять допустимый контекст.

Но промежуточный вызов через SQL-функцию меняет ситуацию:

```text
CALL p1() → SELECT f2() → CALL p3()
```

и последняя процедура уже не получает ту же свободу transaction control.

Кроме того, транзакцию нельзя завершать внутри блока, имеющего обработчик `EXCEPTION`.

### 2.19. Цикл курсора и `COMMIT`

Курсоры обычно закрываются при завершении транзакции. Однако курсор, создаваемый циклом `FOR ... IN SELECT`, при первом `COMMIT`/`ROLLBACK` внутри такого процедурного цикла может быть преобразован в holdable cursor: результат вычисляется целиком, чтобы цикл мог продолжиться после границы транзакции.

Это важно для больших наборов данных: поведение по памяти и момент фактического вычисления результата меняются.

Transaction control не допускается в cursor loop, управляемом запросом, который сам модифицирует данные, например `UPDATE ... RETURNING`.

### 2.20. Конфликты имён переменных и столбцов

Рассмотрим:

```sql
DECLARE
    id integer;
BEGIN
    SELECT id
    INTO id
    FROM users
    WHERE id = id;
END;
```

Здесь человеку уже трудно понять, где столбец, а где переменная. PL/pgSQL также должен разрешить неоднозначность.

По умолчанию, если ссылка синтаксически может означать и переменную PL/pgSQL, и столбец таблицы, PL/pgSQL выдаёт ошибку.

Это хорошая причина не полагаться на неявный выбор.

### 2.21. Как устранять неоднозначность

#### Способ 1: соглашение об именах

Например:

```text
p_*  → parameters
v_*  → local variables
```

```sql
WHERE users.id = p_user_id
```

#### Способ 2: квалифицировать столбец

```sql
SELECT u.id
FROM users AS u
WHERE u.id = p_user_id;
```

#### Способ 3: квалифицировать переменную

Переменная блока:

```sql
<<fn>>
DECLARE
    id integer;
BEGIN
    SELECT u.id
    FROM users AS u
    WHERE u.id = fn.id;
END;
```

Параметр функции можно квалифицировать именем функции:

```sql
WHERE users.id = get_user.id
```

если `id` — параметр функции `get_user`.

### 2.22. `plpgsql.variable_conflict`

Поведение можно менять настройкой:

```text
error        → неоднозначность = ошибка (по умолчанию)
use_variable → предпочитать PL/pgSQL-переменную
use_column   → предпочитать столбец
```

Для отдельной функции можно использовать compiler directive:

```sql
#variable_conflict error
```

или соответствующие `use_variable` / `use_column`.

Но более надёжный код обычно устраняет неоднозначность явно, а не делает смысл зависимым от настройки.

### 2.23. Как PL/pgSQL кеширует статический SQL

При первом выполнении конкретного SQL-оператора или выражения PL/pgSQL подготавливает его через серверный SPI-механизм. При последующих выполнениях подготовленная команда может переиспользоваться.

Планировщик может использовать текущие значения параметров для создания custom plan. При многократном выполнении PostgreSQL также может решить, что выгоднее переиспользовать generic plan, который не зависит от конкретных значений.

Упрощённая модель:

```text
статический SQL
    ↓
подготовленный оператор
    ↓
custom plan по текущим значениям
    или
generic plan для переиспользования
```

Не нужно вручную выбирать этот режим для каждого обычного PL/pgSQL-запроса: PostgreSQL сам принимает решение в рамках механизма prepared statements.

### 2.24. Почему имена объектов должны оставаться постоянными для кешируемого SQL

Чтобы статический SQL мог быть подготовлен и переиспользован, его таблицы и столбцы должны быть определены в тексте команды.

Поэтому это естественно:

```sql
SELECT * FROM users WHERE id = p_id;
```

а такой смысл требует dynamic SQL:

```text
SELECT * FROM <table_chosen_at_runtime>
```

Dynamic `EXECUTE` покупает гибкость ценой разбора и планирования при каждом выполнении.

### 2.25. Когда новый план через `EXECUTE` может быть полезен

Допустим, распределение данных очень неравномерно:

```text
tenant A → 10 строк
tenant B → 100 000 000 строк
```

Оптимальный план одного и того же параметризованного запроса может сильно зависеть от tenant.

В таких случаях explicit `EXECUTE ... USING` может быть инструментом, который гарантирует новое планирование с текущими параметрами.

Это не означает «dynamic EXECUTE всегда быстрее». Наоборот, у него есть дополнительная стоимость planning. Использовать его ради планирования стоит только когда есть реальная причина.

### 2.26. Trigger functions и plan cache

Если одна trigger function используется для нескольких таблиц, PL/pgSQL подготавливает и кеширует операторы отдельно для каждой комбинации trigger function + table.

Это важно, потому что столбцы с одинаковыми именами в разных таблицах могут иметь разные типы.

Полиморфные функции аналогично имеют отдельные кеши для разных комбинаций фактических типов аргументов.

### 2.27. Времязависимые значения и кеширование

Не стоит записывать чувствительное ко времени значение как текстовый литерал там, где анализатор может превратить его в константу при подготовке SQL.

Предпочтительно:

```sql
now()
current_timestamp
```

вместо попытки использовать текст `'now'` как динамически меняющееся значение.

### 2.28. Дополнительные проверки PL/pgSQL

Для разработки полезны:

```text
plpgsql.extra_warnings
plpgsql.extra_errors
```

Им можно задавать `all` или отдельные проверки.

Примеры проверок:

```text
shadowed_variables       → новая переменная скрывает существующую
strict_multi_assignment  → число source/target полей не совпадает
too_many_rows            → INTO получает более одной строки
```

В тестовой/разработческой среде такие проверки помогают ловить ошибки раньше.

### 2.29. `ASSERT` и `RAISE`: разные назначения

`ASSERT`:

```sql
ASSERT total >= 0, 'total must not be negative';
```

проверяет предположение разработчика и предназначен для поиска программных дефектов.

`RAISE EXCEPTION`:

```sql
RAISE EXCEPTION 'user % does not exist', p_id;
```

используется для обычной ошибки приложения/домена или для явного прекращения выполнения.

Не следует превращать `ASSERT` в основной механизм валидации пользовательских данных.

---

## 3. Практические шаблоны

### Безопасный dynamic SQL

```sql
EXECUTE format(
    'UPDATE %I SET %I = $1 WHERE id = $2',
    table_name,
    column_name
)
USING new_value, row_id;
```

Модель:

```text
SQL syntax / identifiers → format(%I)
data values              → USING
```

### Диагностика ошибки

```sql
BEGIN
    ...
EXCEPTION
    WHEN OTHERS THEN
        GET STACKED DIAGNOSTICS
            v_state   = RETURNED_SQLSTATE,
            v_message = MESSAGE_TEXT,
            v_detail  = PG_EXCEPTION_DETAIL,
            v_context = PG_EXCEPTION_CONTEXT;

        RAISE WARNING 'state=%, message=%', v_state, v_message;
        RAISE;
END;
```

### BEFORE trigger, изменяющий строку

```sql
CREATE FUNCTION set_updated_at()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$;
```

### BEFORE trigger, пропускающий строку

```sql
IF NEW.disabled THEN
    RETURN NULL;
END IF;

RETURN NEW;
```

### Явные имена вместо конфликтов

```sql
CREATE FUNCTION get_user(p_id bigint)
RETURNS users
LANGUAGE plpgsql
AS $$
DECLARE
    v_user users%ROWTYPE;
BEGIN
    SELECT u.*
    INTO STRICT v_user
    FROM users AS u
    WHERE u.id = p_id;

    RETURN v_user;
END;
$$;
```

Здесь по именам сразу видно роль каждого идентификатора.

---

# Что запомнить

- В статическом оптимизируемом SQL PL/pgSQL может подставлять переменные как query parameters и переиспользовать подготовленный план.
- Dynamic `EXECUTE` строит план заново при каждом выполнении.
- `$1`, `$2`, ... в `EXECUTE ... USING` предназначены для **значений данных**, а не для имён таблиц/столбцов.
- Идентификаторы безопасно вставляются через `format('%I', ...)` или `quote_ident`.
- `%L` эквивалентен `quote_nullable`; для data values по возможности лучше `USING`.
- Dollar quoting хорош для фиксированного тела функции, но не делает неизвестный dynamic value безопасным.
- Обычный `EXECUTE` не меняет `FOUND`; число строк берётся через `GET DIAGNOSTICS ... ROW_COUNT`.
- `GET DIAGNOSTICS` описывает текущее выполнение; `GET STACKED DIAGNOSTICS` — перехваченную ошибку.
- В `EXCEPTION` изменения базы внутри ошибочной части блока откатываются, а PL/pgSQL-переменные сохраняют значения, достигнутые к моменту ошибки.
- Блоки с `EXCEPTION` имеют дополнительную стоимость, поэтому их не стоит использовать без причины.
- Trigger function получает `NEW`, `OLD`, `TG_OP`, `TG_WHEN`, `TG_LEVEL` и другие специальные переменные.
- `BEFORE ROW` trigger может вернуть `NULL`, чтобы пропустить операцию для строки; возврат `NEW`/`OLD` продолжает обработку по соответствующей семантике.
- Для `AFTER` row trigger и statement-level triggers возвращаемое значение игнорируется.
- `COMMIT`/`ROLLBACK` разрешены в подходящем контексте процедур `CALL` и `DO`, а не в обычной функции, вызванной через `SELECT`.
- После `COMMIT`/`ROLLBACK` в процедуре новая транзакция начинается автоматически; `AND CHAIN` сохраняет характеристики предыдущей.
- По умолчанию неоднозначность «переменная или столбец» вызывает ошибку. Лучшее решение — квалификация и понятные соглашения об именах.
- Статический SQL может использовать custom или generic plan; `EXECUTE` полезен, когда действительно нужен новый plan для текущих parameter values.
