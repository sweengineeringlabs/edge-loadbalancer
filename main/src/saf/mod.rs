//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `LoadbalancerSvc`. Traits are NOT re-exported from this module (SEA Rule 126).
//! Core types are NOT re-exported directly (SEA Rule 47).

mod loadbalancer_svc;

// Public types re-exported from api/
pub use crate::api::error::LoadbalancerError;
pub use crate::api::types::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::types::backend::{Backend, BackendHealth, BackendId};
pub use crate::api::types::config::{BackendConfig, LoadbalancerConfig};
pub use crate::api::types::loadbalancer_svc::LoadbalancerSvc;
pub use crate::api::types::outcome::Outcome;
pub use crate::api::types::pool::BackendPoolInstance;
pub use crate::api::types::strategy::Strategy;

// SAF standalone functions — all take/return api/ types only
pub use loadbalancer_svc::build_backend_pool;
pub use loadbalancer_svc::pool_backend_count;
pub use loadbalancer_svc::report_backend_outcome;
pub use loadbalancer_svc::select_backend;
pub use loadbalancer_svc::validate_loadbalancer_config;
