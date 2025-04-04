use anyhow::{Result, Context, anyhow};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use log::{debug, error, info};

use crate::api::Provider;

/// Proxmox authentication types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxmoxAuth {
    /// Username and password authentication
    UserPass {
        username: String,
        password: String,
        realm: String,
    },
    /// API token authentication
    ApiToken {
        token_id: String,
        token_secret: String,
    },
}

/// Proxmox API client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxConfig {
    /// Proxmox server hostname or IP
    pub host: String,
    /// API port (default: 8006)
    pub port: u16,
    /// Authentication method
    pub auth: ProxmoxAuth,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Verify SSL certificates
    pub verify_ssl: bool,
}

impl Default for ProxmoxConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8006,
            auth: ProxmoxAuth::UserPass {
                username: "root".to_string(),
                password: "".to_string(),
                realm: "pam".to_string(),
            },
            timeout: 30,
            verify_ssl: true,
        }
    }
}

/// Proxmox API client
#[derive(Debug)]
pub struct ProxmoxClient {
    config: ProxmoxConfig,
    http_client: Option<Client>,
    ticket: Option<String>,
    csrf_token: Option<String>,
    connected: bool,
}

impl ProxmoxClient {
    /// Create a new Proxmox API client
    pub fn new(config: ProxmoxConfig) -> Self {
        Self {
            config,
            http_client: None,
            ticket: None,
            csrf_token: None,
            connected: false,
        }
    }
    
    /// Initialize HTTP client
    fn init_http_client(&mut self) -> Result<()> {
        if self.http_client.is_none() {
            let client = Client::builder()
                .timeout(Duration::from_secs(self.config.timeout))
                .danger_accept_invalid_certs(!self.config.verify_ssl)
                .build()
                .context("Failed to build HTTP client")?;
            
            self.http_client = Some(client);
        }
        Ok(())
    }
    
    /// Login to Proxmox and get authentication ticket
    pub async fn login(&mut self) -> Result<()> {
        // Ensure HTTP client is initialized
        self.init_http_client()?;
        
        let client = self.http_client.as_ref().unwrap();
        let url = format!("https://{}:{}/api2/json/access/ticket", self.config.host, self.config.port);
        
        debug!("Logging in to Proxmox: {}", url);
        
        match &self.config.auth {
            ProxmoxAuth::UserPass { username, password, realm } => {
                // Build form data for username/password auth
                let params = [
                    ("username", username.as_str()),
                    ("password", password.as_str()),
                    ("realm", realm.as_str()),
                ];
                
                let response = client.post(&url)
                    .form(&params)
                    .send()
                    .await
                    .context("Failed to send login request")?;
                
                if response.status().is_success() {
                    let json: serde_json::Value = response.json()
                        .await
                        .context("Failed to parse login response")?;
                    
                    // Extract authentication data
                    if let Some(data) = json.get("data") {
                        self.ticket = data.get("ticket").and_then(|v| v.as_str()).map(|s| s.to_string());
                        self.csrf_token = data.get("CSRFPreventionToken").and_then(|v| v.as_str()).map(|s| s.to_string());
                        
                        if self.ticket.is_some() && self.csrf_token.is_some() {
                            self.connected = true;
                            info!("Successfully logged in to Proxmox: {}", self.config.host);
                            return Ok(());
                        }
                    }
                    
                    Err(anyhow!("Failed to extract auth data from login response"))
                } else {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    Err(anyhow!("Login failed: {} - {}", status, body))
                }
            },
            ProxmoxAuth::ApiToken { token_id, token_secret } => {
                // API token auth doesn't need a login step, just verify we can access the API
                let auth_header = format!("PVEAPIToken={}={}", token_id, token_secret);
                
                // Test connection with a simple API call
                let response = client.get(&format!("https://{}:{}/api2/json/version", self.config.host, self.config.port))
                    .header("Authorization", auth_header)
                    .send()
                    .await
                    .context("Failed to test API token authentication")?;
                
                if response.status().is_success() {
                    self.connected = true;
                    info!("Successfully authenticated to Proxmox with API token: {}", self.config.host);
                    Ok(())
                } else {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    Err(anyhow!("API token authentication failed: {} - {}", status, body))
                }
            }
        }
    }
    
    /// Make an API call to the Proxmox API
    pub async fn api_call(&mut self, path: &str, method: &str, data: Option<serde_json::Value>) -> Result<serde_json::Value> {
        // Ensure we're authenticated
        if !self.connected {
            self.login().await?;
        }
        
        // Ensure HTTP client is initialized
        self.init_http_client()?;
        
        let client = self.http_client.as_ref().unwrap();
        let url = format!("https://{}:{}/api2/json/{}", self.config.host, self.config.port, path);
        
        debug!("Making API call: {} {}", method, url);
        
        let mut request_builder = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
        };
        
        // Add authentication
        match &self.config.auth {
            ProxmoxAuth::UserPass { .. } => {
                // Cookie-based auth
                if let Some(ticket) = &self.ticket {
                    request_builder = request_builder.header("Cookie", format!("PVEAuthCookie={}", ticket));
                    
                    // Add CSRF token for non-GET requests
                    if method != "GET" {
                        if let Some(csrf) = &self.csrf_token {
                            request_builder = request_builder.header("CSRFPreventionToken", csrf);
                        }
                    }
                } else {
                    return Err(anyhow!("Not authenticated - missing ticket"));
                }
            },
            ProxmoxAuth::ApiToken { token_id, token_secret } => {
                // API token auth
                let auth_header = format!("PVEAPIToken={}={}", token_id, token_secret);
                request_builder = request_builder.header("Authorization", auth_header);
            }
        }
        
        // Add JSON body if provided
        if let Some(json_data) = data {
            request_builder = request_builder.json(&json_data);
        }
        
        // Execute the request
        let response = request_builder.send()
            .await
            .context("Failed to execute API request")?;
        
        let status = response.status();
        
        if status.is_success() {
            let body = response.json::<serde_json::Value>()
                .await
                .context("Failed to parse API response")?;
            
            // Check for error in response body (Proxmox might return 200 OK with error in body)
            if let Some(data) = body.get("data") {
                Ok(data.clone())
            } else {
                Ok(body)
            }
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(anyhow!("API request failed: {} - {}", status, body))
        }
    }
    
    /// Get cluster resources
    pub async fn get_resources(&mut self, resource_type: Option<&str>) -> Result<serde_json::Value> {
        let path = match resource_type {
            Some(rtype) => format!("cluster/resources?type={}", rtype),
            None => "cluster/resources".to_string(),
        };
        
        self.api_call(&path, "GET", None).await
    }
    
    /// Get list of nodes in the cluster
    pub async fn get_nodes(&mut self) -> Result<serde_json::Value> {
        self.api_call("nodes", "GET", None).await
    }
    
    /// Get list of VMs on a specific node
    pub async fn get_vms(&mut self, node: &str) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu", node), "GET", None).await
    }
    
    /// Get VM status
    pub async fn get_vm_status(&mut self, node: &str, vmid: u64) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu/{}/status/current", node, vmid), "GET", None).await
    }
    
    /// Start a VM
    pub async fn start_vm(&mut self, node: &str, vmid: u64) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu/{}/status/start", node, vmid), "POST", None).await
    }
    
    /// Stop a VM
    pub async fn stop_vm(&mut self, node: &str, vmid: u64) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu/{}/status/stop", node, vmid), "POST", None).await
    }
    
    /// Create a new VM
    pub async fn create_vm(&mut self, node: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu", node), "POST", Some(params)).await
    }
    
    /// Delete a VM
    pub async fn delete_vm(&mut self, node: &str, vmid: u64) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/qemu/{}", node, vmid), "DELETE", None).await
    }
    
    /// Get storage information
    pub async fn get_storage(&mut self, node: &str) -> Result<serde_json::Value> {
        self.api_call(&format!("nodes/{}/storage", node), "GET", None).await
    }
}

impl Provider for ProxmoxClient {
    fn connect(&self) -> Result<()> {
        // Synchronous version just checks if we're already connected
        // In a real implementation, we would do an actual connection test
        if self.connected {
            Ok(())
        } else {
            Err(anyhow!("Not connected to Proxmox"))
        }
    }
    
    fn check_connection(&self) -> Result<bool> {
        Ok(self.connected)
    }
    
    fn name(&self) -> &str {
        "Proxmox"
    }
}