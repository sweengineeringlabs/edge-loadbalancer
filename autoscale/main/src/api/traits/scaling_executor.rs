//! `ScalingExecutor` trait — applies scaling decisions.

use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::types::identity::HandlerId;
use crate::api::types::scaling::ScaleOutHint;

/// Applies the decisions a scaling policy produces (ADR-013).
///
/// Two implementations are composed in the feedback loop: a runtime-side
/// executor mapping `scale_up` / `scale_down` to concurrency-cap changes on
/// instance pools, and an infrastructure-side executor mapping `scale_out` /
/// `scale_in` to node membership changes on the ingress load balancer. Each
/// implementation treats the methods outside its scope as no-ops.
pub trait ScalingExecutor: Send + Sync {
    /// Raise the concurrency cap of the `(handler, tenant)` pool by `n`.
    fn scale_up(&self, handler_id: &HandlerId, tenant_id: Option<&TenantId>, n: usize);

    /// Lower the concurrency cap of the `(handler, tenant)` pool by `n`.
    fn scale_down(&self, handler_id: &HandlerId, tenant_id: Option<&TenantId>, n: usize);

    /// Add a node using the given deployment hint.
    fn scale_out(&self, hint: &ScaleOutHint);

    /// Remove a node.
    fn scale_in(&self);
}
