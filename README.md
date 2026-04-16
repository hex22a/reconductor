# reconductor

This is a Reconductor monorepo.
Reconductor is a deployable network scanner.

## Installation

1. Install container manager. [Podman](https://podman.io/docs/installation) is recommended
1. Install [Bun](https://bun.com/docs/installation)
1. [Node](https://github.com/nvm-sh/nvm) and [pnpm](https://pnpm.io/)

Local [postgres](https://www.postgresql.org/download/) and [redis](https://redis.io/docs/latest/operate/oss_and_stack/install/archive/install-redis/) installations are also recommended for local development, though You can absolutely use containerized versions 

Copy and fill **.env** files

```shell
cp .env.example .env
```

```shell
cp .env.test.example .env.test
```

> *TIP:* You can use something like `openssl rand -hex 32` to get random strings for passwords

## Spin up the local environment

[docker-complse.yml](docker-compose.yml) has several profiles for different scenarios

**Backend** spins database, redis, rabbitmq and backend server (useful for workers and frontend development)

```shell
podman compose -f docker-compose.yml --profile=backend up
```

**Backend + Workers** adds rust workers (useful for frontend development and debugging)

```shell
podman compose -f docker-compose.yml --profile=backend --profile=workers up
```

**Backend + Workers + Frontend** spins a production-like environment if You want to play around with the app

```shell
podman compose -f docker-compose.yml --profile=backend --profile=workers --profile=frontend up
```

### Development

Each component has a corresponging README.md file with more details for local development:

* [./backend/README.md](./backend/README.md)
* [./dashboard/README.md](./dashboard/README.md)
* [./workers/scanner/README.md](./workers/scanner/README.md)
* [./workers/scheduler/README.md](./workers/scheduler/README.md)

### Deplyment

This project is in early stage and it doesn't have installation scripts for different environemts.
Moreover each environment is unique and only You know better how deploy it such it fits Your needs.
You may have different database or prefer a different message broker.
To deploy it properly I recommend forking this project and use it as a template for your perfect deplyment. Each component has it's own dockerfile and can be deployed in a containrized environment.
Good luck!
