# BitBuilder Cloud CLI (bbctl)

BitBuilder Cloud CLI is an all-in-one tool for provisioning and managing multi-tenant infrastructure on bare metal servers running VyOS v1.5 or Proxmox. Similar to fly.io's flyctl, bbctl provides a seamless experience for deploying, scaling, and managing your applications across distributed infrastructure.

## Features

- **Manage VMs**: Create, configure, and manage virtual machines across your infrastructure
- **Storage Management**: Provision and attach volumes to your applications
- **Network Configuration**: Set up and manage virtual networks with secure connectivity
- **Multi-provider Support**: Works with VyOS v1.5 and Proxmox
- **Bare Metal Efficiency**: Optimized for bare metal server deployment
- **Future Public Cloud Integration**: Scale out to public clouds with E2E encryption (coming soon)

## Installation

### Using Cargo

```bash
cargo install bbctl
```

### Binary Releases

Download the latest release for your platform from the [releases page](https://github.com/bitbuilder-io/bbctl/releases).

## Quick Start

```bash
# Initialize a new BitBuilder Cloud project
bbctl init

# Deploy an application
bbctl deploy

# List running instances
bbctl instances list

# Create a new volume
bbctl volumes create my-volume --size 10

# Manage networks
bbctl networks create my-network --cidr 192.168.0.0/24
```

## TUI Mode

Run `bbctl` without commands to enter the interactive Terminal UI mode:

```bash
bbctl
```

In TUI mode, you can:
- Navigate with Tab or number keys (1-5)
- Use arrow keys or j/k to select items
- View and manage Instances, Volumes, and Networks
- Configure system settings

## Development

This project uses Rust with async support through Tokio and Ratatui for the terminal interface.

```bash
# Clone the repository
git clone https://github.com/bitbuilder-io/bbctl.git
cd bbctl

# Build
cargo build

# Run
cargo run
```

## License

MIT License