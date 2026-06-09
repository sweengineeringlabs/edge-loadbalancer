# Changelog

All notable changes to `swe-edge-loadbalancer` are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-09

### Added

- `BackendId` newtype wrapping a URL string.
- `Backend` struct with id, url, weight, and health fields.
- `BackendHealth` enum: `Healthy`, `Degraded`, `Dead`.
- `Outcome` enum: `Success`, `Failure { reason }`, `CircuitOpen`.
- `Strategy` enum: `RoundRobin`, `Weighted`, `LeastConnections`.
- `BackendPool` trait: `select()` and `report_outcome()`.
- `DefaultBackendPool` implementation supporting all three strategies.
- `LoadbalancerConfig` TOML schema implementing `ConfigSection` and `OptionalSection`.
- `LoadbalancerSvc` zero-size factory struct.
- `LoadbalancerError` error enum.
- SAF factory functions: `build_backend_pool`, `validate_loadbalancer_config`,
  `select_backend`, `report_backend_outcome`.
- Full integration test suite.
