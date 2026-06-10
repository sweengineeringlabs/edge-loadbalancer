//! `HandlerInstancePool` implementation — atomic slot accounting.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::api::error::LoadbalancerError;
use crate::api::traits::{InstancePool, ScalingSignal};
use crate::api::types::identity::{HandlerId, TenantId};
use crate::api::types::outcome::Outcome;
use crate::api::types::pool::HandlerInstancePool;
use crate::api::types::scaling::PoolSnapshot;

impl HandlerInstancePool {
    /// Construct a pool gating `concurrency_cap` concurrent executions of
    /// the given handler.
    ///
    /// # Errors
    ///
    /// Returns `LoadbalancerError::InvalidConfig` when `concurrency_cap` is 0.
    pub(crate) fn build(
        handler_id: HandlerId,
        tenant_id: Option<TenantId>,
        concurrency_cap: usize,
    ) -> Result<Self, LoadbalancerError> {
        if concurrency_cap == 0 {
            return Err(LoadbalancerError::InvalidConfig(
                "instance pool concurrency_cap must be > 0".to_string(),
            ));
        }
        Ok(Self {
            handler_id,
            tenant_id,
            active: AtomicUsize::new(0),
            cap: AtomicUsize::new(concurrency_cap),
            total_outcomes: AtomicU64::new(0),
            failed_outcomes: AtomicU64::new(0),
        })
    }
}

impl InstancePool for HandlerInstancePool {
    fn select(&self) -> Option<HandlerId> {
        let cap = self.cap.load(Ordering::Acquire);
        let mut current = self.active.load(Ordering::Acquire);
        loop {
            if current >= cap {
                return None;
            }
            match self.active.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(self.handler_id.clone()),
                Err(observed) => current = observed,
            }
        }
    }

    fn report_outcome(&self, id: &HandlerId, outcome: Outcome) {
        if id != &self.handler_id {
            return;
        }
        // Saturating release — a stray report without a matching select must
        // not wrap the counter.
        let _ = self
            .active
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_sub(1));
        self.total_outcomes.fetch_add(1, Ordering::Relaxed);
        if !matches!(outcome, Outcome::Success) {
            self.failed_outcomes.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn active_count(&self) -> usize {
        self.active.load(Ordering::Acquire)
    }

    fn concurrency_cap(&self) -> usize {
        self.cap.load(Ordering::Acquire)
    }

    fn set_concurrency_cap(&self, cap: usize) {
        self.cap.store(cap, Ordering::Release);
    }
}

impl ScalingSignal for HandlerInstancePool {
    fn snapshot(&self) -> PoolSnapshot {
        let active = self.active.load(Ordering::Acquire);
        let cap = self.cap.load(Ordering::Acquire);
        let total = self.total_outcomes.load(Ordering::Relaxed);
        let failed = self.failed_outcomes.load(Ordering::Relaxed);
        PoolSnapshot {
            handler_id: self.handler_id.clone(),
            tenant_id: self.tenant_id.clone(),
            active_instances: active,
            // This pool rejects at the cap instead of queueing.
            queue_depth: 0,
            // No duration data flows through Outcome — latency is populated
            // by runtime-side samplers, not by slot-counting pools.
            latency_p99_ms: 0.0,
            error_rate: if total == 0 { 0.0 } else { failed as f64 / total as f64 },
            saturation: if cap == 0 { 1.0 } else { active as f64 / cap as f64 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool_with_cap(cap: usize) -> HandlerInstancePool {
        HandlerInstancePool::build(HandlerId::new("payment"), None, cap).unwrap()
    }

    #[test]
    fn test_build_zero_cap_returns_invalid_config_error() {
        let err = HandlerInstancePool::build(HandlerId::new("payment"), None, 0).unwrap_err();
        assert!(matches!(err, LoadbalancerError::InvalidConfig(_)));
    }

    #[test]
    fn test_select_under_cap_returns_handler_id() {
        let pool = pool_with_cap(2);
        assert_eq!(pool.select(), Some(HandlerId::new("payment")));
        assert_eq!(pool.active_count(), 1);
    }

    #[test]
    fn test_select_at_cap_returns_none() {
        let pool = pool_with_cap(1);
        assert!(pool.select().is_some());
        assert_eq!(pool.select(), None, "second select must hit the cap of 1");
    }

    #[test]
    fn test_report_outcome_releases_slot() {
        let pool = pool_with_cap(1);
        let id = pool.select().unwrap();
        assert_eq!(pool.select(), None);
        pool.report_outcome(&id, Outcome::Success);
        assert_eq!(pool.active_count(), 0);
        assert!(pool.select().is_some(), "released slot must be acquirable again");
    }

    #[test]
    fn test_report_outcome_foreign_handler_is_ignored() {
        let pool = pool_with_cap(1);
        let _ = pool.select();
        pool.report_outcome(&HandlerId::new("other"), Outcome::Success);
        assert_eq!(pool.active_count(), 1, "foreign handler report must not release the slot");
    }

    #[test]
    fn test_report_outcome_without_select_does_not_underflow() {
        let pool = pool_with_cap(1);
        pool.report_outcome(&HandlerId::new("payment"), Outcome::Success);
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn test_set_concurrency_cap_raised_cap_admits_more() {
        let pool = pool_with_cap(1);
        assert!(pool.select().is_some());
        assert_eq!(pool.select(), None);
        pool.set_concurrency_cap(2);
        assert!(pool.select().is_some(), "raised cap must admit another request");
    }

    #[test]
    fn test_set_concurrency_cap_lowered_cap_blocks_new_acquisitions() {
        let pool = pool_with_cap(2);
        assert!(pool.select().is_some());
        pool.set_concurrency_cap(1);
        assert_eq!(pool.select(), None, "active=1 at cap=1 must reject");
        assert_eq!(pool.active_count(), 1, "in-flight request keeps its slot");
    }

    #[test]
    fn test_snapshot_reflects_active_errors_and_saturation() {
        let pool = pool_with_cap(4);
        let id = pool.select().unwrap();
        let _ = pool.select();
        pool.report_outcome(&id, Outcome::Failure { reason: "boom".to_string() });
        let snap = pool.snapshot();
        assert_eq!(snap.active_instances, 1);
        assert_eq!(snap.error_rate, 1.0, "1 failure of 1 outcome");
        assert_eq!(snap.saturation, 0.25);
        assert_eq!(snap.handler_id, HandlerId::new("payment"));
        assert_eq!(snap.tenant_id, None);
    }
}
