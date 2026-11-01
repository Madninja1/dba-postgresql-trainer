# 1. Tools and Installation

## 1. Topic

The first DBA1 block connects PostgreSQL installation with the structure of a server instance. It is important to distinguish the server program, the data directory, and the database cluster, and then understand which tools create, start, stop, and check that cluster.

Core model:

```text
PostgreSQL binaries
        ↓
initdb
        ↓
database cluster (PGDATA)
        ↓
pg_ctl / systemd
        ↓
running server instance
```

---

# 2. Mechanism

## Building from source

The traditional way to build PostgreSQL from source is based on GNU Make and Autoconf. Starting with PostgreSQL 16, the Meson build system can also be used. Readline provides command-line editing, command history, and completion.

Meson uses Ninja by default. After configuring the build directory, compilation is performed with the ninja command. The short traditional build procedure consists of configuration with ./configure, compilation with make, and installation with make install.

The material emphasizes the value of understanding source builds for nonstandard options, specific architectures, and a general understanding of the installation process. Git gives access to any commit but requires additional tools; release archives already contain some generated files. The required software listed in the material includes tar, gzip/bzip2, GNU make, and a C89 compiler.

Zlib, lz4, and zstd (Zstandard) are used for compression. Readline is primarily needed for a convenient interactive command line; the material notes that it may be unnecessary if psql is not used interactively in a terminal. Perl, Python, and Tcl are required to use the PL/Perl, PL/Python, and PL/Tcl procedural languages.

In the optional software section, the material associates Kerberos, OpenSSL, OpenLDAP, and PAM with authentication and encryption. ICU is listed as a library for cross-platform Unicode support. After building and before installation, it is useful to run `make check`; regression tests are run as a regular user, not `root`.

`make world` includes the core system, additional modules from `contrib`, and documentation, while `make world-bin` includes the core system and `contrib` without documentation. The --prefix option sets the installation tree prefix instead of the default /usr/local/pgsql. The `--with-python`, `--with-lz4`, and `--with-zstd` options enable the corresponding features at configure time.

--without-readline disables Readline/libedit, so psql loses command-line editing and history. Meson setup creates and configures the build directory; the builddir argument is mandatory for this command. After the initial meson setup, build-directory settings are changed with meson configure.

With Meson, regression tests after building and before installation are run with `meson test`; they should also be run as a regular user. The `ninja install`, `ninja clean`, and `ninja uninstall` commands are used respectively to install, clean build results, and remove the installed files. The runtime chapter notes that packaged products are usually integrated by the vendor with the system's installation and startup mechanisms, so details may differ from a "clean" source build.


## Creating a cluster: `initdb`

Initdb creates a new PostgreSQL database cluster. Initdb must be run as the owner of the future server process; running it as the operating-system superuser is not allowed. The data directory is specified with -D/--pgdata or the PGDATA environment variable.

By default, initdb uses the libc locale provider. --no-sync disables waiting for files to be safely written to disk. This can make initdb faster, but it increases the risk of data corruption after a failure and is not recommended for production. -U/--username sets the name of the initial superuser; by default, the name of the operating-system user running initdb is used.

-X/--waldir sets the directory where the write-ahead log (WAL) will be stored. --pwfile makes initdb read the initial superuser password from the first line of the specified file. --sync-only synchronizes database files to disk and exits without performing the other initdb operations.

`--wal-segsize` sets the WAL segment size at `initdb` time; the default is 16 MB. The value must be a power of two from 1 to 1024 MB and cannot later be changed with this option after initialization.

## Managing the server: `pg_ctl`

Pg_ctl is used to initialize and manage the server: start, stop, restart, reload, status, promote, and other operations. Pg_ctl stop supports smart, fast, and immediate modes. If -m is not specified, pg_ctl uses fast mode.

Smart shutdown rejects new connections and waits for existing clients to disconnect. Immediate shutdown terminates server processes immediately without a clean shutdown, so crash recovery will be required at the next startup. Reload sends SIGHUP to the server, after which PostgreSQL rereads postgresql.conf, pg_hba.conf, and other configuration files.

For a running server, pg_ctl status displays the PID and command-line options used to start it. Restart is a stop followed by a start and can be used for changes that require a full server restart. Promote takes a server out of standby mode and switches it to normal read/write operation.

When starting the server, the -l option to pg_ctl directs server output to the specified log file.

## Practical reference

```bash
# traditional build
./configure --prefix=/usr/local/pgsql
make
make check
make install

# Meson/Ninja
meson setup builddir
ninja -C builddir
meson test -C builddir
ninja -C builddir install

# new cluster
initdb -D /srv/postgresql/data -k

# management
pg_ctl -D /srv/postgresql/data -l server.log start
pg_ctl -D /srv/postgresql/data reload
pg_ctl -D /srv/postgresql/data stop -m fast
```

---

# 3. Important Details

## Checksums: `pg_checksums`

-k/--data-checksums enables data-page checksums; when enabled, they apply to all objects in all databases in the cluster. Checksum errors detected by the server while reading pages are reflected in `pg_stat_database` statistics. Before running pg_checksums, the PostgreSQL server must be shut down cleanly. Pg_checksums can check, enable, or disable data checksums for a cluster.

When enabling checksums, pg_checksums calculates a checksum for each relation block and rewrites the blocks; when disabling checksums, only pg_control is changed. A practical sequence is: stop the server, check the checksum state, enable checksums, and then start the server. --check is the default mode; the exit status indicates whether checksum errors were found.

Enabling and disabling checksums can take a long time; the server must remain stopped. `--no-sync` can speed up modifying operations, but at the cost of a risk of data-directory corruption if the operating system fails; in `--check` mode this option has no effect. With physical replication, the checksum mode must be consistent across nodes. If pg_checksums is interrupted during enable or disable, the checksum configuration state remains unchanged and the operation can be repeated.

## Package installation, Ubuntu, and cloud environments

Package installation is usually easier to maintain and update; the PGDG repository is often used to obtain current PostgreSQL versions. On Ubuntu, pg_createcluster wraps initdb; the package creates a cluster named main, checksums are disabled by default, and server startup/shutdown is integrated with the operating system. `pg_createcluster` and `pg_dropcluster` are specific to Debian/Ubuntu; on other systems, `initdb` and the corresponding operating-system tools are used.

In the Ubuntu package distribution, pg_ctlcluster is used as a wrapper around pg_ctl; on other systems pg_ctl can be used directly. In a VM, the administrator gets full control over the operating system at the cost of additional overhead; in DBaaS, some access is restricted and more operational responsibility is handled by the provider.

## Cluster and server are different entities

A single PostgreSQL instance works with multiple databases at the same time; this set of databases is called a database cluster. The cluster is the data stored in files, while the server or server instance is the program that manages that cluster.

---

# What to Remember

```text
• a database cluster is the set of data in PGDATA; the server is the process that manages that cluster
• `initdb` creates a cluster and must not be run as root
• `pg_ctl` manages the server lifecycle; the normal default shutdown mode is `fast`
• checksums are enabled for the entire cluster, and `pg_checksums` must be run on a cleanly stopped server
• building from source is not always necessary, but it is useful for understanding capabilities and nonstandard configurations
```

Before the test, it is useful not just to recognize command names, but to be able to explain **what problem the mechanism solves, at what level it operates, and what constraints it has**.
