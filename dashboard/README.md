# Reconductor Dashboard

This is a React SPA Dashboard for Reconductor project

## Features

[react-router](https://reactrouter.com/)
[GraphQL](https://graphql.org/)
[relay](https://relay.dev/)
[Tailwind CSS](https://tailwindcss.com/)

## Getting Started

```shell
corepack enable
```

```shell
corepack use pnpm@latest
```

### Installation

Install the dependencies:

```bash
pnpm install
```

### Development

Start the development server with HMR:

```bash
pnpm dev
```

Your application will be available at `http://localhost:5173`.

## Building for Production

Create a production build:

```bash
pnpm build
```

## Deployment

The containerized application can be deployed to any platform that supports Docker, including:

- AWS ECS
- Google Cloud Run
- Azure Container Apps
- Digital Ocean App Platform
- Fly.io
- Railway

### DIY Deployment

If you're familiar with deploying Node applications, the built-in app server is production-ready.

Make sure to deploy the output of `pnpm build`

```
├── package.json
├── pnpm-lock.yaml
├── build/
│   ├── client/    # Static assets
│   └── server/    # Server-side code
```

