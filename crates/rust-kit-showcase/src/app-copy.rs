use leptos::prelude::*;
use ui::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="p-8 space-y-4">
            <h1 class="text-2xl font-bold">"Rust-Kit Showcase"</h1>
            <Button variant=ButtonVariant::Default>"Click me"</Button>
        </div>
    }
}