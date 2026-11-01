# 16. Privileges and Access Control

## 1. Topic

After successful authentication, PostgreSQL checks the role's permissions on specific objects. The access-control system is composed of:

```text
role attributes
+ role membership
+ ownership
+ object privileges
+ PUBLIC/default privileges
```

A role may be allowed to connect to a database but not read a particular table; conversely, a granted `SELECT` is useless if the role cannot connect to the database or use the required schema.

---

# 2. Ownership and Object Privileges

An object gets an owner when it is created. The owner initially receives the applicable privileges and retains inherent powers over the object: for example, the owner can drop it or grant back to itself an ordinary privilege that was revoked.

The material distinguishes three categories:

```text
SUPERUSER → normal access-control checks are bypassed
owner     → owns the object and manages its privileges
others    → operate only within granted privileges
```

## Tables and Views

The main privileges are:

```text
SELECT      read
INSERT      insert
UPDATE      modify
DELETE      delete
TRUNCATE    empty the table
REFERENCES  create foreign keys
TRIGGER     create triggers
```

`SELECT`, `INSERT`, `UPDATE`, and `REFERENCES` can, where appropriate, be granted at the level of individual columns.

## Other Object Types

DBA1 separately covers:

```text
sequence:
  SELECT → currval
  UPDATE → nextval, setval
  USAGE  → currval, nextval

tablespace:
  CREATE

database:
  CONNECT
  CREATE
  TEMPORARY

schema:
  USAGE
  CREATE

function/procedure:
  EXECUTE
```

`CONNECT` on a database does not imply `SELECT` on its tables. `USAGE` on a schema allows access to objects located in it, but privileges on those objects are checked separately.

---

# 3. `GRANT`, `REVOKE`, and Privilege Checks

```sql
GRANT SELECT ON app.orders TO reader;
REVOKE UPDATE ON app.orders FROM reader;
```

`GRANT` and `REVOKE` are used both for object privileges and for role membership.

The documentation allows privileges to be granted on sets of objects, for example `ALL TABLES IN SCHEMA`, and also allows delegation of the right to grant a privilege further through `WITH GRANT OPTION`.

For programmatic checks, PostgreSQL provides the `has_*_privilege` family of functions, for example:

```sql
SELECT has_table_privilege('reader', 'app.orders', 'SELECT');
```

---

# 4. Group Roles and Membership

PostgreSQL has no separate "group" type: a group role is an ordinary role, usually created with `NOLOGIN`, that other roles are made members of.

```sql
CREATE ROLE readers NOLOGIN;
GRANT readers TO alice;
REVOKE readers FROM alice;
```

A role can be a member of several other roles, but cyclic membership is prohibited.

Privileges of a containing role can be used through inheritance and/or `SET ROLE`, depending on membership options. Attributes such as `LOGIN`, `SUPERUSER`, `CREATEDB`, and `CREATEROLE` are not inherited like ordinary object privileges.

## `PUBLIC`

`PUBLIC` is a pseudo-role that implicitly includes every role, including roles created in the future. A privilege granted to `PUBLIC` is effectively available to everyone.

PostgreSQL automatically grants some privileges to `PUBLIC`. For PostgreSQL 16, the documentation specifically lists:

```text
database            → CONNECT, TEMPORARY
function/procedure  → EXECUTE
language/type       → USAGE
```

Tables, schemas, and sequences have no object privileges granted to `PUBLIC` by default.

---

# 5. Predefined Roles

PostgreSQL provides administrative roles that let you delegate selected capabilities without granting `SUPERUSER`, for example:

```text
pg_read_all_settings
pg_read_all_stats
pg_stat_scan_tables
pg_read_all_data
pg_write_all_data
pg_read_server_files
pg_write_server_files
pg_execute_server_program
pg_monitor
```

The full role list, including system roles, can be displayed in psql with:

```text
\duS
```

The exact permissions of predefined roles can change between PostgreSQL versions, so they should be checked against the documentation for the version in use.

---

# 6. Default Privileges

`ALTER DEFAULT PRIVILEGES` changes privileges for **future** objects. It does not rewrite privileges on objects that already exist.

```sql
ALTER DEFAULT PRIVILEGES
FOR ROLE writer
IN SCHEMA public
GRANT SELECT ON TABLES TO reader;
```

An important detail is that PostgreSQL considers the default privileges of the role that directly creates the object. They are not automatically accumulated from every role of which the creator is a member.

Default privileges can be configured for tables, sequences, functions/procedures, types, and schemas within the capabilities of the command.

---

# 7. Routines and Security Context

For functions and procedures, the main object privilege is `EXECUTE`.

```text
SECURITY INVOKER  → runs with the privileges of the calling role (default)
SECURITY DEFINER  → runs with the privileges of the routine owner
```

`SECURITY DEFINER` can provide a controlled operation on objects to which the caller has no direct access, but it also increases privilege-escalation risk.

For a secure `SECURITY DEFINER` function, `search_path` must not contain schemas writable by untrusted users; `pg_temp` should be placed at the end of a safe path. It is also often necessary to revoke the automatically granted `PUBLIC EXECUTE` privilege and grant `EXECUTE` only to the required roles.

---

# 8. Practical Pattern

```sql
CREATE ROLE writer NOLOGIN;
CREATE ROLE reader NOLOGIN;

REVOKE ALL ON SCHEMA public FROM PUBLIC;
GRANT USAGE, CREATE ON SCHEMA public TO writer;
GRANT USAGE ON SCHEMA public TO reader;

ALTER DEFAULT PRIVILEGES FOR ROLE writer IN SCHEMA public
GRANT SELECT ON TABLES TO reader;

GRANT writer TO w1;
GRANT reader TO r1;
```

This pattern separates "who can log in" from "what actions are available after login" and makes it possible to manage permissions centrally through group roles.

---

# What to Remember

```text
• privileges depend on object type: databases, schemas, sequences, and tables have different privilege sets
• owners have special inherent powers, while SUPERUSER bypasses normal checks
• `GRANT`/`REVOKE` manage both object privileges and role membership
• `PUBLIC` means all roles; some privileges are granted to PUBLIC automatically
• group roles reduce the number of individual GRANT statements
• `ALTER DEFAULT PRIVILEGES` affects only future objects created by a specific role
• `SECURITY DEFINER` requires a protected `search_path` and careful EXECUTE grants
```
