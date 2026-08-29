# Reconductor Dashboard

This is a Svelte SPA Dashboard for Reconductor project

## Features

[Tailwind CSS](https://tailwindcss.com/)

## Getting Started

```bash
corepack enable
```

```bash
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

# or start the server and open the app in a new browser tab
pnpm dev --open
```

Your application will be available at `http://localhost:5173`.

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Building

To create a production version of your app:

```sh
pnpm build
```

You can preview the production build with `pnpm run preview`.

## Creating a project

To recreate this project with the same configuration:

```sh
# recreate this project
pnpm dlx sv@0.15.3 create --template minimal --types ts --add prettier eslint vitest="usages:unit,component" playwright tailwindcss="plugins:forms" sveltekit-adapter="adapter:static" --install pnpm dashboard
```
