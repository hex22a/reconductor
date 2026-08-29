# reconductor

This is a Reconductor monorepo.
Reconductor is a deployable network scanner.

## Installation

1. Install container manager. [Podman](https://podman.io/docs/installation) is recommended
1. Install [Rust](https://rust-lang.org/tools/install/)
1. [Node](https://github.com/nvm-sh/nvm) and [pnpm](https://pnpm.io/)

Local [postgres](https://www.postgresql.org/download/)
and [redis](https://redis.io/docs/latest/operate/oss_and_stack/install/archive/install-redis/)
installations are also recommended for local development,
though You can absolutely use containerized versions

Copy and fill **.env** files

```bash
cp .env.example .env
```

```bash
cp .env.test.example .env.test
```

> *TIP:* You can use something like `openssl rand -hex 32` to get random strings for passwords

## Spin up the local environment

[docker-complse.yml](docker-compose.yml) has several profiles for different scenarios

**Infra** spins database, redis and rabbitmq (useful for development)

```bash
podman compose -f docker-compose.yml --profile=infra up -d
```

**Infra + Backend** spins infrastructure, API and backgorund workers (useful for frontend development and debugging)

```bash
podman compose -f docker-compose.yml --profile=infra --profile=backend up -d
```

**Infra + Backend + Frontend** spins a production-like environment if You want to play around with the app

```bash
podman compose -f docker-compose.yml --profile=backend --profile=workers --profile=frontend up -d
```

Running podman-compose in foreground (without `-d`) may cause unexpected bahavior. It's recommended to always run it detached (with `-d`).
If You want to access logs use

```bash
podman compose --profile backend logs -f
```

### Development

Each component has a corresponging README.md file with more details for local development:

* [./server/README.md](./server/README.md)
* [./dashboard/README.md](./dashboard/README.md)
* [./workers/scanner/README.md](./workers/scanner/README.md)
* [./workers/scheduler/README.md](./workers/scheduler/README.md)

### Deplyment

This project is in early stage and it doesn't have installation scripts for different environemts.
Moreover each environment is unique and only You know better how deploy it such it fits Your needs.
You may have different database or prefer a different message broker.
To deploy it properly I recommend forking this project and use it as a template for your perfect deplyment. Each component has it's own dockerfile and can be deployed in a containrized environment.
Good luck!
