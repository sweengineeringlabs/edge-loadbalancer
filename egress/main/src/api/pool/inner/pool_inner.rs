//! Interface contract for pool inner state types.
//!
//! The pool inner state carries the strategy, backend entries, and round-robin
//! counter as a single atomically-reference-counted unit. It is stored behind
//! a trait object in [`BackendPoolInstance`] so that `api/types/pool/` never
//! names any core types directly.
//!
//! [`BackendPoolInstance`]: crate::api::types::pool::BackendPoolInstance

use std::any::Any;

/// Contract for types that carry the pool's strategy, backend entries,
/// and round-robin counter as a single, atomically-reference-counted unit.
///
/// `core/pool/inner/PoolInner` is the default implementation.
pub trait PoolInner: Send + Sync {
    /// Returns the number of backends currently tracked by the pool.
    fn backend_count(&self) -> usize;

    /// Exposes the concrete type for downcasting.
    ///
    /// Implementations should return `self`.
    fn as_any(&self) -> &dyn Any;
}
