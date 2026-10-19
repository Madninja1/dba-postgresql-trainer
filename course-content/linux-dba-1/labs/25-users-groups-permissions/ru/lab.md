# Лабораторная №25 — пользователи, группы и права Linux

**Бюджет: 1,5 часа**

Цель:

> Понять, от какого Linux-пользователя работает PostgreSQL, как Linux решает «можно ли этому процессу открыть файл/каталог», и уметь диагностировать `Permission denied`.

После лабораторной ты должен понимать:

```text
user
group
UID / GID
owner / group / others
r / w / x
chmod
chown
chgrp
umask
```

И уметь пройти цепочку:

```text
PostgreSQL получил Permission denied
            ↓
какой Linux-user выполняет операцию?
            ↓
кто владелец файла/каталога?
            ↓
какая группа?
            ↓
какие rwx?
            ↓
есть ли доступ по всей цепочке каталогов?
```

---

# 1. Пользователь — это не просто имя

Начнём на `pg01`.

Выполни:

```bash
whoami
```

Команда показывает пользователя, от имени которого работает текущий shell.

Справка:

```bash
whoami --help
```

или:

```bash
man whoami
```

Теперь:

```bash
id
```

Получишь примерно:

```text
uid=1000=...
gid=1000=...
groups=...
```

Команда `id` показывает идентификаторы пользователя и групп.

Посмотреть справку:

```bash
id --help
```

```bash
man id
```

---

## UID и GID

Linux внутри работает не с именами вроде:

```text
daniil
postgres
root
```

а прежде всего с числами:

```text
UID = User ID
GID = Group ID
```

Например условно:

```text
postgres
UID = 114
GID = 120
```

Имя `postgres` — удобное человеку представление UID.

Посмотри:

```bash
id postgres
```

Например:

```text
uid=114(postgres) gid=120(postgres) groups=120(postgres),...
```

---

# 2. От какого пользователя работает PostgreSQL

Мы уже умеем:

```bash
ps -ef | grep postgres
```

Посмотри первый столбец:

```text
UID
```

У PostgreSQL-процессов должен быть пользователь:

```text
postgres
```

Можно точнее:

```bash
pgrep -a postgres
```

А затем:

```bash
ps -fp PID
```

Смысл:

```text
PostgreSQL server
      ↓
Linux processes
      ↓
Linux user postgres
```

Поэтому если у пользователя `postgres` нет доступа к файлу, PostgreSQL тоже не сможет его открыть.

Это фундаментально.

---

# 3. Посмотрим группы

Для текущего пользователя:

```bash
groups
```

Для `postgres`:

```bash
groups postgres
```

Либо:

```bash
id postgres
```

Разница:

```text
groups → быстро список групп

id → UID + primary GID + дополнительные группы
```

Справка:

```bash
groups --help
```

```bash
man groups
```

---

# 4. Создаём лабораторного пользователя и группу

Теперь будет практический стенд.

Создадим группу:

```bash
sudo groupadd dbadmins
```

### `groupadd`

Создаёт Linux-группу.

Проверить:

```bash
groupadd --help
```

или:

```bash
man groupadd
```

Проверим:

```bash
getent group dbadmins
```

## `getent`

Очень полезная системная команда.

Она получает записи из системных баз данных NSS:

```text
passwd
group
hosts
services
...
```

То есть:

```bash
getent group dbadmins
```

означает:

> покажи запись группы `dbadmins`.

Справка:

```bash
getent --help
```

```bash
man getent
```

---

# 5. Создаём пользователя `dba1`

```bash
sudo useradd -m dba1
```

Разберём.

## `useradd`

Создаёт пользователя.

```bash
useradd --help
```

```bash
man useradd
```

### `-m`

Полная идея:

```text
--create-home
```

Создать домашний каталог пользователя.

Без этого в некоторых конфигурациях пользователь может появиться без `/home/dba1`.

Проверяем:

```bash
id dba1
```

```bash
getent passwd dba1
```

Получишь строку вроде:

```text
dba1:x:1001:1001::/home/dba1:/bin/sh
```

Формат `/etc/passwd` пока глубоко разбирать не надо, но основные части:

```text
username
UID
GID
home directory
shell
```

Можешь посмотреть:

```bash
man 5 passwd
```

Обрати внимание на `5`.

Man pages разделены на секции.

Например:

```text
1 → пользовательские команды
5 → форматы файлов
8 → системные/admin-команды
```

Поэтому:

```bash
man passwd
```

и:

```bash
man 5 passwd
```

могут означать разные вещи.

Это очень полезно знать.

---

# 6. Добавляем пользователя в группу

```bash
sudo usermod -aG dbadmins dba1
```

Вот эту команду важно понять, а не зазубрить.

## `usermod`

Изменяет существующего пользователя.

```bash
usermod --help
```

или:

```bash
man usermod
```

### `-G`

Supplementary groups.

То есть список дополнительных групп.

### `-a`

Append.

Добавить к существующим группам, а не заменить весь список.

Поэтому:

```bash
usermod -aG dbadmins dba1
```

читается:

> добавить `dba1` в дополнительную группу `dbadmins`, сохранив остальные его группы.

Очень важный момент:

```bash
usermod -G dbadmins dba1
```

без `-a` может заменить supplementary groups пользователя.

Поэтому привычная безопасная связка:

```text
-aG
```

Проверяем:

```bash
id dba1
```

или:

```bash
groups dba1
```

---

# 7. Создаём каталог для лаборатории

```bash
sudo mkdir -p /srv/pgbackup
```

## `mkdir`

Make directory.

### `-p`

Создать родительские каталоги при необходимости и не ругаться, если они уже существуют.

Проверить:

```bash
mkdir --help
```

или:

```bash
man mkdir
```

Теперь:

```bash
ls -ld /srv/pgbackup
```

Обрати внимание:

```text
-d
```

Без `-d`:

```bash
ls -l /srv/pgbackup
```

будет показывать **содержимое каталога**.

С `-d`:

```bash
ls -ld /srv/pgbackup
```

показываем информацию **о самом каталоге**.

### `-l`

Long listing format.

Проверить:

```bash
ls --help
```

или:

```bash
man ls
```

---

# 8. Разбираем права `rwx`

Допустим:

```text
drwxr-x---
```

Разделим:

```text
d | rwx | r-x | ---
    │     │     │
    │     │     └── others
    │     └──────── group
    └────────────── owner
```

Первый символ:

```text
d = directory
- = regular file
l = symbolic link
```

Дальше три тройки.

```text
r = read
w = write
x = execute
```

Но смысл для **файла и каталога отличается**.

---

# 9. `rwx` для обычного файла

Для файла:

```text
r → прочитать содержимое
w → изменить содержимое
x → выполнить файл как программу/скрипт
```

Например:

```text
-rw-r-----
```

означает:

```text
owner:  rw-
group:  r--
others: ---
```

---

# 10. `rwx` для каталога

Вот это для DBA намного важнее.

Для каталога:

### `r`

Позволяет прочитать список имён внутри каталога.

То есть примерно:

```bash
ls directory
```

### `w`

Позволяет создавать/удалять/переименовывать entries внутри каталога.

### `x`

Позволяет **проходить через каталог** и обращаться к объектам внутри него.

Это очень важный момент.

Например пользователь может иметь read на файле:

```text
-rw-r--r--
```

но если у него нет `x` на одном из родительских каталогов:

```text
/srv/project/data/file
```

он всё равно не сможет открыть файл.

Это классическая причина:

```text
Permission denied
```

---

# 11. Посмотрим права через `stat`

```bash
stat /srv/pgbackup
```

`stat` показывает гораздо больше информации, чем `ls -l`.

Там будут:

```text
Size
Blocks
IO Block
Device
Uid
Gid
Access
Modify
Change
```

Справка:

```bash
stat --help
```

или:

```bash
man stat
```

Для нас сейчас особенно:

```text
Access: (0755/drwxr-xr-x)
Uid:
Gid:
```

---

# 12. `chmod`

Теперь меняем permissions.

```bash
sudo chmod 750 /srv/pgbackup
```

`chmod` = change mode.

Справка:

```bash
chmod --help
```

```bash
man chmod
```

Но что означает:

```text
750
```

Это octal notation.

Каждому разрешению соответствует число:

```text
r = 4
w = 2
x = 1
```

Складываем.

### 7

```text
4 + 2 + 1 = 7

rwx
```

### 6

```text
4 + 2 = 6

rw-
```

### 5

```text
4 + 1 = 5

r-x
```

### 4

```text
r--
```

Поэтому:

```text
750
```

это:

```text
7 → owner → rwx
5 → group → r-x
0 → others → ---
```

То есть:

```text
rwxr-x---
```

Проверяем:

```bash
ls -ld /srv/pgbackup
```

---

# 13. Быстрая таблица chmod

Эту таблицу стоит понимать:

| Число | Права |
|---:|---|
| 0 | `---` |
| 1 | `--x` |
| 2 | `-w-` |
| 3 | `-wx` |
| 4 | `r--` |
| 5 | `r-x` |
| 6 | `rw-` |
| 7 | `rwx` |

Не обязательно каждый раз зубрить комбинации.

Просто:

```text
r = 4
w = 2
x = 1
```

и складываешь.

---

# 14. `chown` — владелец

Посмотрим текущего владельца:

```bash
ls -ld /srv/pgbackup
```

Теперь:

```bash
sudo chown postgres /srv/pgbackup
```

`chown` = change owner.

Справка:

```bash
chown --help
```

```bash
man chown
```

Проверка:

```bash
ls -ld /srv/pgbackup
```

Теперь owner должен быть:

```text
postgres
```

---

# 15. Меняем owner и group одновременно

Очень распространённый синтаксис:

```bash
sudo chown postgres:dbadmins /srv/pgbackup
```

Здесь:

```text
postgres   → owner
dbadmins   → group
```

Проверка:

```bash
ls -ld /srv/pgbackup
```

Получаем концептуально:

```text
drwxr-x--- postgres dbadmins ...
```

---

# 16. `chgrp`

Если владельца менять не надо, а только группу:

```bash
sudo chgrp dbadmins /srv/pgbackup
```

`chgrp` = change group.

Справка:

```bash
chgrp --help
```

```bash
man chgrp
```

На практике часто используют `chown owner:group`, но `chgrp` тоже нужно знать.

---

# 17. Практическая задача №1

Наша цель:

```text
postgres → полный доступ
dba1     → чтение + вход в каталог
others   → ничего
```

Сделай:

```bash
sudo chown postgres:dbadmins /srv/pgbackup
sudo chmod 750 /srv/pgbackup
```

Получаем:

```text
owner postgres:
rwx

group dbadmins:
r-x

others:
---
```

Поскольку `dba1 ∈ dbadmins`, он получает group permissions.

---

# 18. Проверяем права не глазами

Это принципиально.

Нельзя закончить на:

```bash
ls -l
```

и сказать:

> вроде правильно.

Нужно реально выполнить операцию от конкретного пользователя.

## От `postgres`

```bash
sudo -u postgres touch /srv/pgbackup/postgres.txt
```

Должно работать.

### `touch`

Создаёт пустой файл, если его нет, либо обновляет timestamps существующего.

```bash
touch --help
```

```bash
man touch
```

---

## От `dba1`

Попробуй:

```bash
sudo -u dba1 touch /srv/pgbackup/dba1.txt
```

Что произойдёт?

С текущими:

```text
750
```

группа имеет:

```text
r-x
```

но **не имеет `w`**.

Поэтому создать файл `dba1` не сможет.

Это ожидаемо.

Но он должен суметь посмотреть содержимое:

```bash
sudo -u dba1 ls /srv/pgbackup
```

---

# 19. Теперь разрешаем группе запись

Меняем:

```bash
sudo chmod 770 /srv/pgbackup
```

Что значит:

```text
7 → owner rwx
7 → group rwx
0 → others ---
```

Теперь:

```bash
sudo -u dba1 touch /srv/pgbackup/dba1.txt
```

должно работать.

Проверяем:

```bash
ls -l /srv/pgbackup
```

Обрати внимание:

созданный `dba1` файл, скорее всего, будет принадлежать:

```text
dba1 : dba1
```

а не:

```text
dba1 : dbadmins
```

К этому ещё вернёмся позже, когда будем разбирать setgid на каталогах. Сейчас достаточно заметить поведение.

---

# 20. Практикуемся с файлами

Создай:

```bash
sudo -u postgres touch /srv/pgbackup/test.txt
```

Теперь:

```bash
sudo chmod 640 /srv/pgbackup/test.txt
```

Разбери сам:

```text
6 = ?
4 = ?
0 = ?
```

Ответ:

```text
owner → rw-
group → r--
others → ---
```

Но посмотри:

```bash
ls -l /srv/pgbackup/test.txt
```

Какая группа у файла?

Если группа не `dbadmins`, наш `dba1` может не получить ожидаемый доступ.

Исправим:

```bash
sudo chown postgres:dbadmins /srv/pgbackup/test.txt
```

Теперь:

```bash
sudo -u dba1 cat /srv/pgbackup/test.txt
```

должно работать.

Но:

```bash
sudo -u dba1 sh -c 'echo test >> /srv/pgbackup/test.txt'
```

должно закончиться:

```text
Permission denied
```

Потому что group имеет `r--`, но не `w`.

---

# 21. Зачем здесь `sh -c`

Вот интересная shell-деталь.

На первый взгляд хочется написать:

```bash
sudo -u dba1 echo test >> /srv/pgbackup/test.txt
```

Но это может вести себя не так, как ожидаешь.

Почему?

Потому что:

```text
sudo -u dba1 echo test
```

выполнится как `dba1`.

А вот перенаправление:

```text
>>
```

обрабатывает **твой текущий shell**, ещё до запуска `sudo`.

Поэтому для честной проверки:

```bash
sudo -u dba1 sh -c 'echo test >> /srv/pgbackup/test.txt'
```

Здесь уже shell `sh` сам работает от `dba1` и выполняет redirect от имени `dba1`.

Это очень полезная вещь для Linux.

---

# 22. Символьная форма chmod

Не всегда надо использовать числа.

Например:

```bash
chmod u+x script.sh
```

где:

```text
u = user/owner
g = group
o = others
a = all
```

и:

```text
+ добавить
- убрать
= установить ровно
```

Например:

```bash
chmod g+w file
```

Добавить группе write.

```bash
chmod o-r file
```

Убрать read у others.

```bash
chmod u=rw,g=r,o= file
```

Установить:

```text
owner rw-
group r--
others ---
```

Это эквивалент:

```bash
chmod 640 file
```

Попрактикуйся на `/tmp/testperm`.

---

# 23. `umask`

Теперь создадим:

```bash
umask
```

Получишь что-то вроде:

```text
0022
```

Что это такое?

`umask` определяет, какие permissions **убираются по умолчанию при создании новых файлов/каталогов**.

Упрощённо:

```text
file base:      666
directory base: 777
```

Если:

```text
umask = 022
```

получаем обычно:

```text
files:
666 - 022 → 644

directories:
777 - 022 → 755
```

Создай:

```bash
mkdir /tmp/umask-lab
cd /tmp/umask-lab
touch file1
mkdir dir1
ls -ld file1 dir1
```

Скорее всего увидишь:

```text
file1 → 644
dir1  → 755
```

---

# 24. Временно меняем umask

В текущем shell:

```bash
umask 0077
```

Теперь:

```bash
touch file2
mkdir dir2
ls -ld file2 dir2
```

Ожидаемо:

```text
file2 → 600
dir2  → 700
```

Потом обязательно верни предыдущий `umask`.

Если был:

```text
0022
```

то:

```bash
umask 0022
```

Важно:

> эта команда изменяет `umask` текущего shell и его дочерних процессов, а не глобально всей ОС.

---

# 25. DBA-инцидент: `Permission denied`

Теперь главная часть лабораторной.

Сделаем каталог:

```bash
sudo mkdir -p /srv/dba-lab/data
```

Создадим файл:

```bash
sudo touch /srv/dba-lab/data/report.txt
```

Сделаем:

```bash
sudo chown postgres:postgres /srv/dba-lab/data/report.txt
sudo chmod 600 /srv/dba-lab/data/report.txt
```

Проверяем:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

Работает.

Теперь испортим **родительский каталог**:

```bash
sudo chmod 700 /srv/dba-lab
sudo chown root:root /srv/dba-lab
```

И снова:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

Получишь:

```text
Permission denied
```

Хотя сам файл:

```bash
ls -l /srv/dba-lab/data/report.txt
```

может выглядеть совершенно правильно.

Вот почему смотреть только файл недостаточно.

---

# 26. Как диагностировать такую проблему

Начинаем с:

```bash
ls -l /srv/dba-lab/data/report.txt
```

Файл вроде нормальный.

Теперь:

```bash
ls -ld /srv
ls -ld /srv/dba-lab
ls -ld /srv/dba-lab/data
```

И видим:

```text
/srv/dba-lab
root:root
700
```

У `postgres` нет `x`.

Значит он не может пройти:

```text
/srv
 ↓
dba-lab  ← BLOCKED
 ↓
data
 ↓
report.txt
```

---

# 27. `namei` — очень полезный инструмент

Есть ещё замечательная команда:

```bash
namei -l /srv/dba-lab/data/report.txt
```

Если установлена.

### `-l`

Long listing.

Она показывает permissions каждого элемента пути примерно так:

```text
drwxr-xr-x root     root     /
drwxr-xr-x root     root     srv
drwx------ root     root     dba-lab
drwxr-xr-x root     root     data
-rw------- postgres postgres report.txt
```

И сразу видно место, где доступ ломается.

Справка:

```bash
namei --help
```

```bash
man namei
```

Для DBA это очень полезная диагностическая команда.

---

# 28. Исправляем

Не делаем:

```bash
chmod -R 777 /srv/dba-lab
```

Это плохая привычка.

Вместо этого решаем, **какой доступ реально нужен**.

Например позволим проходить каталогу:

```bash
sudo chmod 711 /srv/dba-lab
```

Разбираем:

```text
owner → rwx
group → --x
others → --x
```

Теперь PostgreSQL способен пройти через каталог, но не обязательно перечислять его содержимое.

Проверяем:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

---

# 29. Почему `chmod 777` — не решение

`777`:

```text
owner  rwx
group  rwx
others rwx
```

То есть вообще любой пользователь получает полный доступ.

DBA должен мыслить:

```text
не "как убрать Permission denied?"

а

"какие минимальные права нужны этому процессу?"
```

Это принцип least privilege.

---

# Итоговая практика — 15 минут

Теперь самостоятельно настрой:

```text
/srv/pgbackup
```

Требования:

```text
owner: postgres
group: dbadmins

postgres:
читать
писать
создавать файлы
заходить в каталог

dba1:
читать
писать
создавать файлы
заходить в каталог

остальные:
никакого доступа
```

Нужный режим каталога ты уже проходил.

Проверяй:

```bash
ls -ld /srv/pgbackup
```

```bash
stat /srv/pgbackup
```

```bash
sudo -u postgres touch /srv/pgbackup/from-postgres
```

```bash
sudo -u dba1 touch /srv/pgbackup/from-dba1
```

И негативный тест от пользователя, который не входит в группу. Например если существует `nobody`:

```bash
sudo -u nobody ls /srv/pgbackup
```

Он должен получить отказ.

---

# Что должно остаться после №25

Не список команд, а модель:

```text
PROCESS
   │
   └─ работает от USER
            │
            ├─ UID
            ├─ primary group
            └─ supplementary groups

FILE / DIRECTORY
   │
   ├─ OWNER
   ├─ GROUP
   │
   └─ PERMISSIONS
       ├─ owner rwx
       ├─ group rwx
       └─ others rwx
```

И Linux решает доступ, сопоставляя эти вещи.

Для PostgreSQL:

```text
postgres process
      ↓
Linux user postgres
      ↓
permissions файлов PGDATA / WAL / backup / logs / certificates...
```

Поэтому Linux permissions для DBA — не отдельная тема от PostgreSQL.

---

## Мини-экзамен №25

Только из материала выше.

1. Как узнать UID, GID и группы пользователя `postgres`?

2. Как посмотреть owner/group/permissions каталога `/srv/pgbackup`?

3. Что означает:

```text
drwxr-x---
```

4. Что означает:

```bash
chmod 640 file
```

5. В чём разница между `chown` и `chmod`?

6. Что делает:

```bash
usermod -aG dbadmins dba1
```

и зачем там `-a`?

7. Почему пользователь может иметь `r` на файле, но всё равно получить `Permission denied`?

8. Как проверить доступ именно от пользователя `postgres`, а не от своего пользователя?

9. Что обычно создаст `umask 022`:

```text
обычный файл → ?
каталог      → ?
```

10. Почему при `Permission denied` не надо автоматически делать `chmod 777`?

11. Какой командой можно удобно посмотреть permissions **всех каталогов по пути** до файла?

И отдельно маленькая практическая задача:

```text
/srv/backup

owner = postgres
group = dbadmins

postgres → rwx
dbadmins → rwx
others   → ---
```

Напиши две команды, которыми ты выставишь владельца/группу и permissions.

После этого №25 можно будет закрывать и переходить к **№26 filesystem + mounts**.
