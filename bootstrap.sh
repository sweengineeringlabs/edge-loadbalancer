#!/usr/bin/env bash
set -euo pipefail

echo "Bootstrapping swe-edge-loadbalancer..."

# Ensure toolchain is current
rustup update

# Fetch dependencies
cargo fetch

echo "Bootstrap complete. Run 'cargo build' to compile."
