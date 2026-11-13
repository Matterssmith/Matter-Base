use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"{
    "operation_mode": "normal",
    "audience": "SFW",
    "bot": {
        "enabled": false,
        "prefix": "#"
    },
    "whatsapp": {
        "provider": "whatsapp-rust",
        "endpoint": "linked-device-qr"
    },
    "storage": {
        "database": "data/whatsapp/session.db"
    }
}"#;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub operation_mode: String,
    pub audience: String,
    pub bot: BotSettings,
    pub whatsapp: WhatsAppSettings,
    pub storage: StorageSettings,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct BotSettings {
    pub enabled: bool,
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct WhatsAppSettings {
    pub provider: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct StorageSettings {
    pub database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            operation_mode: "normal".into(),
            audience: "SFW".into(),
            bot: BotSettings::default(),
            whatsapp: WhatsAppSettings::default(),
            storage: StorageSettings::default(),
        }
    }
}

impl Default for BotSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            prefix: "#".into(),
        }
    }
}

impl Default for WhatsAppSettings {
    fn default() -> Self {
        Self {
            provider: "whatsapp-rust".into(),
            endpoint: "linked-device-qr".into(),
        }
    }
}

impl Default for StorageSettings {
    fn default() -> Self {
        Self {
            database: "data/whatsapp/session.db".into(),
        }
    }
}

impl AppConfig {
    pub fn load_or_create() -> Result<Self> {
        let directory = Path::new("config");
        let path = directory.join("bot.json");

        fs::create_dir_all(directory)
            .context("No se pudo crear el directorio de configuración")?;

        if !path.exists() {
            fs::write(&path, DEFAULT_CONFIG)
                .context("No se pudo crear config/bot.json")?;
        }

        let content = fs::read_to_string(&path)
            .context("No se pudo leer config/bot.json")?;

        let config: Self = serde_json::from_str(&content)
            .context("La configuración JSON no es válida")?;

        if config.bot.prefix.trim().is_empty() {
            anyhow::bail!("El prefijo del bot no puede estar vacío");
        }

        if config.storage.database.trim().is_empty() {
            anyhow::bail!("La ruta de la base de datos está vacía");
        }

        Ok(config)
    }
}
