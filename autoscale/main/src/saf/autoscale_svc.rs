//! Public factory entry point for `swe-edge-loadbalancer-autoscale`.

use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::error::AutoscaleError;
use crate::api::types::autoscale_svc::AutoscaleSvc;
use crate::api::types::identity::HandlerId;
use crate::api::types::pool::HandlerInstancePool;

impl AutoscaleSvc {
    /// Build a [`HandlerInstancePool`] gating `concurrency_cap` concurrent
    /// executions of `handler_id`, optionally scoped to `tenant_id`.
    ///
    /// # Errors
    ///
    /// Returns [`AutoscaleError::InvalidConfig`] when `concurrency_cap` is 0.
    pub fn build_handler_pool(
        handler_id: HandlerId,
        tenant_id: Option<TenantId>,
        concurrency_cap: usize,
    ) -> Result<HandlerInstancePool, AutoscaleError> {
        HandlerInstancePool::build(handler_id, tenant_id, concurrency_cap)
    }
}
