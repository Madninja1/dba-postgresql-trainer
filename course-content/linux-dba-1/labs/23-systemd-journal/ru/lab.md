# Лабораторная №23 — systemd + journalctl

**Время: 2 часа**

Стенд:

```text
client01
    │
    │ SSH
    ▼
pg01
Ubuntu 24.04 LTS
PostgreSQL 16
```

Цель:

> Получить сообщение «PostgreSQL не работает», определить состояние сервиса, найти причину через systemd/journal и восстановить БД.

Заодно научиться нормально пользоваться:

```text
man
--help
apropos
whatis
systemctl
journalctl
```

---

# 0. Сначала учимся получать справку в Linux

На это выдели **10–15 минут**.

## `man`

Основной встроенный справочник Linux.

Например:

```bash
man systemctl
```

Откроется manual page.

Навигация внутри `man`:

```text
↑ ↓          листать
PageUp       страница вверх
PageDown     страница вниз
/слово       поиск вперёд
?слово       поиск назад
n            следующее совпадение
N            предыдущее совпадение
g            начало
G            конец
q            выход
```

Например, открой:

```bash
man systemctl
```

и введи:

```text
/status
```

Нажми Enter.

Потом `n`, чтобы искать следующее упоминание.

---

## Как искать конкретный флаг

Допустим, встретил:

```bash
journalctl -n 50
```

и забыл, что такое `-n`.

Пишешь:

```bash
man journalctl
```

Внутри:

```text
/-n
```

Но такой поиск иногда неудобен из-за большого количества совпадений.

Можно искать полное название:

```text
/--lines
```

И увидишь описание примерно такого вида:

```text
-n, --lines=
```

То есть короткий вариант:

```bash
-n 50
```

и длинный:

```bash
--lines=50
```

означают одно и то же.

---

# `--help`

Быстрая справка.

```bash
systemctl --help
```

```bash
journalctl --help
```

Она обычно значительно короче `man`.

Например:

```bash
journalctl --help
```

позволяет быстро увидеть:

```text
-u --unit=UNIT
-n --lines=N
-f --follow
-b --boot
-p --priority=RANGE
```

Запомни принцип:

```text
--help → быстро посмотреть синтаксис

man → нормально разобраться
```

---

# `whatis`

Если вообще не помнишь, что делает программа:

```bash
whatis journalctl
```

Например получишь примерно:

```text
journalctl - Query the systemd journal
```

Аналогично:

```bash
whatis systemctl
```

---

# `apropos`

Очень полезная штука.

Когда ты **не знаешь название команды**, но знаешь, что хочешь сделать.

Например:

```bash
apropos journal
```

или:

```bash
apropos systemd
```

Linux покажет подходящие man pages.

То есть:

```text
whatis → знаю название, хочу узнать назначение

apropos → не знаю название, ищу инструмент
```

Если `apropos` ничего не находит, иногда надо обновить базу:

```bash
sudo mandb
```

`mandb` = создаёт/обновляет индекс man-страниц.

---

# Ещё одна полезная команда — `type`

Например:

```bash
type systemctl
```

Она показывает, что именно будет запущено shell.

Попробуй:

```bash
type cd
type ls
type systemctl
```

Ты увидишь разницу между:

```text
shell builtin
alias
external executable
```

Для DBA это полезно, потому что одна и та же внешне «команда» может быть alias или builtin.

---

# Этап 1. Исследуем PostgreSQL service

**Время: ~20 минут**

Заходим с `client01`:

```bash
ssh <user>@pg01
```

С `ssh` разберёмся подробно в лабораторной №33. Сейчас он просто средство доступа.

---

## Ищем PostgreSQL services

```bash
systemctl list-units --type=service
```

Разберём.

### `systemctl`

CLI для управления systemd.

Через него мы работаем с:

```text
services
sockets
timers
mounts
targets
...
```

---

### `list-units`

Это **не флаг**, а подкоманда `systemctl`.

Общая структура:

```text
systemctl <COMMAND> [OPTIONS]
```

Например:

```bash
systemctl status
systemctl start
systemctl stop
systemctl restart
systemctl list-units
```

Посмотреть команды:

```bash
systemctl --help
```

или:

```bash
man systemctl
```

В `man`:

```text
/list-units
```

---

### `--type=service`

Фильтрует units по типу.

То есть:

```bash
systemctl list-units
```

может показать:

```text
services
mounts
sockets
targets
devices
...
```

А:

```bash
systemctl list-units --type=service
```

только:

```text
*.service
```

Узнать самому:

```bash
systemctl --help
```

или:

```bash
man systemctl
```

и:

```text
/--type
```

---

Теперь нам не нужен весь огромный список.

Используем pipeline:

```bash
systemctl list-units --type=service | grep -i postgres
```

Здесь появилась важная конструкция:

```text
|
```

Это **pipe**.

Он передаёт stdout одной команды на stdin следующей:

```text
systemctl
   │
   │ stdout
   ▼
grep
```

Условно:

```text
systemctl → список из 200 строк
             ↓
         grep postgres
             ↓
          2 строки
```

---

## `grep -i postgres`

`grep` ищет строки.

```bash
grep postgres
```

ищет точное совпадение с учётом регистра.

А:

```bash
grep -i postgres
```

### `-i`

`ignore case`.

То есть совпадут:

```text
postgres
Postgres
POSTGRES
PostgreSQL
```

Самостоятельно:

```bash
grep --help
```

Найди:

```text
-i, --ignore-case
```

Полная документация:

```bash
man grep
```

и:

```text
/--ignore-case
```

---

На Ubuntu ты, скорее всего, увидишь:

```text
postgresql.service
postgresql@16-main.service
```

И тут важный момент.

На Debian/Ubuntu:

```text
postgresql.service
```

может быть верхнеуровневым wrapper/meta-unit.

А конкретный кластер:

```text
postgresql@16-main.service
```

Именно его состояние нам особенно интересно.

---

# Смотрим status

```bash
systemctl status postgresql@16-main
```

Здесь вообще **нет флагов**.

Синтаксис:

```text
systemctl
    status
    postgresql@16-main
```

Где:

```text
systemctl             программа
status                команда
postgresql@16-main    unit
```

Смотри примерно на:

```text
Loaded:
Active:
Main PID:
Tasks:
Memory:
CPU:
```

Особенно:

```text
Active: active (running)
```

---

## Почему `.service` можно не писать?

Мы можем написать:

```bash
systemctl status postgresql@16-main.service
```

или часто:

```bash
systemctl status postgresql@16-main
```

systemd способен определить подразумеваемый `.service`.

Но в диагностике полезно понимать полное имя:

```text
postgresql@16-main.service
```

---

# Проверяем состояние машинно

```bash
systemctl is-active postgresql@16-main
```

Вывод:

```text
active
```

Теперь:

```bash
systemctl is-enabled postgresql@16-main
```

Это уже другой вопрос.

### `active`

Означает:

> работает ли unit сейчас?

### `enabled`

Означает примерно:

> настроен ли unit на запуск через механизм systemd при соответствующем target/boot?

То есть:

```text
active ≠ enabled
```

Сервис вполне может быть:

```text
active
disabled
```

или:

```text
inactive
enabled
```

Это разные характеристики.

---

## Посмотри exit code

После:

```bash
systemctl is-active postgresql@16-main
```

выполни:

```bash
echo $?
```

`$?` — специальная переменная shell, содержащая exit status **последней выполненной команды**.

Обычно:

```text
0 = success
не 0 = другое состояние / ошибка
```

Попробуй:

```bash
systemctl is-active postgresql@16-main
echo $?
```

А потом позже на остановленном PostgreSQL повторим.

Это нам пригодится для Bash-скриптов в №32.

---

# Этап 2. Изучаем journalctl

Теперь:

```bash
journalctl -u postgresql@16-main
```

Разбираем.

## `journalctl`

Инструмент чтения systemd journal.

Проверить назначение:

```bash
whatis journalctl
```

Быстрая документация:

```bash
journalctl --help
```

Полная:

```bash
man journalctl
```

---

## `-u`

```bash
journalctl -u postgresql@16-main
```

`-u` означает:

```text
--unit=
```

То есть:

> показывать сообщения определённого systemd unit.

Полный вариант:

```bash
journalctl --unit=postgresql@16-main
```

То же самое.

Проверить:

```bash
journalctl --help
```

Ищи:

```text
-u --unit=UNIT
```

Или:

```bash
man journalctl
```

```text
/--unit
```

---

## Последние N сообщений

```bash
journalctl -u postgresql@16-main -n 50
```

### `-n`

Полная форма:

```text
--lines=
```

То есть:

```bash
-n 50
```

= показать последние 50 записей.

То же самое:

```bash
journalctl \
    --unit=postgresql@16-main \
    --lines=50
```

Я специально иногда буду показывать тебе длинный вариант: по нему намного легче понять смысл команды.

---

# Только сообщения после определённого времени

```bash
journalctl -u postgresql@16-main --since "30 minutes ago"
```

### `--since`

Фильтр по времени начала.

То есть:

```text
покажи сообщения начиная с...
```

Попробуй также:

```bash
journalctl -u postgresql@16-main --since today
```

и:

```bash
journalctl -u postgresql@16-main --since "10 minutes ago"
```

Для расследования аварии это один из самых полезных параметров.

Например:

> База упала примерно в 16:30.

Можно использовать:

```bash
journalctl -u postgresql@16-main \
    --since "16:20" \
    --until "16:40"
```

### `--until`

Противоположная граница:

```text
до указанного времени
```

Получаем окно:

```text
16:20 ───── incident ───── 16:40
  ↑                           ↑
--since                     --until
```

Как найти:

```bash
man journalctl
```

```text
/--since
```

---

# Follow — смотреть журнал в реальном времени

Открой отдельный SSH-терминал:

```bash
journalctl -u postgresql@16-main -f
```

### `-f`

Полная форма:

```text
--follow
```

Работает примерно как:

```bash
tail -f
```

То есть команда не завершается, а ждёт новые события.

Остановить:

```text
Ctrl+C
```

Теперь во втором терминале позже можно перезапускать PostgreSQL и сразу видеть события.

Это очень полезный режим:

```text
Terminal 1:

journalctl ... -f


Terminal 2:

systemctl restart ...
```

---

# Этап 3. Управляем PostgreSQL

**~15 минут**

Остановим:

```bash
sudo systemctl stop postgresql@16-main
```

Разбираем.

### `sudo`

Выполнить команду с повышенными полномочиями, обычно от root.

Справка:

```bash
sudo --help
```

или:

```bash
man sudo
```

---

### `stop`

Подкоманда `systemctl`.

Это не:

```text
--stop
```

а:

```text
systemctl stop UNIT
```

Ищем:

```bash
man systemctl
```

затем:

```text
/stop
```

---

После:

```bash
systemctl status postgresql@16-main
```

Посмотри состояние.

И:

```bash
systemctl is-active postgresql@16-main
```

А теперь обязательно:

```bash
echo $?
```

Сравни exit code с работающим сервисом.

---

# Проверяем порт

```bash
ss -lnt
```

Пока `ss` подробно изучим в №30, но сейчас базово.

Флаги здесь объединены:

```text
-lnt
```

Это то же самое, что:

```text
-l -n -t
```

### `-l`

```text
--listening
```

Показать listening sockets.

### `-n`

```text
--numeric
```

Не преобразовывать адреса/порты в имена.

### `-t`

```text
--tcp
```

Показать TCP.

Как посмотреть:

```bash
ss --help
```

или:

```bash
man ss
```

---

Фильтруем:

```bash
ss -lnt | grep 5432
```

При остановленном PostgreSQL там не должно быть его listener.

---

# Запускаем

```bash
sudo systemctl start postgresql@16-main
```

Потом:

```bash
systemctl is-active postgresql@16-main
```

```bash
ss -lnt | grep 5432
```

---

# Restart

```bash
sudo systemctl restart postgresql@16-main
```

`restart`:

> остановить и снова запустить unit.

После него:

```bash
journalctl -u postgresql@16-main -n 30
```

Попробуй сам найти:

```text
остановку
запуск
PID
время события
```

---

# Этап 4. Инцидент №1 — сломанный postgresql.conf

**~30 минут**

Теперь начинается настоящая DBA-практика.

Перед изменениями:

```bash
sudo cp /etc/postgresql/16/main/postgresql.conf \
        /etc/postgresql/16/main/postgresql.conf.lab23.bak
```

## `cp`

Copy.

```text
cp SOURCE DESTINATION
```

Справка:

```bash
cp --help
```

или:

```bash
man cp
```

Здесь флагов вообще нет.

Мы копируем:

```text
postgresql.conf

→

postgresql.conf.lab23.bak
```

Проверь:

```bash
ls -l /etc/postgresql/16/main/postgresql.conf*
```

---

Теперь добавь в конфигурацию:

```conf
shared_buffers = ABC
```

Например:

```bash
sudo nano /etc/postgresql/16/main/postgresql.conf
```

И попробуй:

```bash
sudo systemctl restart postgresql@16-main
```

---

# Теперь самое важное

Ты знаешь, что именно сломал.

Но **делаешь вид, что не знаешь**.

Получил тикет:

> PostgreSQL pg01 недоступен.

---

## Шаг 1

```bash
systemctl status postgresql@16-main
```

Ответь:

```text
Работает?
Нет.

Каким состоянием systemd это показывает?
```

---

## Шаг 2

```bash
journalctl -u postgresql@16-main -n 50
```

Разбери:

```text
journalctl
-u          конкретный unit
-n 50       последние 50 записей
```

---

## Шаг 3

Нужен только текущий инцидент:

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

Здесь `\` — интересная вещь.

## Обратный слеш `\`

В Bash он позволяет продолжить текущую команду на следующей строке.

То есть:

```bash
journalctl -u postgresql@16-main --since "5 minutes ago"
```

и:

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

одинаковы.

Очень полезно для длинных DBA-команд.

---

# `journalctl -xeu`

Теперь:

```bash
journalctl -xeu postgresql@16-main
```

Обрати внимание:

```text
-xeu
```

= несколько коротких флагов вместе:

```text
-x
-e
-u
```

### `-x`

```text
--catalog
```

Добавлять пояснительный текст из message catalog там, где он доступен.

### `-e`

```text
--pager-end
```

Открыть вывод сразу ближе к концу journal.

Удобно, когда интересует последняя проблема.

### `-u`

Мы уже знаем:

```text
--unit
```

Поэтому:

```bash
journalctl -xeu postgresql@16-main
```

можно мысленно читать как:

```bash
journalctl \
    --catalog \
    --pager-end \
    --unit=postgresql@16-main
```

Проверяй всё это самостоятельно:

```bash
journalctl --help
```

и:

```bash
man journalctl
```

---

После нахождения ошибки исправляешь конфиг.

Можно перед restart проверить конфигурацию через сам PostgreSQL/кластерные инструменты, но это отдельно разберём позже. Сейчас задача — systemd/journal.

Запускаем:

```bash
sudo systemctl start postgresql@16-main
```

Проверяем:

```bash
systemctl is-active postgresql@16-main
```

```bash
ss -lnt | grep 5432
```

```bash
sudo -u postgres psql -c "SELECT now();"
```

---

# Разберём последнюю команду

```bash
sudo -u postgres psql -c "SELECT now();"
```

### `sudo -u postgres`

`-u` у **sudo** означает:

```text
--user
```

То есть:

> выполнить команду от пользователя `postgres`.

Важно:

```text
-u у journalctl
```

и:

```text
-u у sudo
```

**вообще не обязаны означать одно и то же.**

Флаги принадлежат конкретной программе.

Это фундаментальная вещь.

Проверить:

```bash
sudo --help
```

увидишь:

```text
-u, --user=user
```

---

### `psql -c`

У `psql`:

```text
-c
```

означает:

```text
--command
```

То есть:

> выполнить SQL-команду и завершить psql.

Проверить:

```bash
psql --help
```

или:

```bash
man psql
```

---

# Этап 5. Инцидент №2 — порт занят

**~20 минут**

Останавливаем PostgreSQL:

```bash
sudo systemctl stop postgresql@16-main
```

Проверка:

```bash
ss -lnt | grep 5432
```

Теперь:

```bash
sudo nc -l 5432
```

Если синтаксис твоей версии `nc` отличается:

```bash
nc -h
```

или:

```bash
man nc
```

### `-l`

Обычно:

```text
listen mode
```

То есть `nc` не подключается куда-то, а сам ждёт входящее TCP-соединение.

Тем самым мы искусственно занимаем:

```text
TCP :5432
```

---

Во втором терминале:

```bash
sudo systemctl start postgresql@16-main
```

Он должен не суметь нормально забиндить порт.

Расследование:

```bash
systemctl status postgresql@16-main
```

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

Потом подтверждение:

```bash
sudo ss -lntp | grep 5432
```

Появился новый флаг.

### `-p`

Для `ss`:

```text
--processes
```

Показывает процесс, использующий socket.

Итого:

```bash
ss -lntp
```

это:

```text
-l  listening
-n  numeric
-t  TCP
-p  process
```

Проверить:

```bash
ss --help
```

---

Заверши `nc`:

```text
Ctrl+C
```

И:

```bash
sudo systemctl start postgresql@16-main
```

---

# Этап 6. Работа с pager

Это тоже часть владения терминалом.

Некоторые команды:

```bash
systemctl status ...
journalctl ...
man ...
```

могут открывать вывод через программу:

```text
less
```

Ты увидишь внизу что-то вроде:

```text
lines 1-23
```

Навигация:

```text
↑ ↓        строки
Space      страница вниз
b          страница вверх
/ERROR     найти ERROR
n          следующее
N          предыдущее
G          конец
g          начало
q          выйти
```

То есть в:

```bash
journalctl -u postgresql@16-main
```

прямо внутри можно нажать:

```text
/ERROR
```

и искать сообщения.

Это надо начать использовать сразу.

---

# Этап 7. Полезные фильтры journalctl

Если останется время, обязательно попробуй.

## Текущий boot

```bash
journalctl -b
```

### `-b`

```text
--boot
```

Показывать сообщения текущей загрузки ОС.

Можно комбинировать:

```bash
journalctl -b -u postgresql@16-main
```

То есть:

> PostgreSQL events только после текущего boot.

---

## Только ошибки определённого priority

```bash
journalctl -p err
```

### `-p`

```text
--priority
```

Фильтрация по уровню сообщения.

Например:

```bash
journalctl -p warning
```

или вместе:

```bash
journalctl \
    -u postgresql@16-main \
    -p warning
```

Справка:

```bash
journalctl --help
```

и:

```bash
man journalctl
```

ищем:

```text
/--priority
```

---

# Что тебе НЕ надо запоминать

Не надо зубрить:

```text
journalctl -xeu
```

как магическое заклинание.

Ты должен понимать:

```text
journalctl
│
├── -u → какой unit
├── -n → сколько последних записей
├── -f → следить в реальном времени
├── -b → какой boot
├── -p → какой priority
├── -e → открыть в конце
├── -x → дополнительные пояснения
│
├── --since → от какого времени
└── --until → до какого времени
```

Тогда если через полгода забудешь `-p`, ты просто:

```bash
journalctl --help
```

и через пять секунд вспомнишь.

---

# Шпаргалка: как искать информацию прямо на сервере

Это хочу, чтобы после №23 у тебя уже вошло в привычку:

```bash
COMMAND --help
```

быстро посмотреть параметры.

```bash
man COMMAND
```

полная документация.

```bash
whatis COMMAND
```

что вообще делает команда.

```bash
apropos keyword
```

найти подходящую команду/man page.

```bash
type COMMAND
```

понять, что именно выполняет shell.

И внутри `man`/`less`:

```text
/слово
n
N
q
```

---

# Финальный мини-экзамен №23

В конце двух часов остановись и **не смотри мои команды**.

Получил:

```text
ALERT

pg01
PostgreSQL connection refused
```

Твои действия:

### 1. Проверить service

Сам вспомни команду.

### 2. Узнать exit/status.

### 3. Посмотреть журнал только PostgreSQL.

### 4. Ограничить journal последними 10 минутами.

### 5. Найти ERROR через pager.

### 6. Проверить, слушается ли TCP/5432.

### 7. Определить процесс, если 5432 занят.

### 8. Исправить проблему.

### 9. Проверить:

```text
systemd → OK
TCP → OK
SQL → OK
```

И отдельно ты должен уметь ответить **без зубрёжки**:

```text
Как узнать, что делает `journalctl -f`?
→ journalctl --help
→ man journalctl
→ /--follow

Как узнать, что значит `ss -p`?
→ ss --help
→ man ss

Как узнать, что делает `systemctl restart`?
→ man systemctl
→ /restart

Как найти утилиту, связанную с system journal,
если забыл название journalctl?
→ apropos journal
```

Вот такой формат я предлагаю использовать и дальше для **всего Linux-блока 23–34**: я буду давать не только лабораторию, но и учить тебя самостоятельно разбираться с незнакомыми командами, флагами, man-pages, exit codes, pipes и поведением shell. Это гораздо ближе к реальной работе DBA.
