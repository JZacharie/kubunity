use async_trait::async_trait;
use anyhow::Result;
use crate::domain::{
    ClusterProfile, ComplianceReport, ComplianceStatus, PolicyCheckResult, PolicySeverity,
};
use crate::ports::PolicyAuditor;

pub struct KyvernoPolicyAuditor;

impl KyvernoPolicyAuditor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for KyvernoPolicyAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PolicyAuditor for KyvernoPolicyAuditor {
    async fn audit_manifests(
        &self,
        cluster_name: &str,
        profile: ClusterProfile,
        manifests: &str,
    ) -> Result<ComplianceReport> {
        let mut checks = Vec::new();

        // 1. Check Rootless Execution
        let has_rootless_policy = manifests.contains("kubunity-disallow-root");
        let has_run_as_non_root = manifests.contains("runAsNonRoot: true");
        checks.push(PolicyCheckResult {
            rule_id: "SEC-PSS-001".to_string(),
            category: "Pod Security (Restricted)".to_string(),
            severity: PolicySeverity::Critical,
            status: if has_rootless_policy || has_run_as_non_root {
                ComplianceStatus::Pass
            } else {
                ComplianceStatus::Fail
            },
            message: if has_rootless_policy || has_run_as_non_root {
                "Rootless container enforcement active (runAsNonRoot: true).".to_string()
            } else {
                "No rootless enforcement detected in manifests.".to_string()
            },
        });

        // 2. Check Privilege Escalation
        let has_priv_esc_policy = manifests.contains("kubunity-disallow-privilege-escalation");
        checks.push(PolicyCheckResult {
            rule_id: "SEC-PSS-002".to_string(),
            category: "Pod Security (Restricted)".to_string(),
            severity: PolicySeverity::High,
            status: if has_priv_esc_policy {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_priv_esc_policy {
                "Privilege escalation prevention policy active.".to_string()
            } else {
                "Privilege escalation policy missing.".to_string()
            },
        });

        // 3. Check Host Namespaces (hostNetwork, hostPID, hostIPC)
        let has_host_ns_policy = manifests.contains("kubunity-disallow-host-namespaces");
        checks.push(PolicyCheckResult {
            rule_id: "SEC-PSS-003".to_string(),
            category: "Isolation & Sandboxing".to_string(),
            severity: PolicySeverity::High,
            status: if has_host_ns_policy {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_host_ns_policy {
                "Host namespaces (PID/IPC/Network) sharing prohibited.".to_string()
            } else {
                "Host namespace restriction policy missing.".to_string()
            },
        });

        // 4. Check Immutable Image Tags (:latest)
        let has_latest_tag_policy = manifests.contains("kubunity-disallow-latest-tag");
        checks.push(PolicyCheckResult {
            rule_id: "SEC-SCM-001".to_string(),
            category: "Supply Chain Security".to_string(),
            severity: PolicySeverity::Medium,
            status: if has_latest_tag_policy {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_latest_tag_policy {
                "Immutable image tags enforced (no :latest).".to_string()
            } else {
                "Mutable image tags (:latest) policy missing.".to_string()
            },
        });

        // 5. Check Resource Limits & Requests (FinOps / Node Stability)
        let has_resource_policy = manifests.contains("kubunity-require-resource-requests-limits");
        checks.push(PolicyCheckResult {
            rule_id: "GOV-RES-001".to_string(),
            category: "Resource Governance".to_string(),
            severity: PolicySeverity::Medium,
            status: if has_resource_policy {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_resource_policy {
                "CPU & Memory requests and limits enforced.".to_string()
            } else {
                "Resource requests/limits enforcement policy missing.".to_string()
            },
        });

        // 6. Check Zero Trust Network Policy Generator
        let has_netpol_policy = manifests.contains("kubunity-generate-default-network-policy");
        checks.push(PolicyCheckResult {
            rule_id: "NET-ZT-001".to_string(),
            category: "Network Security".to_string(),
            severity: PolicySeverity::High,
            status: if has_netpol_policy {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_netpol_policy {
                "Automated namespace isolation NetworkPolicy generator active.".to_string()
            } else {
                "Default NetworkPolicy generator missing.".to_string()
            },
        });

        // 7. Check OpenTelemetry Collector Pipeline
        let has_otel = manifests.contains("otel-collector") || manifests.contains("opentelemetry");
        checks.push(PolicyCheckResult {
            rule_id: "OBS-OTEL-001".to_string(),
            category: "Observability Pipeline".to_string(),
            severity: PolicySeverity::Medium,
            status: if has_otel {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Edge {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Pass
            },
            message: if has_otel {
                "OpenTelemetry Collector OTLP pipelines configured.".to_string()
            } else {
                "OpenTelemetry collector not enabled in profile.".to_string()
            },
        });

        // 8. Check RBAC Hardening (Restrict Cluster-Admin)
        let has_rbac_lock = manifests.contains("kubunity-restrict-cluster-admin-bindings");
        checks.push(PolicyCheckResult {
            rule_id: "SEC-RBAC-001".to_string(),
            category: "RBAC Governance".to_string(),
            severity: PolicySeverity::Critical,
            status: if has_rbac_lock {
                ComplianceStatus::Pass
            } else if profile == ClusterProfile::Dev {
                ComplianceStatus::Warning
            } else {
                ComplianceStatus::Fail
            },
            message: if has_rbac_lock {
                "Strict cluster-admin binding restriction policy active.".to_string()
            } else {
                "cluster-admin binding restriction policy missing.".to_string()
            },
        });

        Ok(ComplianceReport::new(
            cluster_name.to_string(),
            profile.to_string(),
            checks,
        ))
    }
}
