# Lab 26 — filesystem + mounts

**Budget: 2 hours**

Goal:

> Learn to understand where PostgreSQL data physically resides, which filesystem serves PGDATA, which block device is behind it, how much space and how many inodes are available, and walk through the full path disk → partition → filesystem → mount point using a separate lab disk.

> **Safety rule:** do not touch the system disk. Before `fdisk`, `mkfs`, `wipefs`, or similar operations, always run `lsblk -f` first and visually verify the device. In the examples below, `/dev/sdb` is a dedicated lab disk.

## 1. Find PGDATA

```bash
PGDATA=$(sudo -u postgres psql -d lab -Atc "SHOW data_directory;")
echo "$PGDATA"
```

`-A` — unaligned output, `-t` — tuples only, `-c` — execute an SQL command.

## 2. Which filesystem serves PGDATA

```bash
findmnt -T "$PGDATA"
df -hT "$PGDATA"
lsblk -f
```

`findmnt -T` answers the question: which mounted filesystem actually serves the specified path.

`df -hT` shows filesystem size, usage, and type.

## 3. `df` and `du`

```bash
df -h "$PGDATA"
sudo du -sh "$PGDATA"
sudo du -h --max-depth=1 "$PGDATA" | sort -h
```

```text
df → usage of the whole filesystem
du → how much space specific files/directories use
```

## 4. Inodes

```bash
df -i "$PGDATA"
```

`No space left on device` can mean not only that free gigabytes are exhausted, but also that the filesystem has run out of inodes.

## 5. Verify the lab disk

Before making changes:

```bash
sudo fdisk -l /dev/sdb
lsblk -f /dev/sdb
```

Make sure `/dev/sdb` is the dedicated lab disk, contains no needed data, and has no mount point.

## 6. Create a partition

Run:

```bash
sudo fdisk /dev/sdb
```

Inside `fdisk`:

```text
m → help
p → print partition table
n → new partition
w → write table to disk and exit
q → quit without saving
```

Sequence for one partition spanning the entire disk:

```text
m
p
n
Enter
Enter
Enter
p
```

At the final `p`, verify that the partition being created is exactly `/dev/sdb1` and has the expected size. Only after checking should you use `w`.

After exiting:

```bash
lsblk -f /dev/sdb
```

At this point `/dev/sdb1` already exists as a partition, but `FSTYPE` is still empty.

## 7. Create ext4

Check the device once more:

```bash
lsblk -f
```

Only then run:

```bash
sudo mkfs.ext4 /dev/sdb1
```

`mkfs.ext4` creates a new ext4 filesystem. This is a destructive operation: targeting the wrong device can destroy access to existing data.

Afterward:

```bash
lsblk -f /dev/sdb
```

You should see `FSTYPE=ext4` and a UUID.

## 8. Create a mount point

```bash
sudo mkdir -p /mnt/pg-lab
```

For now, `/mnt/pg-lab` is an ordinary directory on its parent filesystem. Check:

```bash
findmnt -T /mnt/pg-lab
```

## 9. Mount

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
```

Syntax:

```text
mount SOURCE TARGET
```

Verify it in several ways:

```bash
findmnt /mnt/pg-lab
findmnt -T /mnt/pg-lab
df -hT /mnt/pg-lab
lsblk -f
```

## 10. Write data

```bash
sudo touch /mnt/pg-lab/test.txt
ls -la /mnt/pg-lab
```

On ext4, you will also usually see `lost+found` — this is a normal filesystem service directory.

Compare:

```bash
df -h /mnt/pg-lab
sudo du -sh /mnt/pg-lab
```

## 11. Umount and data persistence

```bash
sudo umount /mnt/pg-lab
```

Verify:

```bash
findmnt /mnt/pg-lab
lsblk -f /dev/sdb
```

The filesystem still exists, but it is no longer attached to the directory tree.

Mount it again:

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
ls -la /mnt/pg-lab
```

`test.txt` should still be there.

## 12. Why a file can "disappear" after mount

If you create a file in `/mnt/pg-lab` while `/dev/sdb1` is not mounted, that file is created on the parent filesystem. After mounting, the contents of the mounted filesystem cover the original contents of the mount-point directory. The file is not deleted and becomes visible again after unmounting.

This is a real DBA risk: an expected disk may fail to mount, the `/pgdata` directory may still exist, and the application can start writing to the root filesystem instead.

## 13. Diagnose `No space left on device`

Before deleting files, check at least:

```bash
df -hT "$PGDATA"
df -i "$PGDATA"
findmnt -T "$PGDATA"
lsblk -f
sudo du -h --max-depth=1 "$PGDATA" | sort -h
```

You need to answer:

```text
which filesystem?
which device?
how much free space?
are inodes exhausted?
what is using the space?
is the expected disk definitely mounted?
```

## 14. `/etc/fstab`

Take a look at:

```bash
cat /etc/fstab
man 5 fstab
```

Do not edit `fstab` in this lab unless there is a separate reason to do so.

## Mini-exam 26

1. How do you get PGDATA from PostgreSQL?
2. How do you determine which filesystem serves PGDATA?
3. How do you see filesystem size/used/free space?
4. How do you determine the filesystem type?
5. How do you list block devices and their filesystems/mount points?
6. What is the difference between `df` and `du`?
7. How do you check inode usage?
8. What does `findmnt -T /some/path` do?
9. What is a mount point?
10. Why can `/pgdata` exist even when a separate disk is not mounted?
11. What is the difference between a disk, a partition, and a filesystem?
12. What does `mkfs.ext4 /dev/sdb1` do, and why is it dangerous?
13. How do you mount `/dev/sdb1` at `/mnt/pg-lab`?
14. How do you unmount a filesystem?
15. Why can a file in a mount point become hidden after mount and reappear after umount?
16. What checks should you perform for `No space left on device` before deleting data?
