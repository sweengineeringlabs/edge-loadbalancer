//! Integration tests — `HandlerInstancePool` via SAF.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{HandlerId, InstancePool, LoadbalancerSvc, TenantId};

#[test]
fn test_build_handler_pool_zero_cap_returns_error() {
    let err = LoadbalancerSvc::build_handler_pool(HandlerId::new("payment"), None, 0).unwrap_err();
    assert!(
        format!("{err}").contains("concurrency_cap"),
        "error must name the bad field"
    );
}

#[test]
fn test_build_handler_pool_returns_working_pool() {
    let pool = LoadbalancerSvc::build_handler_pool(HandlerId::new("billing"), None, 2).unwrap();
    let id = pool.select();
    assert_eq!(id, Some(HandlerId::new("billing")));
    assert_eq!(pool.active_count(), 1);
}

#[test]
fn test_build_handler_pool_with_tenant_id() {
    let pool =
        LoadbalancerSvc::build_handler_pool(HandlerId::new("notify"), Some(TenantId::new("acme")), 5).unwrap();
    assert_eq!(pool.concurrency_cap(), 5);
}
