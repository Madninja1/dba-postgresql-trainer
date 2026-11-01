# 3. Server Configuration

## 1. Topic

PostgreSQL has a layered parameter system. The same parameter name can receive a value from a file, `ALTER SYSTEM`, database/role settings, or the current session; the effective value is determined by the parameter context and source precedence.

Core model:

```text
defaults
  ↓
postgresql.conf / include
  ↓
postgresql.auto.conf (ALTER SYSTEM)
  ↓
ALTER DATABASE / ALTER ROLE
  ↓
SET in session
  ↓
current value
```

---

# 2. Mechanism

## Parameters and contexts

If a parameter is defined more than once, the last value read takes effect. Therefore, the order of included configuration files matters. The notes distinguish two views: pg_file_settings shows the contents of configuration files, while pg_settings shows effective parameter values. The documentation lists five parameter types: Boolean, string, integer, floating point, and enumerated.

Libpq supports two connection-string formats: a sequence of key=value pairs and a connection URI. In key/value format, each parameter is specified as key=value. Whitespace around the equals sign is optional, and a value containing spaces can be enclosed in single quotes. The basic URI form is postgresql://[user@][host][/database][?parameters]. The postgres:// scheme is also accepted.

Libpq supports multiple hosts in both URI and key=value formats. Addresses are tried sequentially until a connection succeeds. If a connection is established but authentication fails, the remaining hosts are not tried. The practical exercise proposes intentionally introducing an error into an included max_connections file, confirming that the server does not start, checking the server log, fixing the error, and starting the server again.

## Configuration files

The main configuration file is postgresql.conf. It contains server parameter values and is read when PostgreSQL starts. Postgresql.conf is the main server configuration file and can include additional files. Configuration can be reloaded to apply changes, although some parameters are read only at server startup. The location of the main configuration file is reflected by the config_file parameter. The notes specifically suggest using its value to determine the actual path to postgresql.conf.

PostgreSQL supports include, include_if_exists, and include_dir. include_dir allows configuration to be split into multiple .conf files, which are processed in a defined filename order. Postgresql.auto.conf stores ALTER SYSTEM settings, resides in PGDATA, and is read after postgresql.conf. The notes emphasize that it should not be edited manually.

## `ALTER SYSTEM` and reload

A configuration reload can be initiated with pg_ctl reload, the pg_reload_conf() SQL function, or SIGHUP sent to the main server process. This does not mean that parameters that take effect only at startup can be changed without a restart. ALTER SYSTEM is the SQL interface for global parameters and modifies postgresql.auto.conf. The command works with the file system and cannot be executed inside a transaction block. A new value takes effect after a reload, or after a restart for parameters that are read only at startup. The documentation allows parameters to be passed to the server with postgres -c; these values have higher priority than settings from configuration files and ALTER SYSTEM. For an individual libpq session, PGOPTIONS can be used without affecting other sessions.

## Practical reference

```bash
SHOW work_mem;
SET work_mem = '64MB';
SET LOCAL work_mem = '128MB';

ALTER SYSTEM SET shared_buffers = '2GB';
SELECT pg_reload_conf();

ALTER DATABASE appdb SET work_mem = '32MB';
ALTER ROLE analyst SET statement_timeout = '30s';
```

---

# 3. Important Details

## Database, role, and session settings

ALTER SYSTEM RESET removes the specified parameter setting from postgresql.auto.conf. ALTER SYSTEM RESET ALL removes all parameters set this way. In the current session, values can be changed with SET or set_config and read with SHOW or current_setting. These changes do not mean editing the server's postgresql.conf file. The material distinguishes settings that last until the end of the session from settings that last until the end of the transaction. `SET` changes are transactional: if the transaction in which a value was changed is rolled back, the parameter value returns to its state at the beginning of the transaction. `SET LOCAL` limits the change to the current transaction and automatically stops applying when that transaction ends.

Values configured through ALTER DATABASE and ALTER ROLE are applied on new connections. They do not automatically change already-running sessions.

## Source precedence

PostgreSQL also allows custom parameters whose names contain a dot; they can be set with `SET` and read with `current_setting`, which is useful for extensions and applications. The setting source matters together with the parameter context. Values passed to the server through `postgres -c` have priority over configuration files, including `postgresql.auto.conf`. `ALTER DATABASE` and `ALTER ROLE` settings become initial values for a new session, and `SET` can then change the value within that session; `SET LOCAL` limits the change to the current transaction.

---

# What to Remember

```text
• not every parameter can be changed online: some require reload, others restart
• `ALTER SYSTEM` writes to `postgresql.auto.conf`, which overrides `postgresql.conf`
• `ALTER DATABASE` and `ALTER ROLE` define defaults for new sessions
• `SET` changes the current session, `SET LOCAL` the current transaction
• `pg_settings` shows the current value, source, and context of a parameter
```

Before the test, it is useful not just to recognize command names, but to be able to explain **what problem the mechanism solves, at what level it operates, and what constraints it has**.
