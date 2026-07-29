//! Pool interface layer — interface counterpart to `core/pool/`.
//!
//! The `BackendPool` trait (in `api/traits/`) is the contract implemented
//! by `core/pool/backend_pool_instance.rs`. This module re-exports the pool
//! selection policy type alias used by pool implementations.

pub mod inner;

pub(crate) use crate::api::types::strategy::Strategy as PoolStrategy;
