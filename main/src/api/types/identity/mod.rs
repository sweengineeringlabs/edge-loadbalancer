//! Identity newtypes shared across ingress, pool, and scaling contracts.

pub(crate) mod handler_id;
pub(crate) mod node_id;
pub(crate) mod tenant_id;

pub use handler_id::HandlerId;
pub use node_id::NodeId;
pub use tenant_id::TenantId;
