#!/bin/bash

cd "$(dirname "$0")" || exit

# Get environment variables from the .env file
export $(cat "../.env" | xargs)

export PGPASSWORD='thermit-server' # Replace with the password you used for the database
psql "$DATABASE_URL" -f seed.sql
