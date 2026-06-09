# swe-edge-loadbalancer — Documentation

## Architecture

This crate follows the SEA (Structural Engineering Architecture) module layout:

- `api/` — public traits and types.
- `core/` — private implementations (`pub(crate)` only).
- `saf/` — public facade: standalone factory functions and type re-exports.

## Module map

```
main/src/
├── lib.rs                          re-exports only via pub use saf::*
├── api/
│   ├── traits/
│   │   ├── backend_pool.rs         pub trait BackendPool
│   │   └── validator.rs            pub trait Validator
│   ├── types/
│   │   ├── backend/
│   │   │   ├── backend.rs
│   │   │   ├── backend_health.rs
│   │   │   └── backend_id.rs
│   │   ├── config/
│   │   │   └── loadbalancer_config.rs
│   │   ├── loadbalancer_svc.rs
│   │   ├── outcome.rs
│   │   └── strategy.rs
│   └── error/
│       └── loadbalancer_error.rs
├── core/
│   └── pool/
│       └── default_backend_pool.rs
└── saf/
    └── loadbalancer_svc.rs         standalone pub fn wrappers
```

## Config reference

```toml
[loadbalancer]
strategy = "round-robin"   # round-robin | weighted | least-connections

[[loadbalancer.backends]]
url    = "https://api-1.internal"
weight = 1                  # optional, default 1
```
