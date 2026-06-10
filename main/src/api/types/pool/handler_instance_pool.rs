//! `HandlerInstancePool` — public concurrency-gated handler pool declaration.

use std::sync::atomic::{AtomicU64, AtomicUsize};

use crate::api::types::identity::{HandlerId, TenantId};

/// A concurrency-gated pool for one `(handler, tenant)` pair, implementing
/// the [`crate::api::traits::InstancePool`] and
/// [`crate::api::traits::ScalingSignal`] traits.
///
/// Domain handlers are stateless `Arc<dyn Handler>` values — there are no
/// instances to spawn. This pool gates how many requests may execute the
/// handler concurrently: [`select`](crate::api::traits::InstancePool::select)
/// acquires a slot, [`report_outcome`](crate::api::traits::InstancePool::report_outcome)
/// releases it, and a scaling executor adjusts the cap at runtime via
/// [`set_concurrency_cap`](crate::api::traits::InstancePool::set_concurrency_cap).
///
/// Do not construct directly — use the SAF factory functions.
pub struct HandlerInstancePool {
    /// Handler this pool gates.
    pub(crate) handler_id: HandlerId,
    /// Tenant this pool is scoped to; `None` for the shared pool.
    pub(crate) tenant_id: Option<TenantId>,
    /// Requests currently holding a slot.
    pub(crate) active: AtomicUsize,
    /// Maximum concurrent slots; adjusted by scaling executors.
    pub(crate) cap: AtomicUsize,
    /// Total outcomes reported since construction.
    pub(crate) total_outcomes: AtomicU64,
    /// Failed outcomes reported since construction.
    pub(crate) failed_outcomes: AtomicU64,
}

impl std::fmt::Debug for HandlerInstancePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HandlerInstancePool")
            .field("handler_id", &self.handler_id)
            .field("tenant_id", &self.tenant_id)
            .finish_non_exhaustive()
    }
}
