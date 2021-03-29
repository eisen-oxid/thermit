#!/bin/bash

sudo docker run \
  -p 127.0.0.1:5432:5432 \
  -d \
  --name postgres \
  -e POSTGRES_USERS="thermit-server:thermit-server" \
  -e POSTGRES_DATABASES="thermit-server:thermit-server|thermit-server-test:thermit-server" \
  -v thermit-server:/var/lib/postgresql/data \
  --rm \
  lmmdock/postgres-multi
