//! SAF layer — public facade.
//!
//! Re-exports every public type from the five subdomain crates
//! (`swe-edge-loadbalancer-{tenant,egress,autoscale,ingress,registry}`) so
//! `use swe_edge_loadbalancer::{...}` keeps working unchanged for existing
//! consumption patterns, per ADR-001. Extension-point traits (implementors
//! in downstream crates) are re-exported so consumers can hold
//! `Arc<dyn Trait>` or provide their own implementations. This crate no
//! longer has a `core/` layer of its own — `LoadbalancerSvc`'s methods
//! delegate straight into each subdomain crate's own constructors.

mod loadbalancer_svc;

// Public traits — re-exported for downstream dispatchers and executors
// that hold `Arc<dyn Trait>` or implement the trait themselves.
pub use swe_edge_loadbalancer_autoscale::{InstancePool, ScalingExecutor, ScalingSignal};
pub use swe_edge_loadbalancer_egress::{BackendPool, Validator};
pub use swe_edge_loadbalancer_ingress::IngressLoadBalancer;
pub use swe_edge_loadbalancer_registry::{PoolRegistry, TenantRegistry};

// Public types re-exported from swe-edge-loadbalancer-autoscale.
//
// `Outcome` collides by name with `swe_edge_loadbalancer_egress::Outcome` —
// the two are separate, structurally-identical types by design (see
// `swe-edge-loadbalancer-autoscale`'s own `Outcome` doc comment). The bare
// `Outcome` name is reserved for egress's copy below, matching every
// pre-split consumption pattern (`LoadbalancerSvc::report_outcome` takes
// egress's `Outcome`); autoscale's copy is re-exported as `HandlerOutcome`.
pub use swe_edge_loadbalancer_autoscale::{
    AutoscaleError, AutoscaleSvc, HandlerId, HandlerInstancePool, Outcome as HandlerOutcome,
    PoolSnapshot, ScaleOutHint, ScalingDecision,
};

// Public types re-exported from swe-edge-loadbalancer-egress.
pub use swe_edge_loadbalancer_egress::{
    ApplicationConfigBuilder, Backend, BackendConfig, BackendHealth, BackendId,
    BackendPoolInstance, EgressError, LoadbalancerConfig, Outcome, Strategy,
};

// Public types re-exported from swe-edge-loadbalancer-ingress.
pub use swe_edge_loadbalancer_ingress::{
    IngressError, LoadBalancerHint, NodeId, NoopIngressLoadBalancer,
};

// Public types re-exported from swe-edge-loadbalancer-registry.
pub use swe_edge_loadbalancer_registry::{
    InMemoryPoolRegistry, RegistryError, RegistrySvc, TomlTenantRegistry,
};

// Public types re-exported from swe-edge-loadbalancer-tenant.
pub use swe_edge_loadbalancer_tenant::TenantId;

// Local to this crate — the aggregating error type (ADR-001) and the facade.
pub use crate::error::LoadbalancerError;
pub use loadbalancer_svc::LoadbalancerSvc;
