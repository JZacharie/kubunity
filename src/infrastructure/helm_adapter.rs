use async_trait::async_trait;
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};
use crate::domain::ClusterProfile;
use crate::ports::{ClusterDriver, TemplateEngine};

pub struct HelmCliAdapter;

impl HelmCliAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HelmCliAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemplateEngine for HelmCliAdapter {
    async fn render_profile(&self, chart_path: &Path, profile: ClusterProfile) -> Result<String> {
        let profile_file = chart_path.join("profiles").join(format!("values-{}.yaml", profile));

        let mut cmd = Command::new("helm");
        cmd.arg("template")
            .arg("kubunity-release")
            .arg(chart_path);

        if profile_file.exists() {
            cmd.arg("-f").arg(&profile_file);
        }

        let output = cmd.output().context("Failed to execute helm template command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Helm template rendering failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }

    async fn lint_chart(&self, chart_path: &Path, profile: Option<ClusterProfile>) -> Result<bool> {
        let mut cmd = Command::new("helm");
        cmd.arg("lint").arg(chart_path);

        if let Some(prof) = profile {
            let profile_file = chart_path.join("profiles").join(format!("values-{}.yaml", prof));
            if profile_file.exists() {
                cmd.arg("-f").arg(&profile_file);
            }
        }

        let output = cmd.output().context("Failed to execute helm lint command")?;
        Ok(output.status.success())
    }

    async fn build_dependencies(&self, chart_path: &Path) -> Result<()> {
        let output = Command::new("helm")
            .arg("dependency")
            .arg("build")
            .arg(chart_path)
            .output()
            .context("Failed to execute helm dependency build")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Helm dependency build failed: {}", stderr);
        }

        Ok(())
    }
}

#[async_trait]
impl ClusterDriver for HelmCliAdapter {
    async fn deploy_stack(&self, chart_path: &Path, profile: ClusterProfile, namespace: &str) -> Result<()> {
        let profile_file = chart_path.join("profiles").join(format!("values-{}.yaml", profile));

        let mut cmd = Command::new("helm");
        cmd.arg("upgrade")
            .arg("--install")
            .arg("kubunity")
            .arg(chart_path)
            .arg("--namespace")
            .arg(namespace)
            .arg("--create-namespace");

        if profile_file.exists() {
            cmd.arg("-f").arg(&profile_file);
        }

        let output = cmd.output().context("Failed to execute helm upgrade --install")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Helm deployment failed: {}", stderr);
        }

        Ok(())
    }

    async fn get_status(&self, namespace: &str) -> Result<String> {
        let output = Command::new("kubectl")
            .arg("get")
            .arg("all,clusterpolicies,externalsecrets,ciliumnetworkpolicies")
            .arg("-n")
            .arg(namespace)
            .output()
            .context("Failed to execute kubectl get status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Ok(format!("Status check completed with warning: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}
