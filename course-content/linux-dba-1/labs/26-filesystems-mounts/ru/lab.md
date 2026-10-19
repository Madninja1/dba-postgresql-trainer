# Лабораторная №26 — filesystem + mounts

**Бюджет: 2 часа**

Цель:

> Научиться понимать, где физически лежат данные PostgreSQL, какая filesystem обслуживает PGDATA, какой block device за ней стоит, сколько места и inode доступно, а также пройти путь disk → partition → filesystem → mount point на отдельном лабораторном диске.

> **Правило безопасности:** системный диск не трогаем. Перед `fdisk`, `mkfs`, `wipefs` и подобными операциями всегда сначала выполняй `lsblk -f` и глазами проверяй устройство. В примерах ниже `/dev/sdb` — специально выделенный лабораторный диск.

## 1. Находим PGDATA

```bash
PGDATA=$(sudo -u postgres psql -d lab -Atc "SHOW data_directory;")
echo "$PGDATA"
```

`-A` — unaligned output, `-t` — tuples only, `-c` — выполнить SQL-команду.

## 2. Какая filesystem обслуживает PGDATA

```bash
findmnt -T "$PGDATA"
df -hT "$PGDATA"
lsblk -f
```

`findmnt -T` отвечает на вопрос: какая mounted filesystem фактически обслуживает указанный путь.

`df -hT` показывает размер, использование и тип filesystem.

## 3. `df` и `du`

```bash
df -h "$PGDATA"
sudo du -sh "$PGDATA"
sudo du -h --max-depth=1 "$PGDATA" | sort -h
```

```text
df → использование всей filesystem
du → сколько места занимают конкретные файлы/каталоги
```

## 4. Inodes

```bash
df -i "$PGDATA"
```

`No space left on device` может означать не только отсутствие свободных гигабайт, но и исчерпание inode.

## 5. Проверяем лабораторный диск

Перед изменениями:

```bash
sudo fdisk -l /dev/sdb
lsblk -f /dev/sdb
```

Убедись, что `/dev/sdb` — именно отдельный лабораторный диск, без нужных данных и без mountpoint.

## 6. Создаём partition

Запусти:

```bash
sudo fdisk /dev/sdb
```

Внутри:

```text
m → help
p → print partition table
n → new partition
w → write table to disk and exit
q → quit without saving
```

Последовательность для одной partition на весь диск:

```text
m
p
n
Enter
Enter
Enter
p
```

На последнем `p` проверь, что создаётся именно `/dev/sdb1` нужного размера. Только после проверки используй `w`.

После выхода:

```bash
lsblk -f /dev/sdb
```

На этом этапе `/dev/sdb1` уже существует как partition, но `FSTYPE` ещё пустой.

## 7. Создаём ext4

Ещё раз проверь устройство:

```bash
lsblk -f
```

И только затем:

```bash
sudo mkfs.ext4 /dev/sdb1
```

`mkfs.ext4` создаёт новую filesystem ext4. Это destructive operation: неправильный target может уничтожить доступ к существующим данным.

После:

```bash
lsblk -f /dev/sdb
```

Должны появиться `FSTYPE=ext4` и UUID.

## 8. Создаём mount point

```bash
sudo mkdir -p /mnt/pg-lab
```

Пока `/mnt/pg-lab` — обычный каталог на родительской filesystem. Проверь:

```bash
findmnt -T /mnt/pg-lab
```

## 9. Mount

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
```

Синтаксис:

```text
mount SOURCE TARGET
```

Проверка несколькими способами:

```bash
findmnt /mnt/pg-lab
findmnt -T /mnt/pg-lab
df -hT /mnt/pg-lab
lsblk -f
```

## 10. Записываем данные

```bash
sudo touch /mnt/pg-lab/test.txt
ls -la /mnt/pg-lab
```

На ext4 также обычно будет `lost+found` — это нормальный служебный каталог filesystem.

Сравни:

```bash
df -h /mnt/pg-lab
sudo du -sh /mnt/pg-lab
```

## 11. Umount и сохранность данных

```bash
sudo umount /mnt/pg-lab
```

Проверка:

```bash
findmnt /mnt/pg-lab
lsblk -f /dev/sdb
```

Filesystem продолжает существовать, но больше не подключена к дереву каталогов.

Смонтируй обратно:

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
ls -la /mnt/pg-lab
```

`test.txt` должен остаться на месте.

## 12. Почему файл может «исчезнуть» после mount

Если создать файл в `/mnt/pg-lab`, когда `/dev/sdb1` не смонтирован, этот файл окажется на родительской filesystem. После mount содержимое подключённой filesystem закроет содержимое самого mount point. Файл не удалится и снова станет видим после umount.

Это реальный DBA-риск: ожидаемый диск может не смонтироваться, каталог `/pgdata` останется существовать, и приложение начнёт писать на root filesystem.

## 13. Диагностика `No space left on device`

До удаления файлов проверь минимум:

```bash
df -hT "$PGDATA"
df -i "$PGDATA"
findmnt -T "$PGDATA"
lsblk -f
sudo du -h --max-depth=1 "$PGDATA" | sort -h
```

Нужно ответить:

```text
какая filesystem?
какой device?
сколько свободно?
не закончились ли inode?
что занимает место?
точно ли нужный диск mounted?
```

## 14. `/etc/fstab`

Познакомься:

```bash
cat /etc/fstab
man 5 fstab
```

В этой лабораторной не редактируй `fstab` без отдельной необходимости.

## Мини-экзамен №26

1. Как узнать PGDATA из PostgreSQL?
2. Как узнать filesystem, обслуживающую PGDATA?
3. Как посмотреть размер/занято/свободно filesystem?
4. Как узнать тип filesystem?
5. Как посмотреть block devices и их filesystem/mountpoints?
6. В чём разница между `df` и `du`?
7. Как проверить inode usage?
8. Что делает `findmnt -T /some/path`?
9. Что такое mount point?
10. Почему `/pgdata` может существовать, даже если отдельный диск не смонтирован?
11. Чем disk отличается от partition и filesystem?
12. Что делает `mkfs.ext4 /dev/sdb1` и почему команда опасна?
13. Как смонтировать `/dev/sdb1` в `/mnt/pg-lab`?
14. Как размонтировать filesystem?
15. Почему файл в mount point может скрыться после mount и появиться после umount?
16. Какие проверки сделать при `No space left on device` до удаления данных?
