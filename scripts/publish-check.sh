#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty

if [[ "${SKIP_CARGO_PUBLISH_DRY_RUN:-true}" == "true" ]]; then
  echo "skip cargo publish --dry-run (set SKIP_CARGO_PUBLISH_DRY_RUN=false to enable)"
else
  cargo publish --dry-run --allow-dirty
fi
