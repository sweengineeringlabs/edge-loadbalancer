//! `PoolInner` — internal pool state held behind `Arc<dyn pool_inner::PoolInner>`.

use std::any::Any;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::pool::inner::pool_inner::PoolInner as PoolInnerTrait;
use crate::api::pool::PoolStrategy as Strategy;
use crate::core::pool::inner::backend_entry::BackendEntry;

/// Concrete internal state for `BackendPoolInstance`.
///
/// Stored behind `Arc<dyn PoolInner>` (the api trait) so that `BackendPoolInstance`
/// in `api/types/` does not need to reference any core types directly.
pub(crate) struct PoolInner {
    pub(crate) entries: Arc<RwLock<Vec<BackendEntry>>>,
    pub(crate) strategy: Strategy,
    pub(crate) rr_counter: Arc<AtomicU32>,
}

impl PoolInnerTrait for PoolInner {
    fn backend_count(&self) -> usize {
        self.entries.read().len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
