//! Registry types — tenant tiers and instance pools.

pub(crate) mod in_memory_pool_registry;
pub(crate) mod toml_tenant_registry;

pub use in_memory_pool_registry::InMemoryPoolRegistry;
pub use toml_tenant_registry::TomlTenantRegistry;
