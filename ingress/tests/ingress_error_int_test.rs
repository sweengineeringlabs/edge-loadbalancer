//! Integration tests for `IngressError`.
//!
//! `IngressError` is a new type introduced by this crate (ADR-001/ADR-012),
//! split out of the former shared `LoadbalancerError::InvalidConfig`. It is
//! currently unused by `NoopIngressLoadBalancer` (whose defaults are no-ops)
//! but is part of `IngressLoadBalancer::add_node`/`remove_node`'s real public
//! contract for any future non-noop implementor.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_ingress::IngressError;

#[test]
fn test_ingress_error_invalid_config_includes_reason() {
    let err = IngressError::InvalidConfig("node registry unreachable".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("node registry unreachable"),
        "InvalidConfig error must include reason: {msg}"
    );
}

#[test]
fn test_ingress_error_debug_is_implemented() {
    let err = IngressError::InvalidConfig("bad node".to_string());
    let dbg = format!("{err:?}");
    assert!(!dbg.is_empty(), "Debug impl must produce non-empty output");
}
