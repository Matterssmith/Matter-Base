use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy)]
pub enum WhatsAppEndpoint {
    LinkedDeviceQr,
}

pub fn resolve(name: &str) -> Result<WhatsAppEndpoint> {
    match name {
        "linked-device-qr" | "qr" => {
            Ok(WhatsAppEndpoint::LinkedDeviceQr)
        }

        other => {
            bail!("Endpoint de WhatsApp desconocido: {other}");
        }
    }
}
