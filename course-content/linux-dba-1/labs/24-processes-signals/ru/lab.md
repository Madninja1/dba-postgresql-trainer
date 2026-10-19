# Лабораторная №24 — процессы PostgreSQL + сигналы

**Время: 1,5 часа**

Цель:

> Научиться находить процессы PostgreSQL в Linux, связывать Linux PID с PostgreSQL backend и понимать, что происходит при отправке сигналов.

В лабораторной используются:

```text
ps
pgrep
pstree
kill
pg_backend_pid()
pg_stat_activity
pg_cancel_backend()
pg_terminate_backend()
```

## 1. Что такое процесс

В Linux каждый запущенный процесс имеет PID — Process ID.

Посмотри PID текущего shell:

```bash
echo $$
```

`$$` — PID текущего shell. Справка: `man bash`, затем поиск `/$$`.

## 2. `ps`

Начни с:

```bash
ps
```

Затем:

```bash
ps -ef
```

- `-e` — показать все процессы;
- `-f` — full-format listing.

Проверить можно через `ps --help all` или `man ps`.

Найди PostgreSQL:

```bash
ps -ef | grep postgres
```

Обычно будут видны основной серверный процесс и фоновые процессы PostgreSQL.

## 3. `ps aux`

```bash
ps aux
```

У `ps` исторически есть несколько стилей синтаксиса:

```text
UNIX style       ps -ef
BSD style        ps aux
GNU long options ps --something
```

Практически в `ps aux`:

```text
a → процессы всех пользователей
u → user-oriented format
x → процессы без controlling terminal
```

Сравни:

```bash
ps aux | grep postgres
ps -ef | grep postgres
```

## 4. `pgrep`

Вместо `ps | grep`:

```bash
pgrep postgres
pgrep -a postgres
```

`-a` показывает PID вместе с полной командной строкой.

## 5. Иерархия процессов

```bash
pstree -p
```

`-p` показывает PID рядом с процессами. Найди PostgreSQL и посмотри связь основного процесса с дочерними.

## 6. Связываем PostgreSQL backend с Linux PID

Открой `psql`:

```bash
sudo -u postgres psql
```

Выполни:

```sql
SELECT pg_backend_pid();
```

Полученный PID найди в Linux:

```bash
ps -fp PID
```

Здесь `-f` — full format, `-p` — выбрать конкретный PID.

Модель:

```text
PostgreSQL session
→ backend PID
→ Linux process
```

## 7. `pg_stat_activity`

```sql
SELECT pid, usename, datname, state, query
FROM pg_stat_activity;
```

Найди свою сессию и сравни PID с `pg_backend_pid()`.

Создай долгий запрос:

```sql
SELECT pg_sleep(300);
```

В другой сессии:

```sql
SELECT pid, usename, state, query
FROM pg_stat_activity
WHERE query LIKE '%pg_sleep%';
```

## 8. `kill` и сигналы

`kill` не обязательно «убивает» процесс. Он отправляет сигнал:

```bash
kill -SIGNAL PID
```

Посмотреть список сигналов:

```bash
kill -l
```

### SIGINT

Запусти `pg_sleep(300)`, узнай PID и из Linux выполни:

```bash
kill -INT PID
```

Обычно текущий запрос прерывается, а backend остаётся жить. Проверь тем же соединением:

```sql
SELECT 1;
```

### SIGTERM

Узнай PID backend:

```sql
SELECT pg_backend_pid();
```

Из Linux:

```bash
kill -TERM PID
```

Для отдельного PostgreSQL backend это приводит к завершению backend-процесса; клиентская сессия теряет соединение.

## 9. То же через PostgreSQL

Отменить текущую работу backend:

```sql
SELECT pg_cancel_backend(PID);
```

Завершить backend/сессию:

```sql
SELECT pg_terminate_backend(PID);
```

Главное различие:

```text
cancel ≠ terminate
```

## 10. Про `kill -9`

```bash
kill -KILL PID
# или
kill -9 PID
```

SIGKILL нельзя обработать или проигнорировать. Для PostgreSQL это не стандартный способ управления процессами и в этой лабораторной он не используется как обычная операция.

## Что важно понять

```text
PostgreSQL session
↓
pg_stat_activity.pid
↓
Linux PID
↓
signal
```

## Мини-экзамен №24

1. Как показать все PostgreSQL процессы через `ps`?
2. Как показать PostgreSQL процессы через `pgrep` вместе с командной строкой?
3. Как увидеть дерево процессов с PID?
4. Как узнать PID текущей PostgreSQL-сессии?
5. Как найти этот PID в Linux?
6. Как найти текущий запрос и PID через PostgreSQL?
7. Как отправить backend сигнал SIGINT?
8. Как отправить SIGTERM?
9. В чём разница между `pg_cancel_backend()` и `pg_terminate_backend()`?
10. Почему `kill -9` не надо использовать как обычный способ работы с PostgreSQL?
