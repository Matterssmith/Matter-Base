use anyhow::Result;
use tracing::info;

use super::{config::AppConfig, logger};
use crate::providers;

pub async fn run() -> Result<()> {
    logger::init();

    info!("Iniciando Matter Base");

    let config = AppConfig::load_or_create()?;

    info!("Configuración cargada");
    info!("Preparando Provider Manager");

    providers::manager::start(config).await
}
