//! `PoolRegistry` trait — `(handler, tenant)` pool lookup.

use std::sync::Arc;

use swe_edge_loadbalancer_autoscale::{HandlerId, InstancePool};
use swe_edge_loadbalancer_tenant::TenantId;

/// Resolves the [`InstancePool`] gating a `(handler, tenant)` pair.
///
/// Dispatchers call [`get`](Self::get) per request; scaling executors call
/// it per decision. Implementations decide the fallback semantics — the
/// in-memory default falls back from the tenant-scoped pool to the shared
/// `(handler, None)` pool.
pub trait PoolRegistry: Send + Sync {
    /// Return the pool for the `(handler, tenant)` pair, or `None` when no
    /// pool is registered (callers then dispatch ungated).
    fn get(&self, handler_id: &HandlerId, tenant_id: Option<&TenantId>)
        -> Option<Arc<dyn InstancePool>>;
}
