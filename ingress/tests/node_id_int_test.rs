//! Integration tests for `NodeId`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_ingress::NodeId;

#[test]
fn test_node_id_new_stores_node_string() {
    let id = NodeId::new("node-7");
    assert_eq!(id.as_str(), "node-7");
}

#[test]
fn test_node_id_equality_same_node_is_equal() {
    let a = NodeId::new("node-7");
    let b = NodeId::new("node-7");
    assert_eq!(a, b);
}

#[test]
fn test_node_id_equality_different_node_is_not_equal() {
    let a = NodeId::new("node-7");
    let b = NodeId::new("node-8");
    assert_ne!(a, b);
}

#[test]
fn test_node_id_display_renders_node_string() {
    let id = NodeId::new("node-7");
    assert_eq!(id.to_string(), "node-7");
}

#[test]
fn test_node_id_clone_is_equal_to_original() {
    let id = NodeId::new("node-7");
    let cloned = id.clone();
    assert_eq!(id, cloned);
}

#[test]
fn test_node_id_hash_usable_as_map_key() {
    use std::collections::HashMap;

    let mut nodes: HashMap<NodeId, &str> = HashMap::new();
    nodes.insert(NodeId::new("node-7"), "us-east-1a");
    nodes.insert(NodeId::new("node-8"), "us-east-1b");

    assert_eq!(nodes.get(&NodeId::new("node-7")), Some(&"us-east-1a"));
    assert_eq!(nodes.get(&NodeId::new("ghost")), None);
}

#[test]
fn test_node_id_new_accepts_empty_string() {
    let id = NodeId::new("");
    assert_eq!(id.as_str(), "");
}
