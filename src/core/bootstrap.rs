use anyhow::Result;
use tracing::info;

use super::{config::AppConfig, logger};
use crate::providers;

pub async fn run() -> Result<()> {
    logger::init();

    info!("Iniciando Matter Base");

    let config = AppConfig::load_or_create()?;

    info!("Configuración cargada");
    info!("Nombre: {}", config.name);
    info!("Modo: {}", config.operation_mode);
    info!("Público: {}", config.audience);

    if config.bot.enabled {
        info!("Estado del bot: habilitado");
    } else {
        info!("Estado del bot: deshabilitado");
    }

    info!("Preparando Provider Manager");

    providers::manager::start(config).await
}
