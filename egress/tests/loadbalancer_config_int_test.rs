//! Integration tests for `LoadbalancerConfig` TOML parsing with `OptionalSection`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_loadbalancer_egress::{LoadbalancerConfig, Strategy};

#[test]
fn test_loadbalancer_config_parses_strategy_and_backends_from_toml() {
    let toml = r#"
        [loadbalancer]
        strategy = "weighted"

        [[loadbalancer.backends]]
        url = "https://api-1.internal"
        weight = 2

        [[loadbalancer.backends]]
        url = "https://api-2.internal"
        weight = 1
    "#;
    let config = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(config.strategy, Strategy::Weighted);
    assert_eq!(config.backends.len(), 2);
    assert_eq!(config.backends[0].url, "https://api-1.internal");
    assert_eq!(config.backends[0].weight, 2);
    assert_eq!(config.backends[1].url, "https://api-2.internal");
    assert_eq!(config.backends[1].weight, 1);
}

#[test]
fn test_loadbalancer_config_backend_weight_defaults_to_one() {
    let toml = r#"
        [loadbalancer]

        [[loadbalancer.backends]]
        url = "https://api-1.internal"
    "#;
    let config = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(config.backends[0].weight, 1, "default weight must be 1");
}

#[test]
fn test_loadbalancer_config_strategy_defaults_to_round_robin() {
    let toml = r#"
        [loadbalancer]
        [[loadbalancer.backends]]
        url = "https://api-1.internal"
    "#;
    let config = LoadbalancerConfig::from_toml(toml).expect("must parse");
    assert_eq!(config.strategy, Strategy::RoundRobin);
}

#[test]
fn test_loadbalancer_config_malformed_toml_returns_parse_failed_error() {
    let result = LoadbalancerConfig::from_toml("not valid toml [[[");
    assert!(result.is_err(), "malformed TOML must fail");
    assert!(
        matches!(result.unwrap_err(), swe_edge_loadbalancer_egress::EgressError::ParseFailed(_)),
        "must be ParseFailed variant"
    );
}

#[test]
fn test_loadbalancer_config_missing_loadbalancer_section_returns_error() {
    // from_toml requires the [loadbalancer] table wrapper
    let result = LoadbalancerConfig::from_toml("strategy = \"round-robin\"");
    assert!(result.is_err(), "missing [loadbalancer] section must fail");
}

#[test]
fn test_loadbalancer_config_section_name_is_loadbalancer() {
    use swe_edge_configbuilder::ConfigSection;
    assert_eq!(LoadbalancerConfig::section_name(), "loadbalancer");
}

#[test]
fn test_loadbalancer_config_optional_section_name_is_loadbalancer() {
    use swe_edge_configbuilder::OptionalSection;
    assert_eq!(
        <LoadbalancerConfig as OptionalSection>::section_name(),
        "loadbalancer"
    );
}

#[test]
fn test_loadbalancer_config_optional_section_metadata_has_owner() {
    use swe_edge_configbuilder::OptionalSection;
    let meta = LoadbalancerConfig::metadata();
    assert_eq!(meta.owner, "platform-team");
}
