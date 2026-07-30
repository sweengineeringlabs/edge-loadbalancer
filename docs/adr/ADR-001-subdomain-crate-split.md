# ADR-001: Subdomain Crate Split

**Status:** Accepted (shipped in v0.4.0)
**Date:** 2026-07-29
**See also:** [ADR-002](ADR-002-port-adapter.md) — whether to further split each of the
five crates below into a port crate and an adapter crate is a separate, still-open
question, evaluated there rather than here.

## Summary

Split `swe-edge-loadbalancer` along its subdomain boundaries into separate crates.

## Revision note (superseded — kept for history)

This ADR originally (same date, same file) proposed a sixth `swe-edge-loadbalancer-common`
crate holding all three identity newtypes (`HandlerId`, `NodeId`, `TenantId`) plus the
single `LoadbalancerError` enum, on the reasoning that all four were referenced from
more than one subdomain. That was rejected on review as an unjustified grab-bag risk —
"common" crates accumulate unrelated things over time with no enforced membership
criterion. Re-investigated using only *real* signature usage (trait method signatures
and inherent-impl call sites, not doc-comment mentions or "the type is imported in this
file"), per-type and per-error-variant:

- `NodeId` — used only by `ingress` (`IngressLoadBalancer::add_node`/`remove_node`).
  No sharing needed; moves into `-ingress`.
- `HandlerId` — used only by `autoscale` (`InstancePool`, `ScalingExecutor`,
  `HandlerInstancePool`) and `registry` (`PoolRegistry`, `InMemoryPoolRegistry`).
  `-registry` already depends on `-autoscale` for the `InstancePool` trait (the one
  real cross-subdomain edge), so it gets `HandlerId` on that same existing edge — no
  new dependency. Moves into `-autoscale`.
- `TenantId` — genuinely 3-way, in real trait signatures:
  `IngressLoadBalancer::on_accept`, `ScalingExecutor::scale_up`/`scale_down`,
  `TenantRegistry::tier_of`. No asymmetric owner exists without either a dependency
  cycle (`autoscale` and `ingress` don't and shouldn't depend on each other) or forcing
  a subdomain to pull in an entire unrelated crate for one newtype. This is the one
  genuine shared-kernel type — see Decision §1.
- `LoadbalancerError` — checked live signatures, not the enum's import list: each
  variant partitions cleanly to a single owning subdomain or pair
  (`NoHealthyBackends` → egress only; `ParseFailed` → egress + registry;
  `InvalidConfig` → egress + autoscale + ingress). This is not a shared-type case,
  it's "one enum doing four jobs" — the fix is to split it per-subdomain
  (`EgressError`/`IngressError`/`AutoscaleError`/`RegistryError`) with the umbrella
  crate defining an aggregating `LoadbalancerError` purely to keep the public
  `swe_edge_loadbalancer::LoadbalancerError` path stable. No shared crate needed.
  Confirmed separately that only `BackendPool` (egress) and `IngressLoadBalancer`
  (ingress) bake an error type into a cross-implementor trait contract — `autoscale`'s
  and `registry`'s error usage is confined to their own inherent constructors
  (`HandlerInstancePool::build`, `TomlTenantRegistry::build`), not trait methods,
  which makes the per-subdomain split unambiguous for those two.

Net effect: no `-common` crate. Five crates instead of six — `-tenant` (containing
only `TenantId`) replaces it, scoped narrowly enough that its membership criterion
is unambiguous (exactly the one type with no valid single-subdomain owner) rather
than an open invitation to dump anything vaguely shared into it later.

## Problem

`swe-edge-loadbalancer` grew from a v0.1 egress-only scope (`BackendPool`, `Strategy`,
`Outcome`) to a v0.2 "ingress + scaling" scope (`IngressLoadBalancer`, `InstancePool`,
`ScalingSignal`, `TenantRegistry`, `PoolRegistry`) — see `lib.rs`'s own doc comment.
Those two additions bundled in four subdomains that map to three separate ADRs, plus
a supporting lookup layer, all as one crate:

| Subdomain | Governing ADR | Types | Depends on |
|---|---|---|---|
| **egress-pool** | ADR-011 | `BackendPool` (trait), `BackendPoolInstance`, `Strategy`, `Outcome`, `Backend`/`BackendHealth`/`BackendId`, `LoadbalancerConfig`/`BackendConfig`, `EgressError` | — |
| **instance-pool / autoscale** | ADR-013 | `InstancePool`/`ScalingSignal`/`ScalingExecutor` (traits), `HandlerInstancePool`, `PoolSnapshot`, `ScaleOutHint`, `ScalingDecision`, `HandlerId`, `AutoscaleError` | `-tenant` (`TenantId`) |
| **ingress** | ADR-012 | `IngressLoadBalancer` (trait), `NoopIngressLoadBalancer`, `LoadBalancerHint`, `NodeId`, `IngressError` | `-tenant` (`TenantId`) |
| **registry** | (supporting) | `PoolRegistry`/`TenantRegistry` (traits), `InMemoryPoolRegistry`, `TomlTenantRegistry`, `RegistryError` | `-autoscale` (`InstancePool` trait + `HandlerId`), `-tenant` (`TenantId`) |
| **tenant** | (shared kernel) | `TenantId` only | — |

Confirmed by direct grep of every `use crate::api` in each subdomain's files: `ingress`
has zero dependency on `pool` or `registry`; `registry` depends only on the
`InstancePool` *trait*, never a concrete pool type. The `pool/` directory itself
bundles two unrelated subdomains (`BackendPoolInstance` and `HandlerInstancePool`
never reference each other) under one name purely by coincidence of both containing
the word "pool."

As of this ADR, per a separate audit (`edge-loadbalancer#1`/`edge#407`), **no crate in
the `edge` ecosystem consumes `swe-edge-loadbalancer` at all** — this split is
proactive, not a response to a live consumer pulling in more than it needs.

## Decision

### 1. Split by subdomain

Six crates, replacing the current single `swe-edge-loadbalancer` — five leaf/subdomain
crates plus the umbrella (no `-common`; see Revision note above):

- `swe-edge-loadbalancer-tenant` — shared kernel, `TenantId` only. The one type with
  no valid single-subdomain owner. Used by `-autoscale`, `-ingress`, `-registry`.
- `swe-edge-loadbalancer-egress` — ADR-011 egress-pool subdomain, plus its own
  `EgressError` (`NoHealthyBackends`, `InvalidConfig`, `ParseFailed`).
- `swe-edge-loadbalancer-autoscale` — ADR-013 instance-pool/scaling subdomain, plus
  `HandlerId` and its own `AutoscaleError` (`InvalidConfig`). Depends on `-tenant`.
- `swe-edge-loadbalancer-ingress` — ADR-012 ingress subdomain, plus `NodeId` and its
  own `IngressError` (`InvalidConfig`). Depends on `-tenant`.
- `swe-edge-loadbalancer-registry` — registry subdomain, plus its own `RegistryError`
  (`ParseFailed`). Depends on `-autoscale` (`InstancePool` trait, `HandlerId`) and
  `-tenant` (`TenantId`).
- `swe-edge-loadbalancer` — umbrella crate, keeps the current name and public API
  shape. Depends on all five above; `LoadbalancerSvc` stays the unified facade,
  its methods now delegating into each subdomain crate's own constructors instead
  of local `core/` code. Defines the aggregating `LoadbalancerError` enum
  (`Egress(EgressError)` / `Ingress(IngressError)` / `Autoscale(AutoscaleError)` /
  `Registry(RegistryError)` variants, with `From` impls) so `LoadbalancerSvc`'s public
  signatures keep returning `swe_edge_loadbalancer::LoadbalancerError` unchanged.
  Existing/future consumers who want everything keep depending on this one crate
  unchanged; a future consumer who wants only e.g. `ingress` + `registry` can depend
  on those two directly instead of pulling in `egress`/`autoscale`, and gets that
  subdomain's own scoped error type rather than the umbrella's aggregate.

This preserves the current public API surface exactly (umbrella crate = today's
crate, same name, same `LoadbalancerSvc` methods) while making granular consumption
possible for whichever of ADR-011's anticipated future consumers
(`ingress/http`, `ingress/grpc`, `proxy`, `edge-runtime`) only needs one subdomain.

### 2. Port/adapter split — separate question

Whether to further split any of the five crates above into a port crate and an
adapter crate is evaluated in [ADR-002](ADR-002-port-adapter.md), not here. Each of
the five keeps its current `api/`/`core/`/`saf/` layering as shipped.

## Options considered

1. **Subdomain split + port/adapter split together.** Not chosen — see ADR-002's
   Critical Finding for the cost this would have compounded across four crate
   boundaries simultaneously, with zero real consumers to validate any of the
   resulting API shapes against.
2. **Subdomain split now, port/adapter as a separate question.** Chosen — see Decision.
3. **No split; keep one crate, treat subdomains as directory-level convention only.**
   Not chosen — `pool/` already conflates two unrelated subdomains under one name,
   which is a real cohesion problem independent of any future consumer, and ADR-011
   already names four distinct future consumers likely to want different subsets.

## What changes

- `main/src/api/types/identity/tenant_id.rs` → new `swe-edge-loadbalancer-tenant` crate
- `main/src/api/types/{backend,config}/**`, `main/src/api/types/pool/backend_pool_instance.rs`,
  `main/src/core/pool/backend_pool_instance.rs`, `Strategy`, `Outcome`,
  `ApplicationConfigBuilder`, `main/src/api/pool/inner/**` (the `BackendEntry`/
  `PoolInner` traits — already cleanly `core`-only via their trait/private-struct
  split, no entanglement), and a new local `EgressError` (the `NoHealthyBackends`/
  `InvalidConfig`/`ParseFailed` variants currently on `LoadbalancerError`) → new
  `swe-edge-loadbalancer-egress` crate
- `main/src/api/traits/{instance_pool,scaling_executor,scaling_signal}.rs`,
  `main/src/api/types/pool/handler_instance_pool.rs`,
  `main/src/core/pool/handler_instance_pool.rs`, `main/src/api/types/scaling/**`,
  `main/src/api/types/identity/handler_id.rs`, and a new local `AutoscaleError`
  (`InvalidConfig`) → new `swe-edge-loadbalancer-autoscale` crate
- `main/src/api/traits/ingress_load_balancer.rs`, `main/src/api/types/ingress/**`,
  `main/src/core/ingress/**`, `main/src/api/types/identity/node_id.rs`, and a new
  local `IngressError` (`InvalidConfig`) → new `swe-edge-loadbalancer-ingress` crate
- `main/src/api/traits/{pool_registry,tenant_registry}.rs`,
  `main/src/api/types/registry/**`, `main/src/core/registry/**`, and a new local
  `RegistryError` (`ParseFailed`) → new `swe-edge-loadbalancer-registry` crate
- `main/src/api/error/loadbalancer_error.rs` → becomes the umbrella crate's
  aggregating `LoadbalancerError` (wraps `EgressError`/`IngressError`/
  `AutoscaleError`/`RegistryError`), not a separate crate
- `main/src/saf/**` (the `LoadbalancerSvc` facade) → stays in the umbrella
  `swe-edge-loadbalancer` crate, retargeted to call into the five new crates
- `tests/*.rs` (19 files) split by which subdomain each file exercises

## What does not change

- The umbrella crate's name (`swe-edge-loadbalancer`) and its public API shape —
  `LoadbalancerSvc`'s ten associated methods keep their current signatures
- Trait/type shapes themselves — this is a structural move, not a contract redesign
- `api/`/`core/`/`saf/` layering *within* each new crate — unchanged, per Decision §2

## Relationship guidance applied

[`edge-a2ac`'s `scm/README.md`](https://github.com/sweengineeringlabs/edge-a2ac/blob/main/scm/README.md)
documents a "no island" constraint for its own ports split (every package needs a real,
compiler-enforced connection to at least one other, not merely a documented one) and
three concrete Rust patterns for expressing such a connection, chosen deliberately per
case rather than defaulting to one:

1. **Supertrait bound (IS-A)** — reserved for genuine "this type is a specialization
   of that trait" relationships.
2. **Trait-object field/return (USES-A)** — a `&dyn Trait` or `Arc<dyn Trait>`,
   chosen *over* a supertrait bound when the relationship is "needs a collaborator,"
   to keep each trait single-responsibility.
3. **Value-type composition** — a struct/method holds or takes another crate's own
   value type by field/parameter, not a locally-redeclared equivalent shape.

Checked this crate's planned cross-crate edges against that taxonomy — all are already
real, compiler-enforced connections, no island, and no case where a supertrait bound
would have been used but wasn't (i.e. no missed IS-A relationship):

| Edge | Pattern | Real code |
|---|---|---|
| `-autoscale` → `-tenant` | Value-type composition | `ScalingExecutor::scale_up(&self, handler_id: &HandlerId, tenant_id: Option<&TenantId>, ...)`, `HandlerInstancePool.tenant_id: Option<TenantId>` |
| `-ingress` → `-tenant` | Value-type composition | `IngressLoadBalancer::on_accept(&self, tenant_id: Option<&TenantId>)` |
| `-registry` → `-tenant` | Value-type composition | `TenantRegistry::tier_of(&self, tenant_id: &TenantId)`, `PoolRegistry::get(..., tenant_id: Option<&TenantId>)` |
| `-registry` → `-autoscale` (trait) | Trait-object return (USES-A, not IS-A — `InMemoryPoolRegistry` is not itself an `InstancePool`) | `PoolRegistry::get(...) -> Option<Arc<dyn InstancePool>>` |
| `-registry` → `-autoscale` (type) | Value-type composition | `PoolRegistry::get(&self, handler_id: &HandlerId, ...)` |
| umbrella → all five | Composition root (same role as `a2ac-handshake-port` in the precedent) | `LoadbalancerSvc`'s ten associated methods |

No supertrait bound appears in this design, and that's correct, not an oversight —
none of the five subdomains' traits are a genuine specialization of another
(`InMemoryPoolRegistry` *uses* an `InstancePool`, it isn't one).

One aspirational mismatch caught while checking: `ScalingSignal`'s own doc comment
claims it's "implemented by instance pools (in-process side, ADR-013) **and backend
pools (egress side, ADR-011)**" — but grepping the actual code, only
`HandlerInstancePool` (`autoscale`) implements it today; `BackendPoolInstance`
(`egress`) does not. Same class of trap as the original `-common` mistake (doc/comment
claims vs. real code) — verified `ScalingSignal` is autoscale-exclusive in the current
codebase, so no edge from `-egress` to `-autoscale` is needed *now*. If
`BackendPoolInstance` ever gains that impl, `-egress` would need to depend on
`-autoscale` for the trait at that point — noted here so it isn't a surprise later.

Per the precedent's own methodology note ("the real table supersedes that issue's
pre-implementation table where the two differ"): the table above is pre-implementation,
same as `edge-a2ac`'s issue #3 was before its ports were actually built. Expect it to
need correction once #5–#10 land — re-verify against real `Cargo.toml`
`[dependencies]` and real trait signatures at that point, the same way this ADR
verified against the *current* single-crate code rather than trusting doc comments.

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
