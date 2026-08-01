//! swe-edge-loadbalancer-tenant — shared-kernel tenant identity for swe-edge-loadbalancer.
//!
//! Extracted from `swe-edge-loadbalancer` v0.3.0 per ADR-001. `TenantId` is the one
//! identity newtype referenced from more than one subdomain crate in a real trait
//! signature (`IngressLoadBalancer::on_accept`, `ScalingExecutor::scale_up`/
//! `scale_down`, `TenantRegistry::tier_of`) with no valid single-subdomain owner —
//! see ADR-001's "Revision note" for the per-type membership analysis. Consumed by
//! `swe-edge-loadbalancer-autoscale`, `swe-edge-loadbalancer-ingress`, and
//! `swe-edge-loadbalancer-registry`.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod saf;

pub use saf::*;
