# Scheduler worker for Reconductor

This is a scheduler worker for Reconductor project.
It polls the database and sends a scan task into a message queue

## Installation

Install [Rust](https://rust-lang.org/tools/install/)

Install dependenies

```bash
cargo install
```

```bash
cargo install sqlx-cli
```

### Database

This project uses [sqlx](https://docs.rs/sqlx_wasi/latest/sqlx/).
**SQLx** requires having a live database for project to compile.
This may feel inconvenient at fisrt
but it's actually very useful in development because it allows You to identify
issues with SQL queries early.
To use this feature `DATABASE_URL` env variable must be set.

```bash
# Run the database and apply the migrations
podman compose -f docker-compose.yml --profile=infra up -d
```

```bash
export DATABASE_URL=some_url
```

### Running localy

```bash
cargo run
```

### Running tests

```bash
cargo test
```

### Production build

Production builds run independently from database
therefore SQLx queries must be prepared upfront.
Run this command every time You make a change to the database:

```bash
cargo sqlx prepare
```

Whis will create (update) `.sqlx` directory with database metadata.
`.sqlx` direcotory must me checked in version control

Now You can build for broduction:

```bash
cargo build --release
```

Optionally You can make build work offline
by setting `SQLX_OFFLINE` environment variable
but it's not recommended for local developent

```bash
export SQLX_OFFLINE=true
```

```bash
unset DATABASE_URL
```
