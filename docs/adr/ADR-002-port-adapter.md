# ADR-002: Port/Adapter

**Status:** Deferred
**Date:** 2026-07-30
**Precedent:** [`edge-transport-grpc-ingress`'s ADR-004](https://github.com/sweengineeringlabs/edge-transport-grpc-ingress/blob/main/scm/docs/adr/ADR-004-ports-adapters-discipline.md)
(port/adapter workspace split) and its HTTP-ingress sibling. Same organization, same
underlying motivation — a crate bundling more than one consumer-relevant concern into a
single compilation unit — but **not mirrored in the same shape here**; see Critical
Finding below for why.
**Depends on:** [ADR-001](ADR-001-subdomain-crate-split.md) — the five crates this ADR
evaluates (`swe-edge-loadbalancer-{egress,autoscale,ingress,registry}`, plus `-tenant`,
which is out of scope here — see Problem) only exist because ADR-001 shipped first.

## Summary

Evaluate physically splitting each subdomain crate produced by ADR-001 into a `port`
crate (traits/DTOs) and an `adapter` crate (implementation), matching the precedent's
shape. Deferred: the blocker is crate-wide, not a single entangled signature like the
precedent's, and there is no real consumer to validate a redesign against.

## Problem

Each of `swe-edge-loadbalancer-{egress,autoscale,ingress,registry}` still mixes port
(`api/` — trait declarations, DTOs) and adapter (`core/`, `saf/` — implementation) in
one crate, per ADR-001 Decision §2's explicit deferral. A consumer that only needs the
trait/DTO surface of one of these crates today pulls in its full implementation
regardless. (`swe-edge-loadbalancer-tenant` is out of scope for this ADR — it is
`TenantId` only, with no adapter-side implementation to separate from.)

As of this ADR, per `edge-loadbalancer#1`/`edge#407`, no crate in the `edge` ecosystem
consumes any of these crates at all — evaluating this now is proactive, not a response
to a live consumer pulling in more than it needs.

## Critical finding — why the precedent doesn't transfer directly

Before proposing a shape, the same diligence ADR-004 applied was repeated here: check
whether each crate's `api/` port-trait/type signatures name a concrete `core/`-only
type, which would block a physical two-crate split.

ADR-004's blocker was narrow: one DTO field (`ReportRequest.service: &HealthService`)
named a concrete adapter type, fixed with a small interface-segregation change (one new
trait, one field-type swap).

The equivalent problem here is structural and spans four of the five crates, not a
single field. **Each of these primary domain types has its struct declared in `api/`
but its inherent `impl` block — the actual constructor and logic — written in `core/`,
within its own crate:**

| Type | Crate | Struct in | Inherent `impl` in |
|---|---|---|---|
| `BackendPoolInstance` | `-egress` | `api/types/pool/` | `core/pool/` |
| `HandlerInstancePool` | `-autoscale` | `api/types/pool/` | `core/pool/` |
| `NoopIngressLoadBalancer` | `-ingress` | `api/types/ingress/` | `core/ingress/` |
| `InMemoryPoolRegistry` | `-registry` | `api/types/registry/` | `core/registry/` |
| `TomlTenantRegistry` | `-registry` | `api/types/registry/` | `core/registry/` |

This is legal today only because each crate's `api/` and `core/` are the same
compilation unit. Rust's inherent-impl rule (E0116) requires the impl to live in the
same crate as the type's declaration, independent of what the impl body references —
so splitting a crate's `api/` and `core/` in two would break that type's
`impl TypeName { ... }` block immediately, regardless of ADR-004-style redesign of any
single field.

Verified both Rust-legal ways out are bigger than ADR-004's fix, not smaller:

1. **Move the impl block into port.** Checked concretely on `BackendPoolInstance::build()`:
   it directly constructs `PoolInner { entries: Arc::new(RwLock::new(...)), ... }`, a
   concrete adapter-only struct, inline. Moving `build()` to port just relocates the
   same entanglement one level down — it isn't a fix.
2. **Move the struct declaration into adapter**, and have port expose only
   `Arc<dyn BackendPool>`/`Arc<dyn InstancePool>`/etc. This is the more idiomatic
   ports-and-adapters shape, but it changes that crate's public API surface — a
   concrete named return type becomes a trait object — which is a real breaking
   redesign, not a mechanical move, and affects every existing call site.

Neither is a same-day fix like ADR-004's. Doing this for four crates, with no real
consumer to validate the resulting API shape against, is speculative cost with no
near-term payoff.

## Decision

Defer, per-crate, until a real consumer exists. Revisit independently once:

- a real consumer in `edge` actually depends on one of these crates, and
- that consumer's own needs (port-only vs. full implementation) are known — informing
  which of the two Rust-legal redesigns (impl-in-port vs. struct-in-adapter) is
  actually worth its cost for that specific crate.

`swe-edge-loadbalancer-ingress` may turn out not to need it at all — its only concrete
type (`NoopIngressLoadBalancer`) is far simpler than `-egress`'s `BackendPoolInstance`;
worth re-checking for the E0116 pattern per-crate before assuming every one hits the
same wall. Tracked as four independent sub-decisions, one per crate:
`edge-loadbalancer#12` (`-egress`), `#13` (`-autoscale`), `#14` (`-ingress`),
`#15` (`-registry`) — each with its own trigger-condition checklist.

### Target physical layout, once picked up

Two standalone workspaces per crate, matching `edge-transport-grpc-ingress`'s ADR-004
exactly — no `Cargo.toml` at the shared parent level, each side gets its own nested
`Cargo.toml` with its own empty `[workspace]` table:

```
scm/
└── main/
    ├── port/
    │   └── <subdomain>/          own standalone workspace
    │       └── Cargo.toml        [package] swe-edge-loadbalancer-<subdomain>
    └── adapter/
        └── <subdomain>/          own standalone workspace, separate from port's
            └── Cargo.toml        [package] swe-edge-loadbalancer-<subdomain>-adapter
```

This repo does not have an `scm/` top-level layer today (its layout is flat — each
crate's own `main/src/...` at its own root), unlike the sibling repos this pattern is
drawn from (`security/scm/`, `edge-a2ac/scm/`, `edge-transport-grpc-ingress/scm/`).
Adopting this layout for any crate's port/adapter split means introducing `scm/` as
this repo's top-level layer at that point, to stay consistent with the rest of the org
rather than inventing a one-off shape. Recording the target shape now, in advance of
any of #12–#15 actually being picked up, so whoever implements it later doesn't have
to re-derive it — this does not change the deferral decision above.

## Cascade position

Local to this repo. Zero external consumers to audit today (confirmed via
`edge-loadbalancer#1`/`edge#407`) — unlike the precedent ADR, there is no
`swe-edge-runtime-grpc`-equivalent to notify before release. ADR-001's anticipated
future consumers (`ingress/http`, `ingress/grpc`, `proxy`, `edge-runtime`) should
consume the relevant subdomain crate as-is initially, regardless of this ADR, until/
unless one of them specifically wants port-only access.
