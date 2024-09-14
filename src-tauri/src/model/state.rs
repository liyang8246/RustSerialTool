use std::sync::Arc;
use serialport::COMPort;
use tauri::async_runtime::Mutex;


pub type State = Arc<Mutex<AppState>>;
#[derive(Debug)]
pub struct AppState {
    pub serial: Option<COMPort>,
}