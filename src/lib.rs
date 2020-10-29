#![recursion_limit = "1024"]

#[allow(dead_code)]
mod api;
mod app;
mod auth;
mod bindings;
mod components;
mod form_data;
mod loader_task;
mod routes;

use app::App;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    yew::start_app::<App>();
}
