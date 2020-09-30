#![recursion_limit = "1024"]

#[allow(dead_code)]
mod api;
mod app;
mod components;
mod routes;

use app::App;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
