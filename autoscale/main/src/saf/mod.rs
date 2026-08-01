//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `AutoscaleSvc`. Extension-point traits (implementors in downstream
//! crates) are re-exported so consumers can hold `Arc<dyn Trait>` or provide
//! their own implementations. Core types are NOT re-exported directly (SEA
//! Rule 47).

mod autoscale_svc;

// Public traits — re-exported for downstream dispatchers and executors
// that hold `Arc<dyn Trait>` or implement the trait themselves.
pub use crate::api::traits::InstancePool;
pub use crate::api::traits::ScalingExecutor;
pub use crate::api::traits::ScalingSignal;

// Public types re-exported from api/
pub use crate::api::error::AutoscaleError;
pub use crate::api::types::autoscale_svc::AutoscaleSvc;
pub use crate::api::types::identity::HandlerId;
pub use crate::api::types::outcome::Outcome;
pub use crate::api::types::pool::HandlerInstancePool;
pub use crate::api::types::scaling::{PoolSnapshot, ScaleOutHint, ScalingDecision};
