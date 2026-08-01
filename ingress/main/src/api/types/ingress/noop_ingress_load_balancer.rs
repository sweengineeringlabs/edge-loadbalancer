//! `NoopIngressLoadBalancer` — pass-through ingress load balancer.

/// Ingress load balancer that admits every request in-process and ignores
/// node membership changes.
///
/// This is the default wired when no `[loadbalancer]` config section is
/// present (ADR-012): single-node deployments pay zero overhead.
///
/// Construct via [`NoopIngressLoadBalancer::default()`](Default::default) —
/// the private field blocks external struct-literal construction while
/// still allowing the derived `Default` impl.
#[derive(Debug, Default)]
pub struct NoopIngressLoadBalancer {
    pub(crate) _private: (),
}
