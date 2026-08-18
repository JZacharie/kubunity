use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::domain::ClusterProfile;

#[derive(Parser, Debug)]
#[command(
    name = "kubunity",
    author = "JZacharie <contact@kubunity.io>",
    version = "0.2.0",
    about = "Kubunity : Unified & Declarative Kubernetes Platform Engine & CLI in Rust",
    long_about = "Kubunity unifies multi-cluster fleets with declarative CNCF governance, OpenTelemetry observability, Kyverno policy enforcement, and developer Golden Paths."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the Kubunity Helm chart directory (default: ./charts/kubunity)
    #[arg(short, long, global = true, default_value = "./charts/kubunity")]
    pub chart: PathBuf,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download and build CNCF Helm chart dependencies
    Build,

    /// Lint the Helm chart against standard profiles
    Lint {
        /// Optional profile to lint against (dev, cloud, edge)
        #[arg(short, long)]
        profile: Option<ClusterProfile>,
    },

    /// Render and validate Helm templates
    Template {
        /// Target profile to render (dev, cloud, edge)
        #[arg(short, long, default_value = "dev")]
        profile: ClusterProfile,
    },

    /// Run NSA/CISA and PSS compliance security audit on rendered manifests
    Audit {
        /// Target profile to audit (dev, cloud, edge)
        #[arg(short, long, default_value = "cloud")]
        profile: ClusterProfile,

        /// Cluster identifier
        #[arg(short = 'n', long, default_value = "kubunity-cluster")]
        cluster_name: String,
    },

    /// Deploy/Upgrade the Kubunity stack onto the active Kubernetes cluster
    Install {
        /// Target profile (dev, cloud, edge)
        #[arg(short, long, default_value = "dev")]
        profile: ClusterProfile,

        /// Target namespace (default: kubunity-system)
        #[arg(short, long, default_value = "kubunity-system")]
        namespace: String,
    },

    /// Check status of Kubunity workloads and policies on the cluster
    Status {
        /// Namespace to query (default: kubunity-system)
        #[arg(short, long, default_value = "kubunity-system")]
        namespace: String,
    },
}
