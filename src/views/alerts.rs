use leptos::prelude::*;
use crate::components::AlertCard;
use crate::types::{Alert, AlertLevel};
use chrono::{DateTime, Utc};

#[component]
pub fn Alerts() -> impl IntoView {
    let alerts = signal(vec![
        Alert {
            service_name: "Service A".to_string(),
            message: "Service A is down".to_string(),
            level: AlertLevel::Error,
            timestamp: DateTime::parse_from_str("2025-07-08 10:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc),
            details: Some("Failed to connect to database.".to_string()),
        },
        Alert {
            service_name: "Server B".to_string(),
            message: "High CPU usage on Server B".to_string(),
            level: AlertLevel::Warning,
            timestamp: DateTime::parse_from_str("2025-07-08 09:30:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc),
            details: None,
        },
        Alert {
            service_name: "System Update".to_string(),
            message: "New software update available".to_string(),
            level: AlertLevel::Info,
            timestamp: DateTime::parse_from_str("2025-07-08 08:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc),
            details: None,
        },
        Alert {
            service_name: "Critical Service".to_string(),
            message: "Critical service has stopped responding.".to_string(),
            level: AlertLevel::Critical,
            timestamp: DateTime::parse_from_str("2025-07-08 10:15:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc),
            details: Some("Immediate action required.".to_string()),
        },
    ]);

    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-3xl font-bold mb-6">Alerts</h1>
            <div class="grid grid-cols-1 md::grid-cols-2 lg:grid-cols-3 gap-4">
                <For
                    each=move || alerts.0.get().clone()
                    key=|alert| format!("{}-{}-{}", alert.service_name, alert.message, alert.timestamp)
                    children=move |alert| view! { <AlertCard alert=alert/> }
                />
            </div>
        </div>
    }
}