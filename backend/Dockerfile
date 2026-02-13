# Build
FROM oven/bun:1.3.9 as build

WORKDIR /app
COPY package.json bun.lock ./

ENV NODE_ENV=production
RUN bun install

COPY . .
RUN bun build --compile --outfile=server ./src/server.ts
RUN bun build --compile --outfile=migration ./migration.ts

# Server
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=build /app/server /app/migration .

EXPOSE 4000

CMD ["./server"]
