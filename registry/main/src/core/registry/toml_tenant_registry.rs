//! `TomlTenantRegistry` implementation — `[tenant.assignments]` parsing.

use std::collections::HashMap;

use serde::Deserialize;
use swe_edge_loadbalancer_tenant::TenantId;

use crate::api::error::RegistryError;
use crate::api::traits::TenantRegistry;
use crate::api::types::registry::TomlTenantRegistry;

/// Top-level document shape: an optional `[tenant]` section.
#[derive(Debug, Default, Deserialize)]
struct TenantDocument {
    #[serde(default)]
    tenant: TenantSection,
}

/// The `[tenant]` section with its flat `assignments` table.
#[derive(Debug, Default, Deserialize)]
struct TenantSection {
    #[serde(default)]
    assignments: HashMap<String, String>,
}

impl TomlTenantRegistry {
    /// Parse a TOML document containing a `[tenant.assignments]` table.
    ///
    /// A document without a `[tenant]` section yields an empty registry —
    /// every `tier_of` lookup returns `None`.
    ///
    /// # Errors
    ///
    /// Returns `RegistryError::ParseFailed` when the document is not valid
    /// TOML or the section has the wrong shape.
    pub(crate) fn build(toml_str: &str) -> Result<Self, RegistryError> {
        let doc: TenantDocument =
            toml::from_str(toml_str).map_err(|e| RegistryError::ParseFailed(e.to_string()))?;
        Ok(Self { assignments: doc.tenant.assignments })
    }
}

impl TenantRegistry for TomlTenantRegistry {
    fn tier_of(&self, tenant_id: &TenantId) -> Option<&str> {
        self.assignments.get(tenant_id.as_str()).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_assignments_table_resolves_tiers() {
        let reg = TomlTenantRegistry::build(
            "[tenant.assignments]\nacme = \"enterprise\"\nstartup = \"free\"\n",
        )
        .unwrap();
        assert_eq!(reg.tier_of(&TenantId::new("acme")), Some("enterprise"));
        assert_eq!(reg.tier_of(&TenantId::new("startup")), Some("free"));
    }

    #[test]
    fn test_tier_of_unknown_tenant_returns_none() {
        let reg = TomlTenantRegistry::build("[tenant.assignments]\nacme = \"enterprise\"\n")
            .unwrap();
        assert_eq!(reg.tier_of(&TenantId::new("ghost")), None);
    }

    #[test]
    fn test_build_missing_tenant_section_returns_empty_registry() {
        let reg = TomlTenantRegistry::build("[other]\nkey = 1\n").unwrap();
        assert_eq!(reg.tier_of(&TenantId::new("acme")), None);
    }

    #[test]
    fn test_build_malformed_toml_returns_parse_failed_error() {
        let err = TomlTenantRegistry::build("[tenant.assignments\nacme = ").unwrap_err();
        assert!(matches!(err, RegistryError::ParseFailed(_)));
    }

    #[test]
    fn test_build_wrong_shape_returns_parse_failed_error() {
        let err = TomlTenantRegistry::build("[tenant]\nassignments = 5\n").unwrap_err();
        assert!(matches!(err, RegistryError::ParseFailed(_)));
    }
}
