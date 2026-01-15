use anyhow::{Result, anyhow};
use log::info;
use std::collections::HashMap;
use std::net::IpAddr;
use uuid::Uuid;

use crate::models::network::{Network, NetworkType};
use crate::models::provider::ProviderType;
use crate::services::provider::ProviderService;

/// Storage for network data
#[derive(Debug)]
pub struct NetworkStorage {
    networks: HashMap<Uuid, Network>,
}

impl NetworkStorage {
    /// Create a new network storage
    pub fn new() -> Self {
        Self {
            networks: HashMap::new(),
        }
    }
    
    /// Add a network
    pub fn add_network(&mut self, network: Network) {
        self.networks.insert(network.id, network);
    }
    
    /// Get a network by ID
    pub fn get_network(&self, id: &Uuid) -> Option<&Network> {
        self.networks.get(id)
    }
    
    /// Get a mutable reference to a network
    pub fn get_network_mut(&mut self, id: &Uuid) -> Option<&mut Network> {
        self.networks.get_mut(id)
    }
    
    /// Remove a network
    pub fn remove_network(&mut self, id: &Uuid) -> Option<Network> {
        self.networks.remove(id)
    }
    
    /// Get all networks
    pub fn get_all_networks(&self) -> Vec<&Network> {
        self.networks.values().collect()
    }
    
    /// Get networks by provider
    pub fn get_networks_by_provider(&self, provider: ProviderType) -> Vec<&Network> {
        self.networks.values()
            .filter(|n| n.provider == provider)
            .collect()
    }
    
    /// Get networks by region
    pub fn get_networks_by_region(&self, region: &str) -> Vec<&Network> {
        self.networks.values()
            .filter(|n| n.region == region)
            .collect()
    }
}

/// Network service for managing virtual networks
#[allow(dead_code)]
pub struct NetworkService {
    storage: NetworkStorage,
    /// Reserved for future provider-specific network operations.
    provider_service: ProviderService,
}

impl NetworkService {
    /// Create a new network service
    pub fn new(provider_service: ProviderService) -> Self {
        Self {
            storage: NetworkStorage::new(),
            provider_service,
        }
    }
    
    /// List all networks
    pub fn list_networks(&self) -> Vec<&Network> {
        self.storage.get_all_networks()
    }
    
    /// Get a network by ID
    pub fn get_network(&self, id: &Uuid) -> Option<&Network> {
        self.storage.get_network(id)
    }
    
    /// Create a new network
    pub async fn create_network(
        &mut self,
        name: &str,
        provider_type: ProviderType,
        region: &str,
        cidr: &str,
        network_type: NetworkType,
        gateway: Option<IpAddr>,
        dns_servers: Vec<IpAddr>,
    ) -> Result<Uuid> {
        // Create a new network object
        let mut network = Network::new(
            name.to_string(),
            provider_type,
            region.to_string(),
            cidr.to_string(),
            network_type,
        );
        
        // Set gateway and DNS servers if provided
        if let Some(gw) = gateway {
            network.set_gateway(gw);
        }
        for dns in dns_servers {
            network.add_dns_server(dns);
        }
        
        info!("Creating network '{}' with CIDR {} in region '{}'", name, cidr, region);
        
        // Store the network
        let id = network.id;
        self.storage.add_network(network);
        
        info!("Successfully created network: {}", id);
        Ok(id)
    }
    
    /// Connect an instance to a network
    pub async fn connect_instance(&mut self, network_id: &Uuid, instance_id: &Uuid, ip: IpAddr) -> Result<()> {
        // Get the network
        let network = self.storage.get_network_mut(network_id)
            .ok_or_else(|| anyhow!("Network not found: {}", network_id))?;
        
        // Check if instance is already connected
        if network.instances.contains(instance_id) {
            return Err(anyhow!("Instance {} is already connected to network {}", instance_id, network_id));
        }
        
        // Connect the instance and allocate IP
        network.connect_instance(*instance_id);
        network.allocate_ip(ip, *instance_id).map_err(|e| anyhow!(e))?;
        
        info!("Connected instance {} to network {} with IP {}", instance_id, network_id, ip);
        Ok(())
    }
    
    /// Disconnect an instance from a network
    pub async fn disconnect_instance(&mut self, network_id: &Uuid, instance_id: &Uuid) -> Result<()> {
        // Get the network
        let network = self.storage.get_network_mut(network_id)
            .ok_or_else(|| anyhow!("Network not found: {}", network_id))?;
        
        // Check if instance is connected
        if !network.instances.contains(instance_id) {
            return Err(anyhow!("Instance {} is not connected to network {}", instance_id, network_id));
        }
        
        // Disconnect the instance (this also removes IP allocation)
        network.disconnect_instance(instance_id);
        
        info!("Disconnected instance {} from network {}", instance_id, network_id);
        Ok(())
    }
    
    /// Delete a network
    pub async fn delete_network(&mut self, network_id: &Uuid) -> Result<()> {
        // Get the network
        let network = self.storage.get_network(network_id)
            .ok_or_else(|| anyhow!("Network not found: {}", network_id))?;
        
        // Check if any instances are connected
        if !network.instances.is_empty() {
            return Err(anyhow!("Cannot delete network {} with connected instances. Disconnect them first.", network_id));
        }
        
        // Remove the network
        self.storage.remove_network(network_id);
        
        info!("Deleted network {}", network_id);
        Ok(())
    }
}
