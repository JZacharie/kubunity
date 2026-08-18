use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenTelemetryConfig {
    pub enabled: bool,
    pub mode: String,
    pub grpc_port: u16,
    pub http_port: u16,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: "deployment".to_string(),
            grpc_port: 4317,
            http_port: 4318,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenObserveConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub stream_name: String,
    pub organization: String,
}

impl Default for OpenObserveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://o2-openobserve-router.openobserve.svc:5080/api/default".to_string(),
            stream_name: "k8s_telemetry".to_string(),
            organization: "default".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedTelemetryDomain {
    pub otel: OpenTelemetryConfig,
    pub openobserve: OpenObserveConfig,
}
