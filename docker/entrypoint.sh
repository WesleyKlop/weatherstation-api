#!/usr/bin/env bash

set -uo pipefail

COMMAND="$1"

if [ "$COMMAND" = "migrate" ]; then
  exec diesel migration run
fi

if [ "$COMMAND" = "server" ] && [ -z "$2" ] && [ "$2" = "-m" ]; then
  diesel migration run
fi

exec "$COMMAND"