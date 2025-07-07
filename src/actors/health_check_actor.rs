#[cfg(feature = "ssr")]
pub mod health_check_actors {
    use std::collections::VecDeque;
    use crate::actors::broadcaster::broadcast_actor::{
        BroadcastActor, HealthCheckInfo,
    };
    use crate::types::{CheckStatus, HealthCheckStatus, ServiceType};
    use actix::prelude::*;
    use anyhow::Result;
    use chrono::Utc;
    use log::{info, warn, error};
    use reqwest::StatusCode;
    use std::time::{Duration, Instant};
    use tokio_stream::wrappers::IntervalStream;
    use tokio_stream::StreamExt;

    #[derive(Message)]
    #[rtype(result = "()")]
    struct Check;

    #[derive(Message)]
    #[rtype(result = "Result<HealthCheckInfo, anyhow::Error>")]
    pub struct CheckResult;

    #[derive(Message)]
    #[rtype(result = "()")]
    struct HealthCheckResult {
        status: HealthCheckStatus,
    }

    pub struct HttpHealthCheckActor {
        name: String,
        url: String,
        interval_seconds: u64,
        timeout: u64,
        capacity: u64,
        historical_status: VecDeque<HealthCheckStatus>,
        broadcast_actor: Addr<BroadcastActor>,
    }

    impl HttpHealthCheckActor {
        pub fn new(
            name: String,
            url: String,
            interval_seconds: u64,
            timeout: u64,
            capacity: u64,
            broadcast_actor: Addr<BroadcastActor>,
        ) -> Self {
            HttpHealthCheckActor {
                name,
                url,
                interval_seconds,
                timeout,
                capacity,
                historical_status: VecDeque::new(),
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
            let url = self.url.clone();
            let name = self.name.clone();
            let timeout = self.timeout;
            let self_addr = ctx.address();

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
                                    response_time: elapsed,
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
                self_addr.do_send(HealthCheckResult { status: health_status });
            };
            ctx.spawn(fut.into_actor(self));
        }
    }

    impl Handler<HealthCheckResult> for HttpHealthCheckActor {
        type Result = ();

        fn handle(&mut self, msg: HealthCheckResult, _ctx: &mut Context<Self>) {
            if self.historical_status.len() >= self.capacity as usize {
                self.historical_status.pop_back();
            }
            self.historical_status.push_front(msg.status.clone());

            let service_message = HealthCheckInfo {
                name: self.name.clone(),
                url: self.url.clone(),
                service_type: ServiceType::Http,
                interval_seconds: self.interval_seconds,
                latest_status: msg.status,
                historic_status: self.historical_status.clone().into(),
            };
            self.broadcast_actor.do_send(service_message);
        }
    }

    impl Handler<CheckResult> for HttpHealthCheckActor {
        type Result = Result<HealthCheckInfo>;

        fn handle(&mut self, _msg: CheckResult, _ctx: &mut Context<Self>) -> Self::Result {
            Ok(HealthCheckInfo {
                name: self.name.clone(),
                url: self.url.clone(),
                interval_seconds: self.interval_seconds,
                service_type: ServiceType::Http,
                latest_status: self.historical_status.front().map_or(HealthCheckStatus::default(), |s| s.clone()),
                historic_status: self.historical_status.clone().into(),
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
