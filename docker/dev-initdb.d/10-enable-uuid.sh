#!/usr/bin/env bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "weatherstation" <<-EOSQL
	CREATE EXTENSION "uuid-ossp";
EOSQL