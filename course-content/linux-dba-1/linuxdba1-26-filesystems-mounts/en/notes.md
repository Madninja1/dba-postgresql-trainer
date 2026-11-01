# Filesystems and Mounts

## 1. Topic

PostgreSQL stores data in directories, but a Linux directory is only a path in a single filesystem tree. The actual data can reside on a specific block device, partition, and filesystem mounted at that path.

A useful model:

```text
PostgreSQL PGDATA
      ↓ path
mount point
      ↓
filesystem (for example ext4)
      ↓
partition
      ↓
block device / disk
```

A DBA should be able to answer four questions:

```text
where is PGDATA?
which filesystem serves that path?
how much space and how many inodes remain?
what exactly consumes the space?
```

---

## 2. Mechanism

### Finding the actual PGDATA

The most reliable source is PostgreSQL itself:

```sql
SHOW data_directory;
```

For a shell script, this is convenient:

```bash
PGDATA=$(sudo -u postgres psql -Atc "SHOW data_directory;")
```

The `-Atc` combination is used for compact machine-readable output:

```text
-A → unaligned output
-t → tuples only, without headers/footer lines
-c → execute one SQL command
```

---

### Which filesystem serves the path

```bash
df -hT "$PGDATA"
```

`df` reports the state of the filesystem as a whole. In this form:

```text
-h → human-readable sizes
-T → filesystem type
```

For a more precise path-to-mount mapping:

```bash
findmnt -T "$PGDATA"
```

`findmnt -T` determines the filesystem that actually serves the specified path.

For block devices:

```bash
lsblk -f
```

The command shows devices, partitions, filesystems, and mount points.

---

### `df` versus `du`

They answer different questions:

```text
df
→ how much space is used/free on the whole filesystem

du
→ how much space a specific file/directory tree consumes
```

For example:

```bash
df -h "$PGDATA"
du -sh "$PGDATA"
```

The values do not have to match: `df` looks at the filesystem, while `du` sums visible files in the selected tree.

---

### Inodes

Free bytes are not the only limited filesystem resource. Every file and directory requires an inode.

Check them with:

```bash
df -i "$PGDATA"
```

`-i` switches the report to inode usage.

Therefore, the error:

```text
No space left on device
```

can occur for two fundamentally different reasons:

```text
blocks/bytes are exhausted
or
inodes are exhausted
```

---

### Disk → partition → filesystem → mount point

Suppose there is a separate disk:

```text
/dev/sdb
```

After creating a partition:

```text
/dev/sdb1
```

A filesystem can be created on that partition:

```bash
sudo mkfs.ext4 /dev/sdb1
```

Then it can be mounted into a directory:

```bash
sudo mkdir -p /mnt/pg-lab
sudo mount /dev/sdb1 /mnt/pg-lab
```

Hierarchy:

```text
/dev/sdb       disk
/dev/sdb1      partition
ext4           filesystem
/mnt/pg-lab    mount point
```

A mount point is an ordinary directory that becomes the access point to the mounted filesystem after `mount`.

---

### `fdisk`

Safely inspect the partition table:

```bash
sudo fdisk -l /dev/sdb
```

Here `-l` is used for listing.

Important commands in interactive `fdisk`:

```text
m → help
p → print partition table
n → new partition
w → write changes
q → quit without writing
```

Disk partitioning operations are potentially destructive, so verify the selected device several times before writing changes.

---

### `mkfs.ext4`

```bash
sudo mkfs.ext4 /dev/sdb1
```

creates a new ext4 filesystem on the selected partition. This is not an inspection command: it writes filesystem structures and can destroy previous data on the device.

---

### Mount and umount

After:

```bash
sudo mount /dev/sdb1 /mnt/pg-lab
```

the filesystem contents are accessible through `/mnt/pg-lab`.

Unmount it with:

```bash
sudo umount /mnt/pg-lab
```

The data remains on the filesystem itself; only the attachment to this point in the directory tree is removed.

---

### Why a file can “disappear” after mount

Suppose a file existed in the directory before mounting:

```text
/mnt/pg-lab/old.txt
```

After another filesystem is mounted on `/mnt/pg-lab`, the old file is not deleted, but it becomes temporarily hidden by the mounted filesystem contents.

```text
before mount:
/mnt/pg-lab → directory on the root filesystem

after mount:
/mnt/pg-lab → root of another filesystem
```

After `umount`, the original contents become visible again.

This is especially dangerous for data directories: the system may write to the “wrong disk” when an expected mount is missing.

---

### Directory exists ≠ filesystem is mounted

For example, `/pgdata` can always exist as a normal directory on the root filesystem. Therefore, checking only:

```bash
ls -ld /pgdata
```

does not prove that a separate disk is mounted there.

Check instead:

```bash
findmnt -T /pgdata
df -hT /pgdata
```

---

### Loop device

For a lab, a physical extra disk can be simulated by an ordinary file attached as a block device.

```bash
sudo losetup -fP image.img
```

Here:

```text
-f → select a free /dev/loopX
-P → scan the partition table
```

After that, the loop device can be used almost like a normal disk: create partitions, create a filesystem, and mount it.

---

## 3. Important details

### Diagnosing `No space left on device`

Minimum sequence:

```text
1. identify the path/PGDATA
2. determine the filesystem and device
3. check blocks: df -h
4. check inodes: df -i
5. verify the correct mount
6. find space consumers: du
```

So you must not stop at `df -h` alone.

### `/etc/fstab`

A manual `mount` lasts until `umount` or reboot. Persistent mounts are normally described in `/etc/fstab`.

For DBA work, a persistent-mount failure is dangerous because the expected mount point can remain an ordinary directory on the root filesystem.

### Before `mkfs`, the device name matters most

The filesystem creation command does not know whether you selected “the right disk.” The administrator is responsible for choosing `/dev/...`. Before running `mkfs`, verify `lsblk -f`, the device size, and the expected partition layout.

---

## What to remember

```sql
SHOW data_directory;
```

→ actual PGDATA.

```bash
df -hT PATH
findmnt -T PATH
lsblk -f
```

→ filesystem, mount, and block devices.

```text
df → whole filesystem
du → specific file tree
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

Main idea: **an existing directory does not prove that the expected filesystem is actually mounted there**.
