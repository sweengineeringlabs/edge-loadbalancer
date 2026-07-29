//! Interface layer for pool internal types.
//!
//! `core/pool/inner/` implements against the contracts defined here.
//! [`BackendEntry`] and [`PoolInner`] define the behavioural contracts that
//! core implementations must satisfy.
//!
//! [`BackendEntry`]: backend_entry::BackendEntry
//! [`PoolInner`]: pool_inner::PoolInner

pub mod backend_entry;
pub mod pool_inner;
