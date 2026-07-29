# ADR-001: Subdomain Crate Split (Port/Adapter Deferred)

**Status:** Proposed
**Date:** 2026-07-29
**Precedent:** [`edge-transport-grpc-ingress`'s ADR-004](https://github.com/sweengineeringlabs/edge-transport-grpc-ingress/blob/main/scm/docs/adr/ADR-004-ports-adapters-discipline.md)
(Port/Adapter Workspace Split) and its HTTP-ingress sibling. Same organization, same
underlying motivation — a crate bundling more than one consumer-relevant concern into a
single compilation unit — but **not mirrored in the same shape here**; see Critical
Finding below for why.

## Summary

Split `swe-edge-loadbalancer` along its subdomain boundaries into separate crates.
Defer a physical port/adapter (api-vs-implementation) split within each subdomain
until a real consumer's needs justify the redesign cost — unlike the precedent ADR,
which resolved its port/adapter blocker and proceeded same-day, this crate's
equivalent blocker is crate-wide, not a single entangled signature, and there is no
current consumer to validate a redesign against.

## Problem

`swe-edge-loadbalancer` grew from a v0.1 egress-only scope (`BackendPool`, `Strategy`,
`Outcome`) to a v0.2 "ingress + scaling" scope (`IngressLoadBalancer`, `InstancePool`,
`ScalingSignal`, `TenantRegistry`, `PoolRegistry`) — see `lib.rs`'s own doc comment.
Those two additions bundled in four subdomains that map to three separate ADRs, plus
a supporting lookup layer, all as one crate:

| Subdomain | Governing ADR | Types | Depends on |
|---|---|---|---|
| **egress-pool** | ADR-011 | `BackendPool` (trait), `BackendPoolInstance`, `Strategy`, `Outcome`, `Backend`/`BackendHealth`/`BackendId`, `LoadbalancerConfig`/`BackendConfig` | — |
| **instance-pool / autoscale** | ADR-013 | `InstancePool`/`ScalingSignal`/`ScalingExecutor` (traits), `HandlerInstancePool`, `PoolSnapshot`, `ScaleOutHint`, `ScalingDecision` | — |
| **ingress** | ADR-012 | `IngressLoadBalancer` (trait), `NoopIngressLoadBalancer`, `LoadBalancerHint` | — |
| **registry** | (supporting) | `PoolRegistry`/`TenantRegistry` (traits), `InMemoryPoolRegistry`, `TomlTenantRegistry` | `InstancePool` (trait only) |

Confirmed by direct grep of every `use crate::api` in each subdomain's files: `ingress`
has zero dependency on `pool` or `registry`; `registry` depends only on the
`InstancePool` *trait*, never a concrete pool type. The `pool/` directory itself
bundles two unrelated subdomains (`BackendPoolInstance` and `HandlerInstancePool`
never reference each other) under one name purely by coincidence of both containing
the word "pool."

As of this ADR, per a separate audit (`edge-loadbalancer#1`/`edge#407`), **no crate in
the `edge` ecosystem consumes `swe-edge-loadbalancer` at all** — this split is
proactive, not a response to a live consumer pulling in more than it needs.

## Critical finding — why the port/adapter precedent doesn't transfer directly

Before proposing a shape, the same diligence ADR-004 applied was repeated here:
check whether `api/`'s port-trait/type signatures name a concrete `core/`-only type,
which would block a physical two-crate split.

ADR-004's blocker was narrow: one DTO field (`ReportRequest.service: &HealthService`)
named a concrete adapter type, fixed with a small interface-segregation change
(one new trait, one field-type swap).

This crate's equivalent problem is structural and crate-wide, not a single field.
**All five of the crate's primary domain types have their struct declared in `api/`
but their inherent `impl` block — the actual constructor and logic — written in
`core/`:**

| Type | Struct in | Inherent `impl` in |
|---|---|---|
| `BackendPoolInstance` | `api/types/pool/` | `core/pool/` |
| `NoopIngressLoadBalancer` | `api/types/ingress/` | `core/ingress/` |
| `HandlerInstancePool` | `api/types/pool/` | `core/pool/` |
| `InMemoryPoolRegistry` | `api/types/registry/` | `core/registry/` |
| `TomlTenantRegistry` | `api/types/registry/` | `core/registry/` |

This is legal today only because `api/` and `core/` are the same crate. Rust's
inherent-impl rule (E0116) requires the impl to live in the same crate as the type's
declaration, independent of what the impl body references — so splitting `api/` and
`core/` into two crates would break all five of these `impl TypeName { ... }` blocks
immediately, regardless of ADR-004-style redesign of any single field.

Verified both Rust-legal ways out are bigger than ADR-004's fix, not smaller:

1. **Move the impl blocks into port.** Checked concretely on `BackendPoolInstance::build()`:
   it directly constructs `PoolInner { entries: Arc::new(RwLock::new(...)), ... }`, a
   concrete `core`-only struct, inline. Moving `build()` to port just relocates the same
   entanglement one level down — it isn't a fix.
2. **Move the struct declarations into adapter**, and have port expose only
   `Arc<dyn BackendPool>`/`Arc<dyn InstancePool>`/etc. This is the more idiomatic
   ports-and-adapters shape, but it changes the crate's public API surface — concrete
   named return types become trait objects — which is a real breaking redesign per
   type, not a mechanical move, and affects every existing call site.

Neither is a same-day fix like ADR-004's. Doing this for all five types, plus
whichever subdomain crate they land in, with no real consumer to validate the
resulting API shape against, is speculative cost with no near-term payoff.

## Decision

### 1. Split by subdomain now

Six crates, replacing the current single `swe-edge-loadbalancer`:

- `swe-edge-loadbalancer-common` — shared identity newtypes (`HandlerId`, `NodeId`,
  `TenantId`), `LoadbalancerError`. Used by every other subdomain crate below.
- `swe-edge-loadbalancer-egress` — ADR-011 egress-pool subdomain.
- `swe-edge-loadbalancer-autoscale` — ADR-013 instance-pool/scaling subdomain.
- `swe-edge-loadbalancer-ingress` — ADR-012 ingress subdomain.
- `swe-edge-loadbalancer-registry` — registry subdomain. Depends on
  `swe-edge-loadbalancer-autoscale` for the `InstancePool` trait (the one real
  cross-subdomain edge).
- `swe-edge-loadbalancer` — umbrella crate, keeps the current name and public API
  shape. Depends on all five above; `LoadbalancerSvc` stays the unified facade,
  its methods now delegating into each subdomain crate's own constructors instead
  of local `core/` code. Existing/future consumers who want everything keep
  depending on this one crate unchanged; a future consumer who wants only e.g.
  `ingress` + `registry` can depend on those two directly instead of pulling in
  `egress`/`autoscale`.

This preserves the current public API surface exactly (umbrella crate = today's
crate, same name, same `LoadbalancerSvc` methods) while making granular consumption
possible for whichever of ADR-011's anticipated future consumers
(`ingress/http`, `ingress/grpc`, `proxy`, `edge-runtime`) only needs one subdomain.

### 2. Defer port/adapter split, per-subdomain, until a real consumer exists

Each of the five non-umbrella crates keeps its current `api/`/`core/`/`saf/` layering
as-is (single compilation unit per subdomain) rather than also physically splitting
into port/adapter crates now. Revisit per-subdomain, independently, once:

- a real consumer in `edge` actually depends on one of these crates, and
- that consumer's own needs (port-only vs. full implementation) are known — informing
  which of the two Rust-legal redesigns (impl-in-port vs. struct-in-adapter) is
  actually worth its cost for that specific subdomain.

Smaller subdomain crates may turn out not to need it at all — e.g. `ingress`'s only
concrete type (`NoopIngressLoadBalancer`) is far simpler than `egress`'s
`BackendPoolInstance`; worth re-checking for the E0116 pattern per-crate before
assuming every subdomain hits the same wall.

## Options considered

1. **Subdomain split + port/adapter split together now.** Not chosen — compounds the
   E0116 redesign cost across four new crate boundaries simultaneously, with zero
   real consumers to validate any of the resulting API shapes against.
2. **Subdomain split now, port/adapter deferred per-subdomain.** Chosen — see Decision.
3. **No split; keep one crate, treat subdomains as directory-level convention only.**
   Not chosen — `pool/` already conflates two unrelated subdomains under one name,
   which is a real cohesion problem independent of any future consumer, and ADR-011
   already names four distinct future consumers likely to want different subsets.

## What changes

- `main/src/api/types/{backend,config}/**`, `main/src/api/types/pool/backend_pool_instance.rs`,
  `main/src/core/pool/backend_pool_instance.rs`, `Strategy`, `Outcome`,
  `ApplicationConfigBuilder` → new `swe-edge-loadbalancer-egress` crate
- `main/src/api/traits/{instance_pool,scaling_executor,scaling_signal}.rs`,
  `main/src/api/types/pool/handler_instance_pool.rs`,
  `main/src/core/pool/handler_instance_pool.rs`, `main/src/api/types/scaling/**`
  → new `swe-edge-loadbalancer-autoscale` crate
- `main/src/api/traits/ingress_load_balancer.rs`, `main/src/api/types/ingress/**`,
  `main/src/core/ingress/**` → new `swe-edge-loadbalancer-ingress` crate
- `main/src/api/traits/{pool_registry,tenant_registry}.rs`,
  `main/src/api/types/registry/**`, `main/src/core/registry/**`
  → new `swe-edge-loadbalancer-registry` crate
- `main/src/api/types/identity/**`, `main/src/api/error/**`
  → new `swe-edge-loadbalancer-common` crate
- `main/src/saf/**` (the `LoadbalancerSvc` facade) → stays in the umbrella
  `swe-edge-loadbalancer` crate, retargeted to call into the five new crates
- `tests/*.rs` (19 files) split by which subdomain each file exercises
- `main/src/api/pool/inner/**` (the `BackendEntry`/`PoolInner` traits — already
  cleanly `core`-only via their trait/private-struct split, no entanglement) moves
  with `egress`

## What does not change

- The umbrella crate's name (`swe-edge-loadbalancer`) and its public API shape —
  `LoadbalancerSvc`'s ten associated methods keep their current signatures
- Trait/type shapes themselves — this is a structural move, not a contract redesign
- `api/`/`core/`/`saf/` layering *within* each new crate — unchanged, per Decision §2

## Versioning & rollout

Breaking (crate boundaries change; anyone who was depending on internal module paths
rather than the umbrella's re-exports would break, though none exist today per
`edge-loadbalancer#1`/`edge#407`). Bump `0.3.0` → **`0.4.0`** per this org's 0.x
SemVer convention, consistent with the `0.2.0` → `0.3.0` bump for the rule-191
remediation.

## Cascade position

Local to this repo. Zero external consumers to audit today (confirmed via
`edge-loadbalancer#1`/`edge#407`) — unlike the precedent ADR, there is no
`swe-edge-runtime-grpc`-equivalent to notify before release. ADR-011's anticipated
future consumers (`ingress/http`, `ingress/grpc`, `proxy`, `edge-runtime`) should
consume the umbrella crate initially regardless of this split, until/unless one of
them specifically wants a single subdomain crate.
