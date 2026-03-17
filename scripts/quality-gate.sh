#!/usr/bin/env bash
set -euo pipefail

cargo test
cargo clippy --all-targets --all-features -- -D warnings
