#!/usr/bin/env bash

set -e

ENGINE="${ENGINE:-}"
if [[ -z "${ENGINE}" ]]; then
  if command -v docker >/dev/null 2>&1; then
    ENGINE=docker
  elif command -v podman >/dev/null 2>&1; then
    ENGINE=podman
  else
    echo "Neither docker nor podman found in PATH" >&2
    exit 1
  fi
fi

trap "${ENGINE} compose -f ../../docker-compose.test.yml down -v;" EXIT;

${ENGINE} compose -f ../../docker-compose.test.yml up -d;
cargo sqlx migrate run --source ../../migrations
cargo test
