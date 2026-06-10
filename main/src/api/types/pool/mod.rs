//! Pool types.

pub(crate) mod backend_pool_instance;
pub(crate) mod handler_instance_pool;

pub use backend_pool_instance::BackendPoolInstance;
pub use handler_instance_pool::HandlerInstancePool;
