use serde::{Deserialize, Serialize};
use std::fmt;
use tabled::Tabled;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl fmt::Display for PolicySeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Pass,
    Warning,
    Fail,
}

impl fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "PASS"),
            Self::Warning => write!(f, "WARN"),
            Self::Fail => write!(f, "FAIL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct PolicyCheckResult {
    #[tabled(rename = "Rule ID")]
    pub rule_id: String,
    #[tabled(rename = "Category")]
    pub category: String,
    #[tabled(rename = "Severity")]
    pub severity: PolicySeverity,
    #[tabled(rename = "Status")]
    pub status: ComplianceStatus,
    #[tabled(rename = "Details")]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub cluster_name: String,
    pub profile: String,
    pub timestamp: String,
    pub checks: Vec<PolicyCheckResult>,
    pub pass_count: usize,
    pub warn_count: usize,
    pub fail_count: usize,
    pub compliance_score: f64,
}

impl ComplianceReport {
    pub fn new(cluster_name: String, profile: String, checks: Vec<PolicyCheckResult>) -> Self {
        let pass_count = checks
            .iter()
            .filter(|c| c.status == ComplianceStatus::Pass)
            .count();
        let warn_count = checks
            .iter()
            .filter(|c| c.status == ComplianceStatus::Warning)
            .count();
        let fail_count = checks
            .iter()
            .filter(|c| c.status == ComplianceStatus::Fail)
            .count();
        let total = checks.len();

        let compliance_score = if total > 0 {
            ((pass_count as f64 + (warn_count as f64 * 0.5)) / total as f64) * 100.0
        } else {
            100.0
        };

        Self {
            cluster_name,
            profile,
            timestamp: chrono_lite_now(),
            checks,
            pass_count,
            warn_count,
            fail_count,
            compliance_score,
        }
    }
}

fn chrono_lite_now() -> String {
    // Lightweight timestamp representation
    "2026-08-18T10:00:00Z".to_string()
}
