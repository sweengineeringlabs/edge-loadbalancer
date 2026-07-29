//! `Backend` — a single upstream backend in the pool.

use serde::{Deserialize, Serialize};

use crate::api::types::backend::backend_health::BackendHealth;
use crate::api::types::backend::backend_id::BackendId;

/// A single upstream backend tracked by the pool.
///
/// # Examples
///
/// ```rust
/// use swe_edge_loadbalancer_egress::{Backend, BackendHealth, BackendId};
///
/// let backend = Backend {
///     id: BackendId::new("https://api-1.internal"),
///     url: "https://api-1.internal".to_string(),
///     weight: 1,
///     health: BackendHealth::Healthy,
/// };
/// assert_eq!(backend.weight, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backend {
    /// Unique identifier for this backend.
    pub id: BackendId,
    /// Base URL of the backend (e.g. `"https://api-1.internal"`).
    pub url: String,
    /// Relative weight used by the `Weighted` strategy.
    /// Higher weight means proportionally more traffic.
    pub weight: u32,
    /// Current observed health of the backend.
    pub health: BackendHealth,
}
