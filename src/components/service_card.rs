use leptos::prelude::*;
use crate::types::{CheckStatus, ServiceHealthCheckInfo};

#[component]
pub fn ServiceCard(info: RwSignal<ServiceHealthCheckInfo>) -> impl IntoView {
    let color = move || match info.get().latest_status.status {
        CheckStatus::Healthy => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-green-500/10 border-green-500/30 text-green-400 mb-1",
        CheckStatus::Degraded => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-orange-500/10 border-orange-500/30 text-orange-400 mb-1",
        CheckStatus::Unhealthy => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-red-500/10 border-red-500/30 text-red-400 mb-1",
        CheckStatus::Unknown => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-gray-500/10 border-gray-500/30 text-gray-400 mb-1"
    };
    let style = move || match info.get().latest_status.status {
        CheckStatus::Healthy => "service-card rounded-lg shadow-lg overflow-hidden bg-green-500/10 border-green-500 border flex flex-col",
        CheckStatus::Degraded => "service-card rounded-lg shadow-lg overflow-hidden bg-orange-500/10 border-orange-500 border flex flex-col",
        CheckStatus::Unhealthy => "service-card rounded-lg shadow-lg overflow-hidden bg-red-500/10 border-red-500 border flex flex-col",
        CheckStatus::Unknown => "service-card rounded-lg shadow-lg overflow-hidden bg-gray-500/10 border-gray-500 border flex flex-col"
    };
    view! {
        <div class=style>
            <div class="p-3 sm:p-4 flex-grow">
                <div class="flex items-center justify-between mb-2">
                    <div class="flex items-center min-w-0">
                        <CardHeader
                            status=Memo::new(move |_| info.get().latest_status.status)
                            title=Memo::new(move |_| info.get().name)
                        />
                    </div>
                    <div class="flex space-x-1 flex-shrink-0">
                        <button
                            class="expand-btn p-1 text-slate-400 hover:text-slate-200 rounded-full hover:bg-slate-600/50 transition-colors"
                            title="Toggle Details"
                        >
                            <svg
                                class="chevron-down"
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polyline points="6 9 12 15 18 9" />
                            </svg>
                            <svg
                                class="chevron-up hidden"
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polyline points="18 15 12 9 6 15" />
                            </svg>
                        </button>
                    </div>
                </div>
                <p class="text-xs text-slate-400 truncate mb-1" title=info.get().url>
                    <span class="font-semibold">{info.get().service_type.to_string()}@</span>
                    {info.get().url}
                </p>
                <div class=color>{move || info.get().latest_status.status.to_string()}</div>
                <p class="text-xs text-slate-400 mt-1">
                    {move || format!("Status: {}", info.get().latest_status.status_message)}
                </p>
                <div class="flex justify-between text-xs text-slate-400 mt-2">
                    <span class="icon-text-align">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="12"
                            height="12"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="mr-1"
                        >
                            <circle cx="12" cy="12" r="10" />
                            <polyline points="12 6 12 12 16 14" />
                        </svg>
                        {move || format!("{} ms", info.get().latest_status.response_time)}
                    </span>
                    <span class="icon-text-align">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="12"
                            height="12"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="mr-1"
                        >
                            <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
                            <line x1="16" y1="2" x2="16" y2="6" />
                            <line x1="8" y1="2" x2="8" y2="6" />
                            <line x1="3" y1="10" x2="21" y2="10" />
                        </svg>
                        {move || info.get().latest_status.timestamp.format("%H:%M:%S").to_string()}
                    </span>
                </div>
                <p class="text-xs text-slate-400 mt-1">
                    {format!("Interval: {}s", info.get().interval_seconds)}
                </p>
            </div>
        </div>
    }
}

#[component]
fn CardHeader(status: Memo<CheckStatus>, title: Memo<String>) -> impl IntoView {
    view! {
        {move || {
            let color_class = match &*status.read() {
                CheckStatus::Healthy => "text-green-400",
                CheckStatus::Degraded => "text-orange-400",
                CheckStatus::Unhealthy => "text-red-400",
                CheckStatus::Unknown => "text-gray-400",
            };
            let icon = match &*status.read() {
                CheckStatus::Healthy => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewbox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="icon-text-align text-green-500"
                        >
                            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                            <polyline points="22 4 12 14.01 9 11.01"></polyline>
                        </svg>
                    }
                        .into_any()
                }
                CheckStatus::Degraded => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewbox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="icon-text-align text-orange-500"
                        >
                            <path d="m21.73 18-8-14a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>
                            <line x1="12" y1="9" x2="12" y2="13"></line>
                            <line x1="12" y1="17" x2="12.01" y2="17"></line>
                        </svg>
                    }
                        .into_any()
                }
                CheckStatus::Unhealthy => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewbox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="icon-text-align text-red-500"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="15" y1="9" x2="9" y2="15"></line>
                            <line x1="9" y1="9" x2="15" y2="15"></line>
                        </svg>
                    }
                        .into_any()
                }
                CheckStatus::Unknown => {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewbox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="icon-text-align text-red-500"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="15" y1="9" x2="9" y2="15"></line>
                            <line x1="9" y1="9" x2="15" y2="15"></line>
                        </svg>
                    }
                        .into_any()
                }
            };
            view! {
                <div class="flex items-center">
                    {icon}
                </div>
                <h3 class=format!("ml-2 text-base sm:text-lg font-semibold truncate {}", color_class) title=title>
                    {title}
                </h3>
            }
        }}
    }
}