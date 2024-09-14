use std::io::Read;
use log::*;
use crate::State;

#[tauri::command]
pub async fn read_ports(state: tauri::State<'_,State>) -> Result<Vec<u8>, ()> {
    let mut state = state.lock().await;
    let port = state.serial.as_mut().unwrap();
    let mut buffer = [0u8;1024];
    match port.read(&mut buffer) {
        Ok(len) => {
            let buffer = buffer[..len].to_vec();
            debug!("Read bytes: {:?}", buffer);
            return Ok(buffer);
        },
        Err(e) => {
            error!("Error reading port: {}", e);
            return Err(());
        }
    }
}