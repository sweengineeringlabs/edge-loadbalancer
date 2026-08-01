//! Integration tests for the aggregating `LoadbalancerError` (ADR-001).
//!
//! `LoadbalancerError` no longer carries its own variants directly — it
//! wraps each subdomain crate's own scoped error type
//! (`EgressError`/`IngressError`/`AutoscaleError`/`RegistryError`) so
//! `LoadbalancerSvc`'s public method signatures keep returning
//! `swe_edge_loadbalancer::LoadbalancerError` unchanged. These tests cover
//! the `From` impl and the transparent `Display`/`std::error::Error`
//! delegation for all four variants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{AutoscaleError, EgressError, IngressError, LoadbalancerError, RegistryError};

#[test]
fn test_loadbalancer_error_egress_variant_display_delegates_to_wrapped_error() {
    let err = LoadbalancerError::Egress(EgressError::NoHealthyBackends);
    let msg = err.to_string();
    assert_eq!(
        msg,
        EgressError::NoHealthyBackends.to_string(),
        "Egress variant's Display must delegate transparently to EgressError"
    );
}

#[test]
fn test_loadbalancer_error_ingress_variant_display_delegates_to_wrapped_error() {
    let reason = "bad node".to_string();
    let msg = LoadbalancerError::Ingress(IngressError::InvalidConfig(reason.clone())).to_string();
    assert_eq!(
        msg,
        IngressError::InvalidConfig(reason).to_string(),
        "Ingress variant's Display must delegate transparently to IngressError"
    );
}

#[test]
fn test_loadbalancer_error_autoscale_variant_display_delegates_to_wrapped_error() {
    let reason = "concurrency_cap must be > 0".to_string();
    let msg = LoadbalancerError::Autoscale(AutoscaleError::InvalidConfig(reason.clone())).to_string();
    assert_eq!(
        msg,
        AutoscaleError::InvalidConfig(reason).to_string(),
        "Autoscale variant's Display must delegate transparently to AutoscaleError"
    );
}

#[test]
fn test_loadbalancer_error_registry_variant_display_delegates_to_wrapped_error() {
    let reason = "expected table".to_string();
    let msg = LoadbalancerError::Registry(RegistryError::ParseFailed(reason.clone())).to_string();
    assert_eq!(
        msg,
        RegistryError::ParseFailed(reason).to_string(),
        "Registry variant's Display must delegate transparently to RegistryError"
    );
}

#[test]
fn test_loadbalancer_error_from_egress_error_wraps_as_egress_variant() {
    let err: LoadbalancerError = EgressError::NoHealthyBackends.into();
    assert!(
        matches!(err, LoadbalancerError::Egress(EgressError::NoHealthyBackends)),
        "From<EgressError> must produce the Egress variant: {err:?}"
    );
}

#[test]
fn test_loadbalancer_error_from_ingress_error_wraps_as_ingress_variant() {
    let err: LoadbalancerError = IngressError::InvalidConfig("x".to_string()).into();
    assert!(
        matches!(err, LoadbalancerError::Ingress(IngressError::InvalidConfig(_))),
        "From<IngressError> must produce the Ingress variant: {err:?}"
    );
}

#[test]
fn test_loadbalancer_error_from_autoscale_error_wraps_as_autoscale_variant() {
    let err: LoadbalancerError = AutoscaleError::InvalidConfig("x".to_string()).into();
    assert!(
        matches!(err, LoadbalancerError::Autoscale(AutoscaleError::InvalidConfig(_))),
        "From<AutoscaleError> must produce the Autoscale variant: {err:?}"
    );
}

#[test]
fn test_loadbalancer_error_from_registry_error_wraps_as_registry_variant() {
    let err: LoadbalancerError = RegistryError::ParseFailed("x".to_string()).into();
    assert!(
        matches!(err, LoadbalancerError::Registry(RegistryError::ParseFailed(_))),
        "From<RegistryError> must produce the Registry variant: {err:?}"
    );
}

#[test]
fn test_loadbalancer_error_variants_are_distinct() {
    let e1 = LoadbalancerError::Egress(EgressError::NoHealthyBackends);
    let e2 = LoadbalancerError::Ingress(IngressError::InvalidConfig("x".to_string()));
    let e3 = LoadbalancerError::Autoscale(AutoscaleError::InvalidConfig("x".to_string()));
    let e4 = LoadbalancerError::Registry(RegistryError::ParseFailed("x".to_string()));
    // All four have different Display outputs.
    assert_ne!(e1.to_string(), e2.to_string());
    assert_ne!(e1.to_string(), e3.to_string());
    assert_ne!(e1.to_string(), e4.to_string());
    assert_ne!(e2.to_string(), e3.to_string());
    assert_ne!(e2.to_string(), e4.to_string());
    assert_ne!(e3.to_string(), e4.to_string());
}

#[test]
fn test_loadbalancer_error_debug_is_implemented() {
    let err = LoadbalancerError::Egress(EgressError::NoHealthyBackends);
    let dbg = format!("{err:?}");
    assert!(!dbg.is_empty(), "Debug impl must produce non-empty output");
}

#[test]
fn test_loadbalancer_error_source_forwards_transparently_to_wrapped_errors_own_source() {
    // `#[error(transparent)]` forwards `source()` to the *wrapped* error's own
    // `source()` (not to the wrapped error itself) — matching `Display`'s
    // transparent forwarding. None of the four subdomain leaf error types
    // chain a further nested source, so this must be `None`, not `Some`.
    use std::error::Error;
    let err = LoadbalancerError::Egress(EgressError::NoHealthyBackends);
    assert_eq!(
        err.source().map(ToString::to_string),
        EgressError::NoHealthyBackends.source().map(ToString::to_string),
        "aggregating error's source() must forward transparently to the wrapped error's own source()"
    );
}

#[test]
fn test_loadbalancer_error_converts_to_boxed_std_error_via_question_mark() {
    // The whole point of the `From` impls + `#[error(transparent)]` pairing
    // is that `?` works cleanly inside `LoadbalancerSvc`'s methods. Prove the
    // trait object conversion actually compiles and runs end to end.
    fn fallible() -> Result<(), LoadbalancerError> {
        let inner: Result<(), EgressError> = Err(EgressError::NoHealthyBackends);
        inner?;
        Ok(())
    }
    fn boxed() -> Result<(), Box<dyn std::error::Error>> {
        fallible()?;
        Ok(())
    }
    let err = boxed().unwrap_err();
    assert!(!err.to_string().is_empty(), "boxed error must still Display");
}
