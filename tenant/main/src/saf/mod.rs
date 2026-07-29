//! SAF layer — public facade.
//!
//! Re-exports the crate's public types. `TenantId` has no core-only
//! implementation split (unlike the entangled types noted in ADR-001's
//! "Critical finding") — its struct and inherent impl live together in
//! `api/types/identity/tenant_id.rs` — so there is no `core/` layer here.

// Public types re-exported from api/
pub use crate::api::types::identity::TenantId;
