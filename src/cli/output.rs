use colored::*;
use tabled::settings::Style;
use tabled::Table;
use crate::domain::ComplianceReport;

pub struct TerminalReporter;

impl TerminalReporter {
    pub fn print_banner() {
        println!("{}", r#"
  _  ___     _                 _ _         
 | |/ / |   | |               (_) |        
 | ' /| |___| |__  _   _ _ __  _| |_ _   _ 
 |  < | '_  | '_ \| | | | '_ \| | __| | | |
 | . \| |_) | |_) | |_| | | | | | |_| |_| |
 |_|\_\_.__/|_.__/ \__,_|_| |_|_|\__|\__, |
                                      __/ |
                                     |___/ 
  Unified & Declarative Kubernetes Platform Engine [Rust]
"#.cyan().bold());
    }

    pub fn render_compliance_report(report: &ComplianceReport) {
        println!("\n{}", "================================================================================".blue());
        println!(
            "{} {} | {} {} | {} {}",
            "Cluster:".bold(),
            report.cluster_name.yellow(),
            "Profile:".bold(),
            report.profile.green(),
            "Compliance Score:".bold(),
            if report.compliance_score >= 85.0 {
                format!("{:.1}%", report.compliance_score).green().bold()
            } else if report.compliance_score >= 60.0 {
                format!("{:.1}%", report.compliance_score).yellow().bold()
            } else {
                format!("{:.1}%", report.compliance_score).red().bold()
            }
        );
        println!("{}\n", "================================================================================".blue());

        let table = Table::new(&report.checks)
            .with(Style::rounded())
            .to_string();

        println!("{}", table);

        println!(
            "\nSummary: {} Passed | {} Warnings | {} Failed",
            format!("{}", report.pass_count).green().bold(),
            format!("{}", report.warn_count).yellow().bold(),
            format!("{}", report.fail_count).red().bold()
        );

        if report.fail_count > 0 {
            println!(
                "{}",
                "❌ Compliance check failed! Please review policies and manifests.".red().bold()
            );
        } else {
            println!(
                "{}",
                "✅ All policy and governance checks passed successfully.".green().bold()
            );
        }
    }
}
