use std::collections::HashMap;
use chrono::Utc;
use futures_util::StreamExt;
use gloo::net::eventsource::futures::EventSource;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::{error, info};
use crate::types::{Alert, AlertLevel, CheckStatus, ServiceHealthCheckInfo};

#[component]
pub fn Dashboard() -> impl IntoView {
    let (isConnected, setIsConnected) = signal(false);
    let (lastUpdated, _setLastUpdated) = signal(Utc::now());
    let (healthyCount, _setHealthyCount) = signal(4);
    let (degradedCount, _setDegradedCount) = signal(0);
    let (criticalCount, _setCriticalCount) = signal(0);
    let (healthScore, _setHealthScore) = signal(100);
    let (services, setServices) = signal(HashMap::<String, RwSignal<ServiceHealthCheckInfo>>::new());
    let (recentAlerts, _setRecentAlerts) = signal(vec![
        Alert{
            service_name: "Instagram".to_string(),
            level: AlertLevel::Warning,
            message: "Cannot reach".to_string(),
            timestamp: Utc::now(),
        },
        Alert{
            service_name: "Facebook".to_string(),
            level: AlertLevel::Critical,
            message: "Service unreachable".to_string(),
            timestamp: Utc::now(),
        },
    ]);
    Effect::new(move |_| {
        let mut event_source = match EventSource::new("/api/events") {
            Ok(es) => {
                info!("SSE connection established.");
                *setIsConnected.write() = true;
                es
            }
            Err(e) => {
                error!("Failed to connect to SSE endpoint: {:?}", e);
                *setIsConnected.write() = false;
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
                            setServices.update(|s| {
                                if let Some(service_signal) = s.get(&new_message.name) {
                                    service_signal.set(new_message);
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
            *setIsConnected.write() = false;
        });
    });

    let get_status_color = |status: CheckStatus| match status {
        CheckStatus::Healthy => "bg-green-400",
        CheckStatus::Degraded => "bg-orange-400",
        CheckStatus::Unhealthy => "bg-red-400",
        _ => "bg-gray-400",
    };

    view! {
        <div class="min-h-screen bg-white-50 p-6 rounded-xl shadow-md">
            <div class="bg-gray-700 shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center py-6">
                        <h1 class="text-3xl font-bold text-white-900">System Health Dashboard</h1>
                        <div class="flex items-center space-x-4">
                            <div class="flex items-center space-x-2">
                                <div class=move || {
                                    if isConnected.get() {
                                        "w-3 h-3 rounded-full if bg-green-400"
                                    } else {
                                        "w-3 h-3 rounded-full if bg-red-400"
                                    }
                                }></div>
                                <span class="text-sm text-white-600">
                                    {move || {
                                        if isConnected.get() {
                                            " Connected "
                                        } else {
                                            " Disconnected "
                                        }
                                    }}
                                </span>
                            </div>
                            <span class="text-sm text-white-500">
                                Last updated: {lastUpdated.get().format("%H:%M:%S").to_string()}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-green-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-green-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Healthy Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {healthyCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-yellow-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-yellow-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Degraded Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {degradedCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-red-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-red-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Critical Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {criticalCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-blue-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-blue-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Overall Health Score
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {healthScore.get()}%
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2">
                        <div class="bg-gray-700 shadow rounded-lg">
                            <div class="px-6 py-4 border-b border-white-200">
                                <h3 class="text-lg font-medium text-white-900">Services Status</h3>
                            </div>
                            <div class="divide-y divide-white-200">
                                <For
                                    each=move || services.get().into_iter().take(10)
                                    key=|service| service.0.clone()
                                    children=move |(_, service)| {
                                        view! {
                                            <div
                                                class="px-6 py-4 hover:bg-white-50 cursor-pointer"
                                                on:click=move |_| {}
                                            >
                                                <div class="flex items-center justify-between">
                                                    <div class="flex items-center">
                                                        <div class="flex-shrink-0">
                                                            <div class=move || {
                                                                format!(
                                                                    "w-3 h-3 rounded-full {}",
                                                                    get_status_color(service.get().latest_status.status),
                                                                )
                                                            }></div>
                                                        </div>
                                                        <div class="ml-4">
                                                            <p class="text-sm font-medium text-white-900">
                                                                {service.get().name}
                                                            </p>
                                                            <p class="text-sm text-white-500">
                                                                {format!("{:?}", service.get().service_type)}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div class="text-right">
                                                        <p class="text-sm font-medium text-white-900">
                                                            {move || {
                                                                format!("{} ms", service.get().latest_status.response_time)
                                                            }}
                                                        </p>
                                                        <p class="text-sm text-white-500">
                                                            {move || {
                                                                service
                                                                    .get()
                                                                    .latest_status
                                                                    .timestamp
                                                                    .format("%H:%M:%S")
                                                                    .to_string()
                                                            }}
                                                        </p>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                            <div class="px-6 py-3 border-t border-white-200">
                                <a
                                    href="/services"
                                    class="text-sm font-medium text-blue-600 hover:text-blue-500"
                                >
                                    "View all services →"
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="space-y-6">
                        <div class="bg-gray-700 shadow rounded-lg">
                            <div class="px-6 py-4 border-b border-white-200">
                                <h3 class="text-lg font-medium text-white-900">Quick Actions</h3>
                            </div>
                            <div class="px-6 py-4 space-y-3">
                                <button
                                    on:click=move |_| {}
                                    class="w-full flex items-center justify-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-gray bg-blue-600 hover:bg-blue-700"
                                >
                                    View Analytics
                                </button>
                                <button
                                    on:click=move |_| {}
                                    class="w-full flex items-center justify-center px-4 py-2 border border-white-300 text-sm font-medium rounded-md text-white-700 bg-gray hover:bg-white-50"
                                >
                                    Manage Alerts
                                </button>
                                <button
                                    on:click=move |_| {}
                                    class="w-full flex items-center justify-center px-4 py-2 border border-white-300 text-sm font-medium rounded-md text-white-700 bg-gray hover:bg-white-50"
                                >
                                    View Dependencies
                                </button>
                            </div>
                        </div>
                        <div class="bg-gray-700 shadow rounded-lg">
                            <div class="px-6 py-4 border-b border-white-200">
                                <h3 class="text-lg font-medium text-white-900">Recent Alerts</h3>
                            </div>
                            <div class="px-6 py-4">
                                {move || {
                                    let get_alert_color = |level: AlertLevel| match level {
                                        AlertLevel::Critical => "bg-red-500",
                                        AlertLevel::Warning => "bg-orange-400",
                                        AlertLevel::Info => "bg-blue-400",
                                    };
                                    if recentAlerts.with(|alerts| alerts.is_empty()) {
                                        view! {
                                            <p class="text-sm text-white-500">"No recent alerts"</p>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || recentAlerts.get().into_iter().take(5)
                                                key=|alert| alert.service_name.clone()
                                                children=move |alert| {
                                                    view! {
                                                        <div class="flex items-start space-x-3">
                                                            <div class="flex-shrink-0">
                                                                <div class=format!(
                                                                    "w-2 h-2 rounded-full {} mt-2",
                                                                    get_alert_color(alert.level),
                                                                )></div>
                                                            </div>
                                                            <div class="min-w-0 flex-1">
                                                                <p class="text-sm font-medium text-white-900">
                                                                    {alert.service_name}
                                                                </p>
                                                                <p class="text-sm text-white-500">{alert.message}</p>
                                                                <p class="text-xs text-white-400">
                                                                    {alert.timestamp.format("%H:%M:%S").to_string()}
                                                                </p>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        }
                                            .into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}