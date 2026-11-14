use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreConfig {
    pub name: String,
    pub environment: String,
    pub debug: bool,
    pub locale: String,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            name: "Matter Application".to_string(),
            environment: "production".to_string(),
            debug: false,
            locale: "es".to_string(),
        }
    }
}