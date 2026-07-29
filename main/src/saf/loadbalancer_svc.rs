//! Public factory entry point for `swe-edge-loadbalancer`.

use std::sync::Arc;

use swe_edge_loadbalancer_autoscale::{AutoscaleSvc, HandlerId, HandlerInstancePool, InstancePool};
use swe_edge_loadbalancer_egress::{
    Backend, BackendId, BackendPool, BackendPoolInstance, LoadbalancerConfig, Outcome, Validator,
};
use swe_edge_loadbalancer_ingress::NoopIngressLoadBalancer;
use swe_edge_loadbalancer_registry::{InMemoryPoolRegistry, RegistrySvc, TomlTenantRegistry};
use swe_edge_loadbalancer_tenant::TenantId;

use crate::error::LoadbalancerError;

/// Zero-size service struct whose associated functions serve as the public
/// factory entry points for this crate.
///
/// Delegates into each of the five subdomain crates' own constructors
/// (`swe-edge-loadbalancer-{egress,autoscale,ingress,registry}`) per ADR-001
/// — this crate no longer holds any `core/` implementation of its own.
pub struct LoadbalancerSvc;

impl LoadbalancerSvc {
    /// Build a [`BackendPoolInstance`] from a [`LoadbalancerConfig`].
    ///
    /// Validates the config first; returns `Err(LoadbalancerError::Egress(EgressError::InvalidConfig(_)))`
    /// if any backend has an empty URL or zero weight, or the backend list is empty.
    ///
    /// # Errors
    ///
    /// - [`LoadbalancerError::Egress`] — validation failed.
    pub fn build_pool(config: LoadbalancerConfig) -> Result<BackendPoolInstance, LoadbalancerError> {
        Ok(BackendPoolInstance::build(config)?)
    }

    /// Validate a [`LoadbalancerConfig`] without constructing a pool.
    pub fn validate_config(config: &LoadbalancerConfig) -> Result<(), String> {
        BackendPoolInstance::validate(config)
    }

    /// Select a healthy backend from an existing pool.
    pub fn select(pool: &BackendPoolInstance) -> Result<Backend, LoadbalancerError> {
        Ok(pool.select()?)
    }

    /// Record an outcome against a backend, updating its health state.
    pub fn report_outcome(pool: &BackendPoolInstance, id: &BackendId, outcome: Outcome) {
        pool.report_outcome(id, outcome);
    }

    /// Return the number of backends registered in the pool.
    pub fn backend_count(pool: &BackendPoolInstance) -> usize {
        pool.backend_count()
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
    /// Returns [`LoadbalancerError::Autoscale`] when `concurrency_cap` is 0.
    pub fn build_handler_pool(
        handler_id: HandlerId,
        tenant_id: Option<TenantId>,
        concurrency_cap: usize,
    ) -> Result<HandlerInstancePool, LoadbalancerError> {
        Ok(AutoscaleSvc::build_handler_pool(handler_id, tenant_id, concurrency_cap)?)
    }

    /// Build an empty [`InMemoryPoolRegistry`].
    pub fn build_pool_registry() -> InMemoryPoolRegistry {
        RegistrySvc::build_pool_registry()
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
        RegistrySvc::register_handler_pool(registry, handler_id, tenant_id, pool);
    }

    /// Parse a TOML document into a [`TomlTenantRegistry`].
    ///
    /// The document must contain a `[tenant.assignments]` table mapping tenant
    /// id strings to tier name strings.
    ///
    /// # Errors
    ///
    /// Returns [`LoadbalancerError::Registry`] when the document is invalid
    /// TOML or the section has the wrong shape.
    pub fn build_tenant_registry(toml_str: &str) -> Result<TomlTenantRegistry, LoadbalancerError> {
        Ok(RegistrySvc::build_tenant_registry(toml_str)?)
    }
}
