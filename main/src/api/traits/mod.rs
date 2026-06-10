//! Public trait declarations.

pub(crate) mod backend_pool;
pub(crate) mod ingress_load_balancer;
pub(crate) mod instance_pool;
pub(crate) mod pool_registry;
pub(crate) mod scaling_executor;
pub(crate) mod scaling_signal;
pub(crate) mod tenant_registry;
pub(crate) mod validator;

pub use backend_pool::BackendPool;
pub use ingress_load_balancer::IngressLoadBalancer;
pub use instance_pool::InstancePool;
pub use pool_registry::PoolRegistry;
pub use scaling_executor::ScalingExecutor;
pub use scaling_signal::ScalingSignal;
pub use tenant_registry::TenantRegistry;
pub use validator::Validator;
