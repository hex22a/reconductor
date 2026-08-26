# Build
FROM node:24-alpine AS build

WORKDIR /shared
COPY ./shared .

WORKDIR /app
RUN corepack enable

COPY dashboard/package.json dashboard/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY dashboard .
RUN pnpm build

# Server
FROM nginx:alpine

WORKDIR /shared
COPY ./shared .

RUN rm -rf /usr/share/nginx/html/*
COPY --from=build /app/build/client /usr/share/nginx/html
COPY dashboard/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
