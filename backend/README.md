# backend

```shell
cd ./backend
```

Install deps:

```shell
bun install
```

## Available scripts

```shell
bun lint
```

```shell
bun test:unit
```

or

```shell
bun test:unit --watch
```

for platfrom integrational tests run

```shell
./platform_tests.sh
```

This will automatically call `podman compose` and setup the testing environemnt assuming You've done `.env` file step

## Run backend

On a fresh database first run the migration script:

```shell
bun run ./db/migration.ts
```

And then You can run the app:

```shell
bun run ./src/server.ts
```
