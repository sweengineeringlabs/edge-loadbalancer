//! swe-edge-loadbalancer — Load balancer contract for swe-edge-egress-http.
//!
//! Provides `BackendPool`, `Strategy`, `Outcome`, `BackendHealth`, and related
//! types for configuring and operating a pool of HTTP backends with round-robin,
//! weighted, and least-connections selection strategies.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
