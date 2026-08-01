//! Integration tests — `NoopIngressLoadBalancer` via the public API.
//!
//! Migrated from `swe-edge-loadbalancer`'s `tests/ingress_lb_int_test.rs` per
//! ADR-001. Construction changes from `LoadbalancerSvc::build_noop_ingress_lb()`
//! (the umbrella crate's facade, not present in this leaf crate) to
//! `NoopIngressLoadBalancer::default()` directly — the same public
//! construction path `core/ingress/noop_ingress_load_balancer.rs`'s own unit
//! tests already use within this crate. Assertions and covered behaviour are
//! unchanged.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_ingress::{
    IngressLoadBalancer, LoadBalancerHint, NodeId, NoopIngressLoadBalancer,
};
use swe_edge_loadbalancer_tenant::TenantId;

#[test]
fn test_noop_lb_always_admits_in_process_without_tenant() {
    let lb = NoopIngressLoadBalancer::default();
    assert!(matches!(lb.on_accept(None), LoadBalancerHint::UseInProcess));
}

#[test]
fn test_noop_lb_always_admits_in_process_with_tenant() {
    let lb = NoopIngressLoadBalancer::default();
    let tid = TenantId::new("acme");
    assert!(matches!(lb.on_accept(Some(&tid)), LoadBalancerHint::UseInProcess));
}

#[test]
fn test_noop_lb_add_remove_node_return_ok() {
    let lb = NoopIngressLoadBalancer::default();
    let node = NodeId::new("node-1");
    assert!(lb.add_node(&node).is_ok());
    assert!(lb.remove_node(&node).is_ok());
}
