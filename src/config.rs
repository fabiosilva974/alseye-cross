//! Configurações do ALSEYE BEM (Compatível com Setting.ini)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub temperature_unit: u8, // 1 = °C, 0 = °F
    pub auto_start: bool,
    pub tft_open: bool,
    pub tft_speed: u8, // 0 = 5s, 1 = 3s, 2 = 1s
    pub tft_animation: bool,
    pub tft_turn: u8, // 0 = 0°, 1 = 90°, 2 = 180°, 3 = 270°

    // Toggles de Métricas da CPU
    pub tft_ctemp: bool,
    pub tft_cfreq: bool,
    pub tft_ctrpm: bool,
    pub tft_cpower: bool,

    // Toggles de Métricas da GPU
    pub tft_gtemp: bool,
    pub tft_gfreq: bool,
    pub tft_gtrpm: bool,
    pub tft_gpower: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            temperature_unit: 1,
            auto_start: true,
            tft_open: true,
            tft_speed: 2, // 1 segundo
            tft_animation: true,
            tft_turn: 0,
            tft_ctemp: true,
            tft_cfreq: true,
            tft_ctrpm: true,
            tft_cpower: true,
            tft_gtemp: true,
            tft_gfreq: true,
            tft_gtrpm: true,
            tft_gpower: true,
        }
    }
}

impl AppConfig {
    pub fn load_or_default(path: &Path) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(cfg) = serde_json::from_str(&content) {
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save(&self, path: &Path) {
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }
}

