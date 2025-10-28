use std::io;
use std::env;

use clap::{Parser, Subcommand};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new BitBuilder Cloud project
    Init {
        #[arg(long)]
        name: Option<String>,
    },
    /// Deploy an application to BitBuilder Cloud
    Deploy {
        #[arg(long)]
        config: Option<String>,
    },
    /// Manage instances
    Instances {
        #[command(subcommand)]
        action: InstancesCommands,
    },
    /// Manage volumes
    Volumes {
        #[command(subcommand)]
        action: VolumesCommands,
    },
    /// Manage networks
    Networks {
        #[command(subcommand)]
        action: NetworksCommands,
    },
    /// Test connectivity to a VyOS router
    TestVyOS {
        /// VyOS host to connect to
        #[arg(long, default_value = "5.254.54.3")]
        host: String,
        
        /// VyOS SSH port
        #[arg(long, default_value = "60022")]
        port: u16,
        
        /// VyOS username
        #[arg(long, default_value = "vyos")]
        username: String,
        
        /// VyOS password (optional)
        #[arg(long)]
        password: Option<String>,
        
        /// Path to SSH key (optional)
        #[arg(long)]
        key_path: Option<String>,
        
        /// API key for HTTP API (optional)
        #[arg(long)]
        api_key: Option<String>,
    },
}

#[derive(Subcommand)]
enum InstancesCommands {
    /// List all instances
    List,
    /// Create a new instance
    Create {
        name: String,
        #[arg(long)]
        provider: String,
        #[arg(long)]
        region: String,
        #[arg(long)]
        cpu: Option<u8>,
        #[arg(long)]
        memory: Option<u8>,
        #[arg(long)]
        disk: Option<u8>,
    },
    /// Delete an instance
    Delete {
        id: String,
    },
    /// Start an instance
    Start {
        id: String,
    },
    /// Stop an instance
    Stop {
        id: String,
    },
    /// Get instance details
    Show {
        id: String,
    },
}

#[derive(Subcommand)]
enum VolumesCommands {
    /// List all volumes
    List,
    /// Create a new volume
    Create {
        name: String,
        #[arg(long)]
        size: u8,
        #[arg(long)]
        region: Option<String>,
    },
    /// Delete a volume
    Delete {
        id: String,
    },
    /// Attach a volume to an instance
    Attach {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Detach a volume from an instance
    Detach {
        id: String,
    },
    /// Get volume details
    Show {
        id: String,
    },
}

#[derive(Subcommand)]
enum NetworksCommands {
    /// List all networks
    List,
    /// Create a new network
    Create {
        name: String,
        #[arg(long)]
        cidr: String,
    },
    /// Delete a network
    Delete {
        id: String,
    },
    /// Connect an instance to a network
    Connect {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Disconnect an instance from a network
    Disconnect {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Get network details
    Show {
        id: String,
    },
}


fn cli_handler(cli: Cli) -> AppResult<()> {
    match cli.command {
        Some(Commands::Init { name }) => {
            println!("Initializing BitBuilder Cloud project: {}", 
                    name.unwrap_or_else(|| "bitbuilder-app".to_string()));
            // Actual implementation would initialize config files, etc.
        }
        Some(Commands::Deploy { config }) => {
            println!("Deploying to BitBuilder Cloud using config: {}", 
                    config.unwrap_or_else(|| "fly.toml".to_string()));
            // Actual implementation would handle the deployment
        }
        Some(Commands::Instances { action }) => {
            match action {
                InstancesCommands::List => {
                    println!("Listing instances...");
                    println!("ID\t\tNAME\tSTATUS\tREGION\tPROVIDER");
                    println!("i-01234567\tweb-1\trunning\tnyc\tvyos");
                    println!("i-89abcdef\tdb-1\trunning\tnyc\tproxmox");
                }
                InstancesCommands::Create { name, provider, region, cpu, memory, disk } => {
                    println!("Creating instance '{}' with provider '{}' in region '{}'", 
                            name, provider, region);
                    println!("Resources: CPU: {}, Memory: {} GB, Disk: {} GB", 
                            cpu.unwrap_or(1), memory.unwrap_or(2), disk.unwrap_or(10));
                }
                InstancesCommands::Delete { id } => {
                    println!("Deleting instance '{}'", id);
                }
                InstancesCommands::Start { id } => {
                    println!("Starting instance '{}'", id);
                }
                InstancesCommands::Stop { id } => {
                    println!("Stopping instance '{}'", id);
                }
                InstancesCommands::Show { id } => {
                    println!("Instance details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: web-1");
                    println!("Status: running");
                    println!("Provider: vyos");
                    println!("Region: nyc");
                    println!("IP: 192.168.1.10");
                    println!("CPU: 2");
                    println!("Memory: 4 GB");
                    println!("Disk: 80 GB");
                }
            }
        }
        Some(Commands::Volumes { action }) => {
            match action {
                VolumesCommands::List => {
                    println!("Listing volumes...");
                    println!("ID\t\tNAME\tSIZE\tREGION\tATTACHED TO");
                    println!("vol-01234567\tdb-data\t100 GB\tnyc\ti-89abcdef");
                }
                VolumesCommands::Create { name, size, region } => {
                    println!("Creating volume '{}' with size {} GB in region '{}'", 
                            name, size, region.unwrap_or_else(|| "nyc".to_string()));
                }
                VolumesCommands::Delete { id } => {
                    println!("Deleting volume '{}'", id);
                }
                VolumesCommands::Attach { id, instance } => {
                    println!("Attaching volume '{}' to instance '{}'", id, instance);
                }
                VolumesCommands::Detach { id } => {
                    println!("Detaching volume '{}'", id);
                }
                VolumesCommands::Show { id } => {
                    println!("Volume details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: db-data");
                    println!("Size: 100 GB");
                    println!("Region: nyc");
                    println!("Attached to: i-89abcdef (db-1)");
                }
            }
        }
        Some(Commands::Networks { action }) => {
            match action {
                NetworksCommands::List => {
                    println!("Listing networks...");
                    println!("ID\t\tNAME\tCIDR\t\tINSTANCES");
                    println!("net-01234567\tdefault\t192.168.1.0/24\t2");
                }
                NetworksCommands::Create { name, cidr } => {
                    println!("Creating network '{}' with CIDR '{}'", name, cidr);
                }
                NetworksCommands::Delete { id } => {
                    println!("Deleting network '{}'", id);
                }
                NetworksCommands::Connect { id, instance } => {
                    println!("Connecting instance '{}' to network '{}'", instance, id);
                }
                NetworksCommands::Disconnect { id, instance } => {
                    println!("Disconnecting instance '{}' from network '{}'", instance, id);
                }
                NetworksCommands::Show { id } => {
                    println!("Network details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: default");
                    println!("CIDR: 192.168.1.0/24");
                    println!("Instances: i-01234567 (web-1), i-89abcdef (db-1)");
                }
            }
        }
        Some(Commands::TestVyOS { host, port, username }) => {
            // This would block, so we need to call it outside the CLI handler
            // Will be implemented in main()
            return Err("Use tokio runtime to test VyOS connectivity".into());
        }
        None => {
            // If no subcommand is provided, we'll exit and let the main function
            // launch the TUI mode
            return Ok(());
        }
    }
    
    Ok(())
}

async fn run_tui() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // Setup logging
    env_logger::init();
    
    // Initialize configuration
    if let Err(e) = crate::config::init_config() {
        eprintln!("Warning: Failed to initialize configuration: {}", e);
        eprintln!("Some functionality may be limited.");
    }
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // If we have command-line arguments, handle them
    if env::args().len() > 1 {
        // Handle special async commands first
        match &cli.command {
            Some(Commands::TestVyOS { host, port, username, password, key_path, api_key }) => {
                println!("Testing connection to VyOS router at {}:{}...", host, port);
                
                // Create a VyOS client using our API
                use crate::api::vyos::{VyOSClient, VyOSConfig};
                use crate::api::Provider;
                
                let config = VyOSConfig {
                    host: host.clone(),
                    ssh_port: port,
                    api_port: 443, // Default API port
                    username: username.clone(),
                    password: password.clone(),
                    key_path: key_path.clone(),
                    api_key: api_key.clone(),
                    timeout: 30,
                };
                
                let client = VyOSClient::new(config);
                
                // First try the synchronous connection test
                match client.connect() {
                    Ok(_) => {
                        println!("\n✅ SSH connection successful!");
                        
                        // If API key is provided, also test the API
                        if let Some(api_key) = &api_key {
                            println!("\nTesting VyOS HTTP API...");
                            
                            let mut client_mut = client;
                            match client_mut.get_system_info().await {
                                Ok(info) => {
                                    println!("\n✅ API connection successful!");
                                    println!("\nVyOS system information:");
                                    println!("{}", serde_json::to_string_pretty(&info).unwrap_or_else(|_| info.to_string()));
                                },
                                Err(e) => {
                                    println!("\n❌ API connection failed: {}", e);
                                }
                            }
                        }
                        
                        return Ok(());
                    },
                    Err(e) => {
                        // Fallback to manual SSH connection if the client connect fails
                        println!("VyOS client connection failed: {}", e);
                        println!("Falling back to direct SSH connection...");
                        
                        // Let's try connecting interactively - we'll just verify the connection first
                        let ssh_command = format!("ssh -o StrictHostKeyChecking=no -p {} {}@{}", 
                                                port, username, host);
                        println!("Running: {}", ssh_command);
                        
                        let output = tokio::process::Command::new("sh")
                            .arg("-c")
                            .arg(ssh_command)
                            .output()
                            .await
                            .map_err(|e| format!("Failed to execute SSH command: {}", e))?;
                        
                        if output.status.success() || output.status.code() == Some(255) {
                            // If we got output, even with a non-zero exit code, that likely means
                            // we connected successfully but then got disconnected properly after the welcome message
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            if stdout.contains("VyOS") {
                                println!("\n✅ Connection successful!");
                                println!("\nVyOS system information:");
                                // Extract just the version information
                                if let Some(idx) = stdout.find("VyOS") {
                                    let version_info = &stdout[idx..];
                                    println!("{}", version_info);
                                } else {
                                    println!("{}", stdout);
                                }
                                return Ok(());
                            } else {
                                return Err(format!("Connected but did not receive VyOS welcome message").into());
                            }
                        } else {
                            let error = String::from_utf8_lossy(&output.stderr);
                            return Err(format!("Connection failed: {}", error).into());
                        }
                    }
                }
            },
            _ => {
                // For other commands, use the synchronous handler
                cli_handler(cli)?;
            }
        }
        
        Ok(())
    } else {
        // Otherwise, launch the TUI
        run_tui().await
    }
}
use std::io;
use std::env;

use clap::{Parser, Subcommand};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new BitBuilder Cloud project
    Init {
        #[arg(long)]
        name: Option<String>,
    },
    /// Deploy an application to BitBuilder Cloud
    Deploy {
        #[arg(long)]
        config: Option<String>,
    },
    /// Manage instances
    Instances {
        #[command(subcommand)]
        action: InstancesCommands,
    },
    /// Manage volumes
    Volumes {
        #[command(subcommand)]
        action: VolumesCommands,
    },
    /// Manage networks
    Networks {
        #[command(subcommand)]
        action: NetworksCommands,
    },
    /// Test connectivity to a VyOS router
    TestVyOS {
        /// VyOS host to connect to
        #[arg(long, default_value = "5.254.54.3")]
        host: String,
        
        /// VyOS SSH port
        #[arg(long, default_value = "60022")]
        port: u16,
        
        /// VyOS username
        #[arg(long, default_value = "vyos")]
        username: String,
        
        /// VyOS password (optional)
        #[arg(long)]
        password: Option<String>,
        
        /// Path to SSH key (optional)
        #[arg(long)]
        key_path: Option<String>,
        
        /// API key for HTTP API (optional)
        #[arg(long)]
        api_key: Option<String>,
    },
    /// Test connectivity to a VyOS router
    TestVyOS {
        /// VyOS host to connect to
        #[arg(long, default_value = "5.254.54.3")]
        host: String,
        
        /// VyOS SSH port
        #[arg(long, default_value = "60022")]
        port: u16,
        
        /// VyOS username
        #[arg(long, default_value = "vyos")]
        username: String,
        
        /// VyOS password (optional)
        #[arg(long)]
        password: Option<String>,
        
        /// Path to SSH key (optional)
        #[arg(long)]
        key_path: Option<String>,
        
        /// API key for HTTP API (optional)
        #[arg(long)]
        api_key: Option<String>,
    },
}

#[derive(Subcommand)]
enum InstancesCommands {
    /// List all instances
    List,
    /// Create a new instance
    Create {
        name: String,
        #[arg(long)]
        provider: String,
        #[arg(long)]
        region: String,
        #[arg(long)]
        cpu: Option<u8>,
        #[arg(long)]
        memory: Option<u8>,
        #[arg(long)]
        disk: Option<u8>,
    },
    /// Delete an instance
    Delete {
        id: String,
    },
    /// Start an instance
    Start {
        id: String,
    },
    /// Stop an instance
    Stop {
        id: String,
    },
    /// Get instance details
    Show {
        id: String,
    },
}

#[derive(Subcommand)]
enum VolumesCommands {
    /// List all volumes
    List,
    /// Create a new volume
    Create {
        name: String,
        #[arg(long)]
        size: u8,
        #[arg(long)]
        region: Option<String>,
    },
    /// Delete a volume
    Delete {
        id: String,
    },
    /// Attach a volume to an instance
    Attach {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Detach a volume from an instance
    Detach {
        id: String,
    },
    /// Get volume details
    Show {
        id: String,
    },
}

#[derive(Subcommand)]
enum NetworksCommands {
    /// List all networks
    List,
    /// Create a new network
    Create {
        name: String,
        #[arg(long)]
        cidr: String,
    },
    /// Delete a network
    Delete {
        id: String,
    },
    /// Connect an instance to a network
    Connect {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Disconnect an instance from a network
    Disconnect {
        id: String,
        #[arg(long)]
        instance: String,
    },
    /// Get network details
    Show {
        id: String,
    },
}

fn cli_handler(cli: Cli) -> AppResult<()> {
    match cli.command {
        Some(Commands::Init { name }) => {
            println!("Initializing BitBuilder Cloud project: {}", 
                    name.unwrap_or_else(|| "bitbuilder-app".to_string()));
            // Actual implementation would initialize config files, etc.
        }
        Some(Commands::Deploy { config }) => {
            println!("Deploying to BitBuilder Cloud using config: {}", 
                    config.unwrap_or_else(|| "fly.toml".to_string()));
            // Actual implementation would handle the deployment
        }
        Some(Commands::Instances { action }) => {
            match action {
                InstancesCommands::List => {
                    println!("Listing instances...");
                    println!("ID\t\tNAME\tSTATUS\tREGION\tPROVIDER");
                    println!("i-01234567\tweb-1\trunning\tnyc\tvyos");
                    println!("i-89abcdef\tdb-1\trunning\tnyc\tproxmox");
                }
                InstancesCommands::Create { name, provider, region, cpu, memory, disk } => {
                    println!("Creating instance '{}' with provider '{}' in region '{}'", 
                            name, provider, region);
                    println!("Resources: CPU: {}, Memory: {} GB, Disk: {} GB", 
                            cpu.unwrap_or(1), memory.unwrap_or(2), disk.unwrap_or(10));
                }
                InstancesCommands::Delete { id } => {
                    println!("Deleting instance '{}'", id);
                }
                InstancesCommands::Start { id } => {
                    println!("Starting instance '{}'", id);
                }
                InstancesCommands::Stop { id } => {
                    println!("Stopping instance '{}'", id);
                }
                InstancesCommands::Show { id } => {
                    println!("Instance details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: web-1");
                    println!("Status: running");
                    println!("Provider: vyos");
                    println!("Region: nyc");
                    println!("IP: 192.168.1.10");
                    println!("CPU: 2");
                    println!("Memory: 4 GB");
                    println!("Disk: 80 GB");
                }
            }
        }
        Some(Commands::Volumes { action }) => {
            match action {
                VolumesCommands::List => {
                    println!("Listing volumes...");
                    println!("ID\t\tNAME\tSIZE\tREGION\tATTACHED TO");
                    println!("vol-01234567\tdb-data\t100 GB\tnyc\ti-89abcdef");
                }
                VolumesCommands::Create { name, size, region } => {
                    println!("Creating volume '{}' with size {} GB in region '{}'", 
                            name, size, region.unwrap_or_else(|| "nyc".to_string()));
                }
                VolumesCommands::Delete { id } => {
                    println!("Deleting volume '{}'", id);
                }
                VolumesCommands::Attach { id, instance } => {
                    println!("Attaching volume '{}' to instance '{}'", id, instance);
                }
                VolumesCommands::Detach { id } => {
                    println!("Detaching volume '{}'", id);
                }
                VolumesCommands::Show { id } => {
                    println!("Volume details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: db-data");
                    println!("Size: 100 GB");
                    println!("Region: nyc");
                    println!("Attached to: i-89abcdef (db-1)");
                }
            }
        }
        Some(Commands::Networks { action }) => {
            match action {
                NetworksCommands::List => {
                    println!("Listing networks...");
                    println!("ID\t\tNAME\tCIDR\t\tINSTANCES");
                    println!("net-01234567\tdefault\t192.168.1.0/24\t2");
                }
                NetworksCommands::Create { name, cidr } => {
                    println!("Creating network '{}' with CIDR '{}'", name, cidr);
                }
                NetworksCommands::Delete { id } => {
                    println!("Deleting network '{}'", id);
                }
                NetworksCommands::Connect { id, instance } => {
                    println!("Connecting instance '{}' to network '{}'", instance, id);
                }
                NetworksCommands::Disconnect { id, instance } => {
                    println!("Disconnecting instance '{}' from network '{}'", instance, id);
                }
                NetworksCommands::Show { id } => {
                    println!("Network details for '{}':", id);
                    println!("ID: {}", id);
                    println!("Name: default");
                    println!("CIDR: 192.168.1.0/24");
                    println!("Instances: i-01234567 (web-1), i-89abcdef (db-1)");
                }
            }
        }
        Some(Commands::TestVyOS { host, port, username }) => {
            // This would block, so we need to call it outside the CLI handler
            // Will be implemented in main()
            return Err("Use tokio runtime to test VyOS connectivity".into());
        }
        None => {
            // If no subcommand is provided, we'll exit and let the main function
            // launch the TUI mode
            return Ok(());
        }
    }
    
    Ok(())
}

async fn run_tui() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // Setup logging
    env_logger::init();
    
    // Initialize configuration
    if let Err(e) = crate::config::init_config() {
        eprintln!("Warning: Failed to initialize configuration: {}", e);
        eprintln!("Some functionality may be limited.");
    }
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // If we have command-line arguments, handle them
    if env::args().len() > 1 {
        cli_handler(cli)
    } else {
        // Otherwise, launch the TUI
        run_tui().await
    }
}
