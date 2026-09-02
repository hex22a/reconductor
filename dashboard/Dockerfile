# Build
FROM node:26-alpine3.24 AS build

ENV CI=true

WORKDIR /app
RUN corepack enable

COPY dashboard/package.json dashboard/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY dashboard .
RUN pnpm build

# Server
FROM nginx:1.31.4-alpine3.24-slim

RUN rm -rf /usr/share/nginx/html/*
COPY --from=build /app/build /usr/share/nginx/html
COPY dashboard/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
