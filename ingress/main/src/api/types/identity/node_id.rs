//! `NodeId` — newtype wrapper identifying a runtime node.

use serde::{Deserialize, Serialize};

/// Identifies a runtime node (instance) behind an ingress load balancer.
///
/// Wraps an opaque node string; equality and hashing are string-based so a
/// `NodeId` can be used as a map key.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer_ingress::NodeId;
///
/// let id = NodeId::new("node-7");
/// assert_eq!(id.as_str(), "node-7");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(String);

impl NodeId {
    /// Create a new `NodeId` from a node identifier string.
    pub fn new(node: impl Into<String>) -> Self {
        Self(node.into())
    }

    /// Return the underlying node identifier string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
