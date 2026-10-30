# systemd и journalctl

## 1. Тема

Для администратора PostgreSQL важно уметь отличать проблему самого сервера БД от проблемы Linux-сервиса, журнала или TCP-порта. В systemd-системах основная цепочка диагностики выглядит так:

```text
PostgreSQL недоступен
        ↓
systemctl: работает ли unit?
        ↓
journalctl: почему он не работает?
        ↓
ss: слушается ли нужный TCP-порт?
        ↓
psql: отвечает ли сама БД?
```

В этой теме используются четыре группы инструментов:

```text
справка      → man, --help, whatis, apropos, type
systemd      → systemctl
журнал       → journalctl
TCP sockets  → ss
```

Главная цель — не запомнить все параметры, а научиться быстро находить состояние системы и уточнять синтаксис прямо на сервере.

---

## 2. Механизм

### Справка: `man`, `--help`, `whatis`, `apropos`, `type`

`man COMMAND` открывает подробную manual page команды. Это основной встроенный справочник Linux для назначения, параметров и поведения программы.

```bash
man systemctl
man journalctl
```

Внутри `man` обычно используется pager `less`. Полезная навигация:

```text
/слово  → поиск вперёд
?слово  → поиск назад
n       → следующее совпадение
N       → предыдущее совпадение
g       → начало
G       → конец
q       → выход
```

Если нужна только краткая подсказка по синтаксису, удобнее:

```bash
systemctl --help
journalctl --help
```

Практическая разница:

```text
--help  → быстро вспомнить ключи
man     → разобраться подробно
```

`whatis COMMAND` полезен, когда имя команды уже известно, но нужно одной строкой понять её назначение:

```bash
whatis journalctl
```

`apropos KEYWORD` решает обратную задачу: название инструмента неизвестно, но известно, что нужно сделать.

```bash
apropos journal
apropos systemd
```

Если индекс man-страниц устарел, его можно обновить:

```bash
sudo mandb
```

`type COMMAND` показывает, что именно выполнит shell:

```bash
type cd
type ls
type systemctl
```

Команда может оказаться shell builtin, alias или внешним executable. Это важно при диагностике: внешне одинаковое имя не всегда означает отдельный бинарный файл.

---

### Units и services в systemd

`systemctl` — CLI для управления systemd. Systemd работает не только с сервисами: существуют также sockets, timers, mounts, targets и другие unit-типы.

Список загруженных сервисов:

```bash
systemctl list-units --type=service
```

Здесь `list-units` — подкоманда, а `--type=service` ограничивает вывод объектами типа `service`.

Для PostgreSQL удобно сразу отфильтровать список:

```bash
systemctl list-units --type=service | grep -i postgres
```

Pipe `|` передаёт stdout первой команды на stdin второй:

```text
systemctl → большой список
              ↓ pipe
grep -i postgres
              ↓
      только нужные строки
```

`-i` у `grep` означает регистронезависимый поиск.

---

### `status`, `is-active`, `is-enabled`

Подробное состояние unit:

```bash
systemctl status postgresql@16-main
```

На Debian/Ubuntu полезно различать общий `postgresql.service` и конкретный кластер, например:

```text
postgresql@16-main.service
```

Суффикс `.service` во многих командах systemctl можно опускать.

Для скриптов и быстрых проверок есть более узкие команды:

```bash
systemctl is-active postgresql@16-main
systemctl is-enabled postgresql@16-main
```

Это разные характеристики:

```text
active   → работает ли unit сейчас
enabled  → настроен ли запуск unit через systemd
```

Unit может быть `active`, но `disabled`: его могли запустить вручную.

После любой shell-команды код завершения доступен через:

```bash
echo $?
```

Обычно:

```text
0     → успех
не 0  → ошибка или отрицательный результат
```

Именно поэтому `is-active` и подобные команды удобны в скриптах.

---

### Управление сервисом

Основные действия:

```bash
sudo systemctl start postgresql@16-main
sudo systemctl stop postgresql@16-main
sudo systemctl restart postgresql@16-main
```

`sudo` запускает команду с повышенными правами или от имени другого пользователя.

После изменения конфигурации полезно не ограничиваться `restart`, а проверить состояние и журнал:

```bash
systemctl status postgresql@16-main
journalctl -u postgresql@16-main -n 50
```

---

### `journalctl`: журнал systemd

`journalctl` читает systemd journal. Для DBA почти всегда нужно сначала сузить поток сообщений.

По конкретному unit:

```bash
journalctl -u postgresql@16-main
```

Последние N записей:

```bash
journalctl -u postgresql@16-main -n 50
```

Ограничение по времени:

```bash
journalctl -u postgresql@16-main --since "10 minutes ago"
journalctl -u postgresql@16-main --since today --until now
```

Просмотр новых событий в реальном времени:

```bash
journalctl -u postgresql@16-main -f
```

`-f` — короткая форма `--follow`: команда не завершается после текущего вывода, а продолжает показывать новые записи.

Полезные фильтры:

```bash
journalctl -b
journalctl -p err
```

`-b` ограничивает записи выбранным boot, а `-p` — priority сообщений. Фильтры можно сочетать с `-u`.

Часто используется:

```bash
journalctl -xeu postgresql@16-main
```

Это объединение коротких параметров:

```text
-x  → добавить поясняющие тексты, где они доступны
-e  → перейти к концу журнала
-u  → выбрать unit
```

Важно: одинаковая буква параметра у разных программ может означать разное. Нельзя переносить смысл `-u` от `journalctl` к `sudo` или наоборот.

Например:

```bash
sudo -u postgres psql -c "SELECT now();"
```

Здесь `-u postgres` относится к `sudo` и означает пользователя, от имени которого нужно запустить `psql`.

---

### Проверка TCP-порта через `ss`

Если сервис выглядит запущенным, но клиент не подключается, нужно проверить socket:

```bash
ss -lnt
```

В этой команде:

```text
-l → listening sockets
-n → numeric addresses/ports
-t → TCP
```

Проверить PostgreSQL-порт:

```bash
ss -lnt | grep ':5432'
```

Если нужно увидеть процесс, которому принадлежит socket:

```bash
sudo ss -lntp | grep ':5432'
```

`-p` у `ss` добавляет сведения о процессе. Это особенно полезно при конфликте портов: PostgreSQL может не запускаться, потому что `5432` уже занят другой программой.

---

## 3. Важные детали

### Диагностика сломанного `postgresql.conf`

После ошибки конфигурации сервис часто просто не стартует. Правильная последовательность:

```text
1. systemctl status
2. journalctl -u ...
3. найти конкретную ошибку конфигурации
4. исправить файл
5. start/restart
6. проверить порт
7. выполнить SQL-проверку
```

Не следует начинать с случайного редактирования конфигурации: systemd и journal обычно уже содержат причину отказа.

### Конфликт порта

Если journal сообщает, что PostgreSQL не может привязаться к порту:

```bash
sudo ss -lntp | grep ':5432'
```

Нужно определить владельца socket, понять, ожидаемый ли это процесс, и только потом решать, что останавливать или перенастраивать.

### Pager — часть рабочего процесса

И `man`, и длинный вывод `journalctl` часто просматриваются через pager. Поэтому поиск `/`, переход к следующему совпадению `n` и выход `q` — не мелочь, а часть реальной серверной диагностики.

### Проверка после исправления

Исправление считается завершённым не тогда, когда команда `restart` вернула успех, а когда подтверждены все уровни:

```text
systemd → active
journal → нет новой критической ошибки
TCP     → порт слушается
SQL     → PostgreSQL отвечает
```

---

## Что запомнить

```text
man COMMAND
→ подробная справка

whatis COMMAND
→ краткое назначение известной команды

apropos WORD
→ поиск инструмента по назначению

type COMMAND
→ что именно выполнит shell
```

```text
systemctl status UNIT
→ подробное состояние

systemctl is-active UNIT
→ работает ли сейчас

systemctl is-enabled UNIT
→ настроен ли автозапуск
```

```text
journalctl -u UNIT
→ журнал unit

-n N
→ последние N записей

-f
→ follow

--since / --until
→ временной диапазон

-b
→ выбранный boot

-p
→ priority
```

```text
ss -lnt
→ listening TCP sockets

ss -lntp
→ то же + процессы
```

Главная диагностическая схема:

```text
service → journal → socket → SQL
```
