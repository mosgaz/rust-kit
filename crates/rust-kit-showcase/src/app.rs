use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="text-2xl font-bold">"Rust-Kit Showcase"</h1>
            <p>"Если видишь эту страницу — витрина собралась."</p>
        </div>
    }
}