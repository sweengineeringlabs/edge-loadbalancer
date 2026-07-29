//! Integration tests for `Strategy` deserialization from TOML.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_egress::{LoadbalancerConfig, Strategy};

fn parse_strategy(toml: &str) -> Strategy {
    LoadbalancerConfig::from_toml(toml).expect("valid toml").strategy
}

#[test]
fn test_strategy_round_robin_deserializes_from_toml_kebab_case() {
    let s = parse_strategy(
        r#"
        [loadbalancer]
        strategy = "round-robin"
        "#,
    );
    assert_eq!(s, Strategy::RoundRobin);
}

#[test]
fn test_strategy_weighted_deserializes_from_toml_kebab_case() {
    let s = parse_strategy(
        r#"
        [loadbalancer]
        strategy = "weighted"
        "#,
    );
    assert_eq!(s, Strategy::Weighted);
}

#[test]
fn test_strategy_least_connections_deserializes_from_toml_kebab_case() {
    let s = parse_strategy(
        r#"
        [loadbalancer]
        strategy = "least-connections"
        "#,
    );
    assert_eq!(s, Strategy::LeastConnections);
}

#[test]
fn test_strategy_default_is_round_robin() {
    // When strategy key is absent the default must be RoundRobin.
    let s = parse_strategy(
        r#"
        [loadbalancer]
        "#,
    );
    assert_eq!(s, Strategy::RoundRobin);
}

#[test]
fn test_strategy_unknown_value_returns_parse_error() {
    let result = LoadbalancerConfig::from_toml(
        r#"
        [loadbalancer]
        strategy = "not-a-strategy"
        "#,
    );
    assert!(
        result.is_err(),
        "unknown strategy value must fail to parse"
    );
}
