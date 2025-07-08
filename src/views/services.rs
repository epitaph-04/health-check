use std::collections::HashMap;
use futures_util::StreamExt;
use gloo::net::eventsource::futures::EventSource;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::{error, info};
use crate::components::ServiceCard;
use crate::types::ServiceHealthCheckInfo;

#[component]
pub fn Services() -> impl IntoView {
    let (servicesMap, setServicesMap) = signal(HashMap::<String, RwSignal<ServiceHealthCheckInfo>>::new());
    Effect::new(move |_| {
        let mut event_source = match EventSource::new("/api/events") {
            Ok(es) => {
                info!("SSE connection established.");
                es
            }
            Err(e) => {
                error!("Failed to connect to SSE endpoint: {:?}", e);
                return;
            }
        };
        let mut event_stream = match event_source.subscribe("sse") {
            Ok(stream) => stream,
            Err(e) => {
                error!("Failed to subscribe to 'sse' event: {:?}", e);
                return;
            }
        };
        spawn_local(async move {
            while let Some(Ok((_, msg))) = event_stream.next().await {
                if let Some(data_str) = msg.data().as_string() {
                    match serde_json::from_str::<ServiceHealthCheckInfo>(&data_str) {
                        Ok(new_message) => {
                            setServicesMap.update(|s| {
                                if let Some(service_signal) = s.get(&new_message.name) {
                                    *service_signal.write() = new_message;
                                } else {
                                    s.insert(new_message.name.clone(), RwSignal::new(new_message));
                                }
                            });
                        },
                        Err(e) => {
                            error!("Failed to deserialize message: {:?}, error: {}", data_str, e);
                        }
                    }
                }
            }
            info!("SSE connection closed.");
            event_source.close();
        });
    });
    view! {
        <div class="mb-6">
            <h1 class="text-2xl font-bold text-slate-100 mb-2">Service Dashboard</h1>
            <p class="text-slate-400">Overview of all monitored services</p>
        </div>
        <div
            id="dashboardView"
            class="view-content grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-6"
        >
            <For
                each=move || servicesMap.get()
                key=|state| state.0.clone()
                children=move |(_, val)| {
                    view! { <ServiceCard info=val /> }
                }
            />
        </div>
    }
}