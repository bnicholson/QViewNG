#!/bin/bash
set -a
source ../.env.test
set +a
diesel migration run