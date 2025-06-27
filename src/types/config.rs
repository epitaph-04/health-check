#[cfg(feature = "ssr")]
pub mod configs {
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct GlobalSettings {
        pub check_interval_seconds: u64,
        pub timeout_seconds: u64,
    }

    impl Default for GlobalSettings {
        fn default() -> Self {
            Self {
                check_interval_seconds: 60,
                timeout_seconds: 5,
            }
        }
    }

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

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ServiceSettings {
        pub name: String,
        pub url: String,
        pub check_interval_seconds: Option<u16>,
        pub timeout_seconds: Option<u64>,
        pub response_code: Option<u16>,
        #[serde(rename = "type")]
        pub service_type: ServiceType,
        pub headers: Vec<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ServiceConfiguration {
        #[serde(rename = "globalSettings")]
        pub global: GlobalSettings,
        #[serde(rename = "services")]
        pub services: Vec<ServiceSettings>,
    }

    impl ServiceConfiguration {
        pub fn load_from_file(path: &str) -> Result<Self> {
            let file = std::fs::read_to_string(path)?;
            let service_config: ServiceConfiguration = toml::from_str(&file)?;
            Ok(service_config)
        }
    }
}
