use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use std::collections::HashMap;
use log::{debug, info, error};

use crate::config::{read_config_file, write_config_file, CREDENTIALS_FILE};
use crate::models::provider::ProviderType;

/// VyOS credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyOSCredentials {
    /// Username
    pub username: String,
    /// Password (if using password auth)
    pub password: Option<String>,
    /// SSH key path (if using key auth)
    pub key_path: Option<String>,
    /// API key for HTTP API
    pub api_key: Option<String>,
    /// SSH port
    pub ssh_port: Option<u16>,
    /// HTTP API port
    pub api_port: Option<u16>,
}

/// Proxmox API token authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxTokenAuth {
    /// Token ID
    pub token_id: String,
    /// Token secret
    pub token_secret: String,
}

/// Proxmox username/password authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxUserPassAuth {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Authentication realm
    pub realm: String,
}

/// Proxmox credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxCredentials {
    /// API port
    pub port: Option<u16>,
    /// Use token authentication
    pub use_token_auth: bool,
    /// Token authentication
    pub token_auth: Option<ProxmoxTokenAuth>,
    /// Username/password authentication
    pub user_pass_auth: Option<ProxmoxUserPassAuth>,
    /// Verify SSL certificates
    pub verify_ssl: bool,
}

/// Provider credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderCredentials {
    VyOS(VyOSCredentials),
    Proxmox(ProxmoxCredentials),
}

/// Credentials storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Credentials by provider name
    pub credentials: HashMap<String, ProviderCredentials>,
}

impl Default for Credentials {
    fn default() -> Self {
        Self {
            credentials: HashMap::new(),
        }
    }
}

impl Credentials {
    /// Load credentials from file
    pub fn load() -> Result<Self> {
        debug!("Loading credentials from file");
        
        // Read credentials file
        let content = match read_config_file(CREDENTIALS_FILE) {
            Ok(content) => content,
            Err(e) => {
                info!("Failed to read credentials file, using defaults: {}", e);
                return Ok(Self::default());
            }
        };
        
        // Parse TOML
        let credentials: Credentials = toml::from_str(&content)
            .context("Failed to parse credentials TOML")?;
        
        Ok(credentials)
    }
    
    /// Save credentials to file
    pub fn save(&self) -> Result<()> {
        debug!("Saving credentials to file");
        
        // Serialize to TOML
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize credentials")?;
        
        // Write to file
        write_config_file(CREDENTIALS_FILE, &content)
            .context("Failed to write credentials file")?;
        
        info!("Credentials saved successfully");
        Ok(())
    }
    
    /// Add VyOS credentials
    pub fn add_vyos_credentials(
        &mut self,
        provider_name: &str,
        username: &str,
        password: Option<String>,
        key_path: Option<String>,
        api_key: Option<String>,
        ssh_port: Option<u16>,
        api_port: Option<u16>,
    ) -> Result<()> {
        let creds = VyOSCredentials {
            username: username.to_string(),
            password,
            key_path,
            api_key,
            ssh_port,
            api_port,
        };
        
        self.credentials.insert(provider_name.to_string(), ProviderCredentials::VyOS(creds));
        info!("Added VyOS credentials for provider: {}", provider_name);
        Ok(())
    }
    
    /// Add Proxmox token credentials
    pub fn add_proxmox_token_credentials(
        &mut self,
        provider_name: &str,
        token_id: &str,
        token_secret: &str,
        port: Option<u16>,
        verify_ssl: bool,
    ) -> Result<()> {
        let token_auth = ProxmoxTokenAuth {
            token_id: token_id.to_string(),
            token_secret: token_secret.to_string(),
        };
        
        let creds = ProxmoxCredentials {
            port,
            use_token_auth: true,
            token_auth: Some(token_auth),
            user_pass_auth: None,
            verify_ssl,
        };
        
        self.credentials.insert(provider_name.to_string(), ProviderCredentials::Proxmox(creds));
        info!("Added Proxmox token credentials for provider: {}", provider_name);
        Ok(())
    }
    
    /// Add Proxmox username/password credentials
    pub fn add_proxmox_user_pass_credentials(
        &mut self,
        provider_name: &str,
        username: &str,
        password: &str,
        realm: &str,
        port: Option<u16>,
        verify_ssl: bool,
    ) -> Result<()> {
        let user_pass_auth = ProxmoxUserPassAuth {
            username: username.to_string(),
            password: password.to_string(),
            realm: realm.to_string(),
        };
        
        let creds = ProxmoxCredentials {
            port,
            use_token_auth: false,
            token_auth: None,
            user_pass_auth: Some(user_pass_auth),
            verify_ssl,
        };
        
        self.credentials.insert(provider_name.to_string(), ProviderCredentials::Proxmox(creds));
        info!("Added Proxmox user/pass credentials for provider: {}", provider_name);
        Ok(())
    }
    
    /// Remove credentials for a provider
    pub fn remove_credentials(&mut self, provider_name: &str) -> Result<()> {
        if !self.credentials.contains_key(provider_name) {
            return Err(anyhow!("Credentials for provider '{}' do not exist", provider_name));
        }
        
        self.credentials.remove(provider_name);
        info!("Removed credentials for provider: {}", provider_name);
        Ok(())
    }
    
    /// Get credentials for a provider
    pub fn get_credentials(&self, provider_name: &str) -> Option<&ProviderCredentials> {
        self.credentials.get(provider_name)
    }
    
    /// Get VyOS credentials for a provider
    pub fn get_vyos_credentials(&self, provider_name: &str) -> Result<&VyOSCredentials> {
        match self.credentials.get(provider_name) {
            Some(ProviderCredentials::VyOS(creds)) => Ok(creds),
            Some(_) => Err(anyhow!("Provider '{}' does not have VyOS credentials", provider_name)),
            None => Err(anyhow!("No credentials found for provider '{}'", provider_name)),
        }
    }
    
    /// Get Proxmox credentials for a provider
    pub fn get_proxmox_credentials(&self, provider_name: &str) -> Result<&ProxmoxCredentials> {
        match self.credentials.get(provider_name) {
            Some(ProviderCredentials::Proxmox(creds)) => Ok(creds),
            Some(_) => Err(anyhow!("Provider '{}' does not have Proxmox credentials", provider_name)),
            None => Err(anyhow!("No credentials found for provider '{}'", provider_name)),
        }
    }
}