# Build
FROM rust:1.98.0-alpine3.24 as build

ENV SQLX_OFFLINE=true

WORKDIR /app
COPY ./workers/scheduler .
RUN cargo build --release

# Scheduler
FROM alpine:3.24.1

WORKDIR /app
COPY --from=build /app/target/release/scheduler /app/scheduler

CMD ["./scheduler"]
