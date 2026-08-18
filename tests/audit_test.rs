use std::sync::Arc;
use kubunity::application::AuditService;
use kubunity::domain::{ClusterProfile, ComplianceStatus};
use kubunity::infrastructure::{HelmCliAdapter, KyvernoPolicyAuditor};

#[tokio::test]
async fn test_audit_service_with_cloud_profile() {
    let helm_adapter = Arc::new(HelmCliAdapter::new());
    let kyverno_auditor = Arc::new(KyvernoPolicyAuditor::new());
    let audit_service = AuditService::new(helm_adapter, kyverno_auditor);

    let chart_path = std::path::Path::new("./charts/kubunity");
    if chart_path.exists() {
        let result = audit_service
            .run_audit(chart_path, "test-cloud-cluster", ClusterProfile::Cloud)
            .await;

        assert!(result.is_ok(), "Audit service failed: {:?}", result.err());
        let report = result.unwrap();
        assert_eq!(report.cluster_name, "test-cloud-cluster");
        assert_eq!(report.profile, "cloud");
        assert!(report.checks.len() >= 6, "Expected at least 6 compliance checks");

        // Verify rootless check passed
        let rootless_check = report.checks.iter().find(|c| c.rule_id == "SEC-PSS-001");
        assert!(rootless_check.is_some());
        assert_eq!(rootless_check.unwrap().status, ComplianceStatus::Pass);
    }
}
