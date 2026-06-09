//! `BackendId` — newtype wrapper identifying a backend by URL string.

use serde::{Deserialize, Serialize};

/// Identifies a backend within the pool.
///
/// Wraps a URL string; equality and hashing are string-based so a
/// `BackendId` can be used as a map key.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer::BackendId;
///
/// let id = BackendId::new("https://api-1.internal");
/// assert_eq!(id.as_str(), "https://api-1.internal");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BackendId(String);

impl BackendId {
    /// Create a new `BackendId` from a URL string.
    pub fn new(url: impl Into<String>) -> Self {
        Self(url.into())
    }

    /// Return the underlying URL string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BackendId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
