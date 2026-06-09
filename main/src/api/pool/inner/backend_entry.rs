//! Interface contract for backend entry types.
//!
//! A backend entry associates a [`Backend`] with an atomic in-flight
//! connection counter so that `LeastConnections` selection can rank entries
//! without taking a write-lock.
//!
//! [`Backend`]: crate::api::types::backend::Backend

/// Contract for types that track a single backend's in-flight connection
/// count alongside its backend metadata.
///
/// `core/pool/inner/BackendEntry` is the default implementation.
pub trait BackendEntry: Send + Sync {
    /// Returns the current in-flight connection count.
    fn connection_count(&self) -> u32;
}
