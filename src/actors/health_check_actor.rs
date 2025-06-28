#[cfg(feature = "ssr")]
pub mod health_check_actors {
    use crate::actors::broadcaster::broadcast_actor::{
        BroadcastActor, HealthCheckInfo,
    };
    use crate::types::{CheckStatus, HealthCheckStatus, ServiceType};
    use actix::prelude::*;
    use anyhow::Result;
    use chrono::Utc;
    use log::{info, warn, error};
    use reqwest::StatusCode;
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use tokio_stream::wrappers::IntervalStream;
    use tokio_stream::StreamExt;

    #[derive(Message)]
    #[rtype(result = "Result<(), anyhow::Error>")]
    struct Check;

    pub struct HttpHealthCheckActor {
        name: String,
        url: String,
        interval_seconds: u64,
        timeout: u64,
        response_code: u16,
        headers: Vec<String>,
        broadcast_actor: Arc<Addr<BroadcastActor>>,
    }

    impl HttpHealthCheckActor {
        pub fn new(
            name: String,
            url: String,
            interval_seconds: u64,
            timeout: u64,
            response_code: u16,
            headers: Vec<String>,
            broadcast_actor: Arc<Addr<BroadcastActor>>,
        ) -> Self {
            HttpHealthCheckActor {
                name,
                url,
                interval_seconds,
                timeout,
                response_code,
                headers,
                broadcast_actor,
            }
        }
    }

    impl Actor for HttpHealthCheckActor {
        type Context = Context<Self>;

        fn started(&mut self, ctx: &mut Self::Context) {
            info!(
                "Health check actor started, name: {}, type: {}",
                self.name,
                ServiceType::Http
            );
            let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(
                self.interval_seconds,
            )))
            .map(|_| Check);
            ctx.add_stream(stream);
        }

        fn stopped(&mut self, _ctx: &mut Self::Context) {
            info!(
                "Health check actor stoped, name: {}, type: {}",
                self.name,
                ServiceType::Http
            );
        }
    }

    impl StreamHandler<Check> for HttpHealthCheckActor {
        fn handle(&mut self, _msg: Check, ctx: &mut Context<Self>) {
            let broadcast_actor = self.broadcast_actor.clone();
            let url = self.url.clone();
            let name = self.name.clone();
            let timeout = self.timeout;
            let interval = self.interval_seconds;

            let fut = async move {
                let start = Instant::now();
                let client = match reqwest::ClientBuilder::new()
                    .timeout(Duration::from_secs(timeout))
                    .build()
                {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to build reqwest client: {}", e);
                        return;
                    }
                };
                
                let health_status = match client.get(&url).send().await {
                    Ok(response) => {
                        let elapsed = start.elapsed().as_millis();
                        info!("checked successful. name: {}, response time: {}ms", name, elapsed);
                        match response.error_for_status() {
                            Ok(successful_response) => {
                                let status = successful_response.status();
                                let status_message = status.canonical_reason().unwrap_or("OK").to_string();
                                HealthCheckStatus {
                                    status: CheckStatus::from(status),
                                    status_message,
                                    response_time: start.elapsed().as_millis(),
                                    timestamp: Utc::now(),
                                }
                            }
                            Err(e) => {
                                let status = e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                                warn!("HTTP status error for '{}': {}", name, e);
                                HealthCheckStatus {
                                    status: CheckStatus::from(status),
                                    status_message: e.to_string(),
                                    response_time: elapsed,
                                    timestamp: Utc::now(),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Request failed for '{}': {}", name, e);
                        HealthCheckStatus {
                            status: CheckStatus::Unhealthy,
                            status_message: e.to_string(),
                            response_time: start.elapsed().as_millis(),
                            timestamp: Utc::now(),
                        }
                    }
                };
                broadcast_actor.do_send(HealthCheckInfo {
                    name,
                    service_type: ServiceType::Http,
                    url,
                    interval_seconds: interval,
                    latest_status: health_status,
                });
            };
            ctx.spawn(fut.into_actor(self));
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
