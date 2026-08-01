//! `TenantRegistry` trait — tenant-to-tier lookup.

use swe_edge_loadbalancer_tenant::TenantId;

/// Maps a tenant to its tier name for per-tier concurrency caps and scaling
/// limits (ADR-012).
///
/// Consumers with dynamic tenant assignment (e.g. database-backed) inject a
/// custom implementation; the default is a TOML-backed registry.
pub trait TenantRegistry: Send + Sync {
    /// Return the tier name the tenant is assigned to, or `None` when the
    /// tenant has no assignment.
    fn tier_of(&self, tenant_id: &TenantId) -> Option<&str>;
}
