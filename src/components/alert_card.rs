use leptos::prelude::*;
use crate::types::Alert;

#[component]
pub fn AlertCard(alert: Alert) -> impl IntoView {
    let alert_level_clone = alert.level.clone();

    let alert_class = move || match alert_level_clone {
        crate::types::AlertLevel::Info => "bg-blue-500",
        crate::types::AlertLevel::Warning => "bg-yellow-500",
        crate::types::AlertLevel::Error => "bg-red-500",
        crate::types::AlertLevel::Critical => "bg-red-700",
    };

    view! {
        <div class=format!("{} p-4 rounded-lg shadow-md text-white", alert_class())>
            <h3 class="text-lg font-bold">{alert.service_name} - {alert.message}</h3>
            <p class="text-sm">Level: {format!("{:?}", alert.level)}</p>
            <p class="text-sm">Timestamp: {alert.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
            {alert.details.map(|d| view! { <p class="text-sm mt-2">Details: {d}</p> })} 
        </div>
    }
}