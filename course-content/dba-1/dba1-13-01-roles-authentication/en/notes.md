# 15. Roles and Authentication

## 1. Topic

In PostgreSQL, a "user" is a role with login capability (`LOGIN`). Roles exist at the cluster level rather than inside a single database, and conceptually they do not have to match operating-system users.

A successful connection is a chain of several checks:

```text
client
  ↓
server listens on this interface?
  ↓
first matching pg_hba.conf rule
  ↓
authentication method
  ↓
role has LOGIN
  ↓
role has CONNECT to database
  ↓
session
```

Authentication answers the question "who is connecting," while the object privileges in the next topic answer "what can this role do after login."

---

# 2. Roles and Attributes

A role is used in two main ways:

1. as a DBMS user;
2. as a container of privileges/group that includes other roles.

During `initdb`, an initial superuser role with `LOGIN` is created; it is normally named `postgres`.

The main attributes in the lesson:

| Attribute | Purpose |
|---|---|
| `LOGIN` / `NOLOGIN` | ability to establish a session |
| `SUPERUSER` | bypass normal access checks |
| `CREATEDB` | create databases |
| `CREATEROLE` | manage roles within permitted limits |

The linked documentation also shows other important properties: `INHERIT`, `REPLICATION`, `BYPASSRLS`, `CONNECTION LIMIT`, password, and password expiration.

```sql
CREATE ROLE app LOGIN PASSWORD 'secret';
CREATE ROLE readers NOLOGIN;
```

`CREATE ROLE` without an explicit `LOGIN` creates a role that cannot log in as an ordinary user.

---

# 3. Connections and `pg_hba.conf`

## Rule Selection

`pg_hba.conf` is scanned **from top to bottom**. The server chooses the first record matching the connection parameters:

```text
TYPE
DATABASE
USER
ADDRESS   (for network connections)
METHOD
```

Once the first matching row is selected, later records are not considered — even if authentication using the selected rule fails. If no rule matches, the connection is rejected.

The main types are:

```text
local → Unix-domain socket
host  → TCP/IP
```

Example:

```conf
# TYPE  DATABASE  USER  ADDRESS       METHOD
local   all       postgres            peer
local   all       all                 peer
host    all       all   127.0.0.1/32  scram-sha-256
host    all       all   ::1/128       scram-sha-256
```

Changes to `pg_hba.conf` must be reloaded by the server:

```sql
SELECT pg_reload_conf();
```

or by an equivalent `reload` through the server-control utility.

`listen_addresses` solves a different problem: which network interfaces the server listens on for TCP connections. `pg_hba.conf` then determines which incoming connections are allowed and how they are authenticated.

---

# 4. Authentication Methods

The main methods from the lesson:

```text
trust          → allow without a password check
reject         → reject unconditionally
scram-sha-256  → SCRAM password authentication
md5            → older password method
peer           → obtain the local user name from the OS
```

For `peer`, the OS user name and PostgreSQL role name can either match directly or be mapped through a map in `pg_ident.conf`.

In PostgreSQL 16, SCRAM-SHA-256 is the modern password-authentication option; MD5 is retained for compatibility but is considered a legacy approach.

---

# 5. Role Passwords and Client-Side Storage

If a password method is selected, the role must have a password. The server stores not the clear-text password but its verifier/hash in the restricted system catalog `pg_authid`.

The client can obtain the password in three ways shown in the lesson:

```text
manual input
PGPASSWORD
~/.pgpass
```

A `.pgpass` line has the format:

```text
host:port:database:user:password
```

On Unix, the file must be accessible only to its owner (`0600`); PostgreSQL ignores it if the permissions are too broad.

`PGPASSWORD` is convenient for short-lived scenarios, but the material does not recommend it as secure permanent secret storage.

---

# 6. Practice with `peer`

The practical exercise demonstrates an important distinction:

```text
pg_hba.conf selects the peer method
        ≠
the role name automatically matches the OS user
```

For mapping, `pg_ident.conf` can be used, for example:

```conf
# MAPNAME  SYSTEM-USERNAME  PG-USERNAME
stmap      student          alice
stmap      student          bob
```

and the map is then specified in the corresponding `pg_hba.conf` rule.

---

# What to Remember

```text
• roles are cluster-wide and do not have to match OS users
• `LOGIN` makes a role capable of establishing a session
• `pg_hba.conf` is read from top to bottom; only the first matching rule is used
• changes to `pg_hba.conf` require reload, while `listen_addresses` controls network listening
• `trust`, `reject`, `peer`, SCRAM, and MD5 solve different authentication tasks
• a password can be entered manually, passed through PGPASSWORD, or stored in `.pgpass`
• on Unix, `.pgpass` must have `0600` permissions
• successful authentication does not by itself grant privileges on objects
```
