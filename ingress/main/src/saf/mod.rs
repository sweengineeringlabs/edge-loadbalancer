//! SAF layer — public facade.
//!
//! Re-exports the crate's public trait and types. `IngressLoadBalancer` is
//! re-exported so downstream dispatchers can hold `Arc<dyn IngressLoadBalancer>`
//! or implement the trait themselves. Core types are NOT re-exported directly
//! (SEA Rule 47) — `NoopIngressLoadBalancer`'s inherent impl lives in `core/`,
//! but the struct itself (and its public constructor, `Default`) is declared
//! in `api/`, so this re-export is of the `api/`-owned struct, not `core/`.

// Public trait — re-exported for downstream dispatchers/executors that hold
// `Arc<dyn IngressLoadBalancer>` or implement the trait themselves.
pub use crate::api::traits::IngressLoadBalancer;

// Public types re-exported from api/
pub use crate::api::error::IngressError;
pub use crate::api::types::identity::NodeId;
pub use crate::api::types::ingress::{LoadBalancerHint, NoopIngressLoadBalancer};
