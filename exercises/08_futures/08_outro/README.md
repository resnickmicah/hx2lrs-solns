# Running Locally #
1. [Install Stable Rust](https://www.rust-lang.org/tools/install)
2. Install PostgreSQL 16:
  * Ubuntu: instructions from [dev.to](https://dev.to/johndotowl/postgresql-16-installation-on-ubuntu-2204-51ia). Note: This may conflict with Ubuntu's built-in postgres server. You may want to run [Postgres in a container](https://github.com/docker-library/docs/blob/master/postgres/README.md#how-to-use-this-image) on a non-default port instead, using `-p 5433:5432` option for `docker run`
  * Mac: `brew install postgresql@16` and `brew services start postgresql@16`
3. [Install sqlx cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
4. From this directory (once you have Postgres running):
  * `cp .env.example .env`
  * `source .env`
  * `cargo sqlx database create`
  * `cargo sqlx migrate run`
  * `cargo sqlx prepare --check`
  * if --check doesn't throw any errors: `cargo sqlx prepare`
4. `cp Secrets.toml.example Secrets.toml` (and make sure your DATABASE_URL matches the one from your .env)
5. `cargo shuttle run`