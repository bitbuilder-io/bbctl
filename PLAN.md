# BitBuilder Cloud CLI (bbctl) Implementation Plan

## Overview

bbctl is a CLI tool for provisioning and managing multi-tenant infrastructure on bare metal servers running VyOS v1.5 or Proxmox. Similar to fly.io's flyctl, bbctl provides a seamless experience for deploying, scaling, and managing applications across distributed infrastructure.

## Architecture

The architecture consists of multiple components:

1.  **Command-line interface (CLI)** - The user-facing interface with subcommands for resource management
2.  **Terminal User Interface (TUI)** - Interactive dashboard for visualizing and managing resources
3.  **API Client** - For communicating with infrastructure providers (VyOS, Proxmox)
4.  **Configuration** - Local config files for storing settings, credentials, and state
5.  **Resource Controllers** - For managing instances, volumes, networks, etc.

## Implementation Phases

### Phase 1: Base Infrastructure Setup

-   Create directory structure for core components
-   Implement VyOS and Proxmox provider interfaces
-   Setup test environment with containers
-   Implement SSH connectivity to provider hosts
-   Basic authentication mechanism

### Phase 2: Resource Management Implementation

-   Complete API for VM/instance management
-   Storage (volume) provisioning and attachment
-   Network creation and configuration
-   IP address management

### Phase 3: TUI Enhancement

-   Improve dashboard with real-time status updates
-   Resource creation wizards
-   Detailed views for resources
-   Settings management

### Phase 4: Multi-Tenancy & Security

-   User and organization management
-   Role-based access control
-   Secure credential management
-   Encryption for data in transit

### Phase 5: CI/CD Integration

-   Deployment workflows
-   Integration with external CI/CD systems
-   Scaling and update policies

## Phase 1 Implementation Details

### 1. Provider Interfaces

#### VyOS Provider

Create interfaces for managing VyOS routers: - SSH-based configuration management using VyOS operational mode - HTTP API integration for automated provisioning - Configuration templating for standard network setups

#### Proxmox Provider

Create interfaces for managing Proxmox clusters: - REST API integration for VM management - Resource allocation and monitoring - Template management for quick deployments

### 2. Test Environment

-   Create containerized test environments for local development
-   Mock API responses for testing without actual infrastructure
-   Integration tests with real infrastructure in CI environment

### 3. Authentication

-   Implement authentication mechanisms for VyOS and Proxmox
-   Secure credential storage in local configuration
-   Token-based authentication for API calls

### 4. Basic Commands

Initial implementation will focus on: - `bbctl init` - Initialize a new project - `bbctl instances` - List/create/manage VMs - `bbctl volumes` - Manage storage - `bbctl networks` - Configure virtual networks

## Directory Structure

```
bbctl/
├── src/
│   ├── api/
│   │   ├── vyos.rs      # VyOS API client
│   │   └── proxmox.rs   # Proxmox API client
│   ├── commands/        # CLI command handlers
│   ├── models/          # Data models for resources
│   ├── tui/             # Terminal UI components
│   ├── main.rs          # Main entry point
│   └── config.rs        # Configuration management
├── tests/
│   ├── integration/     # Integration tests
│   ├── fixtures/        # Test data
│   └── containers/      # Test containers
├── docs/                # Documentation
└── examples/            # Example configurations
```

## Next Steps

1.  Implement the VyOS API client with basic authentication
2.  Create test containers for local development
3.  Implement the core resource models and commands
4.  Develop mock backends for testing without real infrastructure
5.  Create initial TUI dashboard components
