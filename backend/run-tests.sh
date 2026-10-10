#!/bin/bash
set -e

# Work from backend/ wherever this is run from (the paths below are relative to it).
cd "$(dirname "$0")"

# Load .env.test (sourced, not `export $(cat … | xargs)`, so comments and quoted values work)
set -a
. ../.env.test
set +a

# Start the throwaway, RAM-backed test Postgres (see ../compose.test.yaml) and wait until it is
# healthy. A no-op when it is already running.
docker compose -f ../compose.test.yaml up -d --wait

# Run the tests with cargo-nextest (install once: `cargo install --locked cargo-nextest`): every
# test in its own process, in parallel across the unit tests and the single integration binary,
# `it` (tests/it/). Settings live in .config/nextest.toml. Extra arguments pass straight through:
#   ./run-tests.sh                                  # everything
#   ./run-tests.sh -E 'test(/^game_test::/)'        # one integration-test file
#   ./run-tests.sh -E 'test(=game_test::create_works)'  # one test
#   ./run-tests.sh --run-ignored only -E 'test(/^seed_data_test::/)'  # the slow seed tests
# Nextest doesn't run doctests; run those with `cargo test --doc`.
cargo nextest run "$@"
