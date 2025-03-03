use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Home,
    Instances,
    Volumes,
    Networks,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub status: String,
    pub provider: String,
    pub region: String,
    pub ip: String,
    pub cpu: u8,
    pub memory_gb: u8,
    pub disk_gb: u8,
}

#[derive(Debug, Clone)]
pub struct Volume {
    pub id: String,
    pub name: String,
    pub size_gb: u8,
    pub attached_to: Option<String>,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub cidr: String,
    pub instances: Vec<String>,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current application mode
    pub mode: AppMode,
    /// Selected index for list views
    pub selected_index: usize,
    /// List of instances (VMs)
    pub instances: Vec<Instance>,
    /// List of volumes
    pub volumes: Vec<Volume>,
    /// List of networks
    pub networks: Vec<Network>,
}

impl Default for App {
    fn default() -> Self {
        let mut instances = Vec::new();
        // Add some demo instances for testing
        instances.push(Instance {
            id: "i-01234567".to_string(),
            name: "web-1".to_string(),
            status: "running".to_string(),
            provider: "vyos".to_string(),
            region: "nyc".to_string(),
            ip: "192.168.1.10".to_string(),
            cpu: 2,
            memory_gb: 4,
            disk_gb: 80,
        });
        
        instances.push(Instance {
            id: "i-89abcdef".to_string(),
            name: "db-1".to_string(),
            status: "running".to_string(),
            provider: "proxmox".to_string(),
            region: "nyc".to_string(),
            ip: "192.168.1.11".to_string(),
            cpu: 4,
            memory_gb: 16,
            disk_gb: 160,
        });
        
        let mut volumes = Vec::new();
        volumes.push(Volume {
            id: "vol-01234567".to_string(),
            name: "db-data".to_string(),
            size_gb: 100,
            attached_to: Some("i-89abcdef".to_string()),
            region: "nyc".to_string(),
        });
        
        let mut networks = Vec::new();
        networks.push(Network {
            id: "net-01234567".to_string(),
            name: "default".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            instances: vec!["i-01234567".to_string(), "i-89abcdef".to_string()],
        });
        
        Self {
            running: true,
            mode: AppMode::Home,
            selected_index: 0,
            instances,
            volumes,
            networks,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    
    pub fn next_item(&mut self) {
        let max_index = match self.mode {
            AppMode::Instances => self.instances.len().saturating_sub(1),
            AppMode::Volumes => self.volumes.len().saturating_sub(1),
            AppMode::Networks => self.networks.len().saturating_sub(1),
            _ => 0,
        };
        
        if max_index > 0 {
            self.selected_index = if self.selected_index >= max_index {
                0
            } else {
                self.selected_index + 1
            };
        }
    }
    
    pub fn previous_item(&mut self) {
        let max_index = match self.mode {
            AppMode::Instances => self.instances.len().saturating_sub(1),
            AppMode::Volumes => self.volumes.len().saturating_sub(1),
            AppMode::Networks => self.networks.len().saturating_sub(1),
            _ => 0,
        };
        
        if max_index > 0 {
            self.selected_index = if self.selected_index == 0 {
                max_index
            } else {
                self.selected_index - 1
            };
        }
    }
    
    pub fn change_mode(&mut self, mode: AppMode) {
        self.mode = mode;
        self.selected_index = 0;
    }
}
