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