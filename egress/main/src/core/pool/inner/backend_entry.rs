//! `BackendEntry` — internal per-backend entry type.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::api::pool::inner::backend_entry::BackendEntry as BackendEntryContract;
use crate::api::types::backend::Backend;

/// Internal per-backend entry holding backend data and in-flight connection count.
pub(crate) struct BackendEntry {
    pub(crate) backend: Backend,
    pub(crate) connections: AtomicU32,
}

impl BackendEntryContract for BackendEntry {
    fn connection_count(&self) -> u32 {
        self.connections.load(Ordering::Relaxed)
    }
}
