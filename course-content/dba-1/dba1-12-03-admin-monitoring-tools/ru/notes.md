# 14. Средства мониторинга и статистика запросов

## 1. Тема

Разовый просмотр `pg_stat_activity` отвечает на вопрос «что происходит сейчас». Полноценный мониторинг должен собирать метрики постоянно, хранить историю, строить графики и оповещать о выходе показателей за допустимые границы.

Для анализа SQL-нагрузки внутри PostgreSQL используется `pg_stat_statements`: расширение агрегирует статистику множества выполнений структурно одинаковых запросов.

Главная модель:

```text
PostgreSQL + OS metrics
        ↓
external monitoring
        ↓
history → dashboards → alerts

SQL executions
        ↓
pg_stat_statements
        ↓
queryid / calls / time / rows / I/O
```

---

# 2. Внешний мониторинг

Материал разделяет решения на несколько групп.

Универсальные системы:

```text
Zabbix
Munin
Cacti
```

Облачные сервисы:

```text
Okmeter
NewRelic
Datadog
```

Средства, ориентированные на PostgreSQL:

```text
pg_profile
pgpro_pwr
PGObserver
PoWA
OPM
```

PostgreSQL предоставляет метрики, но сам по себе не является полноценной системой мониторинга. Практическая система должна:

- собирать показатели PostgreSQL и ОС;
- хранить историю;
- показывать графики и тенденции;
- формировать оповещения;
- позволять сравнивать интервалы времени, а не только текущий снимок.

`pg_profile` строит снимки накопительной статистики и позволяет сравнивать интервалы. `pgpro_pwr` — расширенное решение Postgres Pro Enterprise для построения отчётов о нагрузке.

---

# 3. `pg_stat_statements`

## Подключение

Модуль требует дополнительной разделяемой памяти, поэтому его необходимо добавить в `shared_preload_libraries` и перезапустить сервер:

```conf
shared_preload_libraries = 'pg_stat_statements'
```

После загрузки модуля объект расширения создаётся в нужной базе:

```sql
CREATE EXTENSION pg_stat_statements;
```

В PostgreSQL 16 для встроенного вычисления идентификаторов запросов используется `compute_query_id`; при обычной конфигурации `auto`/`on` позволяет получать `queryid`.

## Нормализация и `queryid`

`pg_stat_statements` не создаёт отдельную строку на каждое выполнение SQL. Запросы с одинаковой внутренней структурой объединяются в одну запись. Значения констант обычно не участвуют в группировке и в отображаемом тексте заменяются параметрами:

```sql
SELECT * FROM orders WHERE id = 10;
SELECT * FROM orders WHERE id = 20;

-- агрегированное представление может выглядеть как
SELECT * FROM orders WHERE id = $1;
```

`queryid` — хеш внутренней структуры запроса. Это удобный идентификатор внутри конкретной установки и версии, но его нельзя считать универсальным вечным ID между любыми серверами и major-версиями.

## Основные показатели

Полезные столбцы:

```text
queryid
query
calls
plans
total_plan_time / mean_plan_time
total_exec_time / mean_exec_time
rows
shared_blks_hit / shared_blks_read
...
```

Для поиска суммарно дорогих запросов часто важнее `total_exec_time`, чем только среднее время: частый умеренно медленный запрос может создавать больше нагрузки, чем редкий очень медленный.

Пример:

```sql
SELECT queryid,
       calls,
       total_exec_time,
       mean_exec_time,
       rows,
       query
FROM pg_stat_statements
ORDER BY total_exec_time DESC
LIMIT 20;
```

---

# 4. Область сбора и сброс статистики

`pg_stat_statements.track` определяет, какие операторы учитывать:

```text
top   → только операторы верхнего уровня (по умолчанию)
all   → также вложенные операторы, например внутри функций
none  → сбор статистики отключён
```

`pg_stat_statements.track_utility` управляет учётом служебных команд, а `pg_stat_statements.track_planning` — статистикой планирования. Включение дополнительного сбора может иметь стоимость.

Статистику можно сбросить:

```sql
SELECT pg_stat_statements_reset();
```

Функция также позволяет ограничить сброс конкретным пользователем, базой и `queryid`. Поэтому при анализе всегда важно знать, **с какого момента собираются данные**. В `pg_stat_statements_info` поле `stats_reset` показывает момент последнего общего сброса.

---

# 5. Практический ориентир

```conf
# postgresql.conf
shared_preload_libraries = 'pg_stat_statements'
```

```sql
CREATE EXTENSION pg_stat_statements;

-- выполнить несколько запросов, затем:
SELECT queryid, calls, total_exec_time, mean_exec_time, rows, query
FROM pg_stat_statements
ORDER BY total_exec_time DESC;
```

---

# Что запомнить

```text
• полноценный мониторинг — это история, визуализация и оповещения, а не только текущий снимок
• внешний мониторинг объединяет метрики PostgreSQL и ОС
• `pg_stat_statements` агрегирует статистику структурно одинаковых SQL-запросов
• модуль требует `shared_preload_libraries` и перезапуска сервера
• `queryid` идентифицирует структуру запроса, а константы обычно нормализуются
• `top`, `all`, `none` задают область сбора `pg_stat_statements.track`
• после `pg_stat_statements_reset()` новый период анализа начинается с нуля
```
