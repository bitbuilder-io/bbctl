pub mod vyos;
pub mod proxmox;

use anyhow::Result;

/// Common trait for all infrastructure providers
pub trait Provider {
    /// Connect to the provider
    fn connect(&self) -> Result<()>;
    
    /// Check connection status
    fn check_connection(&self) -> Result<bool>;
    
    /// Get provider name
    fn name(&self) -> &str;
}

/// Result type for provider operations
pub type ProviderResult<T> = Result<T>;