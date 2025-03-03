use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use std::fs;
use log::{debug, info, error};

use crate::config::{read_config_file, write_config_file, SETTINGS_FILE};

/// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Default provider to use
    pub default_provider: Option<String>,
    /// Default region to use
    pub default_region: Option<String>,
    /// Enable telemetry
    pub telemetry_enabled: bool,
    /// Enable auto-update
    pub auto_update_enabled: bool,
    /// Terminal colors
    pub colors_enabled: bool,
    /// Default instance size (CPU cores)
    pub default_cpu: u8,
    /// Default instance size (memory in GB)
    pub default_memory_gb: u8,
    /// Default instance size (disk in GB)
    pub default_disk_gb: u8,
    /// Logging level
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_provider: None,
            default_region: None,
            telemetry_enabled: false,
            auto_update_enabled: true,
            colors_enabled: true,
            default_cpu: 1,
            default_memory_gb: 2,
            default_disk_gb: 10,
            log_level: "info".to_string(),
        }
    }
}

impl Settings {
    /// Load settings from file
    pub fn load() -> Result<Self> {
        debug!("Loading settings from file");
        
        // Read settings file
        let content = match read_config_file(SETTINGS_FILE) {
            Ok(content) => content,
            Err(e) => {
                info!("Failed to read settings file, using defaults: {}", e);
                return Ok(Self::default());
            }
        };
        
        // Parse TOML
        let settings: Settings = toml::from_str(&content)
            .context("Failed to parse settings TOML")?;
        
        Ok(settings)
    }
    
    /// Save settings to file
    pub fn save(&self) -> Result<()> {
        debug!("Saving settings to file");
        
        // Serialize to TOML
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize settings")?;
        
        // Write to file
        write_config_file(SETTINGS_FILE, &content)
            .context("Failed to write settings file")?;
        
        info!("Settings saved successfully");
        Ok(())
    }
    
    /// Update a setting
    pub fn update(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default_provider" => {
                self.default_provider = Some(value.to_string());
            },
            "default_region" => {
                self.default_region = Some(value.to_string());
            },
            "telemetry_enabled" => {
                self.telemetry_enabled = value.parse::<bool>()
                    .context("Invalid boolean value for telemetry_enabled")?;
            },
            "auto_update_enabled" => {
                self.auto_update_enabled = value.parse::<bool>()
                    .context("Invalid boolean value for auto_update_enabled")?;
            },
            "colors_enabled" => {
                self.colors_enabled = value.parse::<bool>()
                    .context("Invalid boolean value for colors_enabled")?;
            },
            "default_cpu" => {
                self.default_cpu = value.parse::<u8>()
                    .context("Invalid value for default_cpu")?;
            },
            "default_memory_gb" => {
                self.default_memory_gb = value.parse::<u8>()
                    .context("Invalid value for default_memory_gb")?;
            },
            "default_disk_gb" => {
                self.default_disk_gb = value.parse::<u8>()
                    .context("Invalid value for default_disk_gb")?;
            },
            "log_level" => {
                // Validate log level
                match value.to_lowercase().as_str() {
                    "trace" | "debug" | "info" | "warn" | "error" => {
                        self.log_level = value.to_lowercase();
                    },
                    _ => return Err(anyhow!("Invalid log level. Use: trace, debug, info, warn, error")),
                }
            },
            _ => return Err(anyhow!("Unknown setting: {}", key)),
        }
        
        Ok(())
    }
}