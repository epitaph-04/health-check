#[cfg(feature = "ssr")]
pub mod broadcast_actor {
    use crate::types::ServiceType;
    use actix::prelude::*;
    use anyhow::Result;
    use chrono::{DateTime, Utc};
    use log::info;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub enum CheckStatus {
        Healthy,
        Degraded,
        Unhealthy,
    }
    #[derive(Serialize, Deserialize, Clone)]
    pub struct HealthCheckStatus {
        pub status: CheckStatus,
        pub status_message: String,
        pub response_time: u128,
        pub timestamp: DateTime<Utc>,
    }

    #[derive(Message, Serialize, Deserialize, Clone)]
    #[rtype(result = "Result<(), anyhow::Error>")]
    pub struct HealthCheckInfo {
        pub name: String,
        pub service_type: ServiceType,
        pub url: String,
        pub interval_seconds: u64,
        pub latest_status: HealthCheckStatus,
    }

    pub struct BroadcastActor {}

    impl Actor for BroadcastActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {
            info!("Broadcast actor started");
        }

        fn stopped(&mut self, _ctx: &mut Self::Context) {
            info!("Broadcast actor stopped");
        }
    }

    impl Handler<HealthCheckInfo> for BroadcastActor {
        type Result = ResponseFuture<Result<()>>;

        fn handle(&mut self, _msg: HealthCheckInfo, _ctx: &mut Context<Self>) -> Self::Result {
            Box::pin(async move { Ok(()) })
        }
    }
}
