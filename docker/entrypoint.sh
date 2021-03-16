#!/usr/bin/env bash

set -uo pipefail

COMMAND="${1:?"Specify a command to run"}"
FLAG="${2:-''}"

if [ "$COMMAND" = "migrate" ]; then
  exec diesel migration run
fi

if [ "$COMMAND" = "server" ] && [ "$FLAG" = "-m" ]; then
  diesel migration run
fi

exec "$COMMAND"
