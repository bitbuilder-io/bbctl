use anyhow::{anyhow, Result};
use log::info;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::provider::ProviderType;
use crate::models::volume::{Volume, VolumeStatus, VolumeType};
use crate::services::provider::ProviderService;

/// Storage for volume data
#[derive(Debug)]
pub struct VolumeStorage {
    volumes: HashMap<Uuid, Volume>,
}

impl VolumeStorage {
    /// Create a new volume storage
    pub fn new() -> Self {
        Self {
            volumes: HashMap::new(),
        }
    }

    /// Add a volume
    pub fn add_volume(&mut self, volume: Volume) {
        self.volumes.insert(volume.id, volume);
    }

    /// Get a volume by ID
    pub fn get_volume(&self, id: &Uuid) -> Option<&Volume> {
        self.volumes.get(id)
    }

    /// Get a mutable reference to a volume
    pub fn get_volume_mut(&mut self, id: &Uuid) -> Option<&mut Volume> {
        self.volumes.get_mut(id)
    }

    /// Remove a volume
    pub fn remove_volume(&mut self, id: &Uuid) -> Option<Volume> {
        self.volumes.remove(id)
    }

    /// Get all volumes
    pub fn get_all_volumes(&self) -> Vec<&Volume> {
        self.volumes.values().collect()
    }

    /// Get volumes by provider
    pub fn get_volumes_by_provider(&self, provider: ProviderType) -> Vec<&Volume> {
        self.volumes
            .values()
            .filter(|v| v.provider == provider)
            .collect()
    }

    /// Get volumes by region
    pub fn get_volumes_by_region(&self, region: &str) -> Vec<&Volume> {
        self.volumes
            .values()
            .filter(|v| v.region == region)
            .collect()
    }
}

/// Volume service for managing storage volumes
#[allow(dead_code)]
pub struct VolumeService {
    storage: VolumeStorage,
    /// Reserved for future provider-specific volume operations.
    provider_service: ProviderService,
}

impl VolumeService {
    /// Create a new volume service
    pub fn new(provider_service: ProviderService) -> Self {
        Self {
            storage: VolumeStorage::new(),
            provider_service,
        }
    }

    /// List all volumes
    pub fn list_volumes(&self) -> Vec<&Volume> {
        self.storage.get_all_volumes()
    }

    /// Get a volume by ID
    pub fn get_volume(&self, id: &Uuid) -> Option<&Volume> {
        self.storage.get_volume(id)
    }

    /// Create a new volume
    pub async fn create_volume(
        &mut self,
        name: &str,
        provider_type: ProviderType,
        region: &str,
        size_gb: u16,
        volume_type: VolumeType,
    ) -> Result<Uuid> {
        // Create a new volume object
        let volume = Volume::new(
            name.to_string(),
            provider_type,
            region.to_string(),
            size_gb,
            volume_type,
        );

        info!(
            "Creating volume '{}' with size {} GB in region '{}'",
            name, size_gb, region
        );

        // Store the volume
        let id = volume.id;
        self.storage.add_volume(volume);

        info!("Successfully created volume: {}", id);
        Ok(id)
    }

    /// Attach a volume to an instance
    pub async fn attach_volume(
        &mut self,
        volume_id: &Uuid,
        instance_id: &Uuid,
        device: &str,
    ) -> Result<()> {
        // Get the volume
        let volume = self
            .storage
            .get_volume_mut(volume_id)
            .ok_or_else(|| anyhow!("Volume not found: {}", volume_id))?;

        // Check if already attached
        if volume.attached_to.is_some() {
            return Err(anyhow!("Volume {} is already attached", volume_id));
        }

        // Attach the volume
        volume.attach(*instance_id, Some(device.to_string()));

        info!(
            "Attached volume {} to instance {} at {}",
            volume_id, instance_id, device
        );
        Ok(())
    }

    /// Detach a volume from an instance
    pub async fn detach_volume(&mut self, volume_id: &Uuid) -> Result<()> {
        // Get the volume
        let volume = self
            .storage
            .get_volume_mut(volume_id)
            .ok_or_else(|| anyhow!("Volume not found: {}", volume_id))?;

        // Check if attached
        if volume.attached_to.is_none() {
            return Err(anyhow!(
                "Volume {} is not attached to any instance",
                volume_id
            ));
        }

        // Detach the volume
        volume.detach();

        info!("Detached volume {}", volume_id);
        Ok(())
    }

    /// Delete a volume
    pub async fn delete_volume(&mut self, volume_id: &Uuid) -> Result<()> {
        // Get the volume
        let volume = self
            .storage
            .get_volume(volume_id)
            .ok_or_else(|| anyhow!("Volume not found: {}", volume_id))?;

        // Check if attached
        if volume.attached_to.is_some() {
            return Err(anyhow!(
                "Cannot delete attached volume {}. Detach it first.",
                volume_id
            ));
        }

        // Remove the volume
        self.storage.remove_volume(volume_id);

        info!("Deleted volume {}", volume_id);
        Ok(())
    }
}
