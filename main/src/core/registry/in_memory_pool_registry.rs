//! `InMemoryPoolRegistry` implementation — fallback-aware pool lookup.

use std::sync::Arc;

use crate::api::traits::{InstancePool, PoolRegistry};
use crate::api::types::identity::{HandlerId, TenantId};
use crate::api::types::registry::in_memory_pool_registry::PoolKey;
use crate::api::types::registry::InMemoryPoolRegistry;

impl InMemoryPoolRegistry {
    /// Create an empty registry.
    pub(crate) fn build() -> Self {
        Self::default()
    }

    /// Register a pool under `(handler_id, tenant_id)`.
    ///
    /// Registering with `tenant_id = None` creates the shared fallback pool
    /// for that handler. Registering with a `tenant_id` creates a per-tenant
    /// override.
    pub(crate) fn register(
        &self,
        handler_id: &HandlerId,
        tenant_id: Option<&TenantId>,
        pool: Arc<dyn InstancePool>,
    ) {
        let key: PoolKey = (
            handler_id.as_str().to_string(),
            tenant_id.map(|t| t.as_str().to_string()),
        );
        self.pools.write().insert(key, pool);
    }
}

impl PoolRegistry for InMemoryPoolRegistry {
    /// Resolve the pool for `(handler, tenant)`.
    ///
    /// Lookup order:
    /// 1. Exact `(handler, tenant)` key.
    /// 2. Shared `(handler, None)` fallback (only when `tenant_id` is `Some`).
    fn get(
        &self,
        handler_id: &HandlerId,
        tenant_id: Option<&TenantId>,
    ) -> Option<Arc<dyn InstancePool>> {
        let pools = self.pools.read();
        let exact: PoolKey = (
            handler_id.as_str().to_string(),
            tenant_id.map(|t| t.as_str().to_string()),
        );
        if let Some(pool) = pools.get(&exact) {
            return Some(Arc::clone(pool));
        }
        if tenant_id.is_some() {
            let shared: PoolKey = (handler_id.as_str().to_string(), None);
            if let Some(pool) = pools.get(&shared) {
                return Some(Arc::clone(pool));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::api::types::identity::HandlerId;
    use crate::api::types::outcome::Outcome;
    use crate::api::types::registry::InMemoryPoolRegistry;

    struct ConstPool(HandlerId);
    impl InstancePool for ConstPool {
        fn select(&self) -> Option<HandlerId> { Some(self.0.clone()) }
        fn report_outcome(&self, _: &HandlerId, _: Outcome) {}
        fn active_count(&self) -> usize { 0 }
        fn concurrency_cap(&self) -> usize { 1 }
        fn set_concurrency_cap(&self, _: usize) {}
    }

    fn payment() -> HandlerId { HandlerId::new("payment") }
    fn acme() -> TenantId { TenantId::new("acme") }

    #[test]
    fn test_get_exact_tenant_match_returns_pool() {
        let reg = InMemoryPoolRegistry::build();
        let pool: Arc<dyn InstancePool> = Arc::new(ConstPool(payment()));
        reg.register(&payment(), Some(&acme()), Arc::clone(&pool));
        let found = reg.get(&payment(), Some(&acme()));
        assert!(found.is_some());
    }

    #[test]
    fn test_get_falls_back_to_shared_pool_when_no_tenant_match() {
        let reg = InMemoryPoolRegistry::build();
        let shared: Arc<dyn InstancePool> = Arc::new(ConstPool(payment()));
        reg.register(&payment(), None, Arc::clone(&shared));
        // Query with a tenant that has no override
        let found = reg.get(&payment(), Some(&acme()));
        assert!(found.is_some(), "shared pool must be returned as fallback");
    }

    #[test]
    fn test_get_tenant_override_wins_over_shared() {
        let reg = InMemoryPoolRegistry::build();
        let shared: Arc<dyn InstancePool> = Arc::new(ConstPool(HandlerId::new("shared")));
        let tenant_pool: Arc<dyn InstancePool> = Arc::new(ConstPool(HandlerId::new("tenant")));
        reg.register(&payment(), None, shared);
        reg.register(&payment(), Some(&acme()), Arc::clone(&tenant_pool));
        let found = reg.get(&payment(), Some(&acme())).unwrap();
        assert_eq!(found.select(), Some(HandlerId::new("tenant")));
    }

    #[test]
    fn test_get_unregistered_handler_returns_none() {
        let reg = InMemoryPoolRegistry::build();
        assert!(reg.get(&payment(), None).is_none());
    }

    #[test]
    fn test_get_no_fallback_when_no_tenant_requested() {
        let reg = InMemoryPoolRegistry::build();
        let shared: Arc<dyn InstancePool> = Arc::new(ConstPool(payment()));
        reg.register(&payment(), None, shared);
        // Exact `(payment, None)` query must find the shared pool
        assert!(reg.get(&payment(), None).is_some());
    }
}
