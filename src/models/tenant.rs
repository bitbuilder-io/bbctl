use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Tenant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantStatus {
    Active,
    Creating,
    Deleting,
    Error,
    Suspended,
}

impl std::fmt::Display for TenantStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantStatus::Active => write!(f, "active"),
            TenantStatus::Creating => write!(f, "creating"),
            TenantStatus::Deleting => write!(f, "deleting"),
            TenantStatus::Error => write!(f, "error"),
            TenantStatus::Suspended => write!(f, "suspended"),
        }
    }
}

/// VRF (Virtual Routing and Forwarding) configuration for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrfConfig {
    /// VRF name (e.g. "customer-1")
    pub name: String,
    /// VRF route-distinguisher table ID (e.g. 1000)
    pub table_id: u32,
    /// BGP route-target for export (e.g. "65000:1000")
    pub route_target_export: String,
    /// BGP route-target for import (e.g. "65000:1000")
    pub route_target_import: String,
}

/// VXLAN tunnel configuration for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VxlanConfig {
    /// VXLAN Network Identifier
    pub vni: u32,
    /// Source VTEP address
    pub source_address: String,
    /// MTU (typically 9000 for jumbo frames)
    pub mtu: u16,
    /// Remote VTEP address (optional for multicast/EVPN mode)
    pub remote: Option<String>,
}

/// WireGuard interface configuration for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantWireguardConfig {
    /// WireGuard interface index (appended to "wg", e.g. 1 → "wg1")
    pub interface_index: u32,
    /// Tenant WireGuard address (CIDR, e.g. "100.64.1.1/24")
    pub address: String,
    /// Pre-generated public key (base64)
    pub public_key: String,
    /// Pre-generated private key (base64) – stored securely
    pub private_key: String,
    /// UDP listen port
    pub port: u16,
}

/// VRRP configuration for high-availability gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrrpConfig {
    /// VRRP group identifier
    pub group_id: String,
    /// Interface to run VRRP on
    pub interface: String,
    /// Virtual IP address
    pub virtual_ip: String,
    /// VRRP virtual router ID (1-255)
    pub vrid: u8,
    /// Priority (higher = preferred master, 1-254)
    pub priority: u8,
}

/// A fully provisioned tenant with all networking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique tenant identifier
    pub id: Uuid,
    /// Numeric tenant ID used for addressing (e.g. VXLAN VNI offset)
    pub tenant_id: u32,
    /// Human-readable tenant name
    pub name: String,
    /// Current status
    pub status: TenantStatus,
    /// L3VPN VRF configuration
    pub vrf: VrfConfig,
    /// VXLAN data-plane configuration
    pub vxlan: VxlanConfig,
    /// WireGuard management-plane configuration
    pub wireguard: TenantWireguardConfig,
    /// Optional VRRP HA gateway configuration
    pub vrrp: Option<VrrpConfig>,
    /// BGP AS number used across the fabric
    pub bgp_as: u32,
    /// Tenant network CIDR (e.g. "100.65.1.0/24")
    pub network_cidr: String,
    /// Additional metadata
    pub tags: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Tenant {
    /// Create a new tenant with default networking parameters derived from the
    /// numeric `tenant_id`.
    ///
    /// Addressing conventions:
    /// - WireGuard: `100.64.<tenant_id>.1/24`
    /// - VXLAN VNI: `10000 + tenant_id`
    /// - VRF table: `1000 + tenant_id`
    /// - BGP route-target: `<bgp_as>:<1000 + tenant_id>`
    pub fn new(
        name: String,
        tenant_id: u32,
        bgp_as: u32,
        source_address: String,
        wg_public_key: String,
        wg_private_key: String,
    ) -> Self {
        let now = Utc::now();
        let vni = 10000 + tenant_id;
        let table_id = 1000 + tenant_id;
        let rt = format!("{}:{}", bgp_as, table_id);
        let wg_address = format!("100.64.{}.1/24", tenant_id);
        let network_cidr = format!("100.65.{}.0/24", tenant_id);

        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name: name.clone(),
            status: TenantStatus::Creating,
            vrf: VrfConfig {
                name: format!("tenant-{}", tenant_id),
                table_id,
                route_target_export: rt.clone(),
                route_target_import: rt,
            },
            vxlan: VxlanConfig {
                vni,
                source_address,
                mtu: 9000,
                remote: None,
            },
            wireguard: TenantWireguardConfig {
                interface_index: tenant_id,
                address: wg_address,
                public_key: wg_public_key,
                private_key: wg_private_key,
                // Keep port in the valid range (51820–52819) using modulo 1000
                port: 51820 + (tenant_id as u16 % 1000),
            },
            vrrp: None,
            bgp_as,
            network_cidr,
            tags: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Mark the tenant as active (provisioning complete).
    pub fn set_active(&mut self) {
        self.status = TenantStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Mark the tenant as error state.
    pub fn set_error(&mut self) {
        self.status = TenantStatus::Error;
        self.updated_at = Utc::now();
    }

    /// Attach a VRRP HA configuration to the tenant.
    pub fn set_vrrp(&mut self, vrrp: VrrpConfig) {
        self.vrrp = Some(vrrp);
        self.updated_at = Utc::now();
    }

    /// Add a metadata tag.
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
        self.updated_at = Utc::now();
    }
}
