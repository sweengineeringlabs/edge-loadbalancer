//! swe-edge-loadbalancer-egress — egress backend-pool subdomain for swe-edge-loadbalancer.
//!
//! Extracted from `swe-edge-loadbalancer` v0.3.0 per ADR-001 (governing ADR: ADR-011).
//! `BackendPool`, `BackendPoolInstance`, `Strategy`, `Outcome`, the backend/config
//! domain types, and this crate's own `EgressError` (split out of the former shared
//! `LoadbalancerError` — see ADR-001's Revision note). Zero dependency on any other
//! `swe-edge-loadbalancer-*` subdomain crate.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
