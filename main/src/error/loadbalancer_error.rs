//! `LoadbalancerError` — aggregating error type for `swe-edge-loadbalancer`.

use thiserror::Error;

use swe_edge_loadbalancer_autoscale::AutoscaleError;
use swe_edge_loadbalancer_egress::EgressError;
use swe_edge_loadbalancer_ingress::IngressError;
use swe_edge_loadbalancer_registry::RegistryError;

/// Errors produced by `LoadbalancerSvc`'s associated methods.
///
/// Per ADR-001's Revision note, the old single `LoadbalancerError` enum did
/// four unrelated jobs — each variant partitioned cleanly to a single owning
/// subdomain (or pair). Each subdomain crate now defines its own scoped
/// error type (`EgressError`/`IngressError`/`AutoscaleError`/`RegistryError`)
/// and this umbrella crate aggregates them purely to keep the public
/// `swe_edge_loadbalancer::LoadbalancerError` path — and `LoadbalancerSvc`'s
/// existing method signatures — stable. Display and `std::error::Error`
/// delegate transparently to the wrapped error via `#[error(transparent)]`.
#[derive(Debug, Error)]
pub enum LoadbalancerError {
    /// An error produced by [`swe_edge_loadbalancer_egress`].
    #[error(transparent)]
    Egress(#[from] EgressError),

    /// An error produced by [`swe_edge_loadbalancer_ingress`].
    #[error(transparent)]
    Ingress(#[from] IngressError),

    /// An error produced by [`swe_edge_loadbalancer_autoscale`].
    #[error(transparent)]
    Autoscale(#[from] AutoscaleError),

    /// An error produced by [`swe_edge_loadbalancer_registry`].
    #[error(transparent)]
    Registry(#[from] RegistryError),
}
