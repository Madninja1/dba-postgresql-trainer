# Lab 23 — systemd + journalctl

**Time: 2 hours**

Lab environment:

```text
client01
    │
    │ SSH
    ▼
pg01
Ubuntu 24.04 LTS
PostgreSQL 16
```

Goal:

> Receive a "PostgreSQL is not working" report, determine the service state, find the cause through systemd/journal, and restore the database.

At the same time, learn to use these tools properly:

```text
man
--help
apropos
whatis
systemctl
journalctl
```

---

# 0. First, learn how to get help in Linux

Spend **10–15 minutes** on this.

## `man`

The main built-in Linux reference system.

For example:

```bash
man systemctl
```

This opens the manual page.

Navigation inside `man`:

```text
↑ ↓          scroll
PageUp       one page up
PageDown     one page down
/word        search forward
?word        search backward
n            next match
N            previous match
g            beginning
G            end
q            quit
```

For example, open:

```bash
man systemctl
```

and enter:

```text
/status
```

Press Enter.

Then press `n` to find the next occurrence.

---

## How to search for a specific flag

Suppose you see:

```bash
journalctl -n 50
```

and forget what `-n` means.

Run:

```bash
man journalctl
```

Inside it:

```text
/-n
```

This search can sometimes be inconvenient because there may be many matches.

You can search for the full option name instead:

```text
/--lines
```

You will see a description roughly like:

```text
-n, --lines=
```

So the short form:

```bash
-n 50
```

and the long form:

```bash
--lines=50
```

mean the same thing.

---

# `--help`

Quick help.

```bash
systemctl --help
```

```bash
journalctl --help
```

It is usually much shorter than `man`.

For example:

```bash
journalctl --help
```

lets you quickly see:

```text
-u --unit=UNIT
-n --lines=N
-f --follow
-b --boot
-p --priority=RANGE
```

Remember the principle:

```text
--help → quickly check syntax

man → understand it properly
```

---

# `whatis`

If you do not remember at all what a program does:

```bash
whatis journalctl
```

For example, you may get something like:

```text
journalctl - Query the systemd journal
```

Likewise:

```bash
whatis systemctl
```

---

# `apropos`

A very useful tool.

Use it when you **do not know the command name**, but you know what you want to do.

For example:

```bash
apropos journal
```

or:

```bash
apropos systemd
```

Linux will show matching man pages.

So:

```text
whatis → I know the name and want to know what it does

apropos → I do not know the name and am looking for a tool
```

If `apropos` finds nothing, sometimes the database needs to be updated:

```bash
sudo mandb
```

`mandb` creates/updates the man-page index.

---

# Another useful command — `type`

For example:

```bash
type systemctl
```

It shows what the shell will actually execute.

Try:

```bash
type cd
type ls
type systemctl
```

You will see the difference between:

```text
shell builtin
alias
external executable
```

This is useful for a DBA because something that looks like the same "command" may actually be an alias or a builtin.

---

# Stage 1. Explore the PostgreSQL service

**Time: ~20 minutes**

Connect from `client01`:

```bash
ssh <user>@pg01
```

We will study `ssh` in detail in Lab 33. For now, it is simply the access method.

---

## Find PostgreSQL services

```bash
systemctl list-units --type=service
```

Let's break it down.

### `systemctl`

CLI for managing systemd.

Through it we work with:

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

This is **not a flag**; it is a `systemctl` subcommand.

General structure:

```text
systemctl <COMMAND> [OPTIONS]
```

For example:

```bash
systemctl status
systemctl start
systemctl stop
systemctl restart
systemctl list-units
```

See the commands with:

```bash
systemctl --help
```

or:

```bash
man systemctl
```

Inside `man`:

```text
/list-units
```

---

### `--type=service`

Filters units by type.

So:

```bash
systemctl list-units
```

may show:

```text
services
mounts
sockets
targets
devices
...
```

while:

```bash
systemctl list-units --type=service
```

shows only:

```text
*.service
```

Find this yourself with:

```bash
systemctl --help
```

or:

```bash
man systemctl
```

and:

```text
/--type
```

---

Now we do not need the entire huge list.

Use a pipeline:

```bash
systemctl list-units --type=service | grep -i postgres
```

Here we encounter an important construct:

```text
|
```

This is a **pipe**.

It passes stdout from one command to stdin of the next:

```text
systemctl
   │
   │ stdout
   ▼
grep
```

Conceptually:

```text
systemctl → list of 200 lines
             ↓
         grep postgres
             ↓
          2 lines
```

---

## `grep -i postgres`

`grep` searches lines.

```bash
grep postgres
```

searches with case sensitivity.

But:

```bash
grep -i postgres
```

### `-i`

`ignore case`.

So all of these can match:

```text
postgres
Postgres
POSTGRES
PostgreSQL
```

Check it yourself:

```bash
grep --help
```

Find:

```text
-i, --ignore-case
```

Full documentation:

```bash
man grep
```

and:

```text
/--ignore-case
```

---

On Ubuntu you will most likely see:

```text
postgresql.service
postgresql@16-main.service
```

There is an important detail here.

On Debian/Ubuntu:

```text
postgresql.service
```

may be a top-level wrapper/meta-unit.

The concrete cluster is:

```text
postgresql@16-main.service
```

Its state is what we are especially interested in.

---

# Check status

```bash
systemctl status postgresql@16-main
```

There are **no flags at all** here.

Syntax:

```text
systemctl
    status
    postgresql@16-main
```

Where:

```text
systemctl             program
status                command
postgresql@16-main    unit
```

Look for fields such as:

```text
Loaded:
Active:
Main PID:
Tasks:
Memory:
CPU:
```

Especially:

```text
Active: active (running)
```

---

## Why can `.service` be omitted?

We can write:

```bash
systemctl status postgresql@16-main.service
```

or often:

```bash
systemctl status postgresql@16-main
```

systemd can infer the implied `.service` suffix.

But when diagnosing problems, it is useful to understand the full name:

```text
postgresql@16-main.service
```

---

# Check state programmatically

```bash
systemctl is-active postgresql@16-main
```

Output:

```text
active
```

Now:

```bash
systemctl is-enabled postgresql@16-main
```

This answers a different question.

### `active`

Means:

> is the unit running now?

### `enabled`

Means roughly:

> is the unit configured to start through systemd when the appropriate target/boot is reached?

So:

```text
active ≠ enabled
```

A service can be:

```text
active
disabled
```

or:

```text
inactive
enabled
```

These are different properties.

---

## Check the exit code

After:

```bash
systemctl is-active postgresql@16-main
```

run:

```bash
echo $?
```

`$?` is a special shell variable containing the exit status of the **last executed command**.

Usually:

```text
0 = success
nonzero = another state / error
```

Try:

```bash
systemctl is-active postgresql@16-main
echo $?
```

Later, repeat this when PostgreSQL is stopped.

This will be useful for Bash scripts in Lab 32.

---

# Stage 2. Study journalctl

Now run:

```bash
journalctl -u postgresql@16-main
```

Let's break it down.

## `journalctl`

A tool for reading the systemd journal.

Check its purpose:

```bash
whatis journalctl
```

Quick documentation:

```bash
journalctl --help
```

Full documentation:

```bash
man journalctl
```

---

## `-u`

```bash
journalctl -u postgresql@16-main
```

`-u` means:

```text
--unit=
```

That is:

> show messages for a specific systemd unit.

Full form:

```bash
journalctl --unit=postgresql@16-main
```

Same meaning.

Verify it:

```bash
journalctl --help
```

Look for:

```text
-u --unit=UNIT
```

Or:

```bash
man journalctl
```

```text
/--unit
```

---

## Last N messages

```bash
journalctl -u postgresql@16-main -n 50
```

### `-n`

Long form:

```text
--lines=
```

So:

```bash
-n 50
```

means: show the last 50 entries.

Equivalent long form:

```bash
journalctl \
    --unit=postgresql@16-main \
    --lines=50
```

I will deliberately show the long form from time to time because it makes the meaning much easier to understand.

---

# Only messages after a specific time

```bash
journalctl -u postgresql@16-main --since "30 minutes ago"
```

### `--since`

Start-time filter.

That is:

```text
show messages starting from...
```

Also try:

```bash
journalctl -u postgresql@16-main --since today
```

and:

```bash
journalctl -u postgresql@16-main --since "10 minutes ago"
```

This is one of the most useful parameters when investigating an incident.

For example:

> The database went down at about 16:30.

You can use:

```bash
journalctl -u postgresql@16-main \
    --since "16:20" \
    --until "16:40"
```

### `--until`

The opposite boundary:

```text
up to the specified time
```

This gives a window:

```text
16:20 ───── incident ───── 16:40
  ↑                           ↑
--since                     --until
```

How to find it:

```bash
man journalctl
```

```text
/--since
```

---

# Follow — watch the journal in real time

Open a separate SSH terminal:

```bash
journalctl -u postgresql@16-main -f
```

### `-f`

Long form:

```text
--follow
```

Works roughly like:

```bash
tail -f
```

The command does not exit; it waits for new events.

Stop it with:

```text
Ctrl+C
```

Later, in a second terminal, you can restart PostgreSQL and immediately see the events.

This is a very useful mode:

```text
Terminal 1:

journalctl ... -f


Terminal 2:

systemctl restart ...
```

---

# Stage 3. Manage PostgreSQL

**~15 minutes**

Stop it:

```bash
sudo systemctl stop postgresql@16-main
```

Let's break this down.

### `sudo`

Execute a command with elevated privileges, usually as root.

Help:

```bash
sudo --help
```

or:

```bash
man sudo
```

---

### `stop`

A `systemctl` subcommand.

It is not:

```text
--stop
```

but:

```text
systemctl stop UNIT
```

Search for it:

```bash
man systemctl
```

then:

```text
/stop
```

---

After that:

```bash
systemctl status postgresql@16-main
```

Inspect the state.

And:

```bash
systemctl is-active postgresql@16-main
```

Now make sure to check:

```bash
echo $?
```

Compare the exit code with the one from the running service.

---

# Check the port

```bash
ss -lnt
```

We will study `ss` in detail in Lab 30, but for now only the basics.

The flags here are combined:

```text
-lnt
```

This is the same as:

```text
-l -n -t
```

### `-l`

```text
--listening
```

Show listening sockets.

### `-n`

```text
--numeric
```

Do not resolve addresses/ports to names.

### `-t`

```text
--tcp
```

Show TCP.

How to look this up:

```bash
ss --help
```

or:

```bash
man ss
```

---

Filter the output:

```bash
ss -lnt | grep 5432
```

When PostgreSQL is stopped, its listener should not be present.

---

# Start it

```bash
sudo systemctl start postgresql@16-main
```

Then:

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

`restart` means:

> stop and then start the unit again.

Afterward:

```bash
journalctl -u postgresql@16-main -n 30
```

Try to find on your own:

```text
stop event
start event
PID
event time
```

---

# Stage 4. Incident 1 — broken postgresql.conf

**~30 minutes**

Now the real DBA practice begins.

Before making changes:

```bash
sudo cp /etc/postgresql/16/main/postgresql.conf \
        /etc/postgresql/16/main/postgresql.conf.lab23.bak
```

## `cp`

Copy.

```text
cp SOURCE DESTINATION
```

Help:

```bash
cp --help
```

or:

```bash
man cp
```

There are no flags here at all.

We copy:

```text
postgresql.conf

→

postgresql.conf.lab23.bak
```

Verify:

```bash
ls -l /etc/postgresql/16/main/postgresql.conf*
```

---

Now add this to the configuration:

```conf
shared_buffers = ABC
```

For example:

```bash
sudo nano /etc/postgresql/16/main/postgresql.conf
```

Then try:

```bash
sudo systemctl restart postgresql@16-main
```

---

# Now the most important part

You know exactly what you broke.

But **pretend you do not know**.

You receive a ticket:

> PostgreSQL on pg01 is unavailable.

---

## Step 1

```bash
systemctl status postgresql@16-main
```

Answer:

```text
Is it running?
No.

Which systemd state shows this?
```

---

## Step 2

```bash
journalctl -u postgresql@16-main -n 50
```

Break it down:

```text
journalctl
-u          specific unit
-n 50       last 50 entries
```

---

## Step 3

We only need the current incident:

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

The `\` here is worth understanding.

## Backslash `\`

In Bash, it allows the current command to continue on the next line.

So:

```bash
journalctl -u postgresql@16-main --since "5 minutes ago"
```

and:

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

are equivalent.

This is very useful for long DBA commands.

---

# `journalctl -xeu`

Now:

```bash
journalctl -xeu postgresql@16-main
```

Notice:

```text
-xeu
```

is several short flags combined:

```text
-x
-e
-u
```

### `-x`

```text
--catalog
```

Add explanatory text from the message catalog where available.

### `-e`

```text
--pager-end
```

Open the output near the end of the journal.

Convenient when you are interested in the most recent problem.

### `-u`

We already know:

```text
--unit
```

So:

```bash
journalctl -xeu postgresql@16-main
```

can be mentally expanded as:

```bash
journalctl \
    --catalog \
    --pager-end \
    --unit=postgresql@16-main
```

Verify all of this yourself:

```bash
journalctl --help
```

and:

```bash
man journalctl
```

---

After finding the error, fix the configuration.

You can validate configuration through PostgreSQL itself or cluster tools before restarting, but we will study that separately later. The task here is systemd/journal.

Start it:

```bash
sudo systemctl start postgresql@16-main
```

Verify:

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

# Break down the last command

```bash
sudo -u postgres psql -c "SELECT now();"
```

### `sudo -u postgres`

For **sudo**, `-u` means:

```text
--user
```

That is:

> execute the command as user `postgres`.

Important:

```text
-u in journalctl
```

and:

```text
-u in sudo
```

**do not have to mean the same thing at all.**

Flags belong to a specific program.

This is fundamental.

Check:

```bash
sudo --help
```

You will see:

```text
-u, --user=user
```

---

### `psql -c`

For `psql`:

```text
-c
```

means:

```text
--command
```

That is:

> execute an SQL command and exit psql.

Check:

```bash
psql --help
```

or:

```bash
man psql
```

---

# Stage 5. Incident 2 — port already in use

**~20 minutes**

Stop PostgreSQL:

```bash
sudo systemctl stop postgresql@16-main
```

Verify:

```bash
ss -lnt | grep 5432
```

Now:

```bash
sudo nc -l 5432
```

If your version of `nc` uses different syntax:

```bash
nc -h
```

or:

```bash
man nc
```

### `-l`

Usually means:

```text
listen mode
```

So `nc` does not connect somewhere; it waits for an incoming TCP connection itself.

This deliberately occupies:

```text
TCP :5432
```

---

In a second terminal:

```bash
sudo systemctl start postgresql@16-main
```

It should fail to bind the port normally.

Investigate:

```bash
systemctl status postgresql@16-main
```

```bash
journalctl \
    -u postgresql@16-main \
    --since "5 minutes ago"
```

Then confirm:

```bash
sudo ss -lntp | grep 5432
```

There is a new flag.

### `-p`

For `ss`:

```text
--processes
```

Shows the process using the socket.

So:

```bash
ss -lntp
```

means:

```text
-l  listening
-n  numeric
-t  TCP
-p  process
```

Check:

```bash
ss --help
```

---

Stop `nc`:

```text
Ctrl+C
```

And:

```bash
sudo systemctl start postgresql@16-main
```

---

# Stage 6. Working with the pager

This is also part of being comfortable in the terminal.

Some commands:

```bash
systemctl status ...
journalctl ...
man ...
```

may open output through the program:

```text
less
```

At the bottom you may see something like:

```text
lines 1-23
```

Navigation:

```text
↑ ↓        lines
Space      page down
b          page up
/ERROR     find ERROR
n          next
N          previous
G          end
g          beginning
q          quit
```

So inside:

```bash
journalctl -u postgresql@16-main
```

you can directly press:

```text
/ERROR
```

and search the messages.

Start using this immediately.

---

# Stage 7. Useful journalctl filters

If there is time left, make sure to try these.

## Current boot

```bash
journalctl -b
```

### `-b`

```text
--boot
```

Show messages from the current OS boot.

You can combine it:

```bash
journalctl -b -u postgresql@16-main
```

That means:

> PostgreSQL events only since the current boot.

---

## Only messages of a certain priority

```bash
journalctl -p err
```

### `-p`

```text
--priority
```

Filter by message priority.

For example:

```bash
journalctl -p warning
```

or together:

```bash
journalctl \
    -u postgresql@16-main \
    -p warning
```

Help:

```bash
journalctl --help
```

and:

```bash
man journalctl
```

search for:

```text
/--priority
```

---

# What you do NOT need to memorize

Do not memorize:

```text
journalctl -xeu
```

as a magic incantation.

You should understand:

```text
journalctl
│
├── -u → which unit
├── -n → how many latest entries
├── -f → follow in real time
├── -b → which boot
├── -p → which priority
├── -e → open at the end
├── -x → additional explanations
│
├── --since → starting from what time
└── --until → up to what time
```

Then if you forget `-p` six months from now, you simply run:

```bash
journalctl --help
```

and remember it in five seconds.

---

# Cheat sheet: how to find information directly on the server

After Lab 23, this should already start becoming a habit:

```bash
COMMAND --help
```

quickly inspect options.

```bash
man COMMAND
```

full documentation.

```bash
whatis COMMAND
```

what the command does at all.

```bash
apropos keyword
```

find a relevant command/man page.

```bash
type COMMAND
```

understand what the shell will actually execute.

And inside `man`/`less`:

```text
/word
n
N
q
```

---

# Final mini-exam 23

At the end of the two hours, stop and **do not look at my commands**.

You receive:

```text
ALERT

pg01
PostgreSQL connection refused
```

Your actions:

### 1. Check the service

Recall the command yourself.

### 2. Determine exit/status.

### 3. Show only the PostgreSQL journal.

### 4. Limit the journal to the last 10 minutes.

### 5. Find ERROR through the pager.

### 6. Check whether TCP/5432 is listening.

### 7. Identify the process if 5432 is occupied.

### 8. Fix the problem.

### 9. Verify:

```text
systemd → OK
TCP → OK
SQL → OK
```

And separately, you should be able to answer **without memorization**:

```text
How do I find out what `journalctl -f` does?
→ journalctl --help
→ man journalctl
→ /--follow

How do I find out what `ss -p` means?
→ ss --help
→ man ss

How do I find out what `systemctl restart` does?
→ man systemctl
→ /restart

How do I find a utility related to the system journal
if I forgot the name journalctl?
→ apropos journal
```

This is the format I suggest using for the rest of the **Linux block 23–34** as well: I will not only give you a lab, but also teach you to independently understand unfamiliar commands, flags, man pages, exit codes, pipes, and shell behavior. This is much closer to real DBA work.
