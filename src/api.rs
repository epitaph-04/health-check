pub mod server_api {
    use std::convert::Infallible;
    use std::time::Duration;
    use actix_web::{get, web, Responder};
    use actix_web_lab::sse;
    use actix_web_lab::sse::Sse;
    use log::warn;
    use tokio::sync::broadcast;
    use tokio_stream::wrappers::BroadcastStream;
    use futures_util::StreamExt as _;
    use crate::actors::HealthCheckInfo;

    #[get("/api/events")]
    async fn sse_handler(
        sender: web::Data<broadcast::Sender<HealthCheckInfo>>,
    ) -> impl Responder {
        let receiver = sender.subscribe();
        let stream = BroadcastStream::new(receiver)
            .map(|event_result| {
                let event = match event_result {
                    Ok(server_event) => {
                        match serde_json::to_string(&server_event) {
                            Ok(json) => sse::Data::new(json).into(),
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

        Sse::from_stream(stream).with_keep_alive(Duration::from_secs(10))
    }
}