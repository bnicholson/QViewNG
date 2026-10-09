#!/bin/bash
set -e

# Load .env.test (sourced, not `export $(cat … | xargs)`, so comments and quoted values work)
set -a
. ../.env.test
set +a

# Start the throwaway, RAM-backed test Postgres (see ../compose.test.yaml) and wait until it is
# healthy. A no-op when it is already running.
docker compose -f ../compose.test.yaml up -d --wait

# Run tests (*only use one of these lines at a time based on your use case):
# cargo test -- --test-threads=1  # for all integration tests; uses a single test DB synchronously (for now)
cargo test --test game_test -- --test-threads=1  # for single integration test file; uses a single test DB synchronously (for now)
