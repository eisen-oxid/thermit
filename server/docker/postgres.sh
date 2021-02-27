#!/bin/bash

sudo docker run \
  -p 127.0.0.1:5432:5432 \
  -d \
  --name postgres \
  -e POSTGRES_USER=thermit-server \
  -e POSTGRES_PASSWORD=thermit-server \
  -e POSTGRES_DB=thermit-server \
  -v thermit-server:/var/lib/postgresql/data \
  --rm \
  postgres:12
