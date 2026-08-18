use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuotas {
    pub requests_cpu: Option<String>,
    pub requests_memory: Option<String>,
    pub limits_cpu: Option<String>,
    pub limits_memory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub name: String,
    pub environment: String,
    pub quotas: Option<ResourceQuotas>,
    pub network_isolation: bool,
    pub pod_security_standard: Option<String>,
}
