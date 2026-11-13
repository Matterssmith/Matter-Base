use anyhow::{Context, Result};
use std::{
    fs,
    path::Path,
};

use tracing::{info, warn};
use whatsapp_rust::prelude::*;

use crate::{
    core::config::AppConfig,
    endpoints::manager::WhatsAppEndpoint,
    router,
};

pub async fn start(
    config: AppConfig,
    endpoint: WhatsAppEndpoint,
) -> Result<()> {
    match endpoint {
        WhatsAppEndpoint::LinkedDeviceQr => {
            start_qr(config).await
        }
    }
}

async fn start_qr(config: AppConfig) -> Result<()> {
    info!("Preparando conexión WhatsApp");

    let database = config.storage.database.clone();

    let database_path = Path::new(&database);

    if let Some(parent) = database_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .context("No se pudo crear el directorio de sesiones")?;
        }
    }

    let backend = SqliteStore::new(database.as_str())
        .await
        .context("No se pudo inicializar SQLite")?;

    let enabled = config.bot.enabled;
    let prefix = config.bot.prefix.clone();

    if enabled {
        info!("Matter Base habilitado");
    } else {
        info!("Matter Base desactivado; conexión en modo pasivo");
    }

    let bot = Bot::builder()
        .with_backend(backend)

        .on_qr_code(|code, timeout| async move {
            println!();
            println!("======================================");
            println!("        MATTER BASE - WHATSAPP");
            println!("======================================");
            println!("Escanea el siguiente código con WhatsApp:");
            println!();
            println!("{code}");
            println!();
            println!("Tiempo disponible: {} segundos", timeout.as_secs());
        })

        .on_connected(|_client| async {
            info!("WhatsApp conectado correctamente");
        })

        .on_message(move |ctx| {
            let prefix = prefix.clone();

            async move {
                let Some(message) = ctx
                    .message
                    .text_content()
                    .map(str::to_owned)
                else {
                    return;
                };

                if !enabled {
                    return;
                }

                router::dispatch(
                    ctx,
                    message,
                    prefix,
                ).await;
            }
        })

        .build()
        .await
        .context("No se pudo construir el cliente WhatsApp")?;

    info!("Iniciando cliente WhatsApp");

    let reason = bot.run().await;

    warn!("La sesión WhatsApp terminó: {:?}", reason);

    Ok(())
}
