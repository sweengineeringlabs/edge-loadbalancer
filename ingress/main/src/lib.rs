//! swe-edge-loadbalancer-ingress — inbound admission and node membership subdomain.
//!
//! Extracted from `swe-edge-loadbalancer` v0.3.0 per ADR-001. Owns the
//! `IngressLoadBalancer` trait (ADR-012), its default `NoopIngressLoadBalancer`
//! implementation, the `NodeId` identity newtype — referenced only from this
//! subdomain's trait methods, so it moves here rather than into the shared
//! `-tenant` kernel — and this crate's own `IngressError`, split out of the
//! former shared `LoadbalancerError`. Depends on `swe-edge-loadbalancer-tenant`
//! for `TenantId`, used in `IngressLoadBalancer::on_accept`.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
