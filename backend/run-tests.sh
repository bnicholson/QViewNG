#!/bin/bash
set -e

# Load .env.test (sourced, not `export $(cat … | xargs)`, so comments and quoted values work)
set -a
. ../.env.test
set +a

# Start the throwaway, RAM-backed test Postgres (see ../compose.test.yaml) and wait until it is
# healthy. A no-op when it is already running.
docker compose -f ../compose.test.yaml up -d --wait

# Run tests (*only use one of these lines at a time based on your use case). All integration tests
# live in one test binary, `it` (tests/it/); each file is a module, so filter by module name.
# cargo test --test it  # all integration tests
cargo test --test it game_test::  # one file's integration tests
