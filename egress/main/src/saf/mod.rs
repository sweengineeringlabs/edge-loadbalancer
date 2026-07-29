//! SAF layer — public facade.
//!
//! Re-exports the crate's public traits and types. `BackendPoolInstance`'s
//! struct is declared in `api/types/pool/` with its inherent `impl` block
//! (the real constructor and selection logic) in `core/pool/` — legal only
//! because both live in this one crate; see ADR-001's "Critical finding"
//! for why a physical port/adapter split is deferred (issue #4/#12) rather
//! than attempted here. Core types are NOT re-exported directly (SEA Rule 47).

// Public traits
pub use crate::api::traits::BackendPool;
pub use crate::api::traits::Validator;

// Public types re-exported from api/
pub use crate::api::error::EgressError;
pub use crate::api::types::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::types::backend::{Backend, BackendHealth, BackendId};
pub use crate::api::types::config::{BackendConfig, LoadbalancerConfig};
pub use crate::api::types::outcome::Outcome;
pub use crate::api::types::pool::BackendPoolInstance;
pub use crate::api::types::strategy::Strategy;
