use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterProfile {
    Dev,
    Cloud,
    Edge,
}

impl fmt::Display for ClusterProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dev => write!(f, "dev"),
            Self::Cloud => write!(f, "cloud"),
            Self::Edge => write!(f, "edge"),
        }
    }
}

impl std::str::FromStr for ClusterProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" | "local" => Ok(Self::Dev),
            "cloud" | "prod" | "production" => Ok(Self::Cloud),
            "edge" | "k3s" => Ok(Self::Edge),
            other => Err(format!("Unknown profile: '{other}'. Expected 'dev', 'cloud', or 'edge'.")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetadata {
    pub name: String,
    pub environment: String,
    pub profile: ClusterProfile,
    pub region: Option<String>,
}

impl Default for ClusterMetadata {
    fn default() -> Self {
        Self {
            name: "kubunity-cluster".to_string(),
            environment: "development".to_string(),
            profile: ClusterProfile::Dev,
            region: Some("eu-west-1".to_string()),
        }
    }
}
