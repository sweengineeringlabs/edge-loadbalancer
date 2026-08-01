//! `BackendPoolInstance` — public pool type declaration.

use std::sync::Arc;

use crate::api::pool::inner::pool_inner::PoolInner as PoolInnerTrait;
use crate::api::types::strategy::Strategy;

/// A concrete backend pool implementing the `BackendPool` trait.
///
/// Construct via [`BackendPoolInstance::build`]. Thread-safe; the actual
/// backend state is held behind a `PoolInner` (trait) object initialised by
/// the `core/pool/` factory.
///
/// Do not construct directly — always use `BackendPoolInstance::build`.
pub struct BackendPoolInstance {
    /// Opaque inner state initialised by `core/pool/backend_pool_instance.rs`.
    pub(crate) state: Arc<dyn PoolInnerTrait>,
    /// Strategy cached at the api layer for `Debug`.
    pub(crate) strategy: Strategy,
}

impl std::fmt::Debug for BackendPoolInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackendPoolInstance")
            .field("strategy", &self.strategy)
            .finish_non_exhaustive()
    }
}
