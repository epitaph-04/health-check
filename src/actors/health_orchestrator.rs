#[cfg(feature = "ssr")]
pub mod orchestrator {
    use actix::prelude::*;
    use futures_util::future::join_all;
    use log::error;
    use crate::actors::{CheckResult, HealthCheckInfo, HttpHealthCheckActor};

    #[derive(Message)]
    #[rtype(result = "()")]
    pub struct RegisterActor(pub Addr<HttpHealthCheckActor>);

    #[derive(Message)]
    #[rtype(result = "Result<Vec<HealthCheckInfo>, anyhow::Error>")]
    pub struct CheckStatus;

    #[derive(Debug, Default)]
    pub struct OrchestratorActor {
        health_checkers: Vec<Addr<HttpHealthCheckActor>>,
    }

    impl Actor for OrchestratorActor {
        type Context = Context<Self>;
    }

    impl Handler<RegisterActor> for OrchestratorActor {
        type Result = ();

        fn handle(&mut self, msg: RegisterActor, _ctx: &mut Context<Self>) {
            log::info!("OrchestratorActor: Registering new health checker.");
            self.health_checkers.push(msg.0);
        }
    }

    impl Handler<CheckStatus> for OrchestratorActor {
        type Result = ResponseFuture<anyhow::Result<Vec<HealthCheckInfo>>>;

        fn handle(&mut self, _msg: CheckStatus, _ctx: &mut Context<Self>) -> Self::Result {
            let health_checkers = self.health_checkers.clone();

            let fut = async move {
                let futures: Vec<_> = health_checkers
                    .into_iter()
                    .map(|addr| addr.send(CheckResult))
                    .collect();

                let results = join_all(futures).await;

                let mut services_with_status: Vec<HealthCheckInfo> = Vec::new();

                for result in results {
                    match result {
                        Ok(Ok(service)) => {
                            services_with_status.push(service);
                        }
                        Ok(Err(e)) => {
                            error!("A health check handler returned an error: {:?}", e);
                        }
                        Err(e) => {
                            error!("Mailbox error during health check: {:?}", e);
                        }
                    }
                }
                Ok(services_with_status)
            };
            Box::pin(fut)
        }
    }

}