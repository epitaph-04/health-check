#[cfg(feature = "ssr")]
pub mod health_check_actors {
    use crate::actors::broadcaster::broadcast_actor::{
        BroadcastActor, CheckStatus, HealthCheckInfo, HealthCheckStatus,
    };
    use crate::types::ServiceType;
    use actix::prelude::*;
    use anyhow::Result;
    use chrono::Utc;
    use log::info;
    use reqwest::StatusCode;
    use std::sync::Arc;
    use std::time::Instant;

    #[derive(Message)]
    #[rtype(result = "Result<(), anyhow::Error>")]
    pub struct Check;

    pub struct HttpHealthCheckActor {
        name: String,
        url: String,
        timeout: u64,
        response_code: u16,
        headers: Vec<String>,
        broadcast_actor: Arc<Addr<BroadcastActor>>,
    }

    impl HttpHealthCheckActor {
        pub fn new(
            name: String,
            url: String,
            timeout: u64,
            response_code: u16,
            headers: Vec<String>,
            broadcast_actor: Arc<Addr<BroadcastActor>>,
        ) -> Self {
            HttpHealthCheckActor {
                name,
                url,
                timeout,
                response_code,
                headers,
                broadcast_actor,
            }
        }
    }

    impl Actor for HttpHealthCheckActor {
        type Context = Context<Self>;

        fn started(&mut self, _ctx: &mut Self::Context) {
            info!(
                "Health check actor started, name: {}, type: {}",
                self.name,
                ServiceType::Http
            );
        }

        fn stopped(&mut self, _ctx: &mut Self::Context) {
            info!(
                "Health check actor stoped, name: {}, type: {}",
                self.name,
                ServiceType::Http
            );
        }
    }

    impl Handler<Check> for HttpHealthCheckActor {
        type Result = ResponseFuture<Result<()>>;

        fn handle(&mut self, _msg: Check, _ctx: &mut Context<Self>) -> Self::Result {
            let url = self.url.clone();
            let name = self.name.clone();

            let client = reqwest::ClientBuilder::new()
                .timeout(std::time::Duration::from_secs(self.timeout))
                .build();
            let actor = self.broadcast_actor.clone();
            Box::pin(async move {
                let start = Instant::now();

                let response = client?.get(&url).send().await?.error_for_status()?;

                let duration = start.elapsed();
                let status = response.status();

                let status_message = status
                    .canonical_reason()
                    .unwrap_or("Unknown status")
                    .to_string();

                _ = actor
                    .send(HealthCheckInfo {
                        name,
                        service_type: ServiceType::Http,
                        url: url.clone(),
                        interval_seconds: 30,
                        latest_status: HealthCheckStatus {
                            status: CheckStatus::from(status),
                            status_message,
                            response_time: duration.as_millis(),
                            timestamp: Utc::now(),
                        },
                    })
                    .await?;

                Ok(())
            })
        }
    }

    impl From<StatusCode> for CheckStatus {
        fn from(item: StatusCode) -> Self {
            match item {
                item if item.is_success() => CheckStatus::Healthy,
                _ => CheckStatus::Unhealthy,
            }
        }
    }
}
