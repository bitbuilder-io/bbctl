# BitBuilder Cloud CLI User Guide

## Introduction

BitBuilder Cloud CLI (bbctl) is a command-line tool for provisioning and managing infrastructure across multiple providers, focusing on bare metal deployments with VyOS and Proxmox. It's designed to offer a seamless experience similar to fly.io's flyctl, but targeted at self-hosted infrastructure.

This guide will help you understand how to use bbctl effectively, covering installation, basic operations, and advanced features.

## Getting Started

### Installation

#### Using Cargo (Recommended)

If you have Rust installed, the simplest way to install bbctl is via Cargo:

```bash
cargo install bbctl
```

#### From Binary Releases

For systems without Rust, download pre-compiled binaries:

1.  Visit the [releases page]
2.  Download the appropriate binary for your platform
3.  Make it executable: `chmod +x bbctl`
4.  Move it to your PATH: `sudo mv bbctl /usr/local/bin/`

[releases page]: https://github.com/bitbuilder-io/bbctl/releases

#### Building from Source

To build the latest version from source:

```bash
git clone https://github.com/bitbuilder-io/bbctl.git
cd bbctl
cargo build --release
```

The compiled binary will be in `target/release/bbctl`.

### First-Time Setup

When running bbctl for the first time, you'll need to set up your provider credentials:

```bash
# Initialize bbctl configuration
bbctl init

# Add a VyOS provider
bbctl providers add vyos-router --type vyos --host 192.168.1.1 --username vyos --api-key your-api-key

# Add a Proxmox provider
bbctl providers add proxmox-host --type proxmox --host 192.168.1.2 --token-id your-token-id --token-secret your-token-secret
```

## Core Concepts

BitBuilder Cloud CLI organizes resources into the following categories:

-   **Providers**: Infrastructure providers like VyOS routers or Proxmox hosts
-   **Regions**: Logical groupings of infrastructure, typically by location
-   **Instances**: Virtual machines running on the providers
-   **Volumes**: Storage volumes that can be attached to instances
-   **Networks**: Virtual networks for connecting instances

## Working with Providers

### Listing Providers

```bash
bbctl providers list
```

### Testing Provider Connectivity

```bash
bbctl providers test vyos-router
```

### Removing a Provider

```bash
bbctl providers remove vyos-router
```

## Managing Instances

### Creating an Instance

```bash
bbctl instances create web-server-1 \
  --provider vyos-router \
  --region nyc \
  --cpu 2 \
  --memory 4 \
  --disk 80
```

### Listing Instances

```bash
bbctl instances list
```

### Starting and Stopping Instances

```bash
# Start an instance
bbctl instances start i-01234567

# Stop an instance
bbctl instances stop i-01234567
```

### Getting Instance Details

```bash
bbctl instances show i-01234567
```

### Deleting an Instance

```bash
bbctl instances delete i-01234567
```

## Working with Volumes

### Creating a Volume

```bash
bbctl volumes create db-data \
  --size 100 \
  --region nyc
```

### Listing Volumes

```bash
bbctl volumes list
```

### Attaching a Volume to an Instance

```bash
bbctl volumes attach vol-01234567 \
  --instance i-01234567
```

### Detaching a Volume

```bash
bbctl volumes detach vol-01234567
```

## Managing Networks

### Creating a Network

```bash
bbctl networks create app-network \
  --cidr 192.168.1.0/24
```

### Listing Networks

```bash
bbctl networks list
```

### Connecting an Instance to a Network

```bash
bbctl networks connect net-01234567 \
  --instance i-01234567
```

### Disconnecting an Instance

```bash
bbctl networks disconnect net-01234567 \
  --instance i-01234567
```

## Using the Terminal UI (TUI)

BitBuilder Cloud CLI includes an interactive terminal interface that can be launched by running `bbctl` without any commands.

### Navigating the TUI

-   Use Tab or number keys (1-5) to switch between views
-   Use arrow keys or j/k to select items in lists
-   Press Enter to view or interact with a selected item
-   Press ? to view help

### TUI Views

1.  **Home**: Dashboard with summary information
2.  **Instances**: List and manage virtual machines
3.  **Volumes**: Manage storage volumes
4.  **Networks**: Configure virtual networks
5.  **Settings**: Configure bbctl options

### TUI Key Bindings

| Key       | Action                     |
|-----------|----------------------------|
| 1-5       | Switch to numbered view    |
| Tab       | Next view                  |
| Shift+Tab | Previous view              |
| j or ↓    | Move selection down        |
| k or ↑    | Move selection up          |
| Enter     | View or interact with item |
| a         | Add new item               |
| d         | Delete selected item       |
| e         | Edit selected item         |
| r         | Refresh data               |
| q or Esc  | Quit or go back            |
| ?         | Show help                  |

## Configuration Files

BitBuilder Cloud CLI uses the following configuration files in `~/.bbctl/`:

-   `settings.toml`: Global settings for bbctl
-   `providers.toml`: Provider configurations
-   `credentials.toml`: Authentication credentials (API keys, tokens, etc.)

### Example Settings File

```toml
default_provider = "vyos-router"
default_region = "nyc"
telemetry_enabled = false
auto_update_enabled = true
colors_enabled = true
default_cpu = 2
default_memory_gb = 4
default_disk_gb = 80
log_level = "info"
```

## Advanced Usage

### Using Environment Variables

You can use environment variables to override configuration values:

```bash
export BBCTL_DEFAULT_PROVIDER=vyos-router
export BBCTL_LOG_LEVEL=debug
```

### Scripting and Automation

For scripting, you can use the `--json` flag with most commands to get machine-readable output:

```bash
bbctl instances list --json > instances.json
```

### WireGuard Secure Networking

BitBuilder Cloud CLI supports setting up WireGuard for secure connectivity:

```bash
bbctl networks create secure-net \
  --cidr 10.10.0.0/24 \
  --wireguard enabled
```

## Troubleshooting

### Common Issues

#### Connection Problems

If you're having trouble connecting to a provider:

```bash
# Test provider connectivity with verbose output
bbctl providers test vyos-router --verbose

# Ensure credentials are correct
bbctl providers update vyos-router --api-key new-api-key
```

#### Command Failures

For detailed error information, increase the log level:

```bash
bbctl --log-level debug instances list
```

#### Configuration Issues

If you suspect configuration problems:

```bash
# View current configuration
bbctl config show

# Reset configuration
bbctl config reset
```

### Getting Help

For additional help with specific commands:

```bash
bbctl help
bbctl instances --help
```

## Conclusion

BitBuilder Cloud CLI provides a powerful, unified interface for managing multi-tenant infrastructure across VyOS and Proxmox providers. By combining the command-line interface with the interactive TUI, you can efficiently manage your infrastructure whether you're working interactively or scripting automated workflows.

For more detailed information, refer to the other documentation:

-   [Architecture Design]
-   [VyOS Test Lab Setup]
-   [API Reference]

[Architecture Design]: ARCHITECTURE_DESIGN.md
[VyOS Test Lab Setup]: vyos-test-lab-setup.md
[API Reference]: api-readme.md

## Additional Resources

-   [GitHub Repository]
-   [Issue Tracker]

[GitHub Repository]: https://github.com/bitbuilder-io/bbctl
[Issue Tracker]: https://github.com/bitbuilder-io/bbctl/issues
