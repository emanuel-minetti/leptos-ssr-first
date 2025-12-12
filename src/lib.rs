pub mod app;
pub mod layout;
pub mod pages;
pub mod model;
pub mod utils;
pub mod client;
pub mod api;
#[cfg(feature = "ssr")]
pub mod server_utils;

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
