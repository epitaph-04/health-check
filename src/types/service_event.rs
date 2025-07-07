use std::fmt::Display;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::ServiceType;

#[derive(Serialize, Deserialize, Clone)]
pub enum CheckStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
impl Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Healthy => write!(f, "Healthy"),
            CheckStatus::Degraded => write!(f, "Degraded"),
            CheckStatus::Unhealthy => write!(f, "Unhealthy"),
            CheckStatus::Unknown => write!(f, "Unknown"),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct HealthCheckStatus {
    pub status: CheckStatus,
    pub status_message: String,
    pub response_time: u128,
    pub timestamp: DateTime<Utc>,
}
impl Default for HealthCheckStatus {
    fn default() -> Self {
        HealthCheckStatus{
            status: CheckStatus::Unknown,
            response_time: 0,
            status_message: "unknown".to_string(),
            timestamp: Utc::now()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceHealthCheckInfo {
    pub name: String,
    pub service_type: ServiceType,
    pub url: String,
    pub interval_seconds: u64,
    pub latest_status: HealthCheckStatus,
}