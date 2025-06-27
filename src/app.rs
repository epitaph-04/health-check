use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

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
            <main class="min-h-screen">
                <header class="bg-slate-800 shadow-md sticky top-0 z-50">
                    <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex items-center justify-between h-16">
                            <div class="flex items-center">
                                <svg xmlns="http://www.w3.org/2000/svg"
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
                                    <path d="M20 7h-9"/>
                                    <path d="M14 17H5" />
                                    <circle cx="17" cy="17" r="3"/>
                                    <circle cx="7" cy="7" r="3" />
                                </svg>
                                <h1 class="text-2xl font-bold text-slate-100">"Health Monitor"</h1>
                            </div>
                        </div>
                    </div>
                </header>
                <div class="container mx-auto p-4 sm:p-6 lg:p-8">
                    <Routes fallback=move || "Not found.">
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=WildcardSegment("any") view=NotFound />
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
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
