//! Integration tests for `TenantId`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_tenant::TenantId;

#[test]
fn test_tenant_id_new_stores_tenant_string() {
    let id = TenantId::new("acme");
    assert_eq!(id.as_str(), "acme");
}

#[test]
fn test_tenant_id_equality_same_tenant_is_equal() {
    let a = TenantId::new("acme");
    let b = TenantId::new("acme");
    assert_eq!(a, b);
}

#[test]
fn test_tenant_id_equality_different_tenant_is_not_equal() {
    let a = TenantId::new("acme");
    let b = TenantId::new("startup_co");
    assert_ne!(a, b);
}

#[test]
fn test_tenant_id_display_renders_tenant_string() {
    let id = TenantId::new("acme");
    assert_eq!(id.to_string(), "acme");
}

#[test]
fn test_tenant_id_clone_is_equal_to_original() {
    let id = TenantId::new("acme");
    let cloned = id.clone();
    assert_eq!(id, cloned);
}

#[test]
fn test_tenant_id_hash_usable_as_map_key() {
    use std::collections::HashMap;

    let mut tiers: HashMap<TenantId, &str> = HashMap::new();
    tiers.insert(TenantId::new("acme"), "enterprise");
    tiers.insert(TenantId::new("startup_co"), "free");

    assert_eq!(tiers.get(&TenantId::new("acme")), Some(&"enterprise"));
    assert_eq!(tiers.get(&TenantId::new("ghost")), None);
}

#[test]
fn test_tenant_id_new_accepts_empty_string() {
    let id = TenantId::new("");
    assert_eq!(id.as_str(), "");
}
