//! Integration tests — `InMemoryPoolRegistry` via SAF.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use swe_edge_loadbalancer::{
    HandlerId, InstancePool, LoadbalancerSvc, PoolRegistry, TenantId,
};

fn payment() -> HandlerId {
    HandlerId::new("payment")
}

fn acme() -> TenantId {
    TenantId::new("acme")
}

#[test]
fn test_build_pool_registry_starts_empty() {
    let reg = LoadbalancerSvc::build_pool_registry();
    assert!(reg.get(&payment(), None).is_none());
}

#[test]
fn test_register_and_get_shared_pool() {
    let reg = LoadbalancerSvc::build_pool_registry();
    let pool = Arc::new(LoadbalancerSvc::build_handler_pool(payment(), None, 4).unwrap());
    LoadbalancerSvc::register_handler_pool(&reg, &payment(), None, pool);
    assert!(reg.get(&payment(), None).is_some());
}

#[test]
fn test_register_tenant_pool_fallback_from_shared() {
    let reg = LoadbalancerSvc::build_pool_registry();
    let shared = Arc::new(LoadbalancerSvc::build_handler_pool(payment(), None, 10).unwrap());
    LoadbalancerSvc::register_handler_pool(&reg, &payment(), None, shared);
    // Unknown tenant → falls back to shared
    let found = reg.get(&payment(), Some(&acme()));
    assert!(found.is_some(), "must fall back to shared pool");
}

#[test]
fn test_tenant_override_wins() {
    let reg = LoadbalancerSvc::build_pool_registry();
    let shared: Arc<dyn InstancePool> = Arc::new(LoadbalancerSvc::build_handler_pool(payment(), None, 10).unwrap());
    let tenant_pool: Arc<dyn InstancePool> =
        Arc::new(LoadbalancerSvc::build_handler_pool(payment(), Some(acme()), 3).unwrap());
    LoadbalancerSvc::register_handler_pool(&reg, &payment(), None, shared);
    LoadbalancerSvc::register_handler_pool(&reg, &payment(), Some(&acme()), Arc::clone(&tenant_pool));
    let found = reg.get(&payment(), Some(&acme())).unwrap();
    assert_eq!(found.concurrency_cap(), 3, "tenant pool cap must win");
}
