pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;
pub mod api;
pub mod models;
pub mod config;
pub mod services;

// Re-export commonly used types
pub use app::AppResult;
pub use models::provider::ProviderType;
pub use models::instance::InstanceStatus;
pub use models::volume::VolumeStatus;
pub use models::network::NetworkStatus;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");