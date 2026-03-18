# BitBuilder Cloud CLI (bbctl)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

BitBuilder Cloud CLI is an all-in-one tool for provisioning and managing multi-tenant infrastructure on bare metal servers running VyOS v1.5 or Proxmox. Similar to fly.io's flyctl, bbctl provides a seamless experience for deploying, scaling, and managing your applications across distributed infrastructure.

![bbctl Dashboard](https://github.com/user-attachments/assets/329337d7-9afe-4682-a486-484b9de063ef)

## Features

- **Manage VMs**: Create, configure, and manage virtual machines across your infrastructure
- **Storage Management**: Provision and attach volumes to your applications
- **Network Configuration**: Set up and manage virtual networks with secure connectivity
- **Multi-provider Support**: Works with VyOS v1.5 and Proxmox
- **Interactive TUI**: Terminal-based dashboard for visual resource management
- **Bare Metal Efficiency**: Optimized for bare metal server deployment
- **E2E Encryption**: Secure networking with WireGuard integration (coming soon)
- **Future Public Cloud Integration**: Scale out to public clouds with E2E encryption (coming soon)

## Installation

### Using Cargo

```bash
cargo install bbctl
```

### Binary Releases

Download the latest release for your platform from the [releases page](https://github.com/bitbuilder-io/bbctl/releases).

### Building from Source

```bash
git clone https://github.com/bitbuilder-io/bbctl.git
cd bbctl
cargo build --release
```

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

The TUI provides a visual dashboard to manage your infrastructure with keyboard navigation.

### Navigation

| Key | Action |
|-----|--------|
| `1-5` | Switch tabs |
| `Tab` / `Shift+Tab` | Next/Previous tab |
| `j/k` or `↑/↓` | Navigate items |
| `a` | Add new item |
| `d` | Delete selected |
| `e` | Edit selected |
| `r` | Refresh data |
| `q` or `ESC` | Quit |
| `?` | Show help |

<details>
<summary><strong>View More Screenshots</strong></summary>

### Instances View
Manage virtual machines across all providers with real-time status updates.

![Instances](https://github.com/user-attachments/assets/588f465c-ce0b-4e85-9e6c-f887f6eefab7)

### Volumes View
Provision and manage storage volumes with attachment information.

![Volumes](https://github.com/user-attachments/assets/55bd52d1-86ad-4781-8956-d2a862161c8a)

### Networks View
Configure virtual networks and manage instance connectivity.

![Networks](https://github.com/user-attachments/assets/6144c676-b8e0-412e-9097-fe9543d568b5)

### Help View
Quick reference for all keyboard shortcuts.

![Help](https://github.com/user-attachments/assets/1b015499-0b4d-4ada-97b1-d10db2cb086b)

</details>

## CLI Commands

### Instance Management

```bash
# List all instances
bbctl instances list

# Create a new instance
bbctl instances create web-1 --provider vyos --region nyc --cpu 2 --memory 4 --disk 80

# Start/Stop/Delete instances
bbctl instances start i-01234567
bbctl instances stop i-01234567
bbctl instances delete i-01234567
```

### Volume Management

```bash
# List volumes
bbctl volumes list

# Create and attach volumes
bbctl volumes create db-data --size 100 --region nyc
bbctl volumes attach vol-01234567 --instance i-01234567
bbctl volumes detach vol-01234567
```

### Network Management

```bash
# List networks
bbctl networks list

# Create network and connect instances
bbctl networks create app-network --cidr 192.168.1.0/24
bbctl networks connect net-01234567 --instance i-01234567
```

## Architecture

bbctl is designed with a layered architecture:

```
┌────────────────────────────────────────────────────────────────────┐
│                       User Interface Layer                         │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │   CLI Commands      │  │   Terminal UI       │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
├────────────────────────────────────────────────────────────────────┤
│                        Service Layer                               │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │  Provider Services  │  │  Resource Services  │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
├────────────────────────────────────────────────────────────────────┤
│                         API Layer                                  │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │     VyOS API        │  │    Proxmox API      │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
└────────────────────────────────────────────────────────────────────┘
```

### Supported Providers

| Provider | Status | Features |
|----------|--------|----------|
| **VyOS v1.5** | Supported | SSH, HTTP API, WireGuard |
| **Proxmox** | Supported | REST API, VM/Container management |

## Configuration

Configuration files are stored in `~/.bbctl/`:

- `settings.toml` - Global settings
- `providers.toml` - Provider configurations
- `credentials.toml` - Authentication credentials

### Example Settings

```toml
default_provider = "vyos-router"
default_region = "nyc"
telemetry_enabled = false
log_level = "info"
```

## Network Architecture

bbctl supports advanced networking features:

- **L3VPN with EVPN** - BGP-based control plane for multi-tenant isolation
- **VXLAN Overlay** - Scalable tenant traffic encapsulation
- **WireGuard** - Secure encrypted management plane
- **VRF Isolation** - Complete tenant network separation

For detailed network architecture, see [VyOS Network Plan](docs/vyos-network-plan.md).
In TUI mode, you can: - Navigate with Tab or number keys (1-5) - Use arrow keys or j/k to select items - View and manage Instances, Volumes, and Networks - Configure system settings
In TUI mode, you can:
- Navigate with Tab or number keys (1-5)
- Use arrow keys or j/k to select items
- View and manage Instances, Volumes, and Networks
- Configure system settings

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
bbctl/
├── src/
│   ├── api/           # VyOS and Proxmox API clients
│   ├── models/        # Data models (Instance, Volume, Network)
│   ├── services/      # Business logic
│   ├── config/        # Configuration management
│   ├── app.rs         # Application state
│   ├── ui.rs          # TUI components
│   ├── tui.rs         # Terminal setup
│   └── main.rs        # Entry point
├── tests/             # Test suite
├── docs/              # Documentation
└── vyos-lab/          # VyOS test environment
```

### Testing with VyOS Lab

A containerized VyOS test environment is provided for development:

```bash
# Setup the VyOS test lab
cd tests/vyos-lab
./setup-lab.sh

# Test bbctl against the lab
bbctl test-vyos --host localhost --port 21022 --username vyos --api-key bbctl-test-api

# Cleanup
./cleanup-lab.sh
```

See [VyOS Test Lab Setup](docs/vyos-test-lab-setup.md) for detailed instructions.

## Documentation

### User Documentation

- [User Guide](docs/user-guide.md) - Complete guide for using bbctl
- [Command Reference](docs/command-reference.md) - Detailed command documentation
- [Configuration Guide](docs/configuration-guide.md) - Configuration options

### Technical Documentation

- [Architecture Design](docs/ARCHITECTURE_DESIGN.md) - Technical architecture
- [API Documentation](docs/api-readme.md) - API schema and OpenAPI docs
- [Rust Integration](docs/rust-integration.md) - Rust/TypeScript compatibility

### Network Documentation

- [VyOS Network Plan](docs/vyos-network-plan.md) - Network architecture design
- [VyOS Test Lab Setup](docs/vyos-test-lab-setup.md) - Test environment guide

View the [documentation index](docs/index.md) for a complete list.

## Roadmap

- [x] Phase 1: Base infrastructure and API clients
- [ ] Phase 2: Complete resource management
- [ ] Phase 3: Enhanced TUI with real-time updates
- [ ] Phase 4: Multi-tenancy and RBAC
- [ ] Phase 5: CI/CD integration and public cloud support

See [PLAN.md](PLAN.md) for detailed implementation plans.

## Contributing

Contributions are welcome! Please follow the development guidelines in the [Architecture Design](docs/ARCHITECTURE_DESIGN.md) document.

```bash
# Clone the repository
git clone https://github.com/bitbuilder-io/bbctl.git

# Create a feature branch
git checkout -b feature/my-feature

# Make changes and test
cargo test

# Submit a pull request
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

MIT License.

## Related Projects

- [VyOS](https://vyos.io/) - Open source network operating system
- [Proxmox VE](https://www.proxmox.com/) - Virtualization management platform
- [Ratatui](https://github.com/ratatui-org/ratatui) - Rust TUI library
MIT License
MIT License
