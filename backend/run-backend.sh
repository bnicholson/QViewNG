#!/bin/bash

# Load .env.test file
export $(cat ../.env | xargs)

cargo run --bin backend
