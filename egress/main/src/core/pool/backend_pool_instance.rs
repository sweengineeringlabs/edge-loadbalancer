//! `BackendPoolInstance` implementation — strategy-based backend selection logic.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::error::EgressError;
use crate::api::pool::inner::backend_entry::BackendEntry as BackendEntryContract;
use crate::api::pool::PoolStrategy as Strategy;
use crate::api::traits::{BackendPool, Validator};
use crate::api::types::backend::{Backend, BackendHealth, BackendId};
use crate::api::types::config::LoadbalancerConfig;
use crate::api::types::outcome::Outcome;
use crate::api::types::pool::BackendPoolInstance;
use crate::core::pool::inner::{BackendEntry, PoolInner};

impl BackendPoolInstance {
    /// Construct a pool from the given config.
    ///
    /// # Errors
    ///
    /// Returns `EgressError::InvalidConfig` when the config fails validation.
    pub fn build(config: LoadbalancerConfig) -> Result<Self, EgressError> {
        Self::validate(&config).map_err(EgressError::InvalidConfig)?;
        let strategy = config.strategy;
        let entries: Vec<BackendEntry> = config
            .backends
            .into_iter()
            .map(|bc| BackendEntry {
                backend: Backend {
                    id: BackendId::new(bc.url.clone()),
                    url: bc.url,
                    weight: bc.weight,
                    health: BackendHealth::Healthy,
                },
                connections: AtomicU32::new(0),
            })
            .collect();
        let pool_inner = PoolInner {
            entries: Arc::new(RwLock::new(entries)),
            strategy,
            rr_counter: Arc::new(AtomicU32::new(0)),
        };
        Ok(Self {
            state: Arc::new(pool_inner),
            strategy,
        })
    }

    /// Returns the number of backends tracked by this pool.
    ///
    /// Delegates to [`crate::api::pool::inner::pool_inner::PoolInner::backend_count`].
    pub fn backend_count(&self) -> usize {
        self.state.backend_count()
    }

    /// Downcast helper — extracts `&PoolInner` from the opaque inner state.
    fn pool_inner(pool: &BackendPoolInstance) -> Option<&PoolInner> {
        pool.state.as_any().downcast_ref::<PoolInner>()
    }

    fn healthy_indices(entries: &[BackendEntry]) -> Vec<usize> {
        entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.backend.health == BackendHealth::Healthy)
            .map(|(i, _)| i)
            .collect()
    }
}

impl BackendPool for BackendPoolInstance {
    fn select(&self) -> Result<Backend, EgressError> {
        let pi = Self::pool_inner(self).ok_or(EgressError::NoHealthyBackends)?;
        let entries = pi.entries.read();
        let healthy = Self::healthy_indices(&entries);
        if healthy.is_empty() {
            return Err(EgressError::NoHealthyBackends);
        }

        let idx = match pi.strategy {
            Strategy::RoundRobin => {
                let pos = pi.rr_counter.fetch_add(1, Ordering::Relaxed) as usize;
                healthy[pos % healthy.len()]
            }
            Strategy::Weighted => {
                let total_weight: u32 = healthy.iter().map(|&i| entries[i].backend.weight).sum();
                if total_weight == 0 {
                    return Err(EgressError::NoHealthyBackends);
                }
                let pos = pi.rr_counter.fetch_add(1, Ordering::Relaxed) as u64;
                let mut target = (pos % total_weight as u64) as u32;
                let mut chosen = healthy[0];
                for &i in &healthy {
                    let w = entries[i].backend.weight;
                    if target < w {
                        chosen = i;
                        break;
                    }
                    target -= w;
                }
                chosen
            }
            Strategy::LeastConnections => {
                healthy
                    .iter()
                    .copied()
                    .min_by_key(|&i| entries[i].connection_count())
                    .unwrap_or(healthy[0])
            }
        };

        Ok(entries[idx].backend.clone())
    }

    fn report_outcome(&self, id: &BackendId, outcome: Outcome) {
        let Some(pi) = Self::pool_inner(self) else {
            return;
        };
        let mut entries = pi.entries.write();
        for entry in entries.iter_mut() {
            if &entry.backend.id == id {
                entry.backend.health = match outcome {
                    Outcome::Success => BackendHealth::Healthy,
                    Outcome::Failure { .. } | Outcome::CircuitOpen => BackendHealth::Degraded,
                };
                return;
            }
        }
    }
}

impl Validator for BackendPoolInstance {
    fn validate(config: &LoadbalancerConfig) -> Result<(), String> {
        if config.backends.is_empty() {
            return Err("loadbalancer config must have at least one backend".to_string());
        }
        for (i, b) in config.backends.iter().enumerate() {
            if b.url.is_empty() {
                return Err(format!("backend[{i}].url must not be empty"));
            }
            if b.weight == 0 {
                return Err(format!("backend[{i}].weight must be > 0"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::config::BackendConfig;

    fn two_backend_config() -> LoadbalancerConfig {
        LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![
                BackendConfig { url: "https://api-1.internal".to_string(), weight: 1 },
                BackendConfig { url: "https://api-2.internal".to_string(), weight: 1 },
            ],
        }
    }

    #[test]
    fn test_build_valid_config_returns_pool() {
        let pool = BackendPoolInstance::build(two_backend_config());
        assert!(pool.is_ok(), "valid config must build successfully");
    }

    #[test]
    fn test_build_empty_backends_returns_invalid_config_error() {
        let config = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![],
        };
        let err = BackendPoolInstance::build(config).unwrap_err();
        assert!(matches!(err, EgressError::InvalidConfig(_)));
    }

    #[test]
    fn test_validate_valid_config_returns_ok() {
        assert!(BackendPoolInstance::validate(&two_backend_config()).is_ok());
    }

    #[test]
    fn test_validate_empty_url_returns_error() {
        let config = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![BackendConfig { url: String::new(), weight: 1 }],
        };
        assert!(BackendPoolInstance::validate(&config).is_err());
    }

    #[test]
    fn test_backend_count_returns_number_of_configured_backends() {
        let pool = BackendPoolInstance::build(two_backend_config()).unwrap();
        assert_eq!(pool.backend_count(), 2, "backend_count must equal configured backend count");
    }
}
