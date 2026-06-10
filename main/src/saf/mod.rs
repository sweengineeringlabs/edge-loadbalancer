//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `LoadbalancerSvc`. Extension-point traits (implementors in downstream crates)
//! are re-exported so consumers can hold `Arc<dyn Trait>` or provide their own
//! implementations. Core types are NOT re-exported directly (SEA Rule 47).

mod loadbalancer_svc;

// Public traits — re-exported for downstream dispatchers and executors
// that hold `Arc<dyn Trait>` or implement the trait themselves.
pub use crate::api::traits::IngressLoadBalancer;
pub use crate::api::traits::InstancePool;
pub use crate::api::traits::PoolRegistry;
pub use crate::api::traits::ScalingExecutor;
pub use crate::api::traits::ScalingSignal;
pub use crate::api::traits::TenantRegistry;

// Public types re-exported from api/
pub use crate::api::error::LoadbalancerError;
pub use crate::api::types::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::types::backend::{Backend, BackendHealth, BackendId};
pub use crate::api::types::config::{BackendConfig, LoadbalancerConfig};
pub use crate::api::types::identity::{HandlerId, NodeId, TenantId};
pub use crate::api::types::ingress::{LoadBalancerHint, NoopIngressLoadBalancer};
pub use crate::api::types::loadbalancer_svc::LoadbalancerSvc;
pub use crate::api::types::outcome::Outcome;
pub use crate::api::types::pool::{BackendPoolInstance, HandlerInstancePool};
pub use crate::api::types::registry::{InMemoryPoolRegistry, TomlTenantRegistry};
pub use crate::api::types::scaling::{PoolSnapshot, ScaleOutHint, ScalingDecision};
pub use crate::api::types::strategy::Strategy;

// SAF standalone functions — all take/return api/ types only
pub use loadbalancer_svc::build_backend_pool;
pub use loadbalancer_svc::build_handler_pool;
pub use loadbalancer_svc::build_noop_ingress_lb;
pub use loadbalancer_svc::build_pool_registry;
pub use loadbalancer_svc::build_tenant_registry;
pub use loadbalancer_svc::pool_backend_count;
pub use loadbalancer_svc::register_handler_pool;
pub use loadbalancer_svc::report_backend_outcome;
pub use loadbalancer_svc::select_backend;
pub use loadbalancer_svc::validate_loadbalancer_config;
