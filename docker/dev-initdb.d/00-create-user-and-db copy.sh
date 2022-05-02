#!/usr/bin/env bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER weatherstation WITH PASSWORD 'development';
	CREATE DATABASE weatherstation;
	GRANT ALL PRIVILEGES ON DATABASE weatherstation TO weatherstation;
EOSQL