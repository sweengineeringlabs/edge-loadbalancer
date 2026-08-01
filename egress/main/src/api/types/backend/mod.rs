//! Backend domain types.

#[allow(clippy::module_inception)]
mod backend;
mod backend_health;
mod backend_id;

pub use backend::Backend;
pub use backend_health::BackendHealth;
pub use backend_id::BackendId;
