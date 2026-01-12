#!/bin/bash

# Load .env.test file
export $(cat ../.env.test | xargs)

# Run tests
cargo test