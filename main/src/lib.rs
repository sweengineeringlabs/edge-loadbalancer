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
//!
//! **v0.4** — subdomain crate split (ADR-001,
//! `docs/adr/ADR-001-subdomain-crate-split.md`): this crate is now an
//! umbrella over five subdomain crates (`swe-edge-loadbalancer-{tenant,
//! egress,autoscale,ingress,registry}`). `LoadbalancerSvc`'s ten associated
//! methods keep their pre-split signatures, now delegating into each
//! subdomain crate's own constructors, and every public type from the five
//! subdomain crates is re-exported here so existing consumption patterns
//! (`use swe_edge_loadbalancer::{...}`) are unaffected.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod saf;

pub use saf::*;
