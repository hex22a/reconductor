# Build
FROM oven/bun:1.3.12 as build

WORKDIR /shared
COPY ./shared .

WORKDIR /app
COPY backend/package.json backend/bun.lock ./

ENV NODE_ENV=production
RUN bun install

COPY backend .
RUN bun build --compile --outfile=server ./src/server.ts
RUN bun build --compile --outfile=migration ./db/migration.ts

# Server
FROM debian:13.4-slim

RUN groupadd -r appgroup && useradd -r -g appgroup appuser

WORKDIR /shared
COPY ./shared/schema.graphql .

WORKDIR /migrations
COPY ./migrations .

WORKDIR /app

COPY --from=build /app/server /app/migration .
RUN chown -R appuser:appgroup /app /migrations /shared
USER appuser

EXPOSE 4000

CMD ["./server"]
