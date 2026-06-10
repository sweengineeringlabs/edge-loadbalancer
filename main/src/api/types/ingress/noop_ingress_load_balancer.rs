//! `NoopIngressLoadBalancer` — pass-through ingress load balancer.

/// Ingress load balancer that admits every request in-process and ignores
/// node membership changes.
///
/// This is the default wired when no `[loadbalancer]` config section is
/// present (ADR-012): single-node deployments pay zero overhead.
///
/// Do not construct directly — use the SAF factory functions.
#[derive(Debug, Default)]
pub struct NoopIngressLoadBalancer {
    pub(crate) _private: (),
}
