# Roles, Privileges, and Security in System Catalogs

## Mental model

PostgreSQL security metadata is distributed across several catalogs. It is useful to separate four tasks:

```text
who exists and what a role can do       → pg_authid
which roles contain which members       → pg_auth_members
which settings apply to a role/database → pg_db_role_setting
which privileges and policies apply     → pg_default_acl, pg_parameter_acl,
                                           pg_policy, pg_*seclabel
```

Some of these catalogs are shared across the whole cluster because roles and certain settings do not belong to one specific database.

---

## `pg_authid`: roles as authorization identifiers

`pg_authid` stores PostgreSQL roles. The ideas of a "user" and a "group" are unified here as a single entity: a **role**. A role with `rolcanlogin = true` can be used as the initial authorization identity for a connection.

The catalog is shared by all databases in the cluster.

Main fields:

- `rolname` — role name;
- `rolsuper` — superuser;
- `rolinherit` — automatically inherits privileges of roles it belongs to;
- `rolcreaterole` — can create roles;
- `rolcreatedb` — can create databases;
- `rolcanlogin` — can log in;
- `rolreplication` — replication role;
- `rolbypassrls` — bypasses row-level security;
- `rolconnlimit` — connection limit;
- `rolpassword` — stored password verifier;
- `rolvaliduntil` — password expiration time.

### Why `pg_roles` is usually preferred

`pg_authid` contains `rolpassword`, so the catalog is not intended to be freely readable by every user. For ordinary role inspection, PostgreSQL provides the `pg_roles` view, where the password field is masked.

This illustrates a general PostgreSQL pattern:

```text
sensitive system catalog
        ↓
safe user-facing view
```

---

## `pg_auth_members`: role membership

`pg_auth_members` describes membership relationships between roles. The hierarchy can have multiple levels, but it must not contain cycles.

Key fields:

- `roleid` — the role **into which** another role is granted;
- `member` — the role that is a member of `roleid`;
- `grantor` — who granted the membership;
- `admin_option` — whether `member` may grant this membership to other roles;
- `inherit_option` — whether the member inherits privileges of the granted role;
- `set_option` — whether the member may use `SET ROLE` to assume the granted role.

A useful way to remember the relationship direction is:

```text
member  --is member of-->  roleid
```

`pg_auth_members`, like `pg_authid`, is shared by all databases in the cluster.

---

## `pg_db_role_setting`: settings by role and database

This catalog stores default runtime configuration parameter values for combinations of roles and databases.

Fields:

- `setdatabase` — database OID, or `0` if the setting is not tied to a specific database;
- `setrole` — role OID, or `0` if the setting is not tied to a specific role;
- `setconfig` — array of parameter values.

The catalog is shared across the cluster.

Useful model:

```text
ALTER ROLE ... SET ...
ALTER ROLE ... IN DATABASE ... SET ...
        ↓
pg_db_role_setting
```

---

## `pg_default_acl`: privileges for future objects

`pg_default_acl` stores the **initial privileges that will be assigned to newly created objects**.

It is not a catalog of the "current ACLs of all objects." Its purpose is to determine ACLs when new objects are created.

Main fields:

- `defaclrole` — role owning the default-privilege rule;
- `defaclnamespace` — schema, or `0` for a global entry;
- `defaclobjtype` — object kind;
- `defaclacl` — ACL assigned to the object.

`defaclobjtype` codes described by the catalog:

```text
r  relation
S  sequence
f  function
T  type
n  schema
```

There are two layers of default privileges:

```text
global entry      defaclnamespace = 0
schema-local      defaclnamespace = specific schema
```

Schema-local privileges are added to the global or built-in defaults.

### Important distinction from `NULL` ACLs in other catalogs

If an ACL field on an existing object is `NULL`, that means the built-in default privileges **for that object**, not "recompute privileges from the current `pg_default_acl`." `pg_default_acl` entries are consulted when the object is created.

---

## `pg_init_privs`: initial privileges of system and extension objects

`pg_init_privs` stores initial privileges for objects when those privileges differ from the standard `NULL` set.

Such privileges can originate from:

```text
i  initdb
e  CREATE EXTENSION
```

The code is stored in `privtype`.

Other fields:

- `objoid` — object;
- `classoid` — catalog where the object is described;
- `objsubid` — column number for a column-level object, or `0`;
- `initprivs` — initial ACL.

This allows PostgreSQL and its tools to distinguish the object's original privileges from later privilege changes.

---

## `pg_parameter_acl`: privileges on configuration parameters

`pg_parameter_acl` contains only configuration parameters that have nonstandard privileges assigned to them.

- `parname` — GUC parameter name;
- `paracl` — ACL for that parameter.

Parameters with standard privileges do not appear in this catalog.

`pg_parameter_acl` is a cluster-wide shared catalog.

---

## `pg_policy`: row-level security

`pg_policy` stores table RLS policies.

Main fields:

- `polname` — policy name;
- `polrelid` — table;
- `polcmd` — command to which the policy applies;
- `polpermissive` — permissive or restrictive;
- `polroles` — roles;
- `polqual` — expression added to the security-barrier condition when reading or modifying existing rows;
- `polwithcheck` — `WITH CHECK` expression for new row values.

`polcmd` codes:

```text
r  SELECT
a  INSERT
w  UPDATE
d  DELETE
*  all commands
```

### A policy row alone does not mean RLS is active

RLS must also be enabled on the table:

```text
pg_policy.polrelid → pg_class.oid
pg_class.relrowsecurity = true
```

So diagnosing RLS requires more than checking `pg_policy` alone.

---

## Security labels: `pg_seclabel` and `pg_shseclabel`

A security label associates an object with a provider and a text label.

### `pg_seclabel`

For objects belonging to a specific database:

- `objoid` — object;
- `classoid` — object's catalog;
- `objsubid` — column number or `0`;
- `provider` — security-label provider;
- `label` — label value.

### `pg_shseclabel`

For cluster-wide global objects. This catalog is shared by all databases.

Model:

```text
local database object → pg_seclabel
shared cluster object → pg_shseclabel
```

---

## Practical role inspection

Membership can be read through a self-join to `pg_authid`:

```sql
SELECT
    parent.rolname AS granted_role,
    member.rolname AS member_role,
    m.admin_option,
    m.inherit_option,
    m.set_option
FROM pg_auth_members AS m
JOIN pg_authid AS parent
  ON parent.oid = m.roleid
JOIN pg_authid AS member
  ON member.oid = m.member;
```

For ordinary user-facing tooling, prefer public views when they expose the information you need, and reserve direct reads from sensitive catalogs for administrative tasks.

---

## What to remember

```text
pg_authid          → the role itself and its capabilities
pg_auth_members    → membership between roles
pg_db_role_setting → role/database-specific GUC defaults
pg_default_acl     → ACLs for future objects
pg_init_privs      → original initdb/extension ACLs
pg_parameter_acl   → ACLs of configuration parameters
pg_policy          → RLS policies
pg_seclabel        → labels of objects in the current database
pg_shseclabel      → labels of shared cluster objects
```

When working with security metadata, first determine the scope: **database object**, **role**, **role+database combination**, or **shared cluster object**. That immediately narrows the set of catalogs you need to inspect.
