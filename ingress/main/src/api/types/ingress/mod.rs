//! Ingress-side load balancing types (ADR-012).

pub(crate) mod load_balancer_hint;
pub(crate) mod noop_ingress_load_balancer;

pub use load_balancer_hint::LoadBalancerHint;
pub use noop_ingress_load_balancer::NoopIngressLoadBalancer;
