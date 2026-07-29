//! `NoopIngressLoadBalancer` implementation — always admits in-process.

use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::traits::IngressLoadBalancer;
use crate::api::types::ingress::{LoadBalancerHint, NoopIngressLoadBalancer};

impl IngressLoadBalancer for NoopIngressLoadBalancer {
    fn on_accept(&self, _tenant_id: Option<&TenantId>) -> LoadBalancerHint {
        LoadBalancerHint::UseInProcess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_accept_no_tenant_returns_use_in_process() {
        let lb = NoopIngressLoadBalancer::default();
        assert!(matches!(lb.on_accept(None), LoadBalancerHint::UseInProcess));
    }

    #[test]
    fn test_on_accept_with_tenant_returns_use_in_process() {
        let lb = NoopIngressLoadBalancer::default();
        let tid = TenantId::new("acme");
        assert!(matches!(lb.on_accept(Some(&tid)), LoadBalancerHint::UseInProcess));
    }
}
