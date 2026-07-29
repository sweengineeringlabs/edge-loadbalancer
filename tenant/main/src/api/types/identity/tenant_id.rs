//! `TenantId` — newtype wrapper identifying a tenant.

use serde::{Deserialize, Serialize};

/// Identifies a tenant for pool isolation and tier lookup.
///
/// Wraps an opaque tenant string; equality and hashing are string-based so a
/// `TenantId` can be used as a map key.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer_tenant::TenantId;
///
/// let id = TenantId::new("acme");
/// assert_eq!(id.as_str(), "acme");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(String);

impl TenantId {
    /// Create a new `TenantId` from a tenant string.
    pub fn new(tenant: impl Into<String>) -> Self {
        Self(tenant.into())
    }

    /// Return the underlying tenant string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
