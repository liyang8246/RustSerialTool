// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use log::*;
use tauri::async_runtime::Mutex;
use std::env::set_var;

mod api;
mod model;

use api::*;
use model::*;

fn main() {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Starting app!");
    let state: State = Arc::new(Mutex::new(AppState {
        serial: None
    }));
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![available_ports,connect,disconnect,read_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application !");
}
