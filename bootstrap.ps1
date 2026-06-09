#Requires -Version 7

Write-Host "Bootstrapping swe-edge-loadbalancer..."

# Ensure toolchain is current
rustup update

# Fetch dependencies
cargo fetch

Write-Host "Bootstrap complete. Run 'cargo build' to compile."
