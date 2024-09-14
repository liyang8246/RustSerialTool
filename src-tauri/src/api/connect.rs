use serialport::{DataBits, Parity, SerialPort, StopBits};
use crate::{ConnConfig, State};
use log::*;


#[tauri::command]
pub async fn available_ports() -> Result<Vec<String>, ()> {
    let ports = serialport::available_ports().unwrap();
    let ports = ports.iter().map(|port| port.port_name.to_string()).collect::<Vec<_>>();
    Ok(ports)
}

#[tauri::command]
pub async fn connect(state: tauri::State<'_,State>, config: ConnConfig) -> Result<String, String> {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        if port.port_name != config.port {
            continue;
        }
        let mut rx = serialport::new(port.port_name, config.baudrate).open_native().unwrap();
        match config.databits {
            5 => rx.set_data_bits(DataBits::Five),
            6 => rx.set_data_bits(DataBits::Six),
            7 => rx.set_data_bits(DataBits::Seven),
            8 => rx.set_data_bits(DataBits::Eight),
            _ => panic!("Invalid data bits"),
        }.unwrap();
        match config.parity {
            0 => rx.set_parity(Parity::None),
            1 => rx.set_parity(Parity::Odd),
            2 => rx.set_parity(Parity::Even),
            _ => panic!("Invalid parity"),
        }.unwrap();
        match config.stopbits {
            1 => rx.set_stop_bits(StopBits::One),
            2 => rx.set_stop_bits(StopBits::Two),
            _ => panic!("Invalid stop bits"),
        }.unwrap();
        state.lock().await.serial = Some(rx);
        debug!("Connected");
        return Ok("Connected".to_string());
    }
    error!("Port not found");
    Err("not found".to_string())
}

#[tauri::command]
pub async fn disconnect(state: tauri::State<'_,State>) -> Result<String, String> {
    state.lock().await.serial = None;
    debug!("Disconnected");
    Ok("Disconnected".to_string())
}