
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = r##"{
    "name": "Matter Base",
    "operation_mode": "normal",
    "audience": "SFW",
    "bot": {
        "enabled": false,
        "prefix": "#"
    },
    "whatsapp": {
        "provider": "whatsapp-rust",
        "endpoint": "linked-device-code",
        "phone_number": ""
    },
    "storage": {
        "database": "data/whatsapp/session.db"
    }
}"##;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub name: String,
    pub operation_mode: String,
    pub audience: String,
    pub bot: BotSettings,
    pub whatsapp: WhatsAppSettings,
    pub storage: StorageSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BotSettings {
    pub enabled: bool,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WhatsAppSettings {
    pub provider: String,
    pub endpoint: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageSettings {
    pub database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_CONFIG)
            .expect("La configuración predeterminada de Matter Base no es válida")
    }
}

impl Default for BotSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            prefix: "#".to_string(),
        }
    }
}

impl Default for WhatsAppSettings {
    fn default() -> Self {
        Self {
            provider: "whatsapp-rust".to_string(),
            endpoint: "linked-device-code".to_string(),
            phone_number: String::new(),
        }
    }
}

impl Default for StorageSettings {
    fn default() -> Self {
        Self {
            database: "data/whatsapp/session.db".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load_or_create() -> Result<Self> {
        let path = Path::new("config/bot.json");

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .context("No se pudo crear el directorio de configuración")?;
            }

            fs::write(
                path,
                serde_json::to_string_pretty(&Self::default())?,
            )
            .context("No se pudo crear config/bot.json")?;
        }

        let content = fs::read_to_string(path)
            .context("No se pudo leer config/bot.json")?;

        let config = serde_json::from_str::<Self>(&content)
            .context("config/bot.json contiene JSON inválido")?;

        Ok(config)
    }
}

impl WhatsAppSettings {
    pub fn normalized_phone_number(&self) -> Result<String> {
        let number: String = self
            .phone_number
            .chars()
            .filter(|character| character.is_ascii_digit())
            .collect();

        if number.len() < 8 || number.len() > 15 {
            anyhow::bail!(
                "El número de WhatsApp debe incluir el código internacional y tener entre 8 y 15 dígitos"
            );
        }

        if number.starts_with('0') {
            anyhow::bail!(
                "El número debe incluir el prefijo internacional, sin un cero inicial"
            );
        }

        Ok(number)
    }
}
