# Файловые системы и монтирование

## 1. Тема

PostgreSQL хранит данные в каталогах, но каталог Linux — это только путь в едином дереве файловой системы. Реальные данные могут находиться на конкретном block device, partition и filesystem, смонтированной в этот путь.

Полезная модель:

```text
PostgreSQL PGDATA
      ↓ путь
mount point
      ↓
filesystem (например ext4)
      ↓
partition
      ↓
block device / disk
```

Для DBA важно уметь ответить на четыре вопроса:

```text
где находится PGDATA?
какая filesystem обслуживает этот путь?
сколько места и inode осталось?
что именно занимает пространство?
```

---

## 2. Механизм

### Находим фактический PGDATA

Самый надёжный источник — сам PostgreSQL:

```sql
SHOW data_directory;
```

Для shell-скрипта удобно:

```bash
PGDATA=$(sudo -u postgres psql -Atc "SHOW data_directory;")
```

Комбинация `-Atc` используется для компактного машинно-читаемого результата:

```text
-A → unaligned output
-t → tuples only, без заголовков/служебных строк
-c → выполнить одну SQL-команду
```

---

### Какая filesystem обслуживает путь

```bash
df -hT "$PGDATA"
```

`df` показывает состояние файловой системы целиком. В этой форме:

```text
-h → human-readable размеры
-T → тип filesystem
```

Более точная привязка пути к mount:

```bash
findmnt -T "$PGDATA"
```

`findmnt -T` определяет filesystem, которая фактически обслуживает указанный путь.

Для block devices:

```bash
lsblk -f
```

Команда показывает устройства, разделы, filesystem и точки монтирования.

---

### `df` против `du`

Это разные вопросы:

```text
df
→ сколько занято/свободно на filesystem целиком

du
→ сколько занимает конкретное дерево файлов и каталогов
```

Например:

```bash
df -h "$PGDATA"
du -sh "$PGDATA"
```

Значения не обязаны совпадать: `df` смотрит на filesystem, `du` суммирует видимые файлы выбранного дерева.

---

### Inodes

Свободные байты — не единственный ограниченный ресурс файловой системы. Каждый файл/каталог требует inode.

Проверка:

```bash
df -i "$PGDATA"
```

`-i` переключает отчёт на inode usage.

Поэтому ошибка:

```text
No space left on device
```

может возникнуть в двух принципиально разных случаях:

```text
закончились блоки/байты
или
закончились inode
```

---

### Disk → partition → filesystem → mount point

Допустим, есть отдельный диск:

```text
/dev/sdb
```

После создания раздела:

```text
/dev/sdb1
```

На разделе можно создать filesystem:

```bash
sudo mkfs.ext4 /dev/sdb1
```

А затем смонтировать её в каталог:

```bash
sudo mkdir -p /mnt/pg-lab
sudo mount /dev/sdb1 /mnt/pg-lab
```

Иерархия:

```text
/dev/sdb       disk
/dev/sdb1      partition
ext4           filesystem
/mnt/pg-lab    mount point
```

Mount point — обычный каталог, который после `mount` становится точкой доступа к содержимому подключённой filesystem.

---

### `fdisk`

Безопасно посмотреть partition table:

```bash
sudo fdisk -l /dev/sdb
```

`-l` здесь используется для listing.

В интерактивном `fdisk` важны команды:

```text
m → help
p → print partition table
n → new partition
w → write changes
q → quit without writing
```

Операции разметки диска потенциально разрушительны, поэтому перед записью нужно несколько раз проверить выбранное устройство.

---

### `mkfs.ext4`

```bash
sudo mkfs.ext4 /dev/sdb1
```

создаёт новую ext4 filesystem на выбранном разделе. Это не «формат просмотра»: операция записывает структуру файловой системы и может уничтожить предыдущие данные на устройстве.

---

### Mount и umount

После:

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
```

содержимое filesystem доступно через `/mnt/pg-lab`.

Отключить:

```bash
sudo umount /mnt/pg-lab
```

Данные остаются на самой filesystem; исчезает только её привязка к этому месту дерева каталогов.

---

### Почему файл «исчезает» после mount

Пусть до монтирования в каталоге уже был файл:

```text
/mnt/pg-lab/old.txt
```

После монтирования другой filesystem на `/mnt/pg-lab` старый файл не удаляется, но временно скрывается содержимым смонтированной filesystem.

```text
до mount:
/mnt/pg-lab → каталог root filesystem

после mount:
/mnt/pg-lab → корень другой filesystem
```

После `umount` старое содержимое снова становится видимым.

Это особенно опасно для каталогов данных: система может писать «не на тот диск», если ожидаемое монтирование отсутствует.

---

### Каталог существует ≠ filesystem смонтирована

Например, `/pgdata` может существовать всегда как обычный каталог root filesystem. Поэтому проверка только:

```bash
ls -ld /pgdata
```

не подтверждает наличие отдельного диска.

Нужно проверять:

```bash
findmnt -T /pgdata
df -hT /pgdata
```

---

### Loop device

Для лаборатории физический дополнительный диск можно имитировать обычным файлом, подключённым как block device.

```bash
sudo losetup -fP image.img
```

Здесь:

```text
-f → выбрать свободный /dev/loopX
-P → просканировать partition table
```

После этого loop device можно использовать почти как обычный диск: создавать partitions, filesystem и выполнять mount.

---

## 3. Важные детали

### Диагностика `No space left on device`

Минимальная последовательность:

```text
1. определить путь/PGDATA
2. выяснить filesystem и device
3. проверить блоки: df -h
4. проверить inode: df -i
5. убедиться в правильном mount
6. найти потребителей места: du
```

То есть нельзя ограничиваться только `df -h`.

### `/etc/fstab`

Ручной `mount` действует до `umount` или перезагрузки. Для постоянного монтирования конфигурация обычно описывается в `/etc/fstab`.

При DBA-работе ошибка в постоянном монтировании опасна тем, что ожидаемый mount point может остаться обычным каталогом на root filesystem.

### Перед `mkfs` важнее всего имя устройства

Команда создания filesystem не знает, «тот ли это диск». Ответственность за выбор `/dev/...` лежит на администраторе. Перед `mkfs` полезно сверять `lsblk -f`, размер устройства и ожидаемую разметку.

---

## Что запомнить

```sql
SHOW data_directory;
```

→ фактический PGDATA.

```bash
df -hT PATH
findmnt -T PATH
lsblk -f
```

→ filesystem, mount и block devices.

```text
df → filesystem целиком
du → конкретное дерево файлов
```

```bash
df -i PATH
```

→ inode usage.

```text
disk → partition → filesystem → mount point
```

```bash
fdisk -l /dev/sdb
mkfs.ext4 /dev/sdb1
mount /dev/sdb1 /mnt/pg-lab
umount /mnt/pg-lab
```

Главная мысль: **существующий каталог ещё не доказывает, что нужная filesystem действительно смонтирована**.
