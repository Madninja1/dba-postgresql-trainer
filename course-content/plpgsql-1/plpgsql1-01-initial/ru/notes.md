# Initial — основы PL/pgSQL

## 1. Тема

PL/pgSQL — процедурный язык PostgreSQL. Он нужен тогда, когда одного SQL-оператора недостаточно и логику удобно выполнить **внутри сервера**: сохранить промежуточное значение в переменную, сделать ветвление, пройти цикл, выполнить несколько SQL-команд и вернуть результат.

На PL/pgSQL создают:

- функции;
- процедуры;
- триггерные функции.

Главная модель:

```text
SQL                         PL/pgSQL
-------------------------   ----------------------------------
описать, какие данные нужны добавить последовательность действий
работает с наборами строк   хранить локальное состояние
JOIN / GROUP BY / UPDATE    IF / CASE / LOOP / FOR / WHILE
один SQL-оператор           блок из SQL и процедурных операторов
```

PL/pgSQL не заменяет SQL. Внутри PL/pgSQL обычные `SELECT`, `INSERT`, `UPDATE`, `DELETE` по-прежнему выполняет SQL-движок PostgreSQL. Процедурный язык только связывает эти операции управляющей логикой.

Это особенно полезно, когда несколько действий должны выполняться рядом с данными на сервере: промежуточные результаты не нужно гонять между клиентом и сервером, а SQL можно сочетать с условиями и циклами.

---

## 2. Механизм

### 2.1. Как выглядит функция на PL/pgSQL

Минимальная функция:

```sql
CREATE FUNCTION add_tax(subtotal numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

Тело функции передаётся `CREATE FUNCTION` как строка. На практике его обычно заключают в dollar quotes (`$$ ... $$` или именованный разделитель вроде `$func$ ... $func$`), чтобы не экранировать каждую одинарную кавычку внутри тела.

Сам PL/pgSQL — блочно-структурированный язык. Общая форма блока:

```sql
[ <<label>> ]
[ DECLARE
    declarations ]
BEGIN
    statements
END [ label ];
```

`DECLARE` необязателен. Выполняемая часть `BEGIN ... END` обязательна.

Важно: `BEGIN` и `END` здесь **группируют PL/pgSQL-операторы**. Это не SQL-команды `BEGIN`/`COMMIT` для управления транзакцией.

### 2.2. Точки с запятой и вложенные блоки

Объявления и операторы завершаются `;`:

```sql
DECLARE
    total integer := 0;
BEGIN
    total := total + 1;
    RETURN total;
END;
```

Сразу после `BEGIN` точка с запятой не ставится:

```sql
BEGIN;
    -- неверно
```

Блоки можно вкладывать:

```sql
<<outer_block>>
DECLARE
    value integer := 10;
BEGIN
    DECLARE
        value integer := 20;
    BEGIN
        RAISE NOTICE 'inner = %', value;
        RAISE NOTICE 'outer = %', outer_block.value;
    END;
END;
```

Внутренняя переменная с тем же именем скрывает внешнюю. Метка блока позволяет явно обратиться к внешней переменной.

### 2.3. Объявление переменных

Обычные локальные переменные объявляются в `DECLARE`:

```sql
DECLARE
    counter integer;
    price numeric(10, 2) := 0;
    title text DEFAULT 'unknown';
    started_at CONSTANT timestamptz := now();
BEGIN
    ...
END;
```

Общий смысл объявления:

```text
name [CONSTANT] type [COLLATE ...] [NOT NULL]
     [DEFAULT expression | := expression | = expression]
```

Если начальное значение не задано, переменная получает SQL-значение `NULL`, а не `0` и не пустую строку.

Пример:

```sql
DECLARE
    attempts integer;       -- NULL
    enabled boolean := true;
    code text NOT NULL := 'A';
BEGIN
    ...
END;
```

`CONSTANT` запрещает последующее изменение значения. `NOT NULL` запрещает присваивание `NULL` и поэтому требует непустого начального значения. `COLLATE` позволяет задать правило сортировки для сортируемого типа.

Начальное выражение вычисляется **при каждом входе в блок**. Например:

```sql
DECLARE
    started_at timestamptz := now();
```

получает значение для текущего выполнения блока.

### 2.4. Присваивание

PL/pgSQL допускает обе формы:

```sql
counter := counter + 1;
counter = counter + 1;
```

Чаще используют `:=`, потому что визуально легче отличить присваивание от SQL-сравнения.

Присваивать можно не только простую переменную:

```sql
row_value.id := 10;
items[1] := 'first';
```

Выражение справа вычисляется SQL-движком PostgreSQL.

### 2.5. Параметры функции

Параметры можно именовать прямо в `CREATE FUNCTION`:

```sql
CREATE FUNCTION add_tax(subtotal numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

Также параметры доступны как `$1`, `$2`, ...:

```sql
CREATE FUNCTION add_two(integer, integer)
RETURNS integer
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN $1 + $2;
END;
$$;
```

Старый, но поддерживаемый способ дать имя позиционному параметру — `ALIAS`:

```sql
CREATE FUNCTION add_tax(numeric)
RETURNS numeric
LANGUAGE plpgsql
AS $$
DECLARE
    subtotal ALIAS FOR $1;
BEGIN
    RETURN subtotal * 1.20;
END;
$$;
```

Для нового кода обычно понятнее сразу именовать параметры в заголовке функции.

### 2.6. Обычные SQL-команды внутри PL/pgSQL

SQL-команды, которые не возвращают набор строк вызывающему PL/pgSQL-коду, пишутся почти как обычно:

```sql
INSERT INTO audit_log(message)
VALUES ('started');

UPDATE jobs
SET status = 'done'
WHERE id = job_id;
```

Переменные PL/pgSQL могут использоваться как значения внутри обычного оптимизируемого SQL.

### 2.7. `PERFORM`: выполнить запрос и отбросить результат

В обычном SQL можно написать:

```sql
SELECT refresh_cache();
```

Но внутри PL/pgSQL запрос, возвращающий строки, нельзя просто оставить без обработки результата. Если результат не нужен, используется `PERFORM`:

```sql
PERFORM refresh_cache();
```

Это означает:

```text
выполни запрос
→ результат не сохраняй
→ побочные эффекты сохрани
```

`PERFORM` использует синтаксис запроса, похожий на `SELECT`, но ключевое слово `SELECT` заменяется на `PERFORM`.

Пример:

```sql
PERFORM 1
FROM jobs
WHERE status = 'failed';
```

После `PERFORM` переменная `FOUND` покажет, вернул ли запрос хотя бы одну строку.

### 2.8. `SELECT ... INTO`: получить строку в переменные

Чтобы сохранить результат `SELECT` в переменные:

```sql
DECLARE
    v_name text;
    v_email text;
BEGIN
    SELECT name, email
    INTO v_name, v_email
    FROM users
    WHERE id = 42;
END;
```

Это **PL/pgSQL `INTO`**, то есть захват результата в переменные. Не путайте его с SQL-командой `SELECT INTO`, создающей таблицу.

Без `STRICT` поведение такое:

```text
0 строк     → цели получают NULL, FOUND = false
1 строка    → она записывается в цели, FOUND = true
>1 строки   → берётся первая строка, остальные отбрасываются
```

Если важно, какая именно строка считается первой, нужен явный `ORDER BY`.

Для записи результата можно использовать:

- список скалярных переменных;
- строковую переменную;
- `record`.

`STRICT`, требующий ровно одну строку, будет подробно разобран на следующем уровне.

### 2.9. `FOUND`: состояние последней подходящей операции

`FOUND` — специальная локальная переменная типа `boolean`. При входе в функцию она имеет значение `false`.

Пример:

```sql
SELECT *
INTO user_row
FROM users
WHERE id = p_id;

IF NOT FOUND THEN
    RAISE EXCEPTION 'user % not found', p_id;
END IF;
```

`FOUND` меняют несколько разных операторов.

Для базового уровня полезно запомнить:

```text
SELECT ... INTO      true, если получена строка
PERFORM              true, если запрос вернул хотя бы строку
INSERT/UPDATE/DELETE true, если затронута хотя бы строка
MERGE                true, если затронута хотя бы строка
FETCH                true, если получена строка курсора
MOVE                 true, если перемещение курсора успешно
FOR / FOREACH        true после цикла, если была хотя бы одна итерация
RETURN QUERY         true, если запрос добавил хотя бы одну строку
```

`FOUND` — это не число строк. Для точного количества есть `GET DIAGNOSTICS ... ROW_COUNT`, который будет рассмотрен позже.

### 2.10. Условия: `IF` и `CASE`

Простой `IF`:

```sql
IF balance < 0 THEN
    status := 'debt';
END IF;
```

С альтернативой:

```sql
IF balance < 0 THEN
    status := 'debt';
ELSE
    status := 'ok';
END IF;
```

Несколько условий:

```sql
IF score >= 90 THEN
    grade := 'A';
ELSIF score >= 75 THEN
    grade := 'B';
ELSE
    grade := 'C';
END IF;
```

Поддерживается и `CASE`:

```sql
CASE status
    WHEN 'new' THEN priority := 1;
    WHEN 'retry' THEN priority := 2;
    ELSE priority := 3;
END CASE;
```

Есть также форма `CASE WHEN condition THEN ...`.

### 2.11. Циклы

#### Безусловный `LOOP`

```sql
LOOP
    counter := counter + 1;
    EXIT WHEN counter >= 10;
END LOOP;
```

`EXIT` завершает цикл, `CONTINUE` переходит к следующей итерации.

#### `WHILE`

```sql
WHILE counter < 10 LOOP
    counter := counter + 1;
END LOOP;
```

#### Целочисленный `FOR`

```sql
FOR i IN 1..10 LOOP
    total := total + i;
END LOOP;
```

Переменная `i` создаётся автоматически как `integer` на время цикла.

Есть `REVERSE` и `BY`:

```sql
FOR i IN REVERSE 10..1 BY 2 LOOP
    ...
END LOOP;
```

#### `FOR` по строкам запроса

```sql
FOR r IN
    SELECT id, name FROM users WHERE active
LOOP
    RAISE NOTICE '%: %', r.id, r.name;
END LOOP;
```

Для каждой строки результата тело выполняется один раз.

#### `FOREACH` по массиву

```sql
FOREACH item IN ARRAY items LOOP
    total := total + item;
END LOOP;
```

На следующих уровнях будут разобраны более сложные формы циклов и работа с курсорами.

### 2.12. `RETURN`: завершить функцию

Для функции, возвращающей одно значение:

```sql
RETURN expression;
```

Например:

```sql
RETURN subtotal * 1.20;
```

`RETURN expression` прекращает выполнение функции и отдаёт значение вызывающему коду.

Для `void`-функции или функции с выходными параметрами можно использовать просто:

```sql
RETURN;
```

### 2.13. Set-returning functions: `RETURN NEXT` и `RETURN QUERY`

Если функция возвращает множество строк, она может добавлять результат частями.

`RETURN NEXT` добавляет одну строку:

```sql
RETURN NEXT value;
```

`RETURN QUERY` добавляет все строки запроса:

```sql
RETURN QUERY
SELECT id
FROM users
WHERE active;
```

Ключевое отличие от обычного `RETURN`:

```text
RETURN value       → завершить функцию сейчас
RETURN NEXT value  → добавить строку и продолжить
RETURN QUERY query → добавить строки запроса и продолжить
```

`RETURN NEXT` и `RETURN QUERY` можно смешивать в одной set-returning функции. Окончательный `RETURN;` лишь завершает дальнейшее выполнение.

---

## 3. Практическая модель

Когда пишете простую функцию, полезно идти сверху вниз:

```text
1. Какой результат должна вернуть функция?
2. Какие входные параметры ей нужны?
3. Какие локальные значения нужно хранить?
4. Какие SQL-команды выполняются?
5. Нужно ли ветвление?
6. Нужен ли цикл?
7. Что делать, если SELECT ничего не нашёл?
8. Где функция должна завершиться?
```

Пример:

```sql
CREATE FUNCTION user_label(p_id bigint)
RETURNS text
LANGUAGE plpgsql
AS $$
DECLARE
    v_name text;
    v_active boolean;
BEGIN
    SELECT name, active
    INTO v_name, v_active
    FROM users
    WHERE id = p_id;

    IF NOT FOUND THEN
        RETURN 'missing';
    END IF;

    IF v_active THEN
        RETURN v_name || ' [active]';
    END IF;

    RETURN v_name || ' [inactive]';
END;
$$;
```

Здесь связаны сразу несколько основных механизмов: параметры, `DECLARE`, `SELECT INTO`, `FOUND`, `IF` и `RETURN`.

---

# Что запомнить

- PL/pgSQL — серверный процедурный слой поверх SQL, а не замена SQL.
- Он используется для функций, процедур и триггеров и добавляет переменные, условия, циклы и другую управляющую логику.
- Блок имеет необязательный `DECLARE` и выполняемую часть `BEGIN ... END`.
- `BEGIN/END` PL/pgSQL группируют операторы и сами по себе не управляют транзакцией.
- Обычные локальные переменные объявляются в `DECLARE`; без начального значения они получают `NULL`.
- В объявлении могут использоваться `DEFAULT`/`:=`/`=`, `CONSTANT`, `NOT NULL`, `COLLATE`.
- Для присваивания допустимы `:=` и `=`.
- Параметр функции можно использовать по имени, как `$1`, `$2`, ... или через `ALIAS FOR $n`.
- `PERFORM` выполняет запрос, когда его результат не нужен.
- `SELECT ... INTO` сохраняет одну выбранную строку в переменные; без `STRICT` отсутствие строки даёт `NULL`.
- `FOUND` — boolean-флаг результата ряда операций; при входе в функцию он `false`.
- `IF`, `CASE`, `LOOP`, `WHILE`, `FOR` и `FOREACH` дают процедурное управление потоком.
- `RETURN value` завершает функцию; `RETURN NEXT` и `RETURN QUERY` накапливают строки и продолжают выполнение.
