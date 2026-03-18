use anyhow::{Result, Context, anyhow};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use std::time::Duration;
use log::{debug, error, info};

use crate::api::Provider;

/// VyOS API client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyOSConfig {
    /// VyOS router hostname or IP
    pub host: String,
    /// SSH port (default: 22)
    pub ssh_port: u16,
    /// HTTP API port (default: 443)
    pub api_port: u16,
    /// Username for authentication
    pub username: String,
    /// Password for authentication (optional if using key-based auth)
    pub password: Option<String>,
    /// Path to SSH key (optional if using password auth)
    pub key_path: Option<String>,
    /// API key for HTTP API (required for API operations)
    pub api_key: Option<String>,
    /// Connection timeout in seconds
    pub timeout: u64,
}

impl Default for VyOSConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            ssh_port: 22,
            api_port: 443,
            username: "vyos".to_string(),
            password: None,
            key_path: None,
            api_key: None,
            timeout: 30,
        }
    }
}

/// VyOS API client
#[derive(Debug)]
pub struct VyOSClient {
    config: VyOSConfig,
    http_client: Option<Client>,
    connected: bool,
}

impl VyOSClient {
    /// Create a new VyOS API client
    pub fn new(config: VyOSConfig) -> Self {
        Self {
            config,
            http_client: None,
            connected: false,
        }
    }
    
    /// Execute a command over SSH
    pub async fn execute_ssh_command(&self, command: &str) -> Result<String> {
        debug!("Executing SSH command: {}", command);
        
        let mut ssh_command = format!("ssh -o StrictHostKeyChecking=no -p {} {}@{}", 
                                     self.config.ssh_port, self.config.username, self.config.host);
        
        // Add key if specified
        if let Some(key_path) = &self.config.key_path {
            ssh_command = format!("{} -i {}", ssh_command, key_path);
        }
        
        // Add the actual command
        ssh_command = format!("{} '{}'", ssh_command, command);
        
        // Execute the command
        let output = AsyncCommand::new("sh")
            .arg("-c")
            .arg(ssh_command)
            .output()
            .await
            .context("Failed to execute SSH command")?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            debug!("SSH command output: {}", stdout);
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            error!("SSH command failed: {}", stderr);
            Err(anyhow!("SSH command failed: {}", stderr))
        }
    }
    
    /// Initialize HTTP client for API operations
    fn init_http_client(&mut self) -> Result<()> {
        if self.http_client.is_none() {
            let client = Client::builder()
                .timeout(Duration::from_secs(self.config.timeout))
                .danger_accept_invalid_certs(true) // VyOS might use self-signed certs
                .build()
                .context("Failed to build HTTP client")?;
            
            self.http_client = Some(client);
        }
        Ok(())
    }
    
    /// Make an API call to the VyOS HTTP API
    pub async fn api_call(&mut self, path: &str, method: &str, data: Option<serde_json::Value>) -> Result<serde_json::Value> {
        // Ensure HTTP client is initialized
        self.init_http_client()?;
        
        // Ensure API key is available
        let api_key = self.config.api_key.clone()
            .ok_or_else(|| anyhow!("API key is required for HTTP API operations"))?;
        
        let client = self.http_client.as_ref().unwrap();
        let url = format!("https://{}:{}/api/{}", self.config.host, self.config.api_port, path);
        
        debug!("Making API call: {} {}", method, url);
        
        let request_builder = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
        };
        
        // Add API key header
        let request_builder = request_builder.header("X-API-Key", api_key);
        
        // Add JSON body if provided
        let request_builder = if let Some(json_data) = data {
            request_builder.json(&json_data)
        } else {
            request_builder
        };
        
        // Execute the request
        let response = request_builder.send()
            .await
            .context("Failed to execute API request")?;
        
        let status = response.status();
        let body = response.json::<serde_json::Value>()
            .await
            .context("Failed to parse API response")?;
        
        if status.is_success() {
            Ok(body)
        } else {
            Err(anyhow!("API request failed: {} - {}", status, body))
        }
    }
    
    /// Get configuration from VyOS
    pub async fn get_config(&mut self, path: &str) -> Result<serde_json::Value> {
        self.api_call(&format!("config/{}", path), "GET", None).await
    }
    
    /// Set configuration in VyOS
    pub async fn set_config(&mut self, path: &str, value: serde_json::Value) -> Result<serde_json::Value> {
        self.api_call(&format!("config/{}", path), "PUT", Some(value)).await
    }
    
    /// Delete configuration in VyOS
    pub async fn delete_config(&mut self, path: &str) -> Result<serde_json::Value> {
        self.api_call(&format!("config/{}", path), "DELETE", None).await
    }
    
    /// Commit configuration changes
    pub async fn commit(&mut self) -> Result<serde_json::Value> {
        self.api_call("commit", "POST", None).await
    }
    
    /// Save configuration
    pub async fn save(&mut self) -> Result<serde_json::Value> {
        self.api_call("save", "POST", None).await
    }
    
    /// Check if connected to VyOS
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    /// Get system information
    pub async fn get_system_info(&mut self) -> Result<serde_json::Value> {
        self.api_call("system", "GET", None).await
    }

    // -------------------------------------------------------------------------
    // VyOS configure API helpers
    // -------------------------------------------------------------------------

    /// Send a single `set` operation to the VyOS configure API.
    ///
    /// `path` is a slice of path components, e.g.
    /// `["interfaces", "wireguard", "wg0", "address"]`.
    /// `value` is the optional leaf value string.
    pub async fn configure_set(
        &mut self,
        path: &[&str],
        value: Option<&str>,
    ) -> Result<()> {
        let mut body = serde_json::json!({
            "op": "set",
            "path": path
        });
        if let Some(v) = value {
            body["value"] = serde_json::Value::String(v.to_string());
        }
        self.configure_op(body).await?;
        Ok(())
    }

    /// Send a single `delete` operation to the VyOS configure API.
    pub async fn configure_delete(&mut self, path: &[&str]) -> Result<()> {
        let body = serde_json::json!({
            "op": "delete",
            "path": path
        });
        self.configure_op(body).await?;
        Ok(())
    }

    /// Send a raw operation body to `POST /configure`.
    async fn configure_op(&mut self, body: serde_json::Value) -> Result<serde_json::Value> {
        self.init_http_client()?;
        let api_key = self.config.api_key.clone()
            .ok_or_else(|| anyhow!("API key is required for HTTP API operations"))?;
        let client = self.http_client.as_ref().unwrap();
        let url = format!("https://{}:{}/configure", self.config.host, self.config.api_port);
        debug!("VyOS configure op: POST {} {:?}", url, body);
        let response = client
            .post(&url)
            .header("X-API-Key", api_key)
            .json(&body)
            .send()
            .await
            .context("Failed to execute VyOS configure request")?;
        let status = response.status();
        let resp_body = response.json::<serde_json::Value>()
            .await
            .context("Failed to parse VyOS configure response")?;
        if status.is_success() {
            Ok(resp_body)
        } else {
            Err(anyhow!("VyOS configure failed: {} – {}", status, resp_body))
        }
    }

    /// Query an operational-show endpoint (`GET /show/...`).
    async fn show_op(&mut self, path: &str) -> Result<serde_json::Value> {
        self.init_http_client()?;
        let api_key = self.config.api_key.clone()
            .ok_or_else(|| anyhow!("API key is required for HTTP API operations"))?;
        let client = self.http_client.as_ref().unwrap();
        let url = format!("https://{}:{}/show/{}", self.config.host, self.config.api_port, path);
        debug!("VyOS show: GET {}", url);
        let response = client
            .get(&url)
            .header("X-API-Key", api_key)
            .send()
            .await
            .context("Failed to execute VyOS show request")?;
        let status = response.status();
        let body = response.json::<serde_json::Value>()
            .await
            .context("Failed to parse VyOS show response")?;
        if status.is_success() {
            Ok(body)
        } else {
            Err(anyhow!("VyOS show failed: {} – {}", status, body))
        }
    }

    // -------------------------------------------------------------------------
    // WireGuard configuration
    // -------------------------------------------------------------------------

    /// Configure a WireGuard interface on VyOS.
    ///
    /// `interface` – e.g. `"wg0"`  
    /// `address` – CIDR, e.g. `"172.27.1.1/32"`  
    /// `private_key` – base64-encoded WireGuard private key  
    /// `port` – UDP listen port  
    /// `description` – optional human-readable description
    pub async fn configure_wireguard_interface(
        &mut self,
        interface: &str,
        address: &str,
        private_key: &str,
        port: u16,
        description: Option<&str>,
    ) -> Result<()> {
        self.configure_set(
            &["interfaces", "wireguard", interface, "address"],
            Some(address),
        ).await?;
        self.configure_set(
            &["interfaces", "wireguard", interface, "private-key"],
            Some(private_key),
        ).await?;
        self.configure_set(
            &["interfaces", "wireguard", interface, "port"],
            Some(&port.to_string()),
        ).await?;
        if let Some(desc) = description {
            self.configure_set(
                &["interfaces", "wireguard", interface, "description"],
                Some(desc),
            ).await?;
        }
        info!("Configured WireGuard interface {} on {}", interface, self.config.host);
        Ok(())
    }

    /// Add a WireGuard peer to an existing interface.
    ///
    /// `interface` – e.g. `"wg0"`  
    /// `peer_name` – peer identifier, e.g. `"PE2"`  
    /// `public_key` – base64-encoded peer public key  
    /// `endpoint` – optional `"ip:port"` string  
    /// `allowed_ips` – list of CIDRs the peer is allowed to send  
    /// `keepalive` – optional persistent keepalive in seconds
    pub async fn configure_wireguard_peer(
        &mut self,
        interface: &str,
        peer_name: &str,
        public_key: &str,
        endpoint: Option<&str>,
        allowed_ips: &[&str],
        keepalive: Option<u16>,
    ) -> Result<()> {
        self.configure_set(
            &["interfaces", "wireguard", interface, "peer", peer_name, "public-key"],
            Some(public_key),
        ).await?;
        for cidr in allowed_ips {
            self.configure_set(
                &["interfaces", "wireguard", interface, "peer", peer_name, "allowed-ips"],
                Some(cidr),
            ).await?;
        }
        if let Some(ep) = endpoint {
            self.configure_set(
                &["interfaces", "wireguard", interface, "peer", peer_name, "address"],
                Some(ep),
            ).await?;
        }
        if let Some(ka) = keepalive {
            self.configure_set(
                &["interfaces", "wireguard", interface, "peer", peer_name, "persistent-keepalive"],
                Some(&ka.to_string()),
            ).await?;
        }
        info!("Configured WireGuard peer {} on {}/{}", peer_name, self.config.host, interface);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // VXLAN configuration
    // -------------------------------------------------------------------------

    /// Configure a VXLAN interface on VyOS.
    ///
    /// `vni` – VXLAN Network Identifier  
    /// `source_address` – local VTEP IP  
    /// `mtu` – MTU (typically 9000)  
    /// `vrf` – optional VRF to attach the VXLAN interface to  
    /// `remote` – optional remote VTEP IP (unicast mode)
    pub async fn configure_vxlan(
        &mut self,
        vni: u32,
        source_address: &str,
        mtu: u16,
        vrf: Option<&str>,
        remote: Option<&str>,
    ) -> Result<()> {
        let iface = format!("vxlan{}", vni);
        self.configure_set(
            &["interfaces", "vxlan", &iface, "vni"],
            Some(&vni.to_string()),
        ).await?;
        self.configure_set(
            &["interfaces", "vxlan", &iface, "source-address"],
            Some(source_address),
        ).await?;
        self.configure_set(
            &["interfaces", "vxlan", &iface, "mtu"],
            Some(&mtu.to_string()),
        ).await?;
        if let Some(remote_ip) = remote {
            self.configure_set(
                &["interfaces", "vxlan", &iface, "remote"],
                Some(remote_ip),
            ).await?;
        }
        if let Some(vrf_name) = vrf {
            self.configure_set(
                &["interfaces", "vxlan", &iface, "vrf"],
                Some(vrf_name),
            ).await?;
        }
        info!("Configured VXLAN {} (VNI {}) on {}", iface, vni, self.config.host);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // BGP / EVPN configuration
    // -------------------------------------------------------------------------

    /// Set the BGP system AS number and router-ID.
    pub async fn configure_bgp_system(
        &mut self,
        system_as: u32,
        router_id: &str,
    ) -> Result<()> {
        self.configure_set(
            &["protocols", "bgp", "system-as"],
            Some(&system_as.to_string()),
        ).await?;
        self.configure_set(
            &["protocols", "bgp", "parameters", "router-id"],
            Some(router_id),
        ).await?;
        info!("Configured BGP AS {} router-id {} on {}", system_as, router_id, self.config.host);
        Ok(())
    }

    /// Add a BGP EVPN peer (iBGP neighbor with l2vpn-evpn address-family).
    pub async fn configure_bgp_evpn_peer(
        &mut self,
        peer_ip: &str,
        remote_as: u32,
        update_source: &str,
    ) -> Result<()> {
        self.configure_set(
            &["protocols", "bgp", "neighbor", peer_ip, "remote-as"],
            Some(&remote_as.to_string()),
        ).await?;
        self.configure_set(
            &["protocols", "bgp", "neighbor", peer_ip, "update-source"],
            Some(update_source),
        ).await?;
        self.configure_set(
            &["protocols", "bgp", "neighbor", peer_ip, "address-family", "l2vpn-evpn", "activate"],
            None,
        ).await?;
        info!("Configured BGP EVPN peer {} on {}", peer_ip, self.config.host);
        Ok(())
    }

    /// Enable BGP EVPN VNI advertisement (`advertise-all-vni`).
    pub async fn enable_bgp_evpn(&mut self) -> Result<()> {
        self.configure_set(
            &["protocols", "bgp", "address-family", "l2vpn-evpn", "advertise-all-vni"],
            None,
        ).await?;
        info!("Enabled BGP EVPN advertise-all-vni on {}", self.config.host);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // L3VPN / VRF configuration
    // -------------------------------------------------------------------------

    /// Create an L3VPN VRF and set its BGP route-targets.
    ///
    /// `vrf_name` – e.g. `"tenant-1"`  
    /// `table_id` – kernel routing table ID, e.g. `1001`  
    /// `route_target_export` – e.g. `"65000:1001"`  
    /// `route_target_import` – e.g. `"65000:1001"`
    pub async fn configure_vrf(
        &mut self,
        vrf_name: &str,
        table_id: u32,
        route_target_export: &str,
        route_target_import: &str,
    ) -> Result<()> {
        self.configure_set(
            &["vrf", "name", vrf_name, "table"],
            Some(&table_id.to_string()),
        ).await?;
        self.configure_set(
            &["vrf", "name", vrf_name, "protocols", "bgp", "address-family",
              "ipv4-unicast", "route-target", "vpn", "export"],
            Some(route_target_export),
        ).await?;
        self.configure_set(
            &["vrf", "name", vrf_name, "protocols", "bgp", "address-family",
              "ipv4-unicast", "route-target", "vpn", "import"],
            Some(route_target_import),
        ).await?;
        info!("Configured VRF {} (table {}) on {}", vrf_name, table_id, self.config.host);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // VRRP configuration
    // -------------------------------------------------------------------------

    /// Configure a VRRP group for HA gateway redundancy.
    ///
    /// `group_id` – VRRP group name  
    /// `interface` – network interface to run VRRP on  
    /// `virtual_ip` – shared virtual IP address (CIDR or plain IP)  
    /// `vrid` – VRRP virtual router ID (1–255)  
    /// `priority` – router priority (1–254; higher = master preference)
    pub async fn configure_vrrp(
        &mut self,
        group_id: &str,
        interface: &str,
        virtual_ip: &str,
        vrid: u8,
        priority: u8,
    ) -> Result<()> {
        self.configure_set(
            &["high-availability", "vrrp", "group", group_id, "interface"],
            Some(interface),
        ).await?;
        self.configure_set(
            &["high-availability", "vrrp", "group", group_id, "virtual-address"],
            Some(virtual_ip),
        ).await?;
        self.configure_set(
            &["high-availability", "vrrp", "group", group_id, "vrid"],
            Some(&vrid.to_string()),
        ).await?;
        self.configure_set(
            &["high-availability", "vrrp", "group", group_id, "priority"],
            Some(&priority.to_string()),
        ).await?;
        info!(
            "Configured VRRP group {} on {}/{} (vrid {}, priority {})",
            group_id, self.config.host, interface, vrid, priority
        );
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Tenant provisioning
    // -------------------------------------------------------------------------

    /// Provision all networking for a single tenant on this VyOS router.
    ///
    /// This is a convenience wrapper that calls the individual configure
    /// methods in the correct order and commits the session at the end.
    ///
    /// Returns the VXLAN VNI used for this tenant.
    pub async fn provision_tenant(
        &mut self,
        tenant: &crate::models::tenant::Tenant,
    ) -> Result<u32> {
        info!("Provisioning tenant '{}' (id={}) on {}", tenant.name, tenant.tenant_id, self.config.host);

        // 1. Create VRF
        self.configure_vrf(
            &tenant.vrf.name,
            tenant.vrf.table_id,
            &tenant.vrf.route_target_export,
            &tenant.vrf.route_target_import,
        ).await?;

        // 2. Configure VXLAN
        self.configure_vxlan(
            tenant.vxlan.vni,
            &tenant.vxlan.source_address,
            tenant.vxlan.mtu,
            Some(&tenant.vrf.name),
            tenant.vxlan.remote.as_deref(),
        ).await?;

        // 3. Configure WireGuard interface for the tenant
        let wg_iface = format!("wg{}", tenant.wireguard.interface_index);
        self.configure_wireguard_interface(
            &wg_iface,
            &tenant.wireguard.address,
            &tenant.wireguard.private_key,
            tenant.wireguard.port,
            Some(&format!("Tenant {} WireGuard", tenant.name)),
        ).await?;

        // 4. Optional VRRP
        if let Some(vrrp) = &tenant.vrrp {
            self.configure_vrrp(
                &vrrp.group_id,
                &vrrp.interface,
                &vrrp.virtual_ip,
                vrrp.vrid,
                vrrp.priority,
            ).await?;
        }

        // 5. Commit and save
        self.commit().await?;
        self.save().await?;

        info!(
            "Tenant '{}' provisioned successfully on {} (VNI {})",
            tenant.name, self.config.host, tenant.vxlan.vni
        );
        Ok(tenant.vxlan.vni)
    }

    // -------------------------------------------------------------------------
    // Monitoring / operational show
    // -------------------------------------------------------------------------

    /// Get BGP summary (all address families).
    pub async fn get_bgp_summary(&mut self) -> Result<serde_json::Value> {
        self.show_op("bgp/summary/json").await
    }

    /// Get VXLAN interface status.
    pub async fn get_vxlan_status(&mut self) -> Result<serde_json::Value> {
        self.show_op("interfaces/vxlan/json").await
    }

    /// Get the routing table for all VRFs.
    pub async fn get_vrf_routes(&mut self) -> Result<serde_json::Value> {
        self.show_op("ip/route/vrf/all/json").await
    }

    /// Get WireGuard interface status.
    pub async fn get_wireguard_status(&mut self) -> Result<serde_json::Value> {
        self.show_op("interfaces/wireguard/json").await
    }
}

impl Provider for VyOSClient {
    fn connect(&self) -> Result<()> {
        // Synchronous version for the Provider trait
        let mut cmd = Command::new("ssh");
        cmd.arg("-o")
           .arg("StrictHostKeyChecking=no")
           .arg("-p")
           .arg(self.config.ssh_port.to_string())
           .arg(format!("{}@{}", self.config.username, self.config.host))
           .arg("show system version");
           
        // Add key if specified
        if let Some(key_path) = &self.config.key_path {
            cmd.arg("-i").arg(key_path);
        }
        
        let output = cmd.output().context("Failed to execute SSH command")?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            if stdout.contains("VyOS") {
                info!("Successfully connected to VyOS: {}", self.config.host);
                // We would set self.connected = true here, but self is immutable
                // In a real implementation we'd use interior mutability or refactor
                Ok(())
            } else {
                Err(anyhow!("Connected but not a VyOS system"))
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to connect to VyOS: {}", stderr))
        }
    }
    
    fn check_connection(&self) -> Result<bool> {
        // For simplicity, just check if we're marked as connected
        // In a real implementation, we'd do a lightweight check
        Ok(self.connected)
    }
    
    fn name(&self) -> &str {
        "VyOS"
    }
}