#!/bin/bash

# Load .env.test file
export $(cat ../.env.test | xargs)

# Load PostgreSQL 16 binaries into PATH (mostly for access to initdb)
# This was initially added for optimizing speed of integraiton tests by making DB in-memory
# This may need to be changed based on your PGSQL version and configuration.
export PATH="/usr/lib/postgresql/16/bin:$PATH"

# Run tests
cargo test -- --test-threads=1  # uses a single test DB synchronously (for now)