use anyhow::{bail, Result};
use tracing::info;

use crate::{
    core::config::AppConfig,
    endpoints::manager,
    providers::whatsapp,
};

pub async fn start(config: AppConfig) -> Result<()> {
    info!("Provider Manager iniciado");

    match config.whatsapp.provider.as_str() {
        "whatsapp-rust" | "rust" => {
            info!("Proveedor seleccionado: WhatsApp Rust");

            let endpoint = manager::resolve(
                &config.whatsapp.endpoint
            )?;

            info!("Endpoint seleccionado: {:?}", endpoint);

            whatsapp::start(config, endpoint).await
        }

        other => {
            bail!("Proveedor no registrado: {other}");
        }
    }
}
