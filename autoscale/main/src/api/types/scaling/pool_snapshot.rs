//! `PoolSnapshot` — point-in-time pool load signal.

use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::types::identity::HandlerId;

/// Point-in-time load signal for one `(handler, tenant)` pool, produced by
/// [`ScalingSignal::snapshot`](crate::api::traits::ScalingSignal::snapshot)
/// and consumed by a runtime-side scaling policy.
///
/// In v0.1 of the feedback loop (ADR-013) the signal source is node-level:
/// `handler_id` is the `"*"` sentinel and `tenant_id` is `None`. Per-handler
/// and per-tenant snapshots are produced once per-key counters exist.
#[derive(Debug, Clone, PartialEq)]
pub struct PoolSnapshot {
    /// Handler this snapshot describes (`"*"` sentinel for node-level).
    pub handler_id: HandlerId,
    /// Tenant this snapshot describes; `None` for the shared pool.
    pub tenant_id: Option<TenantId>,
    /// Requests currently holding a concurrency slot.
    pub active_instances: usize,
    /// Requests waiting for a slot; `0` for pools that reject instead of queue.
    pub queue_depth: usize,
    /// 99th-percentile latency in milliseconds; `0.0` when the producer does
    /// not measure latency (e.g. slot-counting pools without duration data).
    pub latency_p99_ms: f64,
    /// Failed requests as a fraction of total requests, in `0.0..=1.0`.
    pub error_rate: f64,
    /// `active_instances / concurrency_cap`, in `0.0..=1.0`.
    pub saturation: f64,
}
