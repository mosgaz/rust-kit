pub mod app;

pub mod components;
pub mod domain;
pub mod utils;
pub mod routes;
pub mod __registry__;
pub mod registry;

pub use app::App;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
