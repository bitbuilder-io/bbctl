use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider types supported by bbctl
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    VyOS,
    Proxmox,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::VyOS => write!(f, "vyos"),
            ProviderType::Proxmox => write!(f, "proxmox"),
        }
    }
}

impl From<&str> for ProviderType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "vyos" => ProviderType::VyOS,
            "proxmox" => ProviderType::Proxmox,
            _ => panic!("Invalid provider type: {}", s),
        }
    }
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type
    pub provider_type: ProviderType,
    /// Provider name (user-friendly identifier)
    pub name: String,
    /// Provider host (IP or hostname)
    pub host: String,
    /// Additional configuration parameters
    pub params: HashMap<String, String>,
}

/// Region where resources can be deployed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// Region ID (e.g., "nyc", "sfo", etc.)
    pub id: String,
    /// Region name (user-friendly)
    pub name: String,
    /// Provider that this region belongs to
    pub provider: ProviderType,
    /// Location (e.g., "New York", "San Francisco", etc.)
    pub location: String,
    /// Whether this region is available for new deployments
    pub available: bool,
    /// Resource limits for this region
    pub limits: ResourceLimits,
}

/// Resource limits for a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum number of instances that can be created
    pub max_instances: Option<u32>,
    /// Maximum number of volumes that can be created
    pub max_volumes: Option<u32>,
    /// Maximum number of networks that can be created
    pub max_networks: Option<u32>,
    /// Maximum CPU cores per instance
    pub max_cpu_per_instance: Option<u8>,
    /// Maximum memory (GB) per instance
    pub max_memory_per_instance: Option<u16>,
    /// Maximum disk (GB) per instance
    pub max_disk_per_instance: Option<u16>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_instances: None,
            max_volumes: None,
            max_networks: None,
            max_cpu_per_instance: None,
            max_memory_per_instance: None,
            max_disk_per_instance: None,
        }
    }
}