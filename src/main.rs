use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use kubunity::application::{AuditService, BootstrapService};
use kubunity::cli::{Cli, Commands, TerminalReporter};
use kubunity::infrastructure::{HelmCliAdapter, KyvernoPolicyAuditor};
use kubunity::ports::TemplateEngine;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Configure logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Dependency Injection (Hexagonal Architecture)
    let helm_adapter = Arc::new(HelmCliAdapter::new());
    let kyverno_auditor = Arc::new(KyvernoPolicyAuditor::new());

    let audit_service = AuditService::new(helm_adapter.clone(), kyverno_auditor);
    let bootstrap_service = BootstrapService::new(helm_adapter.clone(), helm_adapter.clone());

    match cli.command {
        Commands::Build => {
            TerminalReporter::print_banner();
            println!("==> Downloading and building CNCF Helm dependencies...");
            helm_adapter.build_dependencies(&cli.chart).await?;
            println!("✓ Dependencies successfully built.");
        }
        Commands::Lint { profile } => {
            TerminalReporter::print_banner();
            println!("==> Linting chart at {:?} with profile {:?}", cli.chart, profile);
            let valid = helm_adapter.lint_chart(&cli.chart, profile).await?;
            if valid {
                println!("✓ Helm chart passed lint checks.");
            } else {
                eprintln!("❌ Helm lint failed.");
                std::process::exit(1);
            }
        }
        Commands::Template { profile } => {
            let output = helm_adapter.render_profile(&cli.chart, profile).await?;
            println!("{}", output);
        }
        Commands::Audit { profile, cluster_name } => {
            TerminalReporter::print_banner();
            let report = audit_service.run_audit(&cli.chart, &cluster_name, profile).await?;
            TerminalReporter::render_compliance_report(&report);

            if report.fail_count > 0 {
                std::process::exit(1);
            }
        }
        Commands::Install { profile, namespace } => {
            TerminalReporter::print_banner();
            println!("==> Deploying Kubunity [profile: {}] into namespace [{}]...", profile, namespace);
            bootstrap_service.bootstrap_cluster(&cli.chart, profile, &namespace).await?;
            println!("✓ Kubunity stack deployed successfully.");
        }
        Commands::Status { namespace } => {
            TerminalReporter::print_banner();
            let status = bootstrap_service.get_cluster_status(&namespace).await?;
            println!("{}", status);
        }
    }

    Ok(())
}
