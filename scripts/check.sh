#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.

set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."
set -x

cargo install --quiet typos-cli

export RUSTDOCFLAGS="-D warnings"

typos
cargo fmt --all -- --check
cargo clippy --quiet --all-features -- -D warnings
cargo check --quiet  --all-targets
cargo test  --quiet --all-targets
cargo test  --quiet --doc

echo "All checks passed."