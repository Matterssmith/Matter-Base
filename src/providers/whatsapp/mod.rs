use anyhow::{Context, Result};
use std::{
    fs,
    path::Path,
};

use tracing::{info, warn};

use whatsapp_rust::{
    pair_code::PairCodeOptions,
    prelude::*,
};

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
        WhatsAppEndpoint::LinkedDeviceCode => {
            start_pairing_code(config).await
        }
    }
}

async fn start_pairing_code(
    config: AppConfig,
) -> Result<()> {
    info!("Preparando conexión WhatsApp");

    let phone_number = config
        .whatsapp
        .normalized_phone_number()?;

    let database = config.storage.database.clone();
    let database_path = Path::new(&database);

    if let Some(parent) = database_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .context(
                    "No se pudo crear el directorio de sesiones"
                )?;
        }
    }

    let backend = SqliteStore::new(database.as_str())
        .await
        .context("No se pudo inicializar SQLite")?;

    let enabled = config.bot.enabled;
    let prefix = config.bot.prefix.clone();

    let refresh_phone = phone_number.clone();

    if enabled {
        info!("Matter Base está habilitado");
    } else {
        info!("Matter Base está en modo pasivo");
    }

    info!("Proveedor: whatsapp-rust");
    info!("Método de vinculación: código por número");
    info!("Preparando almacenamiento de sesión");

    let bot = Bot::builder()
        .with_backend(backend)

        .with_pair_code(PairCodeOptions {
            phone_number: phone_number.clone(),
            ..Default::default()
        })

        .on_pair_code(|code, timeout| async move {
            println!();

            println!("==========================================");
            println!("             MATTER BASE");
            println!("==========================================");

            println!();
            println!("Código de vinculación:");
            println!();

            println!("              {code}");

            println!();
            println!(
                "Vigencia informada: {} segundos",
                timeout.as_secs()
            );

            println!();
            println!("En tu teléfono:");
            println!("1. Abre WhatsApp.");
            println!("2. Entra en Dispositivos vinculados.");
            println!("3. Selecciona Vincular dispositivo.");
            println!("4. Selecciona Vincular con número de teléfono.");
            println!("5. Introduce el código mostrado aquí.");

            println!();
            println!("==========================================");
            println!();
        })

        .on_pair_code_error(|error, _client| async move {
            warn!(
                "No se pudo generar el código de vinculación: {}",
                error.error
            );

            if let Some(backoff) = error.backoff {
                warn!(
                    "WhatsApp solicita esperar {} segundos antes de reintentar",
                    backoff.as_secs()
                );
            }
        })

        .on_pair_code_refresh(move |force_manual, client| {
            let phone_number = refresh_phone.clone();

            async move {
                info!(
                    "WhatsApp solicitó renovar el código. Manual: {}",
                    force_manual
                );

                let options = PairCodeOptions {
                    phone_number,
                    ..Default::default()
                };

                if let Err(error) = client
                    .pair_with_code(options)
                    .await
                {
                    warn!(
                        "No se pudo renovar el código de vinculación: {}",
                        error
                    );
                }
            }
        })

        .on_connected(|_client| async {
            println!();
            println!("==========================================");
            println!("       MATTER BASE CONECTADO");
            println!("==========================================");
            println!();

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
                )
                .await;
            }
        })

        .build()
        .await
        .context(
            "No se pudo construir el cliente WhatsApp"
        )?;

    info!("Iniciando cliente WhatsApp");

    bot.run().await;

    info!("La sesión de WhatsApp terminó");

    Ok(())
}
