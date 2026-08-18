use anyhow::Result;
use crate::domain::Tenant;

pub struct TenantService;

impl TenantService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_tenant(&self, tenant: &Tenant) -> Result<()> {
        if tenant.name.trim().is_empty() {
            anyhow::bail!("Tenant name cannot be empty");
        }

        if !tenant.name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            anyhow::bail!("Tenant name '{}' must be lowercase RFC 1123 compliant (a-z, 0-9, '-')", tenant.name);
        }

        Ok(())
    }
}

impl Default for TenantService {
    fn default() -> Self {
        Self::new()
    }
}
