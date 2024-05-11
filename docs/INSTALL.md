# Installation Instructions for Mumak Extension

> **Note:** These instructions assume that you are operating on a Linux-based OS with a recent version of PostgreSQL already installed. Familiarity with Linux command line operations and PostgreSQL administration is expected.

## Prerequisites

- PostgreSQL (ensure it is compatible with the Mumak extension).
- Root or sudo access to the PostgreSQL server and files.

## Download

1. Go to the [Mumak Releases Page](https://github.com/txpipe/mumak/releases).
2. Download the binaries suitable for your system architecture. Currently, only Linux AMD64 binaries are available. Support for ARM64 and other architectures may be added based on demand.

   Example file to download for AMD64: `mumak-artifacts-amd64.tar.gz`.

## Installation

### Extract Files

Extract the downloaded tarball using:

```bash
tar -xzf mumak-artifacts-amd64.tar.gz
```

This will extract:

- `mumak--0.0.0.sql`
- `mumak.control`
- `mumak.so`

### Copy Files to PostgreSQL Directory

Copy the files to the appropriate PostgreSQL directory (adjust the version number as per your installation):

```sh
sudo cp mumak.control /usr/share/postgresql/16/extension/
sudo cp mumak--0.0.0.sql /usr/share/postgresql/16/extension/
sudo cp mumak.so /usr/lib/postgresql/16/lib/
```

### Enable the Extension

Restart the PostgreSQL service:

```sh
sudo systemctl restart postgresql
```

Then, connect to your database with psql and enable the Mumak extension:

```sql
CREATE EXTENSION IF NOT EXISTS mumak;
```

### Verification

Run the following query to verify that the Mumak extension is installed correctly:

```sql
SELECT * FROM pg_extension;
```

```sh
postgres=# SELECT * FROM pg_extension;

  oid  | extname | extowner | extnamespace | extrelocatable | extversion | extconfig | extcondition 
-------+---------+----------+--------------+----------------+------------+-----------+--------------
 13545 | plpgsql |       10 |           11 | f              | 1.0        |           | 
 16388 | mumak   |       10 |         2200 | f              | 0.0.0      |           | 
```

If you see the `mumak` extension listed, the installation was successful.

Congratulations 🎊! You have successfully installed the Mumak extension for PostgreSQL.