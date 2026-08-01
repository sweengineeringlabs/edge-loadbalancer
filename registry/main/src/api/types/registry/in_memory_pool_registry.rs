//! `InMemoryPoolRegistry` — public pool registry declaration.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use swe_edge_loadbalancer_autoscale::InstancePool;

/// Key for one registered pool: `(handler id, optional tenant id)`.
pub(crate) type PoolKey = (String, Option<String>);

/// In-memory `(handler, tenant) → pool` map implementing the
/// [`crate::api::traits::PoolRegistry`] trait.
///
/// Lookup falls back from the tenant-scoped pool to the shared
/// `(handler, None)` pool, so single-tenant deployments register one pool
/// per handler and multi-tenant deployments add per-tenant overrides.
///
/// Do not construct directly — use the SAF factory functions.
#[derive(Default)]
pub struct InMemoryPoolRegistry {
    /// Registered pools, keyed by handler id and optional tenant id.
    pub(crate) pools: RwLock<HashMap<PoolKey, Arc<dyn InstancePool>>>,
}

impl std::fmt::Debug for InMemoryPoolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryPoolRegistry")
            .field("pool_count", &self.pools.read().len())
            .finish_non_exhaustive()
    }
}
