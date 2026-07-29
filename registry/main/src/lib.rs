//! swe-edge-loadbalancer-registry — `(handler, tenant)` pool and tenant-tier
//! lookup subdomain of swe-edge-loadbalancer.
//!
//! Extracted from `swe-edge-loadbalancer` v0.3.0 per ADR-001. Owns the
//! `PoolRegistry` and `TenantRegistry` traits, their default in-memory /
//! TOML-backed implementations (`InMemoryPoolRegistry`, `TomlTenantRegistry`),
//! and this crate's own `RegistryError`, split out of the former shared
//! `LoadbalancerError`.
//!
//! Depends on `swe-edge-loadbalancer-autoscale` for the `InstancePool` trait
//! (`PoolRegistry::get` returns `Arc<dyn InstancePool>`) and `HandlerId`, and
//! on `swe-edge-loadbalancer-tenant` for `TenantId` — the only subdomain
//! crate needing two subdomain dependencies (ADR-001).

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
