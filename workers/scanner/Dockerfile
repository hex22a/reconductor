# Build
FROM rust:1.94.1-alpine3.23 as build

ENV SQLX_OFFLINE=true

WORKDIR /app
COPY ./workers/scanner .
RUN cargo build --release

# Scheduler
FROM alpine:3.23.3

RUN apk add --no-cache nmap nmap-scripts

WORKDIR /app
COPY --from=build /app/target/release/scanner /app/scanner

CMD ["./scanner"]
