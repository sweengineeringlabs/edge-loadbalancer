# Changelog

All notable changes to `swe-edge-loadbalancer` are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-07-29

### Changed

- **Breaking:** removed the 10 public SAF free functions from `saf/loadbalancer_svc.rs`
  in favor of associated methods on `LoadbalancerSvc`, closing out SEA rule 191
  ("SEA layer files must not contain free-standing fn") for the `saf/` layer
  (tracked in issue #1 / edge#249). Migration:

  | Old free function | New call |
  |---|---|
  | `build_backend_pool(config)` | `LoadbalancerSvc::build_pool(config)` |
  | `validate_loadbalancer_config(config)` | `LoadbalancerSvc::validate_config(config)` |
  | `select_backend(pool)` | `LoadbalancerSvc::select(pool)` |
  | `report_backend_outcome(pool, id, outcome)` | `LoadbalancerSvc::report_outcome(pool, id, outcome)` |
  | `pool_backend_count(pool)` | `LoadbalancerSvc::backend_count(pool)` |
  | `build_noop_ingress_lb()` | `LoadbalancerSvc::build_noop_ingress_lb()` |
  | `build_handler_pool(handler_id, tenant_id, cap)` | `LoadbalancerSvc::build_handler_pool(handler_id, tenant_id, cap)` |
  | `build_pool_registry()` | `LoadbalancerSvc::build_pool_registry()` |
  | `register_handler_pool(registry, handler_id, tenant_id, pool)` | `LoadbalancerSvc::register_handler_pool(registry, handler_id, tenant_id, pool)` |
  | `build_tenant_registry(toml_str)` | `LoadbalancerSvc::build_tenant_registry(toml_str)` |

## [0.2.0] - 2026-06-10

### Added

- `TenantId`, `HandlerId`, `NodeId` newtypes (identity module).
- `LoadBalancerHint` enum: `UseInProcess`, `RouteToInstance(NodeId)`, `Reject`.
- `NoopIngressLoadBalancer` — pass-through `IngressLoadBalancer` for single-node deployments.
- `IngressLoadBalancer` trait: `on_accept(tenant) → LoadBalancerHint`, with default no-op `add_node`/`remove_node`.
- `PoolSnapshot` struct — saturation metrics snapshot from an `InstancePool`.
- `ScaleOutHint` struct — image + config_url for horizontal scale-out.
- `ScalingDecision` enum: `Hold`, `ScaleUp(n)`, `ScaleDown(n)`, `ScaleOut(hint)`, `ScaleIn`.
- `InstancePool` trait: atomic concurrency gate — `select()`, `report_outcome()`, `active_count()`, `concurrency_cap()`, `set_concurrency_cap()`.
- `HandlerInstancePool` implementation: lock-free CAS slot accounting with `ScalingSignal`.
- `ScalingSignal` trait: `snapshot() → PoolSnapshot`.
- `ScalingExecutor` trait: `scale_up`, `scale_down`, `scale_out`, `scale_in`.
- `TenantRegistry` trait + `TomlTenantRegistry` implementation (TOML `[tenant.assignments]` section).
- `PoolRegistry` trait + `InMemoryPoolRegistry` implementation with tenant-scoped fallback.
- SAF factory functions: `build_noop_ingress_lb`, `build_handler_pool`, `build_pool_registry`,
  `register_handler_pool`, `build_tenant_registry`.
- SAF trait re-exports for downstream dispatchers: `IngressLoadBalancer`, `InstancePool`,
  `PoolRegistry`, `ScalingExecutor`, `ScalingSignal`, `TenantRegistry`.

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
