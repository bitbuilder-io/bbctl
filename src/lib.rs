pub mod api;
pub mod app;
pub mod config;
pub mod event;
pub mod handler;
pub mod models;
pub mod services;
pub mod tui;
pub mod ui;

// Re-export commonly used types
pub use app::AppResult;
pub use models::instance::InstanceStatus;
pub use models::network::NetworkStatus;
pub use models::provider::ProviderType;
pub use models::volume::VolumeStatus;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
