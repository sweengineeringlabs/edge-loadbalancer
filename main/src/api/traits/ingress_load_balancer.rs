//! `IngressLoadBalancer` trait — inbound admission and node membership.

use crate::api::error::LoadbalancerError;
use crate::api::types::identity::{NodeId, TenantId};
use crate::api::types::ingress::LoadBalancerHint;

/// Inbound admission decision point plus the infrastructure plug-in for
/// cross-node scaling (ADR-012).
///
/// Inbound adapters call [`on_accept`](Self::on_accept) before dispatching a
/// request; infrastructure scaling executors call
/// [`add_node`](Self::add_node) / [`remove_node`](Self::remove_node) when a
/// scaling policy decides to scale out or in. Adapters for infrastructure
/// that cannot change node membership (e.g. a static reverse proxy) keep the
/// default no-op implementations.
pub trait IngressLoadBalancer: Send + Sync {
    /// Decide how to handle an accepted request.
    ///
    /// `tenant_id` is the resolved tenant, when ingress-side tenant
    /// resolution produced one.
    fn on_accept(&self, tenant_id: Option<&TenantId>) -> LoadBalancerHint;

    /// Register a new node with the balancer after scale-out.
    ///
    /// Default: no-op `Ok(())` for adapters without membership control.
    ///
    /// # Errors
    ///
    /// Implementations return [`LoadbalancerError::InvalidConfig`] when the
    /// node cannot be registered with the underlying infrastructure.
    fn add_node(&self, node: &NodeId) -> Result<(), LoadbalancerError> {
        let _ = node;
        Ok(())
    }

    /// Deregister a node from the balancer before scale-in.
    ///
    /// Default: no-op `Ok(())` for adapters without membership control.
    ///
    /// # Errors
    ///
    /// Implementations return [`LoadbalancerError::InvalidConfig`] when the
    /// node cannot be deregistered from the underlying infrastructure.
    fn remove_node(&self, node: &NodeId) -> Result<(), LoadbalancerError> {
        let _ = node;
        Ok(())
    }
}
