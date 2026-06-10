//! Public factory entry point for `swe-edge-loadbalancer`.

use std::sync::Arc;

use crate::api::error::LoadbalancerError;
use crate::api::traits::{BackendPool, InstancePool, Validator};
use crate::api::types::backend::{Backend, BackendId};
use crate::api::types::config::LoadbalancerConfig;
use crate::api::types::identity::{HandlerId, TenantId};
use crate::api::types::ingress::NoopIngressLoadBalancer;
use crate::api::types::loadbalancer_svc::LoadbalancerSvc;
use crate::api::types::outcome::Outcome;
use crate::api::types::pool::{BackendPoolInstance, HandlerInstancePool};
use crate::api::types::registry::{InMemoryPoolRegistry, TomlTenantRegistry};

impl LoadbalancerSvc {
    /// Build a [`BackendPoolInstance`] from a [`LoadbalancerConfig`].
    ///
    /// Validates the config first; returns `Err(LoadbalancerError::InvalidConfig)`
    /// if any backend has an empty URL or zero weight, or the backend list is empty.
    ///
    /// # Errors
    ///
    /// - [`LoadbalancerError::InvalidConfig`] — validation failed.
    pub fn build_pool(config: LoadbalancerConfig) -> Result<BackendPoolInstance, LoadbalancerError> {
        BackendPoolInstance::build(config)
    }

    /// Validate a [`LoadbalancerConfig`] without constructing a pool.
    pub fn validate_config(config: &LoadbalancerConfig) -> Result<(), String> {
        BackendPoolInstance::validate(config)
    }

    /// Select a healthy backend from an existing pool.
    pub fn select(pool: &BackendPoolInstance) -> Result<Backend, LoadbalancerError> {
        pool.select()
    }

    /// Record an outcome against a backend, updating its health state.
    pub fn report_outcome(pool: &BackendPoolInstance, id: &BackendId, outcome: Outcome) {
        pool.report_outcome(id, outcome);
    }

    /// Return the number of backends registered in the pool.
    pub fn backend_count(pool: &BackendPoolInstance) -> usize {
        pool.backend_count()
    }
}

/// Build a [`BackendPoolInstance`] from a [`LoadbalancerConfig`].
///
/// Validates the config; returns `Err(LoadbalancerError::InvalidConfig)`
/// if any backend has an empty URL or zero weight, or the backend list is empty.
///
/// # Errors
///
/// - [`LoadbalancerError::InvalidConfig`] — validation failed.
pub fn build_backend_pool(config: LoadbalancerConfig) -> Result<BackendPoolInstance, LoadbalancerError> {
    LoadbalancerSvc::build_pool(config)
}

/// Validate a [`LoadbalancerConfig`] without constructing a pool.
///
/// Returns `Ok(())` when the config is well-formed.
pub fn validate_loadbalancer_config(config: &LoadbalancerConfig) -> Result<(), String> {
    LoadbalancerSvc::validate_config(config)
}

/// Select a healthy backend from an existing pool.
///
/// # Errors
///
/// - [`LoadbalancerError::NoHealthyBackends`] — every backend is `Degraded` or `Dead`.
pub fn select_backend(pool: &BackendPoolInstance) -> Result<Backend, LoadbalancerError> {
    LoadbalancerSvc::select(pool)
}

/// Record an [`Outcome`] against a backend, updating its health state.
pub fn report_backend_outcome(pool: &BackendPoolInstance, id: &BackendId, outcome: Outcome) {
    LoadbalancerSvc::report_outcome(pool, id, outcome);
}

/// Return the number of backends registered in the pool.
pub fn pool_backend_count(pool: &BackendPoolInstance) -> usize {
    LoadbalancerSvc::backend_count(pool)
}

/// Build a [`NoopIngressLoadBalancer`] — the default single-node ingress
/// balancer that admits every request in-process (ADR-012).
pub fn build_noop_ingress_lb() -> NoopIngressLoadBalancer {
    NoopIngressLoadBalancer::default()
}

/// Build a [`HandlerInstancePool`] gating `concurrency_cap` concurrent
/// executions of `handler_id`, optionally scoped to `tenant_id`.
///
/// # Errors
///
/// Returns [`LoadbalancerError::InvalidConfig`] when `concurrency_cap` is 0.
pub fn build_handler_pool(
    handler_id: HandlerId,
    tenant_id: Option<TenantId>,
    concurrency_cap: usize,
) -> Result<HandlerInstancePool, LoadbalancerError> {
    HandlerInstancePool::build(handler_id, tenant_id, concurrency_cap)
}

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
/// Returns [`LoadbalancerError::ParseFailed`] when the document is invalid
/// TOML or the section has the wrong shape.
pub fn build_tenant_registry(toml_str: &str) -> Result<TomlTenantRegistry, LoadbalancerError> {
    TomlTenantRegistry::build(toml_str)
}
