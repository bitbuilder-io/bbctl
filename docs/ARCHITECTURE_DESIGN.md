# BitBuilder Cloud CLI (bbctl) Architecture Design

This document outlines the complete architecture, design decisions, implementation status, and future roadmap for the bbctl project.

## Project Overview

bbctl is a command-line interface (CLI) tool for provisioning and managing multi-tenant infrastructure on bare metal servers running VyOS v1.5 or Proxmox. Similar to fly.io's flyctl, bbctl provides a seamless experience for deploying, scaling, and managing applications across distributed infrastructure.

### Project Goals

1. Provide a single CLI tool for managing infrastructure across multiple providers
2. Enable secure multi-tenant isolation using VRFs, VXLANs, and L3VPNs
3. Support end-to-end encryption using WireGuard
4. Implement gitops-style declarative configuration
5. Deliver an intuitive Terminal UI (TUI) for interactive management

## System Architecture

The bbctl architecture consists of multiple layers:

```
┌────────────────────────────────────────────────────────────────────┐
│                       User Interface Layer                         │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │   CLI Commands      │  │   Terminal UI       │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
│                                                                    │
├────────────────────────────────────────────────────────────────────┤
│                        Service Layer                               │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │  Provider Services  │  │  Resource Services  │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
│                                                                    │
├────────────────────────────────────────────────────────────────────┤
│                         API Layer                                  │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │     VyOS API        │  │    Proxmox API      │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
│                                                                    │
├────────────────────────────────────────────────────────────────────┤
│                        Data Model Layer                            │
│                                                                    │
│  ┌───────────┐ ┌──────────┐ ┌────────────┐ ┌──────────────┐        │
│  │ Instances │ │ Volumes  │ │ Networks   │ │ Providers    │        │
│  └───────────┘ └──────────┘ └────────────┘ └──────────────┘        │
│                                                                    │
├────────────────────────────────────────────────────────────────────┤
│                     Configuration Layer                            │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │   Local Settings    │  │    Credentials      │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
└────────────────────────────────────────────────────────────────────┘
```

### Component Details

1. **User Interface Layer**

- CLI Commands: Handles command-line arguments and options
- Terminal UI (TUI): Interactive dashboard for visualization and management

2. **Service Layer**

- Provider Services: Manages infrastructure providers
- Resource Services: Abstracts operations on instances, volumes, networks

3. **API Layer**

- VyOS API: Client for VyOS HTTP API and SSH interfaces
- Proxmox API: Client for Proxmox REST API

4. **Data Model Layer**

- Instances: VM/container representations
- Volumes: Storage abstractions
- Networks: Network and connectivity abstractions
- Providers: Provider metadata and capabilities

5. **Configuration Layer**

- Local Settings: User preferences and defaults
- Credentials: Secure storage for authentication information
   - CLI Commands: Handles command-line arguments and options
   - Terminal UI (TUI): Interactive dashboard for visualization and management

2. **Service Layer**
   - Provider Services: Manages infrastructure providers
   - Resource Services: Abstracts operations on instances, volumes, networks

3. **API Layer**
   - VyOS API: Client for VyOS HTTP API and SSH interfaces
   - Proxmox API: Client for Proxmox REST API

4. **Data Model Layer**
   - Instances: VM/container representations
   - Volumes: Storage abstractions
   - Networks: Network and connectivity abstractions
   - Providers: Provider metadata and capabilities

5. **Configuration Layer**
   - Local Settings: User preferences and defaults
   - Credentials: Secure storage for authentication information

## Implementation Details

### 1. Core Codebase Structure

```
bbctl/
├── src/
│   ├── api/            # API clients
│   │   ├── mod.rs      # Common API traits
│   │   ├── vyos.rs     # VyOS API client
│   │   └── proxmox.rs  # Proxmox API client
│   ├── models/         # Data models
│   │   ├── mod.rs      # Model exports
│   │   ├── instance.rs # Instance model
│   │   ├── volume.rs   # Volume model
│   │   ├── network.rs  # Network model
│   │   └── provider.rs # Provider model
│   ├── services/       # Business logic
│   │   ├── mod.rs      # Service exports
│   │   ├── provider.rs # Provider management
│   │   ├── instance.rs # Instance operations
│   │   ├── volume.rs   # Volume operations
│   │   └── network.rs  # Network operations
│   ├── config/         # Configuration management
│   │   ├── mod.rs      # Configuration utilities
│   │   ├── settings.rs # User settings
│   │   ├── provider.rs # Provider configurations
│   │   └── credentials.rs # Authentication data
│   ├── app.rs          # Application state
│   ├── tui.rs          # Terminal UI setup
│   ├── ui.rs           # UI components and rendering
│   ├── event.rs        # Event handling
│   ├── handler.rs      # Event handlers
│   ├── main.rs         # Entry point
│   └── lib.rs          # Library exports
├── tests/              # Test suite
│   ├── integration/    # Integration tests
│   ├── vyos-lab/       # VyOS test environment
│   └── containers/     # Container test files
└── docs/               # Documentation
    ├── ARCHITECTURE_DESIGN.md  # This document
    └── vyos-test-lab-setup.md  # Test lab documentation
```

### 2. API Layer Implementation

#### Provider Trait

The `Provider` trait defines the common interface for all infrastructure providers:

```bash
pub trait Provider {
    /// Connect to the provider
    fn connect(&self) -> Result<()>;

    /// Check connection status
    fn check_connection(&self) -> Result<bool>;

```rust
pub trait Provider {
    /// Connect to the provider
    fn connect(&self) -> Result<()>;
    
    /// Check connection status
    fn check_connection(&self) -> Result<bool>;
    
    /// Get provider name
    fn name(&self) -> &str;
}
```

#### VyOS API Client

The VyOS API client supports: - SSH-based configuration management - HTTP API integration for automated provisioning - WireGuard key generation and management - L3VPN and VXLAN configuration

#### Proxmox API Client

The Proxmox API client supports: - REST API integration for VM management - Resource allocation and monitoring - Template management for deployments - Both token and username/password authentication
The VyOS API client supports:
- SSH-based configuration management
- HTTP API integration for automated provisioning
- WireGuard key generation and management
- L3VPN and VXLAN configuration

#### Proxmox API Client

The Proxmox API client supports:
- REST API integration for VM management
- Resource allocation and monitoring
- Template management for deployments
- Both token and username/password authentication

### 3. Data Models

#### Instance Model

Represents virtual machines and containers:

```bash
```rust
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub status: InstanceStatus,
    pub provider: ProviderType,
    pub provider_id: String,
    pub region: String,
    pub size: InstanceSize,
    pub networks: Vec<InstanceNetwork>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}
```

#### Volume Model

Represents storage volumes:

```bash
```rust
pub struct Volume {
    pub id: Uuid,
    pub name: String,
    pub status: VolumeStatus,
    pub provider: ProviderType,
    pub provider_id: String,
    pub region: String,
    pub size_gb: u16,
    pub volume_type: VolumeType,
    pub attached_to: Option<Uuid>,
    pub device: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}
```

#### Network Model

Represents virtual networks:

```bash
```rust
pub struct Network {
    pub id: Uuid,
    pub name: String,
    pub status: NetworkStatus,
    pub provider: ProviderType,
    pub provider_id: String,
    pub region: String,
    pub cidr: String,
    pub network_type: NetworkType,
    pub gateway: Option<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub instances: HashSet<Uuid>,
    pub ip_allocations: Vec<IpAllocation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub config: HashMap<String, String>,
}
```

### 4. Service Layer

#### Provider Service

Manages infrastructure providers, their credentials, and connections:

```bash
```rust
pub struct ProviderService {
    providers: Providers,
    credentials: Credentials,
}
```

#### Instance Service

Handles VM/container lifecycle operations:

```bash
```rust
pub struct InstanceService {
    storage: InstanceStorage,
    provider_service: ProviderService,
}
```

### 5. Configuration Management

Configuration is stored in the user's home directory:

```
~/.bbctl/
├── settings.toml     # User settings
├── providers.toml    # Provider configurations
└── credentials.toml  # Authentication data
```

### 6. CLI Interface

The CLI supports the following main commands:

- `bbctl init` - Initialize a new project
- `bbctl instances` - List/create/manage VMs
- `bbctl volumes` - Manage storage
- `bbctl networks` - Configure virtual networks
- `bbctl test-vyos` - Test connectivity to VyOS router

### 7. Terminal UI (TUI)

The TUI provides an interactive dashboard with:

- Instances view
- Volumes view
- Networks view
- Settings management
- Real-time status updates

## Test Environment

A complete test environment has been implemented to validate bbctl against real infrastructure:

### VyOS Test Lab

The test lab simulates a multi-tenant infrastructure using Docker containers running VyOS:

```
┌────────────────────────────────────────────────────────────────────┐
│                          Host System                               │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │   Docker Container  │  │   Docker Container  │                  │
│  │                     │  │                     │                  │
│  │  ┌───────────────┐  │  │  ┌───────────────┐  │                  │
│  │  │  VyOS Router  │  │  │  │  VyOS Router  │  │                  │
│  │  │  (PE1)        │◄─┼──┼──►  (PE2)        │  │                  │
│  │  └───────────────┘  │  │  └───────────────┘  │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │                     Docker Networks                        │    │
│  └────────────────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────────────┘
```

The test lab implements:

1. **WireGuard Control Plane**: Secure management and control plane using WireGuard VPN
2. **BGP EVPN**: Control plane for multi-tenant VXLAN networks
3. **L3VPN**: Tenant isolation using VRFs and route targets
4. **HTTP API**: Endpoints for bbctl to manage infrastructure

### Test Scripts

The test environment is managed by a set of scripts:

- `setup-base.sh` - Sets up base infrastructure
- `setup-vyos-container.sh` - Deploys VyOS containers
- `configure-l3vpn-evpn.sh` - Configures L3VPN with EVPN
- `configure-wireguard.sh` - Sets up WireGuard secure management
- `setup-lab.sh` - Main orchestration script
- `cleanup-lab.sh` - Teardown script

## Current Status

### Completed Components

1. **API Layer**

- ✅ Provider interface trait
- ✅ VyOS API client
- ✅ Proxmox API client

2. **Data Models**

- ✅ Instance model
- ✅ Volume model
- ✅ Network model
- ✅ Provider model

3. **Configuration Management**

- ✅ Settings model and storage
- ✅ Provider configuration
- ✅ Credential management

4. **Basic Services**

- ✅ Provider service
- ✅ Instance service (partial)

5. **CLI Interface**

- ✅ Basic command structure
- ✅ VyOS connectivity testing

6. **Terminal UI**

- ✅ Basic TUI framework
- ✅ Navigation and layout

7. **Test Environment**

- ✅ VyOS lab setup scripts
- ✅ L3VPN and EVPN configuration
- ✅ WireGuard secure management
   - ✅ Provider interface trait
   - ✅ VyOS API client
   - ✅ Proxmox API client

2. **Data Models**
   - ✅ Instance model
   - ✅ Volume model
   - ✅ Network model
   - ✅ Provider model

3. **Configuration Management**
   - ✅ Settings model and storage
   - ✅ Provider configuration
   - ✅ Credential management

4. **Basic Services**
   - ✅ Provider service
   - ✅ Instance service (partial)

5. **CLI Interface**
   - ✅ Basic command structure
   - ✅ VyOS connectivity testing

6. **Terminal UI**
   - ✅ Basic TUI framework
   - ✅ Navigation and layout

7. **Test Environment**
   - ✅ VyOS lab setup scripts
   - ✅ L3VPN and EVPN configuration
   - ✅ WireGuard secure management

### Work in Progress

1. **Service Layer**

- 🔄 Volume service implementation
- 🔄 Network service implementation
- 🔄 API integration for resources

2. **CLI Interface**

- 🔄 Complete command implementations
- 🔄 Error handling and user feedback

3. **Terminal UI**

- 🔄 Real-time data updates
- 🔄 Resource management wizards
   - 🔄 Volume service implementation
   - 🔄 Network service implementation
   - 🔄 API integration for resources

2. **CLI Interface**
   - 🔄 Complete command implementations
   - 🔄 Error handling and user feedback

3. **Terminal UI**
   - 🔄 Real-time data updates
   - 🔄 Resource management wizards

### Planned Work

1. **Service Layer**

- 📝 Persistence layer for local state
- 📝 Synchronization with remote state
- 📝 Event system for notifications

2. **Security Features**

- 📝 Token rotation
- 📝 Credential encryption
- 📝 Secure remote execution

3. **Advanced Features**

- 📝 Multi-tenant management
- 📝 Role-based access control
- 📝 Audit logging
- 📝 Resource quotas and limits

4. **Integration**

- 📝 Public cloud integration
- 📝 CI/CD workflows
- 📝 Integration with external tools
   - 📝 Persistence layer for local state
   - 📝 Synchronization with remote state
   - 📝 Event system for notifications

2. **Security Features**
   - 📝 Token rotation
   - 📝 Credential encryption
   - 📝 Secure remote execution

3. **Advanced Features**
   - 📝 Multi-tenant management
   - 📝 Role-based access control
   - 📝 Audit logging
   - 📝 Resource quotas and limits

4. **Integration**
   - 📝 Public cloud integration
   - 📝 CI/CD workflows
   - 📝 Integration with external tools

## Implementation Roadmap

### Phase 1: Base Infrastructure (Current Phase)

- ✅ Create directory structure for core components
- ✅ Implement VyOS and Proxmox provider interfaces
- ✅ Setup test environment with containers
- ✅ Implement SSH connectivity to provider hosts
- ✅ Basic authentication mechanism

### Phase 2: Resource Management

- 🔄 Complete API for VM/instance management
- 📝 Storage (volume) provisioning and attachment
- 📝 Network creation and configuration
- 📝 IP address management

### Phase 3: TUI Enhancement

- 📝 Improve dashboard with real-time status updates
- 📝 Resource creation wizards
- 📝 Detailed views for resources
- 📝 Settings management

### Phase 4: Multi-Tenancy & Security

- 📝 User and organization management
- 📝 Role-based access control
- 📝 Secure credential management
- 📝 Encryption for data in transit

### Phase 5: CI/CD Integration

- 📝 Deployment workflows
- 📝 Integration with external CI/CD systems
- 📝 Scaling and update policies

## Design Decisions

### 1. Language and Framework Selection

- **Rust**: Selected for its performance, safety, and excellent async support
- **Tokio**: Used for async runtime
- **Ratatui**: Chosen for TUI implementation due to its flexibility and performance

### 2. API Design

- **Trait-based API**: Uses traits to define common provider interfaces
- **Async-first**: Designed with async operations in mind to prevent UI blocking
- **Error handling**: Consistent error propagation using `anyhow` for user-friendly messages

### 3. Configuration Storage

- **TOML format**: Selected for human-readability and easy editing
- **User directory storage**: Uses `~/.bbctl` to store user configurations
- **Credential separation**: Stores credentials in a separate file for better security

### 4. Network Architecture

- **L3VPN with EVPN**: Chosen for scalable multi-tenant isolation
- **WireGuard**: Selected for secure management plane due to its simplicity and strong encryption
- **VXLAN**: Used for tenant traffic encapsulation to support network virtualization

## Development Guidelines

### Coding Standards

- **Formatting**: Use `cargo fmt` to format code according to Rust standard style
- **Linting**: Run `cargo clippy` for static analysis
- **Naming**:
- Use snake_case for variables, functions, and modules
- Use PascalCase for structs, enums, and traits
- **Naming**: 
  - Use snake_case for variables, functions, and modules
  - Use PascalCase for structs, enums, and traits
- **Error Handling**: Use `AppResult<T>` for functions that can fail
- **Imports**: Group imports by crate, with std first, then external, then internal
- **Document**: Use three slashes (`///`) for public API documentation
- **Async**: Use tokio runtime with futures for async operations

### Testing Strategy

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **System Tests**: Test against the VyOS lab environment
4. **Manual Testing**: Interactive testing of the TUI

## Conclusion

The bbctl project is a comprehensive tool for managing multi-tenant infrastructure on bare metal servers running VyOS or Proxmox. The architecture emphasizes modularity, type safety, and user experience while providing strong security and isolation features.

Phase 1 of the implementation has been completed, establishing the core infrastructure, API clients, data models, and test environment. Ongoing work focuses on completing the service layer implementations and enhancing the CLI and TUI interfaces.

The project follows a clear roadmap with well-defined phases, targeting a complete infrastructure management solution that supports secure multi-tenancy and seamless operations across different providers.
The project follows a clear roadmap with well-defined phases, targeting a complete infrastructure management solution that supports secure multi-tenancy and seamless operations across different providers.
