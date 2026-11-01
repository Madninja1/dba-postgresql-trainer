# Lab 25 — Linux users, groups, and permissions

**Budget: 1.5 hours**

Goal:

> Understand which Linux user PostgreSQL runs as, how Linux decides whether a process may open a file/directory, and how to diagnose `Permission denied`.

After the lab, you should understand:

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

And be able to follow this chain:

```text
PostgreSQL got Permission denied
            ↓
which Linux user performs the operation?
            ↓
who owns the file/directory?
            ↓
which group?
            ↓
which rwx permissions?
            ↓
is there access through the entire directory path?
```

---

# 1. A user is more than just a name

Start on `pg01`.

Run:

```bash
whoami
```

The command shows the user under which the current shell is running.

Help:

```bash
whoami --help
```

or:

```bash
man whoami
```

Now:

```bash
id
```

You will get something like:

```text
uid=1000=...
gid=1000=...
groups=...
```

The `id` command shows user and group identifiers.

See the help:

```bash
id --help
```

```bash
man id
```

---

## UID and GID

Internally, Linux does not primarily work with names such as:

```text
daniil
postgres
root
```

but with numbers:

```text
UID = User ID
GID = Group ID
```

For example, conceptually:

```text
postgres
UID = 114
GID = 120
```

The name `postgres` is a human-friendly representation of a UID.

Check:

```bash
id postgres
```

For example:

```text
uid=114(postgres) gid=120(postgres) groups=120(postgres),...
```

---

# 2. Which user PostgreSQL runs as

We already know:

```bash
ps -ef | grep postgres
```

Look at the first column:

```text
UID
```

PostgreSQL processes should run as user:

```text
postgres
```

More precisely:

```bash
pgrep -a postgres
```

Then:

```bash
ps -fp PID
```

Meaning:

```text
PostgreSQL server
      ↓
Linux processes
      ↓
Linux user postgres
```

Therefore, if user `postgres` cannot access a file, PostgreSQL cannot open it either.

This is fundamental.

---

# 3. Inspect groups

For the current user:

```bash
groups
```

For `postgres`:

```bash
groups postgres
```

Or:

```bash
id postgres
```

Difference:

```text
groups → quickly show the group list

id → UID + primary GID + supplementary groups
```

Help:

```bash
groups --help
```

```bash
man groups
```

---

# 4. Create a lab user and group

Now build a practical lab environment.

Create a group:

```bash
sudo groupadd dbadmins
```

### `groupadd`

Creates a Linux group.

Check:

```bash
groupadd --help
```

or:

```bash
man groupadd
```

Verify:

```bash
getent group dbadmins
```

## `getent`

A very useful system command.

It retrieves entries from NSS system databases:

```text
passwd
group
hosts
services
...
```

So:

```bash
getent group dbadmins
```

means:

> show the `dbadmins` group entry.

Help:

```bash
getent --help
```

```bash
man getent
```

---

# 5. Create user `dba1`

```bash
sudo useradd -m dba1
```

Let's break it down.

## `useradd`

Creates a user.

```bash
useradd --help
```

```bash
man useradd
```

### `-m`

The full idea is:

```text
--create-home
```

Create the user's home directory.

Without this, in some configurations the user may be created without `/home/dba1`.

Verify:

```bash
id dba1
```

```bash
getent passwd dba1
```

You will get a line similar to:

```text
dba1:x:1001:1001::/home/dba1:/bin/sh
```

You do not need to study the `/etc/passwd` format deeply yet, but the main parts are:

```text
username
UID
GID
home directory
shell
```

You can inspect:

```bash
man 5 passwd
```

Notice the `5`.

Man pages are divided into sections.

For example:

```text
1 → user commands
5 → file formats
8 → system/admin commands
```

Therefore:

```bash
man passwd
```

and:

```bash
man 5 passwd
```

can refer to different things.

This is very useful to know.

---

# 6. Add the user to a group

```bash
sudo usermod -aG dbadmins dba1
```

This command is important to understand rather than memorize.

## `usermod`

Modifies an existing user.

```bash
usermod --help
```

or:

```bash
man usermod
```

### `-G`

Supplementary groups.

That is, the list of additional groups.

### `-a`

Append.

Add to the existing groups instead of replacing the entire list.

So:

```bash
usermod -aG dbadmins dba1
```

reads as:

> add `dba1` to the supplementary group `dbadmins` while preserving the user's other groups.

An important detail:

```bash
usermod -G dbadmins dba1
```

without `-a` can replace the user's supplementary groups.

That is why the common safe combination is:

```text
-aG
```

Verify:

```bash
id dba1
```

or:

```bash
groups dba1
```

---

# 7. Create a directory for the lab

```bash
sudo mkdir -p /srv/pgbackup
```

## `mkdir`

Make directory.

### `-p`

Create parent directories when needed and do not complain if they already exist.

Check:

```bash
mkdir --help
```

or:

```bash
man mkdir
```

Now:

```bash
ls -ld /srv/pgbackup
```

Notice:

```text
-d
```

Without `-d`:

```bash
ls -l /srv/pgbackup
```

shows the **contents of the directory**.

With `-d`:

```bash
ls -ld /srv/pgbackup
```

we show information **about the directory itself**.

### `-l`

Long listing format.

Check:

```bash
ls --help
```

or:

```bash
man ls
```

---

# 8. Understand `rwx` permissions

Suppose you have:

```text
drwxr-x---
```

Split it up:

```text
d | rwx | r-x | ---
    │     │     │
    │     │     └── others
    │     └──────── group
    └────────────── owner
```

The first character:

```text
d = directory
- = regular file
l = symbolic link
```

Then come three groups of three characters.

```text
r = read
w = write
x = execute
```

But the meaning is different for a **file and a directory**.

---

# 9. `rwx` for a regular file

For a file:

```text
r → read contents
w → modify contents
x → execute the file as a program/script
```

For example:

```text
-rw-r-----
```

means:

```text
owner:  rw-
group:  r--
others: ---
```

---

# 10. `rwx` for a directory

This is much more important for a DBA.

For a directory:

### `r`

Allows reading the list of names inside the directory.

Roughly:

```bash
ls directory
```

### `w`

Allows creating/deleting/renaming entries inside the directory.

### `x`

Allows **traversing the directory** and accessing objects inside it.

This is extremely important.

For example, a user may have read permission on a file:

```text
-rw-r--r--
```

but if they do not have `x` on one of its parent directories:

```text
/srv/project/data/file
```

they still cannot open the file.

This is a classic cause of:

```text
Permission denied
```

---

# 11. Inspect permissions with `stat`

```bash
stat /srv/pgbackup
```

`stat` shows much more information than `ls -l`.

It includes:

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

Help:

```bash
stat --help
```

or:

```bash
man stat
```

For us, especially:

```text
Access: (0755/drwxr-xr-x)
Uid:
Gid:
```

---

# 12. `chmod`

Now change permissions.

```bash
sudo chmod 750 /srv/pgbackup
```

`chmod` = change mode.

Help:

```bash
chmod --help
```

```bash
man chmod
```

But what does:

```text
750
```

mean?

This is octal notation.

Each permission corresponds to a number:

```text
r = 4
w = 2
x = 1
```

Add them together.

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

Therefore:

```text
750
```

means:

```text
7 → owner → rwx
5 → group → r-x
0 → others → ---
```

That is:

```text
rwxr-x---
```

Verify:

```bash
ls -ld /srv/pgbackup
```

---

# 13. Quick chmod table

This table is worth understanding:

| Number | Permissions |
|---:|---|
| 0 | `---` |
| 1 | `--x` |
| 2 | `-w-` |
| 3 | `-wx` |
| 4 | `r--` |
| 5 | `r-x` |
| 6 | `rw-` |
| 7 | `rwx` |

You do not need to memorize every combination separately.

Just remember:

```text
r = 4
w = 2
x = 1
```

and add them.

---

# 14. `chown` — owner

Check the current owner:

```bash
ls -ld /srv/pgbackup
```

Now:

```bash
sudo chown postgres /srv/pgbackup
```

`chown` = change owner.

Help:

```bash
chown --help
```

```bash
man chown
```

Verify:

```bash
ls -ld /srv/pgbackup
```

The owner should now be:

```text
postgres
```

---

# 15. Change owner and group together

A very common syntax:

```bash
sudo chown postgres:dbadmins /srv/pgbackup
```

Here:

```text
postgres   → owner
dbadmins   → group
```

Verify:

```bash
ls -ld /srv/pgbackup
```

Conceptually, you get:

```text
drwxr-x--- postgres dbadmins ...
```

---

# 16. `chgrp`

If the owner should remain unchanged and only the group needs to change:

```bash
sudo chgrp dbadmins /srv/pgbackup
```

`chgrp` = change group.

Help:

```bash
chgrp --help
```

```bash
man chgrp
```

In practice, `chown owner:group` is often used, but you should know `chgrp` as well.

---

# 17. Practical task 1

Our goal:

```text
postgres → full access
dba1     → read + enter the directory
others   → no access
```

Run:

```bash
sudo chown postgres:dbadmins /srv/pgbackup
sudo chmod 750 /srv/pgbackup
```

This gives:

```text
owner postgres:
rwx

group dbadmins:
r-x

others:
---
```

Because `dba1 ∈ dbadmins`, it receives the group permissions.

---

# 18. Test permissions by actually using them

This is essential.

Do not stop at:

```bash
ls -l
```

and say:

> looks right.

Actually perform the operation as the specific user.

## As `postgres`

```bash
sudo -u postgres touch /srv/pgbackup/postgres.txt
```

This should work.

### `touch`

Creates an empty file if it does not exist, or updates timestamps on an existing file.

```bash
touch --help
```

```bash
man touch
```

---

## As `dba1`

Try:

```bash
sudo -u dba1 touch /srv/pgbackup/dba1.txt
```

What happens?

With the current mode:

```text
750
```

the group has:

```text
r-x
```

but **does not have `w`**.

Therefore `dba1` cannot create the file.

That is expected.

But the user should be able to list the contents:

```bash
sudo -u dba1 ls /srv/pgbackup
```

---

# 19. Now allow group write access

Change the mode:

```bash
sudo chmod 770 /srv/pgbackup
```

Meaning:

```text
7 → owner rwx
7 → group rwx
0 → others ---
```

Now:

```bash
sudo -u dba1 touch /srv/pgbackup/dba1.txt
```

should work.

Check:

```bash
ls -l /srv/pgbackup
```

Notice:

the file created by `dba1` will most likely belong to:

```text
dba1 : dba1
```

rather than:

```text
dba1 : dbadmins
```

We will return to this later when studying setgid on directories. For now, just observe the behavior.

---

# 20. Practice with files

Create:

```bash
sudo -u postgres touch /srv/pgbackup/test.txt
```

Now:

```bash
sudo chmod 640 /srv/pgbackup/test.txt
```

Break it down yourself:

```text
6 = ?
4 = ?
0 = ?
```

Answer:

```text
owner → rw-
group → r--
others → ---
```

But check:

```bash
ls -l /srv/pgbackup/test.txt
```

Which group owns the file?

If the group is not `dbadmins`, our `dba1` user may not receive the expected access.

Fix it:

```bash
sudo chown postgres:dbadmins /srv/pgbackup/test.txt
```

Now:

```bash
sudo -u dba1 cat /srv/pgbackup/test.txt
```

should work.

But:

```bash
sudo -u dba1 sh -c 'echo test >> /srv/pgbackup/test.txt'
```

should fail with:

```text
Permission denied
```

because the group has `r--` but not `w`.

---

# 21. Why `sh -c` is used here

This is an interesting shell detail.

At first glance, you may want to write:

```bash
sudo -u dba1 echo test >> /srv/pgbackup/test.txt
```

But this can behave differently from what you expect.

Why?

Because:

```text
sudo -u dba1 echo test
```

runs as `dba1`.

But the redirection:

```text
>>
```

is handled by **your current shell**, before `sudo` starts.

Therefore, for a proper test:

```bash
sudo -u dba1 sh -c 'echo test >> /srv/pgbackup/test.txt'
```

Now the `sh` shell itself runs as `dba1` and performs the redirection as `dba1`.

This is a very useful Linux detail.

---

# 22. Symbolic chmod form

You do not always need numeric modes.

For example:

```bash
chmod u+x script.sh
```

where:

```text
u = user/owner
g = group
o = others
a = all
```

and:

```text
+ add
- remove
= set exactly
```

For example:

```bash
chmod g+w file
```

Add write permission for the group.

```bash
chmod o-r file
```

Remove read permission from others.

```bash
chmod u=rw,g=r,o= file
```

Set:

```text
owner rw-
group r--
others ---
```

This is equivalent to:

```bash
chmod 640 file
```

Practice on `/tmp/testperm`.

---

# 23. `umask`

Now run:

```bash
umask
```

You may get something like:

```text
0022
```

What is it?

`umask` determines which permissions are **removed by default when new files/directories are created**.

Simplified:

```text
file base:      666
directory base: 777
```

If:

```text
umask = 022
```

you usually get:

```text
files:
666 - 022 → 644

directories:
777 - 022 → 755
```

Create:

```bash
mkdir /tmp/umask-lab
cd /tmp/umask-lab
touch file1
mkdir dir1
ls -ld file1 dir1
```

You will most likely see:

```text
file1 → 644
dir1  → 755
```

---

# 24. Temporarily change umask

In the current shell:

```bash
umask 0077
```

Now:

```bash
touch file2
mkdir dir2
ls -ld file2 dir2
```

Expected:

```text
file2 → 600
dir2  → 700
```

Then make sure to restore the previous `umask`.

If it was:

```text
0022
```

then:

```bash
umask 0022
```

Important:

> this command changes the `umask` of the current shell and its child processes, not the entire OS globally.

---

# 25. DBA incident: `Permission denied`

Now the main part of the lab.

Create a directory:

```bash
sudo mkdir -p /srv/dba-lab/data
```

Create a file:

```bash
sudo touch /srv/dba-lab/data/report.txt
```

Set:

```bash
sudo chown postgres:postgres /srv/dba-lab/data/report.txt
sudo chmod 600 /srv/dba-lab/data/report.txt
```

Verify:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

It works.

Now break the **parent directory**:

```bash
sudo chmod 700 /srv/dba-lab
sudo chown root:root /srv/dba-lab
```

And again:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

You will get:

```text
Permission denied
```

Even though the file itself:

```bash
ls -l /srv/dba-lab/data/report.txt
```

may look completely correct.

This is why looking only at the file is not enough.

---

# 26. How to diagnose this problem

Start with:

```bash
ls -l /srv/dba-lab/data/report.txt
```

The file looks fine.

Now:

```bash
ls -ld /srv
ls -ld /srv/dba-lab
ls -ld /srv/dba-lab/data
```

And we see:

```text
/srv/dba-lab
root:root
700
```

`postgres` has no `x` permission.

So it cannot traverse:

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

# 27. `namei` — a very useful tool

There is another excellent command:

```bash
namei -l /srv/dba-lab/data/report.txt
```

if it is installed.

### `-l`

Long listing.

It shows permissions for every path component, roughly like this:

```text
drwxr-xr-x root     root     /
drwxr-xr-x root     root     srv
drwx------ root     root     dba-lab
drwxr-xr-x root     root     data
-rw------- postgres postgres report.txt
```

You can immediately see where access breaks.

Help:

```bash
namei --help
```

```bash
man namei
```

This is a very useful diagnostic command for a DBA.

---

# 28. Fix it

Do not do this:

```bash
chmod -R 777 /srv/dba-lab
```

That is a bad habit.

Instead, decide **what access is actually required**.

For example, allow traversal through the directory:

```bash
sudo chmod 711 /srv/dba-lab
```

Break it down:

```text
owner → rwx
group → --x
others → --x
```

Now PostgreSQL can traverse the directory, but does not necessarily have permission to list its contents.

Verify:

```bash
sudo -u postgres cat /srv/dba-lab/data/report.txt
```

---

# 29. Why `chmod 777` is not a solution

`777`:

```text
owner  rwx
group  rwx
others rwx
```

That means every user gets full access.

A DBA should think:

```text
not "how do I get rid of Permission denied?"

but

"what minimum permissions does this process need?"
```

This is the principle of least privilege.

---

# Final practice — 15 minutes

Now configure this on your own:

```text
/srv/pgbackup
```

Requirements:

```text
owner: postgres
group: dbadmins

postgres:
read
write
create files
enter the directory

dba1:
read
write
create files
enter the directory

others:
no access
```

You have already covered the required directory mode.

Verify with:

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

And perform a negative test as a user that is not in the group. For example, if `nobody` exists:

```bash
sudo -u nobody ls /srv/pgbackup
```

It should be denied.

---

# What should remain after Lab 25

Not a list of commands, but a model:

```text
PROCESS
   │
   └─ runs as USER
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

Linux decides access by matching these things.

For PostgreSQL:

```text
postgres process
      ↓
Linux user postgres
      ↓
permissions on PGDATA / WAL / backup / logs / certificates...
```

So Linux permissions are not a separate topic from PostgreSQL for a DBA.

---

## Mini-exam 25

Only from the material above.

1. How do you find the UID, GID, and groups of user `postgres`?

2. How do you display owner/group/permissions for `/srv/pgbackup`?

3. What does this mean:

```text
drwxr-x---
```

4. What does this do:

```bash
chmod 640 file
```

5. What is the difference between `chown` and `chmod`?

6. What does this command do:

```bash
usermod -aG dbadmins dba1
```

and why is `-a` there?

7. Why can a user have `r` permission on a file and still get `Permission denied`?

8. How do you test access specifically as user `postgres`, rather than as your own user?

9. What does `umask 022` normally produce for:

```text
regular file → ?
directory    → ?
```

10. Why should you not automatically run `chmod 777` when you see `Permission denied`?

11. Which command conveniently shows permissions for **all directories along the path** to a file?

And one small practical task:

```text
/srv/backup

owner = postgres
group = dbadmins

postgres → rwx
dbadmins → rwx
others   → ---
```

Write two commands that set the owner/group and permissions.

After that, Lab 25 is complete and you can move on to **Lab 26 — filesystem + mounts**.
