use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::models::provider::ProviderType;

/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceStatus {
    Running,
    Stopped,
    Failed,
    Creating,
    Restarting,
    Deleting,
    Unknown,
}

impl std::fmt::Display for InstanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceStatus::Running => write!(f, "running"),
            InstanceStatus::Stopped => write!(f, "stopped"),
            InstanceStatus::Failed => write!(f, "failed"),
            InstanceStatus::Creating => write!(f, "creating"),
            InstanceStatus::Restarting => write!(f, "restarting"),
            InstanceStatus::Deleting => write!(f, "deleting"),
            InstanceStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for InstanceStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "running" => InstanceStatus::Running,
            "stopped" => InstanceStatus::Stopped,
            "failed" => InstanceStatus::Failed,
            "creating" => InstanceStatus::Creating,
            "restarting" => InstanceStatus::Restarting,
            "deleting" => InstanceStatus::Deleting,
            _ => InstanceStatus::Unknown,
        }
    }
}

/// Instance size (VM configuration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSize {
    /// CPU cores
    pub cpu: u8,
    /// Memory in GB
    pub memory_gb: u16,
    /// Disk in GB
    pub disk_gb: u16,
}

/// Instance networking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceNetwork {
    /// Network ID
    pub network_id: String,
    /// IP address
    pub ip: Option<String>,
    /// Interface name
    pub interface: Option<String>,
    /// MAC address
    pub mac: Option<String>,
}

/// Instance resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// Instance ID (UUID)
    pub id: Uuid,
    /// Instance name
    pub name: String,
    /// Instance status
    pub status: InstanceStatus,
    /// Provider type
    pub provider: ProviderType,
    /// Provider-specific ID
    pub provider_id: String,
    /// Region
    pub region: String,
    /// Size configuration
    pub size: InstanceSize,
    /// Networks
    pub networks: Vec<InstanceNetwork>,
    /// Created at timestamp
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
    /// Tags
    pub tags: HashMap<String, String>,
}

impl Instance {
    /// Create a new instance
    pub fn new(name: String, provider: ProviderType, region: String, size: InstanceSize) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            status: InstanceStatus::Creating,
            provider,
            provider_id: String::new(), // Will be set after creation
            region,
            size,
            networks: Vec::new(),
            created_at: now,
            updated_at: now,
            tags: HashMap::new(),
        }
    }
    
    /// Get primary IP address
    pub fn primary_ip(&self) -> Option<&str> {
        self.networks.first()
            .and_then(|network| network.ip.as_deref())
    }
    
    /// Add a network to the instance
    pub fn add_network(&mut self, network_id: String, ip: Option<String>, interface: Option<String>, mac: Option<String>) {
        self.networks.push(InstanceNetwork {
            network_id,
            ip,
            interface,
            mac,
        });
        self.updated_at = Utc::now();
    }
    
    /// Update instance status
    pub fn update_status(&mut self, status: InstanceStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Add a tag to the instance
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Remove a tag from the instance
    pub fn remove_tag(&mut self, key: &str) -> Option<String> {
        let result = self.tags.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
}