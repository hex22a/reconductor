# Install SQLx CLI
FROM rust:1.98.0-alpine3.24 as build

RUN cargo install sqlx-cli \
    --no-default-features \
    --features postgres

# Migration
FROM alpine:3.24.1

COPY --from=build /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

ENTRYPOINT ["sqlx"]
