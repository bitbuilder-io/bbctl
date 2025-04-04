use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use std::collections::HashMap;
use log::{debug, info, error};

use crate::config::{read_config_file, write_config_file, PROVIDERS_FILE};
use crate::models::provider::{ProviderType, ProviderConfig, Region};

/// Provider configuration storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Providers {
    /// Provider configurations by name
    pub providers: HashMap<String, ProviderConfig>,
    /// Regions by ID
    pub regions: HashMap<String, Region>,
}

impl Default for Providers {
    fn default() -> Self {
        Self {
            providers: HashMap::new(),
            regions: HashMap::new(),
        }
    }
}

impl Providers {
    /// Load providers from file
    pub fn load() -> Result<Self> {
        debug!("Loading providers from file");
        
        // Read providers file
        let content = match read_config_file(PROVIDERS_FILE) {
            Ok(content) => content,
            Err(e) => {
                info!("Failed to read providers file, using defaults: {}", e);
                return Ok(Self::default());
            }
        };
        
        // Parse TOML
        let providers: Providers = toml::from_str(&content)
            .context("Failed to parse providers TOML")?;
        
        Ok(providers)
    }
    
    /// Save providers to file
    pub fn save(&self) -> Result<()> {
        debug!("Saving providers to file");
        
        // Serialize to TOML
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize providers")?;
        
        // Write to file
        write_config_file(PROVIDERS_FILE, &content)
            .context("Failed to write providers file")?;
        
        info!("Providers saved successfully");
        Ok(())
    }
    
    /// Add a new provider
    pub fn add_provider(&mut self, name: &str, provider_type: ProviderType, host: &str, params: HashMap<String, String>) -> Result<()> {
        if self.providers.contains_key(name) {
            return Err(anyhow!("Provider with name '{}' already exists", name));
        }
        
        let config = ProviderConfig {
            provider_type,
            name: name.to_string(),
            host: host.to_string(),
            params,
        };
        
        self.providers.insert(name.to_string(), config);
        info!("Added provider: {}", name);
        Ok(())
    }
    
    /// Remove a provider
    pub fn remove_provider(&mut self, name: &str) -> Result<()> {
        if !self.providers.contains_key(name) {
            return Err(anyhow!("Provider with name '{}' does not exist", name));
        }
        
        self.providers.remove(name);
        
        // Also remove any regions that belong to this provider
        self.regions.retain(|_, region| region.provider.to_string() != name);
        
        info!("Removed provider: {}", name);
        Ok(())
    }
    
    /// Add a new region
    pub fn add_region(&mut self, region: Region) -> Result<()> {
        if self.regions.contains_key(&region.id) {
            return Err(anyhow!("Region with ID '{}' already exists", region.id));
        }
        
        // Ensure the provider exists
        let provider_name = region.provider.to_string();
        if !self.providers.iter().any(|(name, p)| p.provider_type == region.provider) {
            return Err(anyhow!("Provider '{}' does not exist", provider_name));
        }
        
        self.regions.insert(region.id.clone(), region.clone());
        info!("Added region: {}", region.id);
        Ok(())
    }
    
    /// Remove a region
    pub fn remove_region(&mut self, id: &str) -> Result<()> {
        if !self.regions.contains_key(id) {
            return Err(anyhow!("Region with ID '{}' does not exist", id));
        }
        
        self.regions.remove(id);
        info!("Removed region: {}", id);
        Ok(())
    }
    
    /// Get provider by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }
    
    /// Get region by ID
    pub fn get_region(&self, id: &str) -> Option<&Region> {
        self.regions.get(id)
    }
    
    /// Get regions by provider
    pub fn get_regions_by_provider(&self, provider_type: ProviderType) -> Vec<&Region> {
        self.regions.values()
            .filter(|r| r.provider == provider_type)
            .collect()
    }
    
    /// Get all providers
    pub fn get_all_providers(&self) -> &HashMap<String, ProviderConfig> {
        &self.providers
    }
    
    /// Get all regions
    pub fn get_all_regions(&self) -> &HashMap<String, Region> {
        &self.regions
    }
}