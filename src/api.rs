pub mod server_api {
    use std::convert::Infallible;
    use std::time::Duration;
    use actix::Addr;
    use actix_web::{get, web, Responder};
    use actix_web_lab::sse;
    use actix_web_lab::sse::{Event, Sse};
    use log::warn;
    use tokio::sync::broadcast;
    use tokio_stream::wrappers::BroadcastStream;
    use futures_util::{stream, StreamExt};
    use crate::actors::{CheckStatus, HealthCheckInfo, OrchestratorActor};

    #[cfg(feature = "ssr")]
    #[derive(Clone)]
    pub struct AppState {
        pub sender: broadcast::Sender<HealthCheckInfo>,
        pub actor: Addr<OrchestratorActor>
    }

    #[get("/api/events")]
    async fn sse_handler(
        app_state: web::Data<AppState>,
    ) -> impl Responder {
        let receiver = app_state.sender.subscribe();
        let mut initial_data = vec![];
        if let Ok(Ok(resp)) = app_state.actor.send(CheckStatus).await {
            initial_data = resp.into_iter().map(|s| {
                let event: Event = match serde_json::to_string(&s) {
                    Ok(json) => sse::Data::new(json).event("sse").into(),
                    Err(_) => sse::Data::new("internal: serialization error").event("error").into(),
                };
                Ok::<_, Infallible>(event)
            }).collect();
        }

        let broadcast_stream = BroadcastStream::new(receiver)
            .map(|event_result| {
                let event = match event_result {
                    Ok(server_event) => {
                        match serde_json::to_string(&server_event) {
                            Ok(json) => sse::Data::new(json).event("sse").into(),
                            Err(_) => sse::Data::new("internal: serialization error").event("error").into(),
                        }
                    }
                    Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n)) => {
                        warn!("SSE client lagged.");
                        sse::Data::new(format!("Connection lagged. You missed {} messages.", n)).event("error").into()
                    }
                };
                Ok::<_, Infallible>(event)
            });
        let combined_stream = stream::iter(initial_data).chain(broadcast_stream);
        Sse::from_stream(combined_stream).with_keep_alive(Duration::from_secs(10))
    }
}