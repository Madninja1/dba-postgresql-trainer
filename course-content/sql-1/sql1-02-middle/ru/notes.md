# Middle — уверенный SQL

## 1. Тема

На этом уровне SQL перестаёт быть только набором команд чтения и изменения строк. Основная задача — научиться выражать правила данных и строить запросы, в которых появляются вложенные наборы, окна, операции над множествами и PostgreSQL-типы.

```text
целостность данных
        ↓
подзапросы и наборы
        ↓
оконные вычисления
        ↓
массивы и JSON
        ↓
удобное изменение данных
```

---

## 2. Ограничения целостности

Тип столбца задаёт только базовый класс допустимых значений. Бизнес-правила выражаются ограничениями.

Основные ограничения:

```text
NOT NULL     значение обязательно
CHECK        значение должно удовлетворять условию
UNIQUE       значения должны быть уникальны
PRIMARY KEY  уникальный идентификатор строки
FOREIGN KEY  ссылка на допустимую строку другой таблицы
EXCLUDE      запрещает строки, конфликтующие по заданным операторам
```

Пример:

```sql
CREATE TABLE products (
    id bigint PRIMARY KEY,
    name text NOT NULL,
    price numeric CHECK (price > 0),
    sku text UNIQUE
);
```

`PRIMARY KEY` сочетает уникальность и запрет `NULL`. В таблице может быть только один первичный ключ, но он может состоять из нескольких столбцов.

`EXCLUDE` задаёт более общий запрет на конфликт между строками. Например, с подходящим индексным методом можно запретить пересекающиеся интервалы. Это отдельный вид ограничения, а не сортировка или вариант `UNIQUE`.

```sql
PRIMARY KEY (order_id, product_id)
```

### FOREIGN KEY

```sql
CREATE TABLE orders (
    id bigint PRIMARY KEY,
    product_id bigint REFERENCES products(id),
    quantity integer CHECK (quantity > 0)
);
```

Внешний ключ обеспечивает ссылочную целостность: нельзя сохранить ненулевой `product_id`, для которого отсутствует допустимая строка в `products`.

При удалении или изменении родительской строки можно определить действие:

```text
NO ACTION / RESTRICT  запретить действие при существующих ссылках
CASCADE               распространить изменение/удаление
SET NULL              установить ссылку в NULL
SET DEFAULT           установить значение по умолчанию
```

Например:

```sql
order_id bigint REFERENCES orders(id) ON DELETE CASCADE
```

подходит, если дочерняя строка не имеет смысла без родительского заказа.

---

## 3. Представления

Представление — именованный запрос, к результату которого можно обращаться подобно таблице.

```sql
CREATE VIEW active_users AS
SELECT id, name
FROM users
WHERE active = true;
```

После этого:

```sql
SELECT id, name
FROM active_users;
```

Представление удобно как стабильный интерфейс к данным: внутреннюю структуру таблиц можно изменять, сохраняя внешний запрос представления совместимым.

---

## 4. Подзапросы

Подзапрос — `SELECT`, вложенный в другой SQL-запрос.

### Скалярный подзапрос

Если подзапрос используется как одно значение, он должен вернуть один столбец и не более одной строки:

```sql
SELECT name,
       (SELECT max(price) FROM products) AS max_price
FROM categories;
```

Если строк нет, скалярный результат становится `NULL`. Если строк больше одной — это ошибка.

### EXISTS

`EXISTS` отвечает только на вопрос: вернул подзапрос хотя бы одну строку или нет.

```sql
SELECT c.id, c.name
FROM customers AS c
WHERE EXISTS (
    SELECT 1
    FROM orders AS o
    WHERE o.customer_id = c.id
);
```

Здесь внутренний запрос коррелирован с текущей строкой `customers`.

Для `EXISTS` содержимое `SELECT` обычно неважно: проверяется факт существования строки, поэтому часто пишут `SELECT 1`.

### Подзапрос в FROM

Подзапрос может создавать временный набор строк для дальнейшего запроса:

```sql
SELECT x.customer_id, x.total
FROM (
    SELECT customer_id, sum(amount) AS total
    FROM orders
    GROUP BY customer_id
) AS x
WHERE x.total > 1000;
```

Такой подзапрос является табличным выражением. Ему нужно имя, чтобы внешний запрос мог обращаться к его столбцам.

---

## 5. UNION, INTERSECT, EXCEPT

Эти операции объединяют **результаты запросов**, а не строки таблиц по условию, как `JOIN`.

```text
UNION      строки из обоих запросов
INTERSECT  строки, присутствующие в обоих
EXCEPT     строки первого, отсутствующие во втором
```

Пример:

```sql
SELECT email FROM customers
UNION
SELECT email FROM suppliers;
```

По умолчанию `UNION` удаляет дубликаты. Чтобы сохранить их:

```sql
UNION ALL
```

Запросы по обе стороны операции должны возвращать совместимое количество столбцов с совместимыми типами.

Полезно различать:

```text
JOIN   → соединяет столбцы связанных строк
UNION  → складывает совместимые наборы строк
```

---

## 6. Оконные функции

Обычный агрегат с `GROUP BY` сворачивает несколько строк в одну строку группы.

Оконная функция делает вычисление по связанному набору строк, **не уничтожая отдельные строки результата**.

```sql
SELECT
    depname,
    empno,
    salary,
    avg(salary) OVER (PARTITION BY depname) AS dep_avg
FROM empsalary;
```

Каждый сотрудник остаётся отдельной строкой, но рядом появляется средняя зарплата его отдела.

### OVER

Наличие `OVER` превращает поддерживаемый вызов в оконный.

```text
PARTITION BY  делит строки на независимые разделы
ORDER BY      задаёт порядок строк внутри раздела
```

Например:

```sql
SELECT
    depname,
    empno,
    salary,
    rank() OVER (
        PARTITION BY depname
        ORDER BY salary DESC
    ) AS salary_rank
FROM empsalary;
```

Ранг считается отдельно внутри каждого отдела.

Важно: `ORDER BY` внутри `OVER (...)` управляет вычислением окна и не обязан совпадать с финальным `ORDER BY` всего запроса.

Оконные функции вычисляются после `WHERE`, `GROUP BY` и `HAVING`, поэтому непосредственно используются в списке `SELECT` и в `ORDER BY`. Если результат окна нужно отфильтровать, обычно требуется внешний запрос.

```sql
SELECT *
FROM (
    SELECT
        empno,
        salary,
        rank() OVER (ORDER BY salary DESC) AS pos
    FROM empsalary
) AS ranked
WHERE pos <= 3;
```

---

## 7. FILTER у агрегатов

`FILTER` ограничивает входные строки **конкретного агрегата**, не отбрасывая строку из запроса целиком.

```sql
SELECT
    count(*) AS all_orders,
    count(*) FILTER (WHERE status = 'paid') AS paid_orders,
    count(*) FILTER (WHERE status = 'cancelled') AS cancelled_orders
FROM orders;
```

Сравнение:

```text
WHERE   влияет на все вычисления запроса
FILTER  влияет только на агрегат, возле которого указан
```

---

## 8. NULL и условные выражения

`NULL` означает отсутствие известного значения. Это не пустая строка и не ноль.

Для проверки используется:

```sql
IS NULL
IS NOT NULL
```

а не `= NULL`.

### CASE

```sql
SELECT
    amount,
    CASE
        WHEN amount >= 1000 THEN 'large'
        WHEN amount >= 100 THEN 'medium'
        ELSE 'small'
    END AS size
FROM orders;
```

### COALESCE

Возвращает первое значение, которое не равно `NULL`:

```sql
SELECT COALESCE(nickname, name, 'unknown')
FROM users;
```

### NULLIF

Возвращает `NULL`, если два выражения равны:

```sql
SELECT NULLIF(value, 0);
```

Это часто удобно для защиты выражения от специального значения.

`GREATEST` и `LEAST` выбирают соответственно наибольшее и наименьшее значение из списка выражений.

---

## 9. Массивы PostgreSQL

PostgreSQL позволяет хранить массив значений одного типа:

```sql
CREATE TABLE articles (
    id bigint PRIMARY KEY,
    tags text[]
);
```

Создание массива:

```sql
SELECT ARRAY['sql', 'postgresql', 'dba'];
```

Обращение к элементу:

```sql
SELECT tags[1]
FROM articles;
```

Индексация PostgreSQL-массивов обычно начинается с `1`.

Можно получать срезы:

```sql
SELECT tags[1:2]
FROM articles;
```

Элементы и срезы массива можно изменять через `UPDATE`:

```sql
UPDATE articles
SET tags[1] = 'postgres'
WHERE id = 1;
```

Массив полезен, когда данные действительно являются одним многозначным атрибутом. Он не заменяет нормальную таблицу связей там, где элементы должны иметь собственные свойства или активно участвовать в реляционных связях.

---

## 10. JSON и JSONB

PostgreSQL имеет типы `json` и `jsonb`.

`json` в первую очередь сохраняет исходное текстовое представление JSON.

`jsonb` хранит JSON в разобранном бинарном представлении, удобном для обработки, и поддерживает индексирование ряда JSON-операций.

```sql
CREATE TABLE events (
    id bigint PRIMARY KEY,
    payload jsonb
);
```

Пример значения:

```sql
INSERT INTO events (id, payload)
VALUES (1, '{"type":"login","user_id":42}');
```

Если JSON нужно не только сохранить, но и регулярно искать по его структуре, обычно важен именно `jsonb`.

---

## 11. Схемы и search_path

Схема — пространство имён внутри базы данных.

```text
public.users
audit.users
```

Это разные объекты, хотя имя таблицы одинаковое.

Полное имя состоит из схемы и объекта:

```sql
SELECT *
FROM audit.events;
```

Если схема не указана, PostgreSQL ищет объект по `search_path`.

Упрощённо:

```text
имя без схемы
    ↓
search_path
    ↓
первая подходящая схема
```

Поэтому schema-qualified имена полезны там, где важно однозначно понимать, к какому объекту обращается запрос.

---

## 12. RETURNING

`INSERT`, `UPDATE` и `DELETE` могут сразу вернуть затронутые строки.

```sql
INSERT INTO users (name)
VALUES ('Alice')
RETURNING id, name;
```

```sql
UPDATE users
SET active = false
WHERE id = 10
RETURNING id, active;
```

Это избавляет от отдельного `SELECT`, когда после изменения нужно сразу получить созданный идентификатор или новое состояние строки.

---

## 13. SAVEPOINT — точечный откат

`SAVEPOINT` работает внутри транзакции:

```sql
BEGIN;

UPDATE accounts SET balance = balance - 100 WHERE id = 1;
SAVEPOINT after_debit;

UPDATE accounts SET balance = balance + 100 WHERE id = 2;
ROLLBACK TO after_debit;

UPDATE accounts SET balance = balance + 100 WHERE id = 3;
COMMIT;
```

После `ROLLBACK TO` сама транзакция остаётся открытой. Сама точка сохранения тоже продолжает существовать, поэтому к ней можно откатиться повторно, пока она не будет удалена или не исчезнет при завершении транзакции. После отката можно продолжить работу и затем выполнить `COMMIT`.

---

# Что запомнить

Целостность:

```text
NOT NULL     значение обязательно
CHECK        проверка условия
UNIQUE       уникальность
PRIMARY KEY  главный идентификатор строки
FOREIGN KEY  ссылочная целостность
EXCLUDE      запрет конфликтующих строк по заданным операторам
```

Запросы:

```text
EXISTS       есть ли хотя бы одна строка
UNION        объединение наборов без дублей
UNION ALL    объединение с дублями
INTERSECT    пересечение
EXCEPT       разность
```

Окна:

```text
aggregate + GROUP BY  → строки сворачиваются
window + OVER         → строки сохраняются

PARTITION BY → раздел окна
ORDER BY     → порядок внутри окна
```

PostgreSQL-возможности:

```text
jsonb       разобранное бинарное JSON-представление
text[]      массив текста
search_path поиск неуточнённых имён по схемам
RETURNING   вернуть изменённые строки сразу
```

И главное различие фильтров:

```text
WHERE   → строки всего запроса
HAVING  → группы
FILTER  → вход конкретного агрегата
```
