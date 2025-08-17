use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn NavLink(
    href: &'static str,
    text: &'static str,
) -> impl IntoView {
    let location = use_location();

    let is_active = move || location.pathname.get() == href;

    view! {
        <a
            href=href
            class=move || {
                if is_active() {
                    "px-4 py-1 rounded bg-purple-600 text-white font-medium transition-colors"
                } else {
                    "px-4 py-1 rounded bg-gray-700 text-white border border-gray-600 hover:bg-gray-600 transition-colors"
                }
            }
        >
            {text}
        </a>
    }
}
