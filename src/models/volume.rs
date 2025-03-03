use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::models::provider::ProviderType;

/// Volume status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolumeStatus {
    Available,
    InUse,
    Creating,
    Deleting,
    Error,
    Unknown,
}

impl std::fmt::Display for VolumeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeStatus::Available => write!(f, "available"),
            VolumeStatus::InUse => write!(f, "in-use"),
            VolumeStatus::Creating => write!(f, "creating"),
            VolumeStatus::Deleting => write!(f, "deleting"),
            VolumeStatus::Error => write!(f, "error"),
            VolumeStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for VolumeStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "available" => VolumeStatus::Available,
            "in-use" | "inuse" | "in_use" => VolumeStatus::InUse,
            "creating" => VolumeStatus::Creating,
            "deleting" => VolumeStatus::Deleting,
            "error" => VolumeStatus::Error,
            _ => VolumeStatus::Unknown,
        }
    }
}

/// Volume type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolumeType {
    Standard,
    SSD,
    NVMe,
    HDD,
    Network,
}

impl std::fmt::Display for VolumeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeType::Standard => write!(f, "standard"),
            VolumeType::SSD => write!(f, "ssd"),
            VolumeType::NVMe => write!(f, "nvme"),
            VolumeType::HDD => write!(f, "hdd"),
            VolumeType::Network => write!(f, "network"),
        }
    }
}

impl From<&str> for VolumeType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "standard" => VolumeType::Standard,
            "ssd" => VolumeType::SSD,
            "nvme" => VolumeType::NVMe,
            "hdd" => VolumeType::HDD,
            "network" => VolumeType::Network,
            _ => VolumeType::Standard,
        }
    }
}

/// Volume resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    /// Volume ID (UUID)
    pub id: Uuid,
    /// Volume name
    pub name: String,
    /// Volume status
    pub status: VolumeStatus,
    /// Provider type
    pub provider: ProviderType,
    /// Provider-specific ID
    pub provider_id: String,
    /// Region
    pub region: String,
    /// Volume size in GB
    pub size_gb: u16,
    /// Volume type
    pub volume_type: VolumeType,
    /// Attached to instance ID (if any)
    pub attached_to: Option<Uuid>,
    /// Device name when attached (e.g., /dev/sda)
    pub device: Option<String>,
    /// Created at timestamp
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
    /// Tags
    pub tags: HashMap<String, String>,
}

impl Volume {
    /// Create a new volume
    pub fn new(name: String, provider: ProviderType, region: String, size_gb: u16, volume_type: VolumeType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            status: VolumeStatus::Creating,
            provider,
            provider_id: String::new(), // Will be set after creation
            region,
            size_gb,
            volume_type,
            attached_to: None,
            device: None,
            created_at: now,
            updated_at: now,
            tags: HashMap::new(),
        }
    }
    
    /// Update volume status
    pub fn update_status(&mut self, status: VolumeStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Attach volume to an instance
    pub fn attach(&mut self, instance_id: Uuid, device: Option<String>) {
        self.attached_to = Some(instance_id);
        self.device = device;
        self.status = VolumeStatus::InUse;
        self.updated_at = Utc::now();
    }
    
    /// Detach volume from an instance
    pub fn detach(&mut self) {
        self.attached_to = None;
        self.device = None;
        self.status = VolumeStatus::Available;
        self.updated_at = Utc::now();
    }
    
    /// Extend volume size
    pub fn extend(&mut self, new_size_gb: u16) -> Result<(), &'static str> {
        if new_size_gb <= self.size_gb {
            return Err("New size must be larger than current size");
        }
        
        self.size_gb = new_size_gb;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Add a tag to the volume
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Remove a tag from the volume
    pub fn remove_tag(&mut self, key: &str) -> Option<String> {
        let result = self.tags.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
}