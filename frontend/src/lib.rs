//! Frontend application with Leptos 0.8.14
//! SSR with Axum + Hydration
#![recursion_limit = "1024"]

pub mod app;
pub mod components;
pub mod pages;

pub use app::App;

#[cfg(feature = "ssr")]
pub use app::shell;

/// Hydration entry point for WASM
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
