//! `ScaleOutHint` — deployment parameters for cross-node scale-out.

use serde::{Deserialize, Serialize};

/// Deployment parameters passed to an infrastructure scaling executor when a
/// scaling policy decides to add a node
/// ([`ScalingDecision::ScaleOut`](crate::api::types::scaling::ScalingDecision::ScaleOut)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaleOutHint {
    /// Container image to deploy on the new node.
    pub image: String,
    /// URL the new node loads its configuration from.
    pub config_url: String,
}
