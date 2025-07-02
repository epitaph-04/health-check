use chrono::Utc;
use leptos::prelude::*;
use leptos::svg::title;
use leptos::web_sys::console::info;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use crate::types::{CheckStatus, HealthCheckStatus, ServiceHealthCheckInfo, ServiceType};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/health-check.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <div class="min-h-screen">
                <header class="bg-slate-800 shadow-md sticky top-0 z-50">
                    <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex items-center justify-between h-16">
                            <div class="flex items-center">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="28"
                                    height="28"
                                    view_box="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="text-purple-500 mr-2 h-7 w-7"
                                >
                                    <path d="M20 7h-9" />
                                    <path d="M14 17H5" />
                                    <circle cx="17" cy="17" r="3" />
                                    <circle cx="7" cy="7" r="3" />
                                </svg>
                                <h1 class="text-2xl font-bold text-slate-100">"Health Monitor"</h1>
                            </div>
                        </div>
                    </div>
                </header>
                <main class="container mx-auto p-4 sm:p-6 lg:p-8">
                    <Routes fallback=move || "Not found.">
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=WildcardSegment("any") view=NotFound />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
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
        <div id="dashboardView" class="view-content grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-6">
            <ServiceCard service_info=service1 />
            <ServiceCard service_info=service2 />
            <ServiceCard service_info=service3 />
        </div>
    }
}

#[component]
fn ServiceCard(service_info: ReadSignal<ServiceHealthCheckInfo>) -> impl IntoView {
    let info = service_info.get();
    let color = match info.latest_status.status { 
        CheckStatus::Healthy => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-green-500/10 border-green-500/30 text-green-400 mb-1",
        CheckStatus::Degraded => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-orange-500/10 border-orange-500/30 text-orange-400 mb-1",
        CheckStatus::Unhealthy => "text-sm font-medium px-2 py-0.5 rounded inline-block bg-red-500/10 border-red-500/30 text-red-400 mb-1"
    };
    view! {
        <div class=match info.latest_status.status {
            CheckStatus::Healthy => {
                "service-card rounded-lg shadow-lg overflow-hidden bg-green-500/10 border-green-500 border flex flex-col"
            }
            CheckStatus::Degraded => {
                "service-card rounded-lg shadow-lg overflow-hidden bg-orange-500/10 border-orange-500 border flex flex-col"
            }
            CheckStatus::Unhealthy => {
                "service-card rounded-lg shadow-lg overflow-hidden bg-red-500/10 border-red-500 border flex flex-col"
            }
        }>
            <div class="p-3 sm:p-4 flex-grow">
                <div class="flex items-center justify-between mb-2">
                    <div class="flex items-center min-w-0">
                        <CardHeader status=info.clone().latest_status.status title=info.clone().name />
                    </div>
                    <div class="flex space-x-1 flex-shrink-0">
                        <button class="expand-btn p-1 text-slate-400 hover:text-slate-200 rounded-full hover:bg-slate-600/50 transition-colors" title="Toggle Details">
                            <svg class="chevron-down" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polyline points="6 9 12 15 18 9"/>
                            </svg>
                            <svg class="chevron-up hidden" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polyline points="18 15 12 9 6 15"/>
                            </svg>
                        </button>
                    </div>
                </div>
               <p class="text-xs text-slate-400 truncate mb-1" title={info.url}>
                    <span class="font-semibold">{info.service_type.to_string()} @ </span>{info.clone().url}
                </p>
                <div class={color}>
                    {info.latest_status.status.to_string()}
                </div>
                <p class="text-xs text-slate-400 mt-1">Status : {info.latest_status.status_message}</p>
                <div class="flex justify-between text-xs text-slate-400 mt-2">
                    <span class="icon-text-align">
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1">
                            <circle cx="12" cy="12" r="10"/>
                            <polyline points="12 6 12 12 16 14"/>
                        </svg>
                        {info.latest_status.response_time} ms
                    </span>
                    <span class="icon-text-align">
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1">
                            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
                            <line x1="16" y1="2" x2="16" y2="6"/>
                            <line x1="8" y1="2" x2="8" y2="6"/>
                            <line x1="3" y1="10" x2="21" y2="10"/>
                        </svg>
                        {info.latest_status.timestamp.format("%H:%M:%S").to_string()}
                    </span>
                </div>
                <p class="text-xs text-slate-400 mt-1">Interval : {info.interval_seconds}s</p>
            </div>
        </div>
    }
}

#[component]
fn CardHeader(status: CheckStatus, title: String) -> impl IntoView {
    let healthy_icon =  view! {
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
                class="icon-text-align text-green-500">
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                <polyline points="22 4 12 14.01 9 11.01"></polyline>
            </svg>
    };
    let degraded_icon = view! {
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
                class="icon-text-align text-orange-500">
                <path d="m21.73 18-8-14a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>
                <line x1="12" y1="9" x2="12" y2="13"></line>
                <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
    };
    let unhealthy_icon = view! {
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
                class="icon-text-align text-red-500">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="15" y1="9" x2="9" y2="15"></line>
                <line x1="9" y1="9" x2="15" y2="15"></line>
            </svg>
    };
    
    let header = match status {
        CheckStatus::Healthy => view! {
            <div class="flex items-center">
            {healthy_icon}
            <div class="ml-2 text-sm font-medium text-green-400">{status.to_string()}</div>
            </div>
            <h3 class="ml-2 text-base sm:text-lg font-semibold truncate text-green-400" title={title}>{title.clone()}</h3>
        }.into_any(),
        CheckStatus::Degraded => view! {
            <div class="flex items-center">
            {degraded_icon}
            <div class="ml-2 text-sm font-medium text-orange-400">{status.to_string()}</div>
            </div>
            <h3 class="ml-2 text-base sm:text-lg font-semibold truncate text-orange-400" title={title}>{title.clone()}</h3>
        }.into_any(),
        CheckStatus::Unhealthy => view! {
            <div class="flex items-center">
            {unhealthy_icon}
            <div class="ml-2 text-sm font-medium text-red-400">{status.to_string()}</div>
            </div>
            <h3 class="ml-2 text-base sm:text-lg font-semibold truncate text-red-400" title={title}>{title.clone()}</h3>
        }.into_any(),
    };

    view! {{header}}
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
