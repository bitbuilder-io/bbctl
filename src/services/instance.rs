use anyhow::{Result, Context, anyhow};
use log::{debug, info, error};
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;
use chrono::Utc;

use crate::models::instance::{Instance, InstanceStatus, InstanceSize, InstanceNetwork};
use crate::models::provider::ProviderType;
use crate::services::provider::ProviderService;

/// Storage for instance data
#[derive(Debug)]
pub struct InstanceStorage {
    instances: HashMap<Uuid, Instance>,
}

impl InstanceStorage {
    /// Create a new instance storage
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }
    
    /// Add an instance
    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.insert(instance.id, instance);
    }
    
    /// Get an instance by ID
    pub fn get_instance(&self, id: &Uuid) -> Option<&Instance> {
        self.instances.get(id)
    }
    
    /// Get a mutable reference to an instance
    pub fn get_instance_mut(&mut self, id: &Uuid) -> Option<&mut Instance> {
        self.instances.get_mut(id)
    }
    
    /// Remove an instance
    pub fn remove_instance(&mut self, id: &Uuid) -> Option<Instance> {
        self.instances.remove(id)
    }
    
    /// Get all instances
    pub fn get_all_instances(&self) -> Vec<&Instance> {
        self.instances.values().collect()
    }
    
    /// Get instances by provider
    pub fn get_instances_by_provider(&self, provider: ProviderType) -> Vec<&Instance> {
        self.instances.values()
            .filter(|i| i.provider == provider)
            .collect()
    }
    
    /// Get instances by region
    pub fn get_instances_by_region(&self, region: &str) -> Vec<&Instance> {
        self.instances.values()
            .filter(|i| i.region == region)
            .collect()
    }
}

/// Instance service for managing VMs
pub struct InstanceService {
    storage: InstanceStorage,
    provider_service: ProviderService,
}

impl InstanceService {
    /// Create a new instance service
    pub fn new(provider_service: ProviderService) -> Self {
        Self {
            storage: InstanceStorage::new(),
            provider_service,
        }
    }
    
    /// List all instances
    pub fn list_instances(&self) -> Vec<&Instance> {
        self.storage.get_all_instances()
    }
    
    /// Get an instance by ID
    pub fn get_instance(&self, id: &Uuid) -> Option<&Instance> {
        self.storage.get_instance(id)
    }
    
    /// Create a new instance on a VyOS provider
    pub async fn create_vyos_instance(
        &mut self,
        name: &str,
        provider_name: &str,
        region: &str,
        size: InstanceSize,
        network_id: Option<String>,
    ) -> Result<Uuid> {
        // Get VyOS client
        let mut client = self.provider_service.get_vyos_client(provider_name)?;
        
        // Create a new instance object
        let mut instance = Instance::new(
            name.to_string(),
            ProviderType::VyOS,
            region.to_string(),
            size.clone(),
        );
        
        info!("Creating VyOS instance '{}' in region '{}'", name, region);
        
        // Use VyOS API to create the VM
        // This is a simplified example - in a real implementation, you would call the VyOS API
        // to create a VM and get the provider-specific ID
        
        // Example: Use the VyOS API to get information about the router
        let result = client.get_system_info().await;
        
        match result {
            Ok(info) => {
                debug!("VyOS system info: {:?}", info);
                
                // In a real implementation, you would parse the response and set the provider ID
                instance.provider_id = format!("vyos-{}", Uuid::new_v4());
                
                // Add network if specified
                if let Some(net_id) = network_id {
                    instance.add_network(net_id, Some("192.168.1.100".to_string()), Some("eth0".to_string()), None);
                }
                
                // Set status to running
                instance.update_status(InstanceStatus::Running);
                
                // Store the instance
                let id = instance.id;
                self.storage.add_instance(instance);
                
                info!("Successfully created VyOS instance: {}", id);
                Ok(id)
            },
            Err(e) => {
                error!("Failed to create VyOS instance: {}", e);
                Err(anyhow!("Failed to create VyOS instance: {}", e))
            }
        }
    }
    
    /// Create a new instance on a Proxmox provider
    pub async fn create_proxmox_instance(
        &mut self,
        name: &str,
        provider_name: &str,
        region: &str,
        size: InstanceSize,
        network_id: Option<String>,
    ) -> Result<Uuid> {
        // Get Proxmox client
        let mut client = self.provider_service.get_proxmox_client(provider_name)?;
        
        // Create a new instance object
        let mut instance = Instance::new(
            name.to_string(),
            ProviderType::Proxmox,
            region.to_string(),
            size.clone(),
        );
        
        info!("Creating Proxmox instance '{}' in region '{}'", name, region);
        
        // Ensure client is connected
        client.login().await?;
        
        // Get first node in the cluster
        let nodes = client.get_nodes().await?;
        
        if let Some(nodes_array) = nodes.as_array() {
            if let Some(first_node) = nodes_array.first() {
                if let Some(node_name) = first_node["node"].as_str() {
                    // Parameters for VM creation
                    let vm_params = json!({
                        "vmid": 1000,  // This would be dynamically generated in a real implementation
                        "name": name,
                        "cores": size.cpu,
                        "memory": size.memory_gb * 1024,
                        "disk": format!("{}G", size.disk_gb),
                        "net0": "virtio,bridge=vmbr0",
                    });
                    
                    // Create the VM
                    let result = client.create_vm(node_name, vm_params).await;
                    
                    match result {
                        Ok(response) => {
                            debug!("Proxmox VM creation response: {:?}", response);
                            
                            // Set provider ID to the VMID
                            if let Some(vmid) = response["vmid"].as_u64() {
                                instance.provider_id = vmid.to_string();
                                
                                // Add network if specified
                                if let Some(net_id) = network_id {
                                    instance.add_network(net_id, Some("192.168.1.100".to_string()), Some("eth0".to_string()), None);
                                }
                                
                                // Set status to running
                                instance.update_status(InstanceStatus::Running);
                                
                                // Store the instance
                                let id = instance.id;
                                self.storage.add_instance(instance);
                                
                                info!("Successfully created Proxmox instance: {}", id);
                                return Ok(id);
                            } else {
                                return Err(anyhow!("Failed to get VMID from Proxmox response"));
                            }
                        },
                        Err(e) => {
                            error!("Failed to create Proxmox instance: {}", e);
                            return Err(anyhow!("Failed to create Proxmox instance: {}", e));
                        }
                    }
                }
            }
            
            Err(anyhow!("No nodes found in Proxmox cluster"))
        } else {
            Err(anyhow!("Invalid response from Proxmox API"))
        }
    }
    
    /// Start an instance
    pub async fn start_instance(&mut self, id: &Uuid) -> Result<()> {
        // Get the instance
        let instance = self.storage.get_instance(id)
            .ok_or_else(|| anyhow!("Instance not found: {}", id))?;
        
        // Clone necessary values for the match block
        let provider = instance.provider;
        let provider_id = instance.provider_id.clone();
        
        // Find the provider name
        let provider_name = self.find_provider_name(instance)?;
        
        match provider {
            ProviderType::VyOS => {
                // Get VyOS client
                let mut client = self.provider_service.get_vyos_client(&provider_name)?;
                
                // Use VyOS API to start the VM
                // Example: Send commands over SSH
                let result = client.execute_ssh_command(&format!("start vm {}", provider_id)).await;
                
                match result {
                    Ok(_) => {
                        // Update instance status
                        if let Some(instance) = self.storage.get_instance_mut(id) {
                            instance.update_status(InstanceStatus::Running);
                        }
                        
                        info!("Successfully started VyOS instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to start VyOS instance: {}", e);
                        Err(anyhow!("Failed to start VyOS instance: {}", e))
                    }
                }
            },
            ProviderType::Proxmox => {
                // Get Proxmox client
                let mut client = self.provider_service.get_proxmox_client(&provider_name)?;
                
                // Ensure client is connected
                client.login().await?;
                
                // Get the node name from the provider ID
                // In a real implementation, you would store or lookup the node name
                let node_name = "pve";  // Placeholder
                
                // Start the VM
                let vmid = provider_id.parse::<u64>()
                    .context("Invalid VMID in provider_id")?;
                
                let result = client.start_vm(node_name, vmid).await;
                
                match result {
                    Ok(_) => {
                        // Update instance status
                        if let Some(instance) = self.storage.get_instance_mut(id) {
                            instance.update_status(InstanceStatus::Running);
                        }
                        
                        info!("Successfully started Proxmox instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to start Proxmox instance: {}", e);
                        Err(anyhow!("Failed to start Proxmox instance: {}", e))
                    }
                }
            }
        }
    }
    
    /// Stop an instance
    pub async fn stop_instance(&mut self, id: &Uuid) -> Result<()> {
        // Get the instance
        let instance = self.storage.get_instance(id)
            .ok_or_else(|| anyhow!("Instance not found: {}", id))?;
        
        // Clone necessary values for the match block
        let provider = instance.provider;
        let provider_id = instance.provider_id.clone();
        
        // Find the provider name
        let provider_name = self.find_provider_name(instance)?;
        
        match provider {
            ProviderType::VyOS => {
                // Get VyOS client
                let mut client = self.provider_service.get_vyos_client(&provider_name)?;
                
                // Use VyOS API to stop the VM
                // Example: Send commands over SSH
                let result = client.execute_ssh_command(&format!("stop vm {}", provider_id)).await;
                
                match result {
                    Ok(_) => {
                        // Update instance status
                        if let Some(instance) = self.storage.get_instance_mut(id) {
                            instance.update_status(InstanceStatus::Stopped);
                        }
                        
                        info!("Successfully stopped VyOS instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to stop VyOS instance: {}", e);
                        Err(anyhow!("Failed to stop VyOS instance: {}", e))
                    }
                }
            },
            ProviderType::Proxmox => {
                // Get Proxmox client
                let mut client = self.provider_service.get_proxmox_client(&provider_name)?;
                
                // Ensure client is connected
                client.login().await?;
                
                // Get the node name from the provider ID
                // In a real implementation, you would store or lookup the node name
                let node_name = "pve";  // Placeholder
                
                // Stop the VM
                let vmid = provider_id.parse::<u64>()
                    .context("Invalid VMID in provider_id")?;
                
                let result = client.stop_vm(node_name, vmid).await;
                
                match result {
                    Ok(_) => {
                        // Update instance status
                        if let Some(instance) = self.storage.get_instance_mut(id) {
                            instance.update_status(InstanceStatus::Stopped);
                        }
                        
                        info!("Successfully stopped Proxmox instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to stop Proxmox instance: {}", e);
                        Err(anyhow!("Failed to stop Proxmox instance: {}", e))
                    }
                }
            }
        }
    }
    
    /// Delete an instance
    pub async fn delete_instance(&mut self, id: &Uuid) -> Result<()> {
        // Get the instance
        let instance = self.storage.get_instance(id)
            .ok_or_else(|| anyhow!("Instance not found: {}", id))?;
        
        // Clone necessary values for the match block
        let provider = instance.provider;
        let provider_id = instance.provider_id.clone();
        
        // Find the provider name
        let provider_name = self.find_provider_name(instance)?;
        
        match provider {
            ProviderType::VyOS => {
                // Get VyOS client
                let mut client = self.provider_service.get_vyos_client(&provider_name)?;
                
                // Use VyOS API to delete the VM
                // Example: Send commands over SSH
                let result = client.execute_ssh_command(&format!("delete vm {}", provider_id)).await;
                
                match result {
                    Ok(_) => {
                        // Remove the instance from storage
                        self.storage.remove_instance(id);
                        
                        info!("Successfully deleted VyOS instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to delete VyOS instance: {}", e);
                        Err(anyhow!("Failed to delete VyOS instance: {}", e))
                    }
                }
            },
            ProviderType::Proxmox => {
                // Get Proxmox client
                let mut client = self.provider_service.get_proxmox_client(&provider_name)?;
                
                // Ensure client is connected
                client.login().await?;
                
                // Get the node name from the provider ID
                // In a real implementation, you would store or lookup the node name
                let node_name = "pve";  // Placeholder
                
                // Delete the VM
                let vmid = provider_id.parse::<u64>()
                    .context("Invalid VMID in provider_id")?;
                
                let result = client.delete_vm(node_name, vmid).await;
                
                match result {
                    Ok(_) => {
                        // Remove the instance from storage
                        self.storage.remove_instance(id);
                        
                        info!("Successfully deleted Proxmox instance: {}", id);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to delete Proxmox instance: {}", e);
                        Err(anyhow!("Failed to delete Proxmox instance: {}", e))
                    }
                }
            }
        }
    }
    
    /// Helper method to find provider name for an instance
    fn find_provider_name(&self, instance: &Instance) -> Result<String> {
        // Iterate through providers to find a matching one
        for (name, provider) in self.provider_service.get_providers() {
            if provider.provider_type == instance.provider {
                return Ok(name.clone());
            }
        }
        
        Err(anyhow!("No provider found for instance: {}", instance.id))
    }
}