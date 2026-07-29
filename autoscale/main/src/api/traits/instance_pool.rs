//! `InstancePool` trait — concurrency gate for in-process handler execution.

use crate::api::types::identity::HandlerId;
use crate::api::types::outcome::Outcome;

/// A concurrency-gated pool for one `(handler, tenant)` pair.
///
/// Domain handlers are stateless shared values, so in-process "scaling" means
/// adjusting how many requests may execute a handler concurrently — not
/// spawning instances (ADR-013). Dispatchers call [`select`](Self::select)
/// before executing a handler and [`report_outcome`](Self::report_outcome)
/// after it completes; scaling executors adjust the cap at runtime.
pub trait InstancePool: Send + Sync {
    /// Acquire a concurrency slot.
    ///
    /// Returns the pool's handler id when a slot was acquired, or `None`
    /// when the pool is at its concurrency cap — dispatchers map `None` to
    /// `429 Too Many Requests` / `RESOURCE_EXHAUSTED`.
    fn select(&self) -> Option<HandlerId>;

    /// Release the slot acquired by [`select`](Self::select) and record the
    /// request outcome for error-rate tracking.
    fn report_outcome(&self, id: &HandlerId, outcome: Outcome);

    /// Number of requests currently holding a slot.
    fn active_count(&self) -> usize;

    /// Current maximum number of concurrent slots.
    fn concurrency_cap(&self) -> usize;

    /// Adjust the concurrency cap (interior mutability — takes `&self`).
    ///
    /// Requests already holding slots are unaffected; a lowered cap only
    /// stops new acquisitions above it.
    fn set_concurrency_cap(&self, cap: usize);
}
