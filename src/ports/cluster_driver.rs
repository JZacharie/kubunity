use crate::domain::ClusterProfile;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait ClusterDriver: Send + Sync {
    /// Deploys the Kubunity stack onto the active cluster with a specific profile.
    async fn deploy_stack(
        &self,
        chart_path: &Path,
        profile: ClusterProfile,
        namespace: &str,
    ) -> Result<()>;

    /// Gets health and status of Kubunity resources in the cluster.
    async fn get_status(&self, namespace: &str) -> Result<String>;
}
