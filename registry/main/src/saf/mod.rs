//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `RegistrySvc`. Extension-point traits (implementors in downstream crates)
//! are re-exported so consumers can hold `Arc<dyn Trait>` or provide their
//! own implementations. Core types are NOT re-exported directly (SEA Rule 47).

mod registry_svc;

// Public traits — re-exported for downstream dispatchers and scaling
// executors that hold `Arc<dyn Trait>` or implement the trait themselves.
pub use crate::api::traits::PoolRegistry;
pub use crate::api::traits::TenantRegistry;

// Public types re-exported from api/
pub use crate::api::error::RegistryError;
pub use crate::api::types::registry::{InMemoryPoolRegistry, TomlTenantRegistry};
pub use crate::api::types::registry_svc::RegistrySvc;
