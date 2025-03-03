use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use std::net::IpAddr;

use crate::models::provider::ProviderType;

/// Network status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkStatus {
    Available,
    Creating,
    Deleting,
    Error,
    Unknown,
}

impl std::fmt::Display for NetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkStatus::Available => write!(f, "available"),
            NetworkStatus::Creating => write!(f, "creating"),
            NetworkStatus::Deleting => write!(f, "deleting"),
            NetworkStatus::Error => write!(f, "error"),
            NetworkStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for NetworkStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "available" => NetworkStatus::Available,
            "creating" => NetworkStatus::Creating,
            "deleting" => NetworkStatus::Deleting,
            "error" => NetworkStatus::Error,
            _ => NetworkStatus::Unknown,
        }
    }
}

/// Network type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    Bridged,
    Routed,
    Isolated,
    VXLAN,
    VPN,
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkType::Bridged => write!(f, "bridged"),
            NetworkType::Routed => write!(f, "routed"),
            NetworkType::Isolated => write!(f, "isolated"),
            NetworkType::VXLAN => write!(f, "vxlan"),
            NetworkType::VPN => write!(f, "vpn"),
        }
    }
}

impl From<&str> for NetworkType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bridged" => NetworkType::Bridged,
            "routed" => NetworkType::Routed,
            "isolated" => NetworkType::Isolated,
            "vxlan" => NetworkType::VXLAN,
            "vpn" => NetworkType::VPN,
            _ => NetworkType::Routed, // Default to routed
        }
    }
}

/// IP allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAllocation {
    /// IP address
    pub ip: IpAddr,
    /// Assigned to instance ID (if any)
    pub instance_id: Option<Uuid>,
    /// Assigned at timestamp
    pub assigned_at: Option<DateTime<Utc>>,
}

/// Network resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Network ID (UUID)
    pub id: Uuid,
    /// Network name
    pub name: String,
    /// Network status
    pub status: NetworkStatus,
    /// Provider type
    pub provider: ProviderType,
    /// Provider-specific ID
    pub provider_id: String,
    /// Region
    pub region: String,
    /// CIDR block (e.g., 192.168.1.0/24)
    pub cidr: String,
    /// Network type
    pub network_type: NetworkType,
    /// Gateway IP (optional)
    pub gateway: Option<IpAddr>,
    /// DNS servers
    pub dns_servers: Vec<IpAddr>,
    /// Connected instances
    pub instances: HashSet<Uuid>,
    /// IP allocations
    pub ip_allocations: Vec<IpAllocation>,
    /// Created at timestamp
    pub created_at: DateTime<Utc>,
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Additional configuration parameters
    pub config: HashMap<String, String>,
}

impl Network {
    /// Create a new network
    pub fn new(name: String, provider: ProviderType, region: String, cidr: String, network_type: NetworkType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            status: NetworkStatus::Creating,
            provider,
            provider_id: String::new(), // Will be set after creation
            region,
            cidr,
            network_type,
            gateway: None,
            dns_servers: Vec::new(),
            instances: HashSet::new(),
            ip_allocations: Vec::new(),
            created_at: now,
            updated_at: now,
            tags: HashMap::new(),
            config: HashMap::new(),
        }
    }
    
    /// Update network status
    pub fn update_status(&mut self, status: NetworkStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Add a gateway IP
    pub fn set_gateway(&mut self, gateway: IpAddr) {
        self.gateway = Some(gateway);
        self.updated_at = Utc::now();
    }
    
    /// Add a DNS server
    pub fn add_dns_server(&mut self, dns_server: IpAddr) {
        if !self.dns_servers.contains(&dns_server) {
            self.dns_servers.push(dns_server);
            self.updated_at = Utc::now();
        }
    }
    
    /// Remove a DNS server
    pub fn remove_dns_server(&mut self, dns_server: &IpAddr) {
        if let Some(idx) = self.dns_servers.iter().position(|ip| ip == dns_server) {
            self.dns_servers.remove(idx);
            self.updated_at = Utc::now();
        }
    }
    
    /// Connect an instance to the network
    pub fn connect_instance(&mut self, instance_id: Uuid) -> bool {
        let result = self.instances.insert(instance_id);
        if result {
            self.updated_at = Utc::now();
        }
        result
    }
    
    /// Disconnect an instance from the network
    pub fn disconnect_instance(&mut self, instance_id: &Uuid) -> bool {
        let result = self.instances.remove(instance_id);
        if result {
            // Also remove any IP allocations for this instance
            self.ip_allocations.retain(|alloc| alloc.instance_id != Some(*instance_id));
            self.updated_at = Utc::now();
        }
        result
    }
    
    /// Allocate an IP address to an instance
    pub fn allocate_ip(&mut self, ip: IpAddr, instance_id: Uuid) -> Result<(), &'static str> {
        // Check if IP is already allocated
        if self.ip_allocations.iter().any(|alloc| alloc.ip == ip) {
            return Err("IP address already allocated");
        }
        
        // Ensure instance is connected to this network
        if !self.instances.contains(&instance_id) {
            return Err("Instance not connected to this network");
        }
        
        // Allocate the IP
        self.ip_allocations.push(IpAllocation {
            ip,
            instance_id: Some(instance_id),
            assigned_at: Some(Utc::now()),
        });
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Release an IP address
    pub fn release_ip(&mut self, ip: &IpAddr) -> Result<(), &'static str> {
        if let Some(idx) = self.ip_allocations.iter().position(|alloc| &alloc.ip == ip) {
            self.ip_allocations.remove(idx);
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err("IP address not found")
        }
    }
    
    /// Add a tag to the network
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Remove a tag from the network
    pub fn remove_tag(&mut self, key: &str) -> Option<String> {
        let result = self.tags.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
    
    /// Set a configuration parameter
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Get a configuration parameter
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    /// Remove a configuration parameter
    pub fn remove_config(&mut self, key: &str) -> Option<String> {
        let result = self.config.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }
}