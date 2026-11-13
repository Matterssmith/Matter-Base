use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"{
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
}"#;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub name: String,
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
    pub phone_number: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct StorageSettings {
    pub database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Matter Base".into(),
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
            endpoint: "linked-device-code".into(),
            phone_number: String::new(),
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

impl WhatsAppSettings {
    pub fn normalized_phone_number(&self) -> Result<String> {
        let raw = self.phone_number.trim();

        if raw.is_empty() {
            bail!(
                "Falta configurar whatsapp.phone_number en config/bot.json"
            );
        }

        let valid_format = raw.chars().all(|character| {
            character.is_ascii_digit()
                || matches!(
                    character,
                    '+' | ' ' | '-' | '(' | ')' | '.'
                )
        });

        if !valid_format {
            bail!(
                "El número de WhatsApp contiene caracteres no válidos"
            );
        }

        let digits: String = raw
            .chars()
            .filter(|character| character.is_ascii_digit())
            .collect();

        if !(8..=15).contains(&digits.len()) {
            bail!(
                "El número debe contener entre 8 y 15 dígitos, incluido el código de país"
            );
        }

        if digits.starts_with('0') {
            bail!(
                "El número internacional no debe comenzar con cero"
            );
        }

        Ok(digits)
    }
}

impl AppConfig {
    pub fn load_or_create() -> Result<Self> {
        let directory = Path::new("config");
        let path = directory.join("bot.json");

        fs::create_dir_all(directory)
            .context("No se pudo crear config")?;

        if !path.exists() {
            fs::write(&path, DEFAULT_CONFIG)
                .context("No se pudo crear config/bot.json")?;
        }

        let content = fs::read_to_string(&path)
            .context("No se pudo leer config/bot.json")?;

        let config: Self = serde_json::from_str(&content)
            .context("La configuración JSON no es válida")?;

        if config.name.trim().is_empty() {
            bail!("El nombre de Matter Base no puede estar vacío");
        }

        if config.bot.prefix.trim().is_empty() {
            bail!("El prefijo del bot no puede estar vacío");
        }

        if config.storage.database.trim().is_empty() {
            bail!("La ruta de almacenamiento está vacía");
        }

        config.whatsapp.normalized_phone_number()?;

        Ok(config)
    }
}
