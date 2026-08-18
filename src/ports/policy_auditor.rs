use crate::domain::{ClusterProfile, ComplianceReport};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PolicyAuditor: Send + Sync {
    /// Audits raw rendered manifests against NSA/CISA and PSS rules.
    async fn audit_manifests(
        &self,
        cluster_name: &str,
        profile: ClusterProfile,
        manifests: &str,
    ) -> Result<ComplianceReport>;
}
