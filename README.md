# reconductor
Network scanner and vulnerability manager. This project is in early development.

## Installation

1. Install container manager. [Podman](https://podman.io/docs/installation) is recommended
1. Install [Bun](https://bun.com/docs/installation)
1. [Node](https://github.com/nvm-sh/nvm) and [pnpm](https://pnpm.io/)

```shell
corepack enable
```

```shell
corepack use pnpm@latest
```

Local [postgres](https://www.postgresql.org/download/) and [redis](https://redis.io/docs/latest/operate/oss_and_stack/install/archive/install-redis/) installations are also recommended for local development, though You can absolutely use containerized versions 

Copy and fill **.env** files

```shell
cp ./backend/.env.example ./backend/.env
```

```shell
cp ./backend/.env.test.example ./backend/.env.test
```

> *TIP:* You can use something like `openssl rand -base64 32` to get random strings for passwords

## Spin up the local environment

For dev enviroment:

```shell
podman compose -f docker-compose.yml --env-file ./backend/.env up
```

For testing (just database and postgres):

```shell
podman compose -f docker-compose.test.yml --env-file ./backend/.env.test up
```

Refer to [./backend/README.md](./backend/README.md) for more details
