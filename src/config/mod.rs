pub mod provider;
pub mod settings;
pub mod credentials;

use std::path::{Path, PathBuf};
use anyhow::{Result, Context, anyhow};
use log::{debug, info, error};
use std::fs;
use dirs::home_dir;

/// Constants
pub const APP_DIR_NAME: &str = ".bbctl";
pub const SETTINGS_FILE: &str = "settings.toml";
pub const PROVIDERS_FILE: &str = "providers.toml";
pub const CREDENTIALS_FILE: &str = "credentials.toml";

/// Get the application config directory
pub fn get_config_dir() -> Result<PathBuf> {
    let home = home_dir().ok_or_else(|| anyhow!("Failed to determine home directory"))?;
    let config_dir = home.join(APP_DIR_NAME);
    
    // Create the directory if it doesn't exist
    if !config_dir.exists() {
        debug!("Creating config directory: {}", config_dir.display());
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
    }
    
    Ok(config_dir)
}

/// Get a path to a config file
pub fn get_config_file(file_name: &str) -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(file_name))
}

/// Check if a config file exists
pub fn config_file_exists(file_name: &str) -> Result<bool> {
    let path = get_config_file(file_name)?;
    Ok(path.exists())
}

/// Read a config file as a string
pub fn read_config_file(file_name: &str) -> Result<String> {
    let path = get_config_file(file_name)?;
    if !path.exists() {
        return Err(anyhow!("Config file does not exist: {}", path.display()));
    }
    
    fs::read_to_string(&path)
        .context(format!("Failed to read config file: {}", path.display()))
}

/// Write a string to a config file
pub fn write_config_file(file_name: &str, content: &str) -> Result<()> {
    let path = get_config_file(file_name)?;
    
    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create parent directory: {}", parent.display()))?;
        }
    }
    
    fs::write(&path, content)
        .context(format!("Failed to write config file: {}", path.display()))
}

/// Delete a config file
pub fn delete_config_file(file_name: &str) -> Result<()> {
    let path = get_config_file(file_name)?;
    if path.exists() {
        fs::remove_file(&path)
            .context(format!("Failed to delete config file: {}", path.display()))?;
    }
    Ok(())
}

/// Initialize configuration directory and default files
pub fn init_config() -> Result<()> {
    // Create config directory
    let config_dir = get_config_dir()?;
    info!("Initializing configuration in: {}", config_dir.display());
    
    // Create default settings file if it doesn't exist
    let settings_path = config_dir.join(SETTINGS_FILE);
    if !settings_path.exists() {
        debug!("Creating default settings file");
        let default_settings = settings::Settings::default();
        let toml = toml::to_string_pretty(&default_settings)
            .context("Failed to serialize default settings")?;
        fs::write(&settings_path, toml)
            .context(format!("Failed to write settings file: {}", settings_path.display()))?;
    }
    
    // Create empty providers file if it doesn't exist
    let providers_path = config_dir.join(PROVIDERS_FILE);
    if !providers_path.exists() {
        debug!("Creating empty providers file");
        let providers = provider::Providers::default();
        let toml = toml::to_string_pretty(&providers)
            .context("Failed to serialize empty providers")?;
        fs::write(&providers_path, toml)
            .context(format!("Failed to write providers file: {}", providers_path.display()))?;
    }
    
    // Create empty credentials file if it doesn't exist
    let credentials_path = config_dir.join(CREDENTIALS_FILE);
    if !credentials_path.exists() {
        debug!("Creating empty credentials file");
        let credentials = credentials::Credentials::default();
        let toml = toml::to_string_pretty(&credentials)
            .context("Failed to serialize empty credentials")?;
        fs::write(&credentials_path, toml)
            .context(format!("Failed to write credentials file: {}", credentials_path.display()))?;
    }
    
    info!("Configuration initialized successfully");
    Ok(())
}