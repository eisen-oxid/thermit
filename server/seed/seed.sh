#!/bin/bash

cd "$(dirname "$0")" || exit
export PGPASSWORD='thermit-server' # Replace with the password you used for the database
psql -h localhost -p 5432 -U thermit-server thermit-server -f seed.sql