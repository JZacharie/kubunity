use crate::domain::{ClusterProfile, ComplianceReport};
use crate::ports::{PolicyAuditor, TemplateEngine};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;

pub struct AuditService {
    template_engine: Arc<dyn TemplateEngine>,
    policy_auditor: Arc<dyn PolicyAuditor>,
}

impl AuditService {
    pub fn new(
        template_engine: Arc<dyn TemplateEngine>,
        policy_auditor: Arc<dyn PolicyAuditor>,
    ) -> Self {
        Self {
            template_engine,
            policy_auditor,
        }
    }

    pub async fn run_audit(
        &self,
        chart_path: &Path,
        cluster_name: &str,
        profile: ClusterProfile,
    ) -> Result<ComplianceReport> {
        let rendered_yaml = self
            .template_engine
            .render_profile(chart_path, profile)
            .await
            .context("Failed to render Helm chart templates for audit")?;

        let report = self
            .policy_auditor
            .audit_manifests(cluster_name, profile, &rendered_yaml)
            .await
            .context("Failed to audit rendered manifests")?;

        Ok(report)
    }
}
