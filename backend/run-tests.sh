#!/bin/bash

# Load .env.test file
export $(cat ../.env.test | xargs)

# Run tests
cargo test -- --test-threads=1  # uses a single test DB synchronously (for now)