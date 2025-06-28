use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "db")]
    Db,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Http => write!(f, "http"),
            ServiceType::Db => write!(f, "db"),
        }
    }
}