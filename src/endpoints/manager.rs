use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy)]
pub enum WhatsAppEndpoint {
    LinkedDeviceCode,
}

pub fn resolve(name: &str) -> Result<WhatsAppEndpoint> {
    match name {
        "linked-device-code"
        | "pair-code"
        | "phone-number" => {
            Ok(WhatsAppEndpoint::LinkedDeviceCode)
        }

        other => {
            bail!(
                "Endpoint de WhatsApp desconocido: {other}"
            );
        }
    }
}
