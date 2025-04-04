use anyhow::{Result, Context, anyhow};
use log::{debug, info, error};
use std::collections::HashMap;

use crate::models::provider::{ProviderType, ProviderConfig, Region, ResourceLimits};
use crate::config::provider::Providers;
use crate::config::credentials::{Credentials, ProviderCredentials};
use crate::api::{Provider, vyos::VyOSClient, vyos::VyOSConfig, proxmox::ProxmoxClient, proxmox::ProxmoxConfig, proxmox::ProxmoxAuth};

/// Provider service for managing infrastructure providers
pub struct ProviderService {
    providers: Providers,
    credentials: Credentials,
}

impl ProviderService {
    /// Create a new provider service
    pub fn new() -> Result<Self> {
        let providers = Providers::load()?;
        let credentials = Credentials::load()?;
        
        Ok(Self {
            providers,
            credentials,
        })
    }
    
    /// Get provider configs
    pub fn get_providers(&self) -> &HashMap<String, ProviderConfig> {
        self.providers.get_all_providers()
    }
    
    /// Get regions
    pub fn get_regions(&self) -> &HashMap<String, Region> {
        self.providers.get_all_regions()
    }
    
    /// Get regions by provider
    pub fn get_regions_by_provider(&self, provider_type: ProviderType) -> Vec<&Region> {
        self.providers.get_regions_by_provider(provider_type)
    }
    
    /// Add a new VyOS provider
    pub fn add_vyos_provider(
        &mut self,
        name: &str,
        host: &str,
        username: &str,
        password: Option<String>,
        key_path: Option<String>,
        api_key: Option<String>,
        ssh_port: Option<u16>,
        api_port: Option<u16>,
    ) -> Result<()> {
        // Create provider params
        let mut params = HashMap::new();
        
        // Add provider
        self.providers.add_provider(name, ProviderType::VyOS, host, params)?;
        
        // Add credentials
        self.credentials.add_vyos_credentials(
            name,
            username,
            password,
            key_path,
            api_key,
            ssh_port,
            api_port,
        )?;
        
        // Save changes
        self.providers.save()?;
        self.credentials.save()?;
        
        info!("Added VyOS provider: {}", name);
        Ok(())
    }
    
    /// Add a new Proxmox provider with token auth
    pub fn add_proxmox_provider_with_token(
        &mut self,
        name: &str,
        host: &str,
        token_id: &str,
        token_secret: &str,
        port: Option<u16>,
        verify_ssl: bool,
    ) -> Result<()> {
        // Create provider params
        let mut params = HashMap::new();
        
        // Add provider
        self.providers.add_provider(name, ProviderType::Proxmox, host, params)?;
        
        // Add credentials
        self.credentials.add_proxmox_token_credentials(
            name,
            token_id,
            token_secret,
            port,
            verify_ssl,
        )?;
        
        // Save changes
        self.providers.save()?;
        self.credentials.save()?;
        
        info!("Added Proxmox provider with token auth: {}", name);
        Ok(())
    }
    
    /// Add a new Proxmox provider with username/password auth
    pub fn add_proxmox_provider_with_user_pass(
        &mut self,
        name: &str,
        host: &str,
        username: &str,
        password: &str,
        realm: &str,
        port: Option<u16>,
        verify_ssl: bool,
    ) -> Result<()> {
        // Create provider params
        let mut params = HashMap::new();
        
        // Add provider
        self.providers.add_provider(name, ProviderType::Proxmox, host, params)?;
        
        // Add credentials
        self.credentials.add_proxmox_user_pass_credentials(
            name,
            username,
            password,
            realm,
            port,
            verify_ssl,
        )?;
        
        // Save changes
        self.providers.save()?;
        self.credentials.save()?;
        
        info!("Added Proxmox provider with user/pass auth: {}", name);
        Ok(())
    }
    
    /// Remove a provider
    pub fn remove_provider(&mut self, name: &str) -> Result<()> {
        // Remove provider config
        self.providers.remove_provider(name)?;
        
        // Remove credentials
        if let Err(e) = self.credentials.remove_credentials(name) {
            debug!("No credentials found for provider '{}': {}", name, e);
        }
        
        // Save changes
        self.providers.save()?;
        self.credentials.save()?;
        
        info!("Removed provider: {}", name);
        Ok(())
    }
    
    /// Add a new region
    pub fn add_region(
        &mut self,
        id: &str,
        name: &str,
        provider_type: ProviderType,
        location: &str,
        available: bool,
        limits: Option<ResourceLimits>,
    ) -> Result<()> {
        let region = Region {
            id: id.to_string(),
            name: name.to_string(),
            provider: provider_type,
            location: location.to_string(),
            available,
            limits: limits.unwrap_or_default(),
        };
        
        self.providers.add_region(region)?;
        self.providers.save()?;
        
        info!("Added region: {}", id);
        Ok(())
    }
    
    /// Remove a region
    pub fn remove_region(&mut self, id: &str) -> Result<()> {
        self.providers.remove_region(id)?;
        self.providers.save()?;
        
        info!("Removed region: {}", id);
        Ok(())
    }
    
    /// Get a VyOS client for a provider
    pub fn get_vyos_client(&self, provider_name: &str) -> Result<VyOSClient> {
        // Get provider config
        let provider = self.providers.get_provider(provider_name)
            .ok_or_else(|| anyhow!("Provider not found: {}", provider_name))?;
            
        // Ensure it's a VyOS provider
        if provider.provider_type != ProviderType::VyOS {
            return Err(anyhow!("Provider '{}' is not a VyOS provider", provider_name));
        }
        
        // Get credentials
        let creds = self.credentials.get_vyos_credentials(provider_name)?;
        
        // Create client config
        let config = VyOSConfig {
            host: provider.host.clone(),
            ssh_port: creds.ssh_port.unwrap_or(22),
            api_port: creds.api_port.unwrap_or(443),
            username: creds.username.clone(),
            password: creds.password.clone(),
            key_path: creds.key_path.clone(),
            api_key: creds.api_key.clone(),
            timeout: 30,
        };
        
        // Create client
        let client = VyOSClient::new(config);
        
        Ok(client)
    }
    
    /// Get a Proxmox client for a provider
    pub fn get_proxmox_client(&self, provider_name: &str) -> Result<ProxmoxClient> {
        // Get provider config
        let provider = self.providers.get_provider(provider_name)
            .ok_or_else(|| anyhow!("Provider not found: {}", provider_name))?;
            
        // Ensure it's a Proxmox provider
        if provider.provider_type != ProviderType::Proxmox {
            return Err(anyhow!("Provider '{}' is not a Proxmox provider", provider_name));
        }
        
        // Get credentials
        let creds = self.credentials.get_proxmox_credentials(provider_name)?;
        
        // Create auth config
        let auth = if creds.use_token_auth {
            if let Some(token) = &creds.token_auth {
                ProxmoxAuth::ApiToken {
                    token_id: token.token_id.clone(),
                    token_secret: token.token_secret.clone(),
                }
            } else {
                return Err(anyhow!("Proxmox provider '{}' is configured to use token auth, but no token is provided", provider_name));
            }
        } else {
            if let Some(user_pass) = &creds.user_pass_auth {
                ProxmoxAuth::UserPass {
                    username: user_pass.username.clone(),
                    password: user_pass.password.clone(),
                    realm: user_pass.realm.clone(),
                }
            } else {
                return Err(anyhow!("Proxmox provider '{}' is configured to use user/pass auth, but no credentials are provided", provider_name));
            }
        };
        
        // Create client config
        let config = ProxmoxConfig {
            host: provider.host.clone(),
            port: creds.port.unwrap_or(8006),
            auth,
            timeout: 30,
            verify_ssl: creds.verify_ssl,
        };
        
        // Create client
        let client = ProxmoxClient::new(config);
        
        Ok(client)
    }
    
    /// Test connection to a provider
    pub async fn test_connection(&self, provider_name: &str) -> Result<bool> {
        // Get provider config
        let provider = self.providers.get_provider(provider_name)
            .ok_or_else(|| anyhow!("Provider not found: {}", provider_name))?;
        
        match provider.provider_type {
            ProviderType::VyOS => {
                let client = self.get_vyos_client(provider_name)?;
                
                // Use the synchronous connect method for testing
                match client.connect() {
                    Ok(_) => {
                        info!("Successfully connected to VyOS provider: {}", provider_name);
                        Ok(true)
                    },
                    Err(e) => {
                        error!("Failed to connect to VyOS provider '{}': {}", provider_name, e);
                        Ok(false)
                    }
                }
            },
            ProviderType::Proxmox => {
                let mut client = self.get_proxmox_client(provider_name)?;
                
                // Login to test connection
                match client.login().await {
                    Ok(_) => {
                        info!("Successfully connected to Proxmox provider: {}", provider_name);
                        Ok(true)
                    },
                    Err(e) => {
                        error!("Failed to connect to Proxmox provider '{}': {}", provider_name, e);
                        Ok(false)
                    }
                }
            }
        }
    }
}