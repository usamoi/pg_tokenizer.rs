# Installation

## Docker

TODO

## From Debian package

TODO

## From ZIP package

TODO

## From Source

Before building from source, you could refer to the [development](02-development.md) guide to set up the development environment.

1. Build and install the extension.

```sh
cargo pgrx install --sudo --release
```

2. Configure your PostgreSQL by modifying the `shared_preload_libraries` and `search_path` to include the extension.

```sh
psql -U postgres -c 'ALTER SYSTEM SET shared_preload_libraries = "pg_tokenizer.so"'
psql -U postgres -c 'ALTER SYSTEM SET search_path TO "$user", public, tokenizer_catalog'
# You need restart the PostgreSQL cluster to take effects.
sudo systemctl restart postgresql.service   # for pg_tokenizer running with systemd
```

3. Connect to the database and enable the extension.

```sql
CREATE EXTENSION pg_tokenizer;
```
