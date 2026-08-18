use crate::domain::ClusterProfile;
use crate::ports::{ClusterDriver, TemplateEngine};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;

pub struct BootstrapService {
    template_engine: Arc<dyn TemplateEngine>,
    cluster_driver: Arc<dyn ClusterDriver>,
}

impl BootstrapService {
    pub fn new(
        template_engine: Arc<dyn TemplateEngine>,
        cluster_driver: Arc<dyn ClusterDriver>,
    ) -> Self {
        Self {
            template_engine,
            cluster_driver,
        }
    }

    pub async fn bootstrap_cluster(
        &self,
        chart_path: &Path,
        profile: ClusterProfile,
        namespace: &str,
    ) -> Result<()> {
        // 1. Build dependencies first
        self.template_engine
            .build_dependencies(chart_path)
            .await
            .context("Failed to build Helm chart dependencies")?;

        // 2. Lint before applying
        let is_valid = self
            .template_engine
            .lint_chart(chart_path, Some(profile))
            .await
            .context("Failed to lint Helm chart")?;

        if !is_valid {
            anyhow::bail!("Chart linting failed for profile: {}", profile);
        }

        // 3. Deploy stack to cluster
        self.cluster_driver
            .deploy_stack(chart_path, profile, namespace)
            .await
            .context("Failed to deploy Kubunity stack to Kubernetes cluster")?;

        Ok(())
    }

    pub async fn get_cluster_status(&self, namespace: &str) -> Result<String> {
        self.cluster_driver
            .get_status(namespace)
            .await
            .context("Failed to retrieve cluster status")
    }
}
