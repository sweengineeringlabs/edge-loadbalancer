//! `HandlerId` — newtype wrapper identifying a domain handler.

use serde::{Deserialize, Serialize};

/// Identifies a domain handler within an instance pool.
///
/// Wraps the handler's registry id; equality and hashing are string-based so
/// a `HandlerId` can be used as a map key.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer::HandlerId;
///
/// let id = HandlerId::new("payment");
/// assert_eq!(id.as_str(), "payment");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandlerId(String);

impl HandlerId {
    /// Create a new `HandlerId` from a handler registry id.
    pub fn new(handler: impl Into<String>) -> Self {
        Self(handler.into())
    }

    /// Return the underlying handler id string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for HandlerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
