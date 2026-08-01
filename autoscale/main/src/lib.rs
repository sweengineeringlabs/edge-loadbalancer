//! swe-edge-loadbalancer-autoscale — instance-pool / scaling subdomain of
//! swe-edge-loadbalancer.
//!
//! Extracted from `swe-edge-loadbalancer` v0.3.0 per ADR-001 (ADR-013 governs
//! this subdomain's contracts: `InstancePool`, `ScalingExecutor`,
//! `ScalingSignal`, `HandlerInstancePool`, and the `PoolSnapshot` /
//! `ScaleOutHint` / `ScalingDecision` scaling-feedback-loop types). Depends
//! on `swe-edge-loadbalancer-tenant` for `TenantId`, referenced by
//! `ScalingExecutor::scale_up`/`scale_down`, `HandlerInstancePool`, and
//! `PoolSnapshot`.
//!
//! Owns `HandlerId` — used only within this subdomain and by
//! `swe-edge-loadbalancer-registry`, which depends on this crate for both
//! `HandlerId` and the `InstancePool` trait (see ADR-001's "Revision note").

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
