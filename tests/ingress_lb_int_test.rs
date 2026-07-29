//! Integration tests — `NoopIngressLoadBalancer` via SAF.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{
    IngressLoadBalancer, LoadBalancerHint, LoadbalancerSvc, NodeId, TenantId,
};

#[test]
fn test_noop_lb_always_admits_in_process_without_tenant() {
    let lb = LoadbalancerSvc::build_noop_ingress_lb();
    assert!(matches!(lb.on_accept(None), LoadBalancerHint::UseInProcess));
}

#[test]
fn test_noop_lb_always_admits_in_process_with_tenant() {
    let lb = LoadbalancerSvc::build_noop_ingress_lb();
    let tid = TenantId::new("acme");
    assert!(matches!(lb.on_accept(Some(&tid)), LoadBalancerHint::UseInProcess));
}

#[test]
fn test_noop_lb_add_remove_node_return_ok() {
    let lb = LoadbalancerSvc::build_noop_ingress_lb();
    let node = NodeId::new("node-1");
    assert!(lb.add_node(&node).is_ok());
    assert!(lb.remove_node(&node).is_ok());
}
