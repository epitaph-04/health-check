use std::fmt::Display;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AlertLevel {
    Critical,
    Warning,
    Info
}

impl Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertLevel::Critical => write!(f, "Critical"),
            AlertLevel::Warning => write!(f, "Warning"),
            AlertLevel::Info => write!(f, "Info"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub service_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}