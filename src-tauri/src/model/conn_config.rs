use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnConfig {
    pub port: String,
    pub baudrate: u32,
    pub databits: u8,
    pub parity: u8,
    pub stopbits: u8,
}