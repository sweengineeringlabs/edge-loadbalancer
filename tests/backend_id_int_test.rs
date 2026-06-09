//! Integration tests for `BackendId`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer::BackendId;

#[test]
fn test_backend_id_new_stores_url_string() {
    let id = BackendId::new("https://api-1.internal");
    assert_eq!(id.as_str(), "https://api-1.internal");
}

#[test]
fn test_backend_id_equality_same_url_is_equal() {
    let a = BackendId::new("https://api-1.internal");
    let b = BackendId::new("https://api-1.internal");
    assert_eq!(a, b);
}

#[test]
fn test_backend_id_equality_different_url_is_not_equal() {
    let a = BackendId::new("https://api-1.internal");
    let b = BackendId::new("https://api-2.internal");
    assert_ne!(a, b);
}

#[test]
fn test_backend_id_display_renders_url() {
    let id = BackendId::new("https://api-1.internal");
    assert_eq!(id.to_string(), "https://api-1.internal");
}

#[test]
fn test_backend_id_clone_is_equal_to_original() {
    let id = BackendId::new("https://api-1.internal");
    let cloned = id.clone();
    assert_eq!(id, cloned);
}
