# Scanner worker for Reconductor

This a scanner worker for Reconductor project. It runs [nmap](https://nmap.org), parses the output and stores results in the database

## Installation

Install [Rust](https://rust-lang.org/tools/install/)

Install dependenies

```shell
cargo install
```

```bash
cargo install sqlx-cli
```

### Database changes

This code uses [sqlx](https://docs.rs/sqlx_wasi/latest/sqlx/) which by default requires a live database in order to compile.
You must set DATABASE_URL env variable first

```bash
export DATABASE_URL=some_url
```

Once that done You can run

```bash
cargo sqlx prepare
```

Whis will create (update) .sqlx directory with database metadata.

To make build work offline

```bash
export SQLX_OFFLINE=true
```

```bash
unset DATABASE_URL
```

### Running localy

```shell
cargo run
```

### Running tests

```shell
cargo test
```

### Build prodcution

```shell
cargo build --release
```
