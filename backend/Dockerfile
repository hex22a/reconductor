# Build
FROM oven/bun:1.3.9 as build

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
FROM debian:bookworm-slim

WORKDIR /shared
COPY ./shared/schema.graphql .

WORKDIR /migrations
COPY ./migrations .

WORKDIR /app

COPY --from=build /app/server /app/migration .

EXPOSE 4000

CMD ["./server"]
