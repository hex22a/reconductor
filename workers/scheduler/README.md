# Scheduler worker for Reconductor

This a scheduler worker for Reconductor project. It polls the database and sends a scan task into
a message queue

## Installation

Install [Rust](https://rust-lang.org/tools/install/)

Install dependenies

```shell
cargo install
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
