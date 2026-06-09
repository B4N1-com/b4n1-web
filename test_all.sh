#!/bin/bash
# Unified Test Runner for B4N1-WEB

set -e

echo "=== Running Rust Core Tests ==="
cd engine/cli-core
cargo test
cd ../..

echo "=== Running Python SDK Tests ==="
cd sdks/python
PYTHONPATH=. pytest b4n1web/tests/test_install_paths.py
cd ../..

echo "=== Running JavaScript SDK Tests ==="
cd sdks/javascript
npm install --silent
npx vitest run
cd ../..

echo "=== ALL TESTS PASSED ==="
