//! Integration tests — `TomlTenantRegistry` via SAF.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::{build_tenant_registry, TenantId, TenantRegistry};

const TOML: &str = r#"
[tenant.assignments]
acme = "enterprise"
startup_co = "free"
"#;

#[test]
fn test_build_tenant_registry_resolves_known_tenants() {
    let reg = build_tenant_registry(TOML).unwrap();
    assert_eq!(reg.tier_of(&TenantId::new("acme")), Some("enterprise"));
    assert_eq!(reg.tier_of(&TenantId::new("startup_co")), Some("free"));
}

#[test]
fn test_build_tenant_registry_unknown_tenant_returns_none() {
    let reg = build_tenant_registry(TOML).unwrap();
    assert_eq!(reg.tier_of(&TenantId::new("ghost")), None);
}

#[test]
fn test_build_tenant_registry_malformed_toml_returns_error() {
    let err = build_tenant_registry("[tenant.assignments\nacme = ").unwrap_err();
    assert!(format!("{err}").contains("parse") || format!("{err}").len() > 0);
}

#[test]
fn test_build_tenant_registry_missing_section_returns_empty() {
    use swe_edge_loadbalancer::TenantRegistry;
    let reg = build_tenant_registry("[other]\nfoo = 1\n").unwrap();
    assert_eq!(reg.tier_of(&TenantId::new("acme")), None);
}
