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

### Testing with VyOS Lab Environment

A VyOS test lab environment is provided for testing bbctl against real infrastructure. The lab uses Docker to create VyOS routers configured with WireGuard, VXLAN, OSPF, and L3VPN to simulate a multi-tenant network environment.

```bash
# Setup the VyOS test lab
cd tests/vyos-lab
./setup-lab.sh

# Test bbctl against the lab environment
bbctl test-vyos --host localhost --port 21022 --username vyos --api-key bbctl-test-api

# Cleanup the lab environment when done
./cleanup-lab.sh
```

For more information about the test lab, see [tests/vyos-lab/README.md](tests/vyos-lab/README.md).

## License

MIT License