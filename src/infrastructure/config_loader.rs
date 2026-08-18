use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::domain::ClusterMetadata;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_cluster_metadata(values_path: &Path) -> Result<ClusterMetadata> {
        let content = fs::read_to_string(values_path)
            .with_context(|| format!("Failed to read configuration file at {:?}", values_path))?;

        let parsed: serde_yaml::Value = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML content from {:?}", values_path))?;

        let cluster_section = parsed.get("cluster");
        if let Some(c) = cluster_section {
            let metadata: ClusterMetadata = serde_yaml::from_value(c.clone())
                .unwrap_or_default();
            Ok(metadata)
        } else {
            Ok(ClusterMetadata::default())
        }
    }
}
