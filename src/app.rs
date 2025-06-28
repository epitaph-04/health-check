use chrono::Utc;
use leptos::prelude::*;
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
    let (service, _) = signal(ServiceHealthCheckInfo{
        name: "Google".to_string(),
        url: "https://google.com".to_string(),
        service_type: ServiceType::Http,
        interval_seconds: 30,
        latest_status: HealthCheckStatus{
            status: CheckStatus::Unhealthy,
            status_message: "Ok".to_string(),
            response_time: 21,
            timestamp: Utc::now(),
        },
    });
    view! {
        <div class="view-content grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-6">
            <ServiceCard service_info=service />
        </div>
    }
}

#[component]
fn ServiceCard(service_info: ReadSignal<ServiceHealthCheckInfo>) -> impl IntoView {
    let info = service_info.get();
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
                <div class="flex items-center min-w-0">
                    <StatusIcon status=info.clone().latest_status.status />
                    <h3
                        class=match info.latest_status.status {
                            CheckStatus::Healthy => {
                                "ml-2 text-base sm:text-lg font-semibold truncate text-green-400"
                            }
                            CheckStatus::Degraded => {
                                "ml-2 text-base sm:text-lg font-semibold truncate text-orange-400"
                            }
                            CheckStatus::Unhealthy => {
                                "ml-2 text-base sm:text-lg font-semibold truncate text-red-400"
                            }
                        }
                        title=info.name
                    >
                        {info.clone().name}
                    </h3>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatusIcon(status: CheckStatus) -> impl IntoView {
    let icon = match status {
        CheckStatus::Healthy => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                view_box="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="icon-text-align text-green-500"
            >
                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                <polyline points="22 4 12 14.01 9 11.01" />
            </svg>
            <div class="ml-2 text-sm font-medium text-green-400">{status.to_string()}</div>
        },
        CheckStatus::Degraded => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                view_box="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="icon-text-align text-orange-500"
            >
                <path d="m21.73 18-8-14a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
                <polyline points="22 4 12 14.01 9 11.01" />
            </svg>
            <div class="ml-2 text-sm font-medium text-orange-400">{status.to_string()}</div>
        },
        CheckStatus::Unhealthy => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                view_box="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="icon-text-align text-red-500"
            >
                <path d="M22,12 A10,10 0 1,1 2,12 A10,10 0 1,1 22,12" />
                <polyline points="15,9 9,15 9,9 15,15" />
            </svg>
            <div class="ml-2 text-sm font-medium text-red-400">{status.to_string()}</div>
        },
    };

    view! { <div class="flex items-center">{icon}</div> }
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
