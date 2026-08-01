//! Public factory entry point for `swe-edge-loadbalancer-registry`.

use std::sync::Arc;

use swe_edge_loadbalancer_autoscale::{HandlerId, InstancePool};
use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::error::RegistryError;
use crate::api::types::registry::{InMemoryPoolRegistry, TomlTenantRegistry};
use crate::api::types::registry_svc::RegistrySvc;

impl RegistrySvc {
    /// Build an empty [`InMemoryPoolRegistry`].
    pub fn build_pool_registry() -> InMemoryPoolRegistry {
        InMemoryPoolRegistry::build()
    }

    /// Register a pool in an existing [`InMemoryPoolRegistry`].
    ///
    /// Overwrites any previous registration for the same `(handler, tenant)` pair.
    pub fn register_handler_pool(
        registry: &InMemoryPoolRegistry,
        handler_id: &HandlerId,
        tenant_id: Option<&TenantId>,
        pool: Arc<dyn InstancePool>,
    ) {
        registry.register(handler_id, tenant_id, pool);
    }

    /// Parse a TOML document into a [`TomlTenantRegistry`].
    ///
    /// The document must contain a `[tenant.assignments]` table mapping tenant
    /// id strings to tier name strings.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::ParseFailed`] when the document is invalid
    /// TOML or the section has the wrong shape.
    pub fn build_tenant_registry(toml_str: &str) -> Result<TomlTenantRegistry, RegistryError> {
        TomlTenantRegistry::build(toml_str)
    }
}
