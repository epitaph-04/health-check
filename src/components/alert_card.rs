use leptos::prelude::*;
use crate::types::Alert;

#[component]
pub fn AlertCard(alert: Alert) -> impl IntoView {
    let alert_level_clone = alert.level.clone();

    let border_class = move || match alert_level_clone {
        crate::types::AlertLevel::Info => "border-blue-500",
        crate::types::AlertLevel::Warning => "border-yellow-500",
        crate::types::AlertLevel::Error => "border-red-600",
        crate::types::AlertLevel::Critical => "border-red-800",
    };

    let icon_svg = move || match alert_level_clone {
        crate::types::AlertLevel::Info => view! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-10 h-10 text-blue-500">
                <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
            </svg>
        }.into_any(),
        crate::types::AlertLevel::Warning => view! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-10 h-10 text-yellow-500">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.305 3.24 1.933 3.24h14.734c1.629 0 2.792-1.74 1.933-3.24L12.865 3.036c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
            </svg>
        }.into_any(),
        crate::types::AlertLevel::Error => view! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-10 h-10 text-red-600">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.305 3.24 1.933 3.24h14.734c1.629 0 2.792-1.74 1.933-3.24L12.865 3.036c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
            </svg>
        }.into_any(),
        crate::types::AlertLevel::Critical => view! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-10 h-10 text-red-800">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.305 3.24 1.933 3.24h14.734c1.629 0 2.792-1.74 1.933-3.24L12.865 3.036c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
            </svg>
        }.into_any(),
    };

    view! {
        <div class=format!("bg-gray-800 p-5 rounded-lg shadow-lg border-l-4 {} flex items-start space-x-4 hover:bg-gray-700 transition-colors duration-200", border_class())>
            <div class="flex-shrink-0 pt-1">
                {icon_svg()}
            </div>
            <div class="flex-1">
                <h3 class="text-xl font-bold text-white mb-2">{alert.service_name} - {alert.message}</h3>
                <div class="text-sm text-gray-300 space-y-1">
                    <p><span class="font-semibold mr-1">Level:</span> <span class=format!("font-medium {}", match alert.level {
                        crate::types::AlertLevel::Info => "text-blue-500",
                        crate::types::AlertLevel::Warning => "text-yellow-500",
                        crate::types::AlertLevel::Error => "text-red-600",
                        crate::types::AlertLevel::Critical => "text-red-800",
                    })>{format!("{:?}", alert.level)}</span></p>
                    <p><span class="font-semibold mr-1">Timestamp:</span> {alert.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                    {alert.details.map(|d| view! { <p><span class="font-semibold mr-1">Details:</span> {d}</p> })} 
                </div>
            </div>
        </div>
    }
}
