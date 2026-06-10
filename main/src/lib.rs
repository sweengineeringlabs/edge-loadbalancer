//! swe-edge-loadbalancer — shared load balancer primitives for swe-edge.
//!
//! **v0.1** — egress slice: `BackendPool`, `Strategy`, `Outcome`, `BackendHealth`,
//! and related types for HTTP backend pools with round-robin, weighted, and
//! least-connections strategies.
//!
//! **v0.2** — ingress + scaling slice: `IngressLoadBalancer`, `InstancePool`,
//! `ScalingSignal`, `TenantRegistry`, `PoolRegistry`, identity newtypes, and
//! default implementations (`NoopIngressLoadBalancer`, `HandlerInstancePool`,
//! `InMemoryPoolRegistry`, `TomlTenantRegistry`) backing ADR-011/012/013.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
