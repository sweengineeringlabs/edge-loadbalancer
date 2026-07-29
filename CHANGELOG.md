# Changelog

All notable changes to `swe-edge-loadbalancer` are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-07-29

### Changed

- **Breaking:** split `swe-edge-loadbalancer` along its subdomain boundaries into
  five new crates plus this umbrella crate, per ADR-001
  (`docs/adr/ADR-001-subdomain-crate-split.md`) — tracked in epic #2 / issue #3.
  Anyone depending on this crate's internal module paths (`swe_edge_loadbalancer::api::...`,
  `swe_edge_loadbalancer::core::...`) rather than its public re-exports breaks; no such
  consumer exists today (confirmed in issue #1 / edge#407). The umbrella crate keeps its
  name and its `LoadbalancerSvc` facade's ten associated methods unchanged, so existing
  consumers who depend on `swe-edge-loadbalancer` alone are unaffected. Migration mapping
  (old module path in this crate → new crate):

  | Old module path (`swe-edge-loadbalancer`) | New crate |
  |---|---|
  | `api::types::identity::TenantId` | `swe-edge-loadbalancer-tenant` (`TenantId`) |
  | `api::traits::BackendPool`, `api::types::pool::BackendPoolInstance`, `api::types::strategy::Strategy`, `api::types::outcome::Outcome`, `api::types::backend::{Backend, BackendHealth, BackendId}`, `api::types::config::{LoadbalancerConfig, BackendConfig}`, `api::types::application_config_builder::ApplicationConfigBuilder`, `api::pool::inner::{BackendEntry, PoolInner}` | `swe-edge-loadbalancer-egress` (own `EgressError`, ADR-011) |
  | `api::traits::{InstancePool, ScalingExecutor, ScalingSignal}`, `api::types::pool::HandlerInstancePool`, `api::types::scaling::{PoolSnapshot, ScaleOutHint, ScalingDecision}`, `api::types::identity::HandlerId` | `swe-edge-loadbalancer-autoscale` (own `AutoscaleError`, ADR-013) |
  | `api::traits::IngressLoadBalancer`, `api::types::ingress::{LoadBalancerHint, NoopIngressLoadBalancer}`, `api::types::identity::NodeId` | `swe-edge-loadbalancer-ingress` (own `IngressError`, ADR-012) |
  | `api::traits::{PoolRegistry, TenantRegistry}`, `api::types::registry::{InMemoryPoolRegistry, TomlTenantRegistry}` | `swe-edge-loadbalancer-registry` (own `RegistryError`) |
  | `api::error::LoadbalancerError` | stays in this crate; unchanged 3-variant shape (`NoHealthyBackends`/`InvalidConfig`/`ParseFailed`) for this release — becomes an aggregating error wrapping `EgressError`/`IngressError`/`AutoscaleError`/`RegistryError` (each with a `From` impl) once issue #10 lands, so `LoadbalancerSvc`'s public signatures keep returning `swe_edge_loadbalancer::LoadbalancerError` throughout |
  | `saf::loadbalancer_svc::LoadbalancerSvc` | stays in this crate |

  Each new crate is independently depended-on today: a consumer that only needs,
  e.g., `ingress` + `registry` can take those two crates directly instead of
  pulling in `egress`/`autoscale`, and gets that subdomain's own scoped error
  type (`IngressError`/`RegistryError`) rather than the umbrella's error type.
  **Not yet done as of this release:**
  the umbrella's internal implementation retarget onto these five crates
  (`LoadbalancerSvc`'s methods delegating into each subdomain crate's own
  constructors instead of local `core/` code, and `LoadbalancerError` becoming
  the aggregating wrapper) — tracked separately in issue #10. Until #10 lands,
  this crate's own `main/src/` and `tests/` are unchanged and continue to build
  and pass their full existing test suite standalone, so no consumer sees any
  behavior change from this release beyond the five new crates becoming
  available to depend on directly. No `-common` grab-bag crate was created;
  see the ADR's Revision note for why `TenantId` alone (not also `HandlerId`/
  `NodeId`/`LoadbalancerError`) needed a shared-kernel crate.

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
