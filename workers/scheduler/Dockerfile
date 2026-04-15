# Build
FROM rust:1.94.1-alpine3.23 as build

ENV SQLX_OFFLINE=true

WORKDIR /app
COPY ./workers/scheduler .
RUN cargo build --release

# Scheduler
FROM alpine:3.23.3

WORKDIR /app
COPY --from=build /app/target/release/scheduler /app/scheduler

CMD ["./scheduler"]
