# Linux Users, Groups, and Permissions

## 1. Topic

When PostgreSQL gets `Permission denied`, the problem is usually not “PostgreSQL in general,” but a concrete Linux access chain:

```text
which user is running the process?
        ↓
who owns the file/directory?
        ↓
which group is assigned to the object?
        ↓
which rwx permissions apply?
        ↓
can the process traverse every parent directory?
```

Linux identifies users and groups with numeric UID/GID values. Names such as `postgres` or `dba1` are convenient representations of those identifiers.

---

## 2. Mechanism

### User, UID, and GID

Check identity information:

```bash
id
id postgres
```

The command shows the UID, primary GID, and supplementary groups.

Information from the system user and group databases can be queried conveniently with `getent`:

```bash
getent passwd postgres
getent group dbadmins
```

### PostgreSQL user

The PostgreSQL server normally runs as a dedicated operating-system user named `postgres`. Therefore, file access is determined by the permissions of **that Linux user**, not by the SQL role used to connect to the database.

```text
SQL role
≠
Linux user running the PostgreSQL process
```

### Creating a group and a user

```bash
sudo groupadd dbadmins
sudo useradd -m dba1
```

`-m` for `useradd` creates a home directory.

Add the user to a supplementary group:

```bash
sudo usermod -aG dbadmins dba1
```

The combination matters:

```text
-G → set supplementary groups
-a → append to the current list
```

Using `-G` without `-a` can replace the existing supplementary-group list. After changing group membership, an already open user session may not see the new group until the user logs in again.

---

### Owner, group, and `rwx`

Inspect directory permissions:

```bash
ls -ld /srv/pgbackup
```

Typical mode:

```text
drwxr-x---
│└─┬─┘└─┬─┘└─┬─┘
│  │    │    └ others
│  │    └ group
│  └ owner
└ directory
```

Bits:

```text
r → read
w → write
x → execute/traverse
```

For a regular file:

```text
r → read
w → modify
x → execute
```

For a directory:

```text
r → read the list of names
w → create/delete directory entries
x → traverse the directory and access objects inside it
```

The `x` bit on every parent directory is critical for accessing a nested file.

---

### Octal notation and `chmod`

```text
r = 4
w = 2
x = 1
```

Therefore:

```text
7 = rwx
6 = rw-
5 = r-x
4 = r--
0 = ---
```

The command:

```bash
chmod 750 /srv/pgbackup
```

produces:

```text
owner  → rwx
group  → r-x
others → ---
```

---

### `chown` and `chgrp`

Change the owner:

```bash
sudo chown postgres /srv/pgbackup
```

Change owner and group together:

```bash
sudo chown postgres:dbadmins /srv/pgbackup
```

Change only the group:

```bash
sudo chgrp dbadmins /srv/pgbackup
```

The `owner:group` syntax lets one command set both properties.

---

### Verify permissions with a real operation

Simply looking at `ls -l` does not prove that the required user can actually perform the operation.

For example:

```bash
sudo -u postgres touch /srv/pgbackup/postgres.txt
sudo -u dba1 touch /srv/pgbackup/dba1.txt
```

This tests access specifically as the selected user.

Shell redirection has an important nuance. This command:

```bash
sudo -u dba1 echo test >> /srv/pgbackup/test.txt
```

can be misleading because `>>` is processed by the original shell before `sudo` starts the command.

A better form is:

```bash
sudo -u dba1 sh -c 'echo test >> /srv/pgbackup/test.txt'
```

Now both the command and the redirection run in the `dba1` context.

---

### `umask`

`umask` does not set a final mode directly; it removes bits from the base permissions of newly created objects.

Simplified model used in the lab:

```text
file      → base 666
directory → base 777
```

With:

```bash
umask 022
```

the usual resulting permissions are:

```text
file      → 644
directory → 755
```

A stricter value:

```bash
umask 0077
```

removes group/other permissions from new objects. `umask` applies in the context of the current shell and is inherited by its child processes; it is not a global setting for the entire operating system.

---

## 3. Important details

### Why correct permissions on the file itself may still be insufficient

Suppose the file is accessible to `postgres`, but one parent directory lacks `x`:

```text
/srv
  ↓ x?
/srv/dba-lab
  ↓ x?
/srv/dba-lab/data
  ↓ x?
report.txt
```

Then the process cannot traverse the path to the file.

For diagnostics:

```bash
namei -l /srv/dba-lab/data/report.txt
```

`namei -l` shows every path component together with ownership and permissions, making it easy to see where access breaks.

Also useful:

```bash
ls -ld PATH
stat PATH
```

`stat` provides detailed metadata for an object.

### Least privilege

```bash
chmod -R 777 /srv/dba-lab
```

may temporarily hide the problem, but it grants much broader access than necessary. The correct approach is to identify the exact cause and grant only the minimum required permissions.

### `Permission denied` diagnostic chain

```text
1. which Linux user runs the process? → id / ps
2. who is owner/group?                → ls -l / stat
3. which rwx bits apply?              → mode
4. is the required group present?     → id / getent
5. can the whole path be traversed?   → namei -l
6. does the real operation work?      → sudo -u ...
```

---

## What to remember

```bash
id USER
getent passwd USER
getent group GROUP
```

→ user, UID/GID, and groups.

```text
file:
r read, w write, x execute

directory:
r list, w modify entries, x traverse
```

```text
chmod 750
→ owner rwx, group r-x, others ---
```

```bash
chown owner:group PATH
chgrp group PATH
```

→ owner and group.

```text
umask 022
→ usually 644 for files and 755 for directories
```

```bash
namei -l /path/to/file
```

→ inspect the entire path when troubleshooting `Permission denied`.

Main principle: **permissions must be tested as the Linux user that actually performs the operation**.
