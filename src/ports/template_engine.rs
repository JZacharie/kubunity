use async_trait::async_trait;
use std::path::Path;
use anyhow::Result;
use crate::domain::ClusterProfile;

#[async_trait]
pub trait TemplateEngine: Send + Sync {
    /// Renders templates for a given profile and returns the combined YAML string.
    async fn render_profile(&self, chart_path: &Path, profile: ClusterProfile) -> Result<String>;

    /// Lints the chart with a given profile.
    async fn lint_chart(&self, chart_path: &Path, profile: Option<ClusterProfile>) -> Result<bool>;

    /// Builds/fetches Helm chart dependencies.
    async fn build_dependencies(&self, chart_path: &Path) -> Result<()>;
}
