use chrono::Utc;
use leptos::prelude::*;
use crate::components::ServiceCard;
use crate::types::{CheckStatus, HealthCheckStatus, ServiceHealthCheckInfo, ServiceType};

#[component]
pub fn Services() -> impl IntoView {
    let (service1, _) = signal(ServiceHealthCheckInfo{
        name: "Google".to_string(),
        url: "https://google.com".to_string(),
        service_type: ServiceType::Http,
        interval_seconds: 30,
        latest_status: HealthCheckStatus{
            status: CheckStatus::Healthy,
            status_message: "Ok".to_string(),
            response_time: 21,
            timestamp: Utc::now(),
        },
    });
    let (service2, _) = signal(ServiceHealthCheckInfo{
        name: "Facebook".to_string(),
        url: "https://facebook.com".to_string(),
        service_type: ServiceType::Http,
        interval_seconds: 30,
        latest_status: HealthCheckStatus{
            status: CheckStatus::Degraded,
            status_message: "Ok".to_string(),
            response_time: 2100,
            timestamp: Utc::now(),
        },
    });
    let (service3, _) = signal(ServiceHealthCheckInfo{
        name: "Instagram".to_string(),
        url: "https://instagram.com".to_string(),
        service_type: ServiceType::Http,
        interval_seconds: 30,
        latest_status: HealthCheckStatus{
            status: CheckStatus::Unhealthy,
            status_message: "Internal error".to_string(),
            response_time: 21,
            timestamp: Utc::now(),
        },
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
            <ServiceCard service_info=service1 />
            <ServiceCard service_info=service2 />
            <ServiceCard service_info=service3 />
        </div>
    }
}