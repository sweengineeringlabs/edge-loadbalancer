//! `LoadBalancerHint` — ingress admission decision.

use crate::api::types::identity::NodeId;

/// The decision an [`IngressLoadBalancer`](crate::api::traits::IngressLoadBalancer)
/// returns for an accepted connection.
///
/// `RouteToInstance` is advisory — in-process inbound adapters that cannot
/// redirect treat it as `UseInProcess`; reverse-proxy integrations honour it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadBalancerHint {
    /// Handle the request on this node.
    UseInProcess,
    /// Advisory redirect to the given node.
    RouteToInstance(NodeId),
    /// Refuse the request (e.g. node draining or tenant over quota);
    /// inbound adapters map this to `503` / `UNAVAILABLE`.
    Reject,
}
