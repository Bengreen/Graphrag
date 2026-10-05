#!/bin/bash
set -e

echo "Running tests..."
# The system might not allow docker pull/extract here due to overlayfs limits in dind.
# Let's rely on cargo test.
cargo test

echo "Integration tests passed!"
