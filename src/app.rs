use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use crate::views::{Alerts, Analytics, Dashboard, Dependencies, NotFound, Services};
use crate::components::NavLink;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/health-check.css"/>
        <Title text="Health Monitor"/>
        <Router>
            <div class="min-h-screen bg-gray-900 text-white font-sans">
                <header class="sticky top-0 z-50 bg-gray-800 p-4 flex items-center justify-between shadow-md">
                    <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="flex items-center justify-between h-16">
                            <div class="flex items-center">
                                <a href="/" class="flex items-center">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="28"
                                        height="28"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="text-purple-500 mr-2 h-7 w-7"
                                    >
                                        <path d="M20 7h-9"/>
                                        <path d="M14 17H5"/>
                                        <circle cx="17" cy="17" r="3"/>
                                        <circle cx="7" cy="7" r="3"/>
                                    </svg>
                                    <h1 class="text-2xl font-bold text-slate-100">Health Monitor</h1>
                                </a>
                            </div>
                            <div class="flex gap-2">
                                <NavLink href="/" text="Dashboard"/>
                                <NavLink href="/services" text="Services"/>
                                <NavLink href="/analytics" text="Analytics"/>
                                <NavLink href="/alerts" text="Alerts"/>
                                <NavLink href="/dependencies" text="Dependencies"/>
                            </div>
                        </div>
                    </div>
                </header>
                <main class="container mx-auto p-4 sm:p-6 lg:p-8">
                    <Routes fallback=move || view! { <NotFound/> }>
                        <Route path=StaticSegment("") view=Dashboard/>
                        <Route path=StaticSegment("services") view=Services/>
                        <Route path=StaticSegment("analytics") view=Analytics/>
                        <Route path=StaticSegment("alerts") view=Alerts/>
                        <Route path=StaticSegment("dependencies") view=Dependencies/>
                        <Route path=WildcardSegment("any") view=NotFound/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

