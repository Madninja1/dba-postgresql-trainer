# systemd and journalctl

## 1. Topic

For a PostgreSQL administrator, it is important to distinguish a database-server problem from a Linux service, journal, or TCP port problem. On systemd-based systems, the main diagnostic chain looks like this:

```text
PostgreSQL is unavailable
        ↓
systemctl: is the unit running?
        ↓
journalctl: why is it not running?
        ↓
ss: is the required TCP port listening?
        ↓
psql: does the database itself respond?
```

This topic uses four groups of tools:

```text
help          → man, --help, whatis, apropos, type
systemd       → systemctl
journal       → journalctl
TCP sockets   → ss
```

The main goal is not to memorize every option, but to learn how to quickly inspect system state and clarify command syntax directly on the server.

---

## 2. Mechanism

### Help: `man`, `--help`, `whatis`, `apropos`, `type`

`man COMMAND` opens the detailed manual page for a command. It is the primary built-in Linux reference for a program's purpose, options, and behavior.

```bash
man systemctl
man journalctl
```

Inside `man`, the pager is usually `less`. Useful navigation:

```text
/word  → search forward
?word  → search backward
n      → next match
N      → previous match
g      → beginning
G      → end
q      → quit
```

If you only need a short syntax reminder, this is often more convenient:

```bash
systemctl --help
journalctl --help
```

Practical difference:

```text
--help  → quickly recall options
man     → study behavior in detail
```

`whatis COMMAND` is useful when the command name is already known and you want a one-line description of its purpose:

```bash
whatis journalctl
```

`apropos KEYWORD` solves the opposite problem: you do not know the tool name, but you know what you want to do.

```bash
apropos journal
apropos systemd
```

If the man-page index is outdated, it can be refreshed:

```bash
sudo mandb
```

`type COMMAND` shows what the shell will actually execute:

```bash
type cd
type ls
type systemctl
```

A command name may resolve to a shell builtin, an alias, or an external executable. This matters during troubleshooting: the same-looking name does not always mean a standalone binary.

---

### Units and services in systemd

`systemctl` is the CLI for managing systemd. Systemd handles more than services: there are also sockets, timers, mounts, targets, and other unit types.

List loaded services:

```bash
systemctl list-units --type=service
```

Here `list-units` is a subcommand, while `--type=service` restricts the output to objects of type `service`.

For PostgreSQL, it is convenient to filter the list immediately:

```bash
systemctl list-units --type=service | grep -i postgres
```

The pipe `|` sends stdout of the first command to stdin of the second:

```text
systemctl → large list
              ↓ pipe
grep -i postgres
              ↓
      only matching lines
```

`-i` for `grep` means case-insensitive matching.

---

### `status`, `is-active`, `is-enabled`

Detailed unit state:

```bash
systemctl status postgresql@16-main
```

On Debian/Ubuntu, it is useful to distinguish the general `postgresql.service` from a concrete cluster, for example:

```text
postgresql@16-main.service
```

The `.service` suffix can be omitted in many `systemctl` commands.

For scripts and quick checks, there are narrower commands:

```bash
systemctl is-active postgresql@16-main
systemctl is-enabled postgresql@16-main
```

These describe different properties:

```text
active   → is the unit running now?
enabled  → is startup through systemd configured?
```

A unit can be `active` while still being `disabled`: it may have been started manually.

After any shell command, the exit status is available through:

```bash
echo $?
```

Typically:

```text
0      → success
non-0  → error or negative result
```

That is why `is-active` and similar commands are convenient in scripts.

---

### Managing a service

Basic actions:

```bash
sudo systemctl start postgresql@16-main
sudo systemctl stop postgresql@16-main
sudo systemctl restart postgresql@16-main
```

`sudo` runs a command with elevated privileges or as another user.

After changing configuration, do not stop at `restart`; verify state and inspect the journal:

```bash
systemctl status postgresql@16-main
journalctl -u postgresql@16-main -n 50
```

---

### `journalctl`: the systemd journal

`journalctl` reads the systemd journal. For DBA work, you almost always want to narrow the message stream first.

For a specific unit:

```bash
journalctl -u postgresql@16-main
```

Last N records:

```bash
journalctl -u postgresql@16-main -n 50
```

Limit by time:

```bash
journalctl -u postgresql@16-main --since "10 minutes ago"
journalctl -u postgresql@16-main --since today --until now
```

Follow new events in real time:

```bash
journalctl -u postgresql@16-main -f
```

`-f` is the short form of `--follow`: the command does not exit after printing current entries, but continues showing new records.

Useful filters:

```bash
journalctl -b
journalctl -p err
```

`-b` restricts records to the selected boot, while `-p` filters by message priority. These filters can be combined with `-u`.

A common command is:

```bash
journalctl -xeu postgresql@16-main
```

This combines short options:

```text
-x  → add explanatory texts where available
-e  → jump near the end of the journal
-u  → select a unit
```

Important: the same option letter can mean different things in different programs. Do not transfer the meaning of `-u` from `journalctl` to `sudo`, or vice versa.

For example:

```bash
sudo -u postgres psql -c "SELECT now();"
```

Here `-u postgres` belongs to `sudo` and means the user under whose account `psql` should run.

---

### Checking the TCP port with `ss`

If the service appears to be running but the client cannot connect, inspect the socket:

```bash
ss -lnt
```

In this command:

```text
-l → listening sockets
-n → numeric addresses/ports
-t → TCP
```

Check the PostgreSQL port:

```bash
ss -lnt | grep ':5432'
```

If you need to see the process that owns the socket:

```bash
sudo ss -lntp | grep ':5432'
```

`-p` for `ss` adds process information. This is especially useful for port conflicts: PostgreSQL may fail to start because `5432` is already occupied by another program.

---

## 3. Important details

### Diagnosing a broken `postgresql.conf`

After a configuration error, the service often simply fails to start. A good sequence is:

```text
1. systemctl status
2. journalctl -u ...
3. find the concrete configuration error
4. fix the file
5. start/restart
6. check the port
7. run an SQL check
```

Do not start by randomly editing configuration: systemd and the journal usually already contain the reason for the failure.

### Port conflict

If the journal says PostgreSQL cannot bind to the port:

```bash
sudo ss -lntp | grep ':5432'
```

Identify the socket owner, determine whether that process is expected, and only then decide what to stop or reconfigure.

### The pager is part of the workflow

Both `man` and long `journalctl` output are often viewed through a pager. Searching with `/`, jumping to the next match with `n`, and quitting with `q` are not minor details; they are part of real server troubleshooting.

### Verification after a fix

A fix is complete not when `restart` merely returns success, but when every layer is confirmed:

```text
systemd → active
journal → no new critical error
TCP     → port is listening
SQL     → PostgreSQL responds
```

---

## What to remember

```text
man COMMAND
→ detailed reference

whatis COMMAND
→ short description of a known command

apropos WORD
→ find a tool by purpose

type COMMAND
→ what the shell will actually execute
```

```text
systemctl status UNIT
→ detailed state

systemctl is-active UNIT
→ is it running now?

systemctl is-enabled UNIT
→ is automatic startup configured?
```

```text
journalctl -u UNIT
→ unit journal

-n N
→ last N records

-f
→ follow

--since / --until
→ time range

-b
→ selected boot

-p
→ priority
```

```text
ss -lnt
→ listening TCP sockets

ss -lntp
→ the same + processes
```

Main diagnostic chain:

```text
service → journal → socket → SQL
```
