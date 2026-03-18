use anyhow::{anyhow, Result};
use log::info;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::tenant::{Tenant, VrrpConfig};
use crate::network::generate_wireguard_keypair;
use crate::services::provider::ProviderService;

/// In-memory storage for tenant records.
#[derive(Debug)]
pub struct TenantStorage {
    tenants: HashMap<Uuid, Tenant>,
    /// Index: numeric tenant_id → UUID
    tenant_id_index: HashMap<u32, Uuid>,
}

impl TenantStorage {
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
            tenant_id_index: HashMap::new(),
        }
    }

    pub fn add_tenant(&mut self, tenant: Tenant) {
        let uuid = tenant.id;
        let tid = tenant.tenant_id;
        self.tenants.insert(uuid, tenant);
        self.tenant_id_index.insert(tid, uuid);
    }

    pub fn get_tenant(&self, id: &Uuid) -> Option<&Tenant> {
        self.tenants.get(id)
    }

    pub fn get_tenant_mut(&mut self, id: &Uuid) -> Option<&mut Tenant> {
        self.tenants.get_mut(id)
    }

    pub fn get_by_tenant_id(&self, tenant_id: u32) -> Option<&Tenant> {
        self.tenant_id_index
            .get(&tenant_id)
            .and_then(|uuid| self.tenants.get(uuid))
    }

    pub fn remove_tenant(&mut self, id: &Uuid) -> Option<Tenant> {
        if let Some(t) = self.tenants.remove(id) {
            self.tenant_id_index.remove(&t.tenant_id);
            Some(t)
        } else {
            None
        }
    }

    pub fn get_all(&self) -> Vec<&Tenant> {
        self.tenants.values().collect()
    }

    /// Return the next available numeric tenant_id (starting at 1).
    pub fn next_tenant_id(&self) -> u32 {
        (1u32..)
            .find(|id| !self.tenant_id_index.contains_key(id))
            .unwrap() // infinite range always has a result
    }
}

/// High-level service for managing tenants and their network provisioning.
pub struct TenantService {
    storage: TenantStorage,
    /// Access to infrastructure provider APIs.
    provider_service: ProviderService,
    /// BGP AS number used across the fabric.
    bgp_as: u32,
    /// Source address (loopback/VTEP) used for VXLAN on managed routers.
    default_source_address: String,
}

impl TenantService {
    /// Create a new TenantService.
    ///
    /// `bgp_as` – the iBGP AS number for the fabric (e.g. 65000).  
    /// `default_source_address` – VTEP/loopback IP used when creating VXLAN interfaces.
    pub fn new(
        provider_service: ProviderService,
        bgp_as: u32,
        default_source_address: String,
    ) -> Self {
        Self {
            storage: TenantStorage::new(),
            provider_service,
            bgp_as,
            default_source_address,
        }
    }

    /// List all tenants.
    pub fn list_tenants(&self) -> Vec<&Tenant> {
        self.storage.get_all()
    }

    /// Get a tenant by UUID.
    pub fn get_tenant(&self, id: &Uuid) -> Option<&Tenant> {
        self.storage.get_tenant(id)
    }

    /// Create a new tenant record with auto-generated WireGuard keys and
    /// derived network configuration.
    ///
    /// This does **not** push any configuration to VyOS; call
    /// [`TenantService::provision_tenant_on_router`] for that.
    pub fn create_tenant(
        &mut self,
        name: &str,
        source_address: Option<&str>,
        vrrp: Option<VrrpConfig>,
    ) -> Result<Uuid> {
        let tenant_id = self.storage.next_tenant_id();

        // Generate a fresh WireGuard key pair for this tenant
        let kp = generate_wireguard_keypair()?;

        let mut tenant = Tenant::new(
            name.to_string(),
            tenant_id,
            self.bgp_as,
            source_address
                .unwrap_or(&self.default_source_address)
                .to_string(),
            kp.public_key,
            kp.private_key,
        );

        if let Some(v) = vrrp {
            tenant.set_vrrp(v);
        }

        let id = tenant.id;
        info!("Created tenant '{}' with id={} (tenant_id={})", name, id, tenant_id);
        self.storage.add_tenant(tenant);
        Ok(id)
    }

    /// Push the tenant network configuration to a named VyOS provider.
    ///
    /// On success the tenant status is updated to [`TenantStatus::Active`].
    pub async fn provision_tenant_on_router(
        &mut self,
        tenant_id: &Uuid,
        provider_name: &str,
    ) -> Result<()> {
        // Clone the tenant to avoid borrow issues while calling the async API
        let tenant = self
            .storage
            .get_tenant(tenant_id)
            .ok_or_else(|| anyhow!("Tenant not found: {}", tenant_id))?
            .clone();

        // Get a VyOS client for the target provider
        let mut client = self.provider_service.get_vyos_client(provider_name)?;

        // Provision on the router
        client.provision_tenant(&tenant).await.map_err(|e| {
            anyhow!(
                "Failed to provision tenant '{}' on '{}': {}",
                tenant.name,
                provider_name,
                e
            )
        })?;

        // Mark as active
        if let Some(t) = self.storage.get_tenant_mut(tenant_id) {
            t.set_active();
        }

        info!(
            "Tenant '{}' is now active on provider '{}'",
            tenant.name, provider_name
        );
        Ok(())
    }

    /// Delete a tenant record.
    ///
    /// This removes it from local storage; it does **not** roll back any
    /// already-applied VyOS configuration.
    pub fn delete_tenant(&mut self, id: &Uuid) -> Result<()> {
        self.storage
            .remove_tenant(id)
            .ok_or_else(|| anyhow!("Tenant not found: {}", id))?;
        info!("Deleted tenant {}", id);
        Ok(())
    }
}
