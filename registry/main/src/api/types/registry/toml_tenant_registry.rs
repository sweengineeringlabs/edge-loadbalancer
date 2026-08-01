//! `TomlTenantRegistry` — public TOML-backed tenant registry declaration.

use std::collections::HashMap;

/// Tenant-to-tier registry loaded from a flat `[tenant.assignments]` TOML
/// table, implementing the [`crate::api::traits::TenantRegistry`] trait.
///
/// ```toml
/// [tenant.assignments]
/// acme    = "enterprise"
/// startup = "free"
/// ```
///
/// A document without a `[tenant]` section produces an empty registry —
/// single-tenant deployments omit the section entirely.
///
/// Do not construct directly — use the SAF factory functions.
#[derive(Debug, Default)]
pub struct TomlTenantRegistry {
    /// Tenant id → tier name, immutable after construction.
    pub(crate) assignments: HashMap<String, String>,
}
