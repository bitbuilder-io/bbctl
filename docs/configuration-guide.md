# BitBuilder Cloud CLI Configuration Guide

## Overview

BitBuilder Cloud CLI (bbctl) uses a set of configuration files to store settings, provider information, and credentials. This guide explains how to configure and customize your bbctl environment.

## Configuration Files

bbctl uses the following configuration files, located in the `~/.bbctl/` directory:

| File               | Purpose                                             |
| ------------------ | --------------------------------------------------- |
| `settings.toml`    | Global settings and defaults                        |
| `providers.toml`   | Provider configurations                             |
| `credentials.toml` | Authentication credentials (API keys, tokens, etc.) |

## Global Settings

The `settings.toml` file contains global configuration for bbctl behavior:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Default provider to use when not specified
default_provider = "vyos-router"

# Default region to use when not specified
default_region = "nyc"

# Enable or disable telemetry (default: false)
telemetry_enabled = false

# Enable or disable auto-update (default: true)
auto_update_enabled = true

# Enable or disable terminal colors (default: true)
colors_enabled = true

# Default instance sizing when not specified
default_cpu = 2
default_memory_gb = 4
default_disk_gb = 80

# Logging level (trace, debug, info, warn, error)
log_level = "info"
```

### Modifying Settings

You can modify settings using the config command:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Set default provider
bbctl config set default_provider vyos-router

# Change log level
bbctl config set log_level debug
```

## Provider Configuration

The `providers.toml` file defines infrastructure providers and regions:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Provider configurations
[providers]

[providers.vyos-router]
provider_type = "VyOS"
name = "vyos-router"
host = "192.168.1.1"
params = { network_type = "routed" }

[providers.proxmox-host]
provider_type = "Proxmox"
name = "proxmox-host"
host = "192.168.1.2"
params = { node = "pve" }

# Region configurations
[regions]

[regions.nyc]
id = "nyc"
name = "New York"
provider = "VyOS"
location = "US East"
available = true
limits = { max_instances = 10, max_cpu_per_instance = 8 }

[regions.sfo]
id = "sfo"
name = "San Francisco"
provider = "Proxmox"
location = "US West"
available = true
limits = { max_instances = 5, max_cpu_per_instance = 4 }
```

### Managing Providers

Provider configuration can be managed using CLI commands:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Add a new VyOS provider
bbctl providers add vyos-router2 --type vyos --host 192.168.1.3 --username vyos

# Add a new Proxmox provider
bbctl providers add proxmox-host2 --type proxmox --host 192.168.1.4

# Remove a provider
bbctl providers remove vyos-router2
```

## Credentials Management

The `credentials.toml` file stores authentication information for providers:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[credentials]

[credentials.vyos-router]
username = "vyos"
api_key = "YOUR_API_KEY"
ssh_port = 22
api_port = 443

[credentials.proxmox-host]
use_token_auth = true
token_auth = { token_id = "USER@pam!TOKEN", token_secret = "YOUR_TOKEN_SECRET" }
verify_ssl = false
```

### Security Best Practices

1. Use API tokens instead of passwords when possible
2. Ensure proper file permissions (600) on credentials.toml
3. Consider using environment variables for sensitive credentials

## Network Configuration

Network configuration is stored within the provider settings:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[networks.app-network]
id = "net-01234567"
name = "app-network"
cidr = "192.168.1.0/24"
network_type = "Routed"
provider = "VyOS"
region = "nyc"
gateway = "192.168.1.1"
dns_servers = ["1.1.1.1", "8.8.8.8"]
```

### WireGuard Configuration

For secure encrypted networks using WireGuard:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[networks.secure-net]
id = "net-89abcdef"
name = "secure-net"
cidr = "10.10.0.0/24"
network_type = "VPN"
provider = "VyOS"
region = "nyc"
config = { wireguard_enabled = "true", persistent_keepalive = "25" }
```

## Environment Variables

You can override configuration using environment variables:

| Variable                 | Description                 |
| ------------------------ | --------------------------- |
| `BBCTL_LOG_LEVEL`        | Override log level          |
| `BBCTL_CONFIG_DIR`       | Use custom config directory |
| `BBCTL_DEFAULT_PROVIDER` | Override default provider   |
| `BBCTL_DEFAULT_REGION`   | Override default region     |

Example:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
export BBCTL_LOG_LEVEL=debug
export BBCTL_DEFAULT_PROVIDER=vyos-router
bbctl instances list  # Will use debug logging and vyos-router as default
```

## Advanced Configuration

### Multi-tenant Resource Limits

Configure resource limits by tenant:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[tenants.eng-team]
max_instances = 20
max_volumes = 40
max_networks = 5
max_cpu_total = 64
max_memory_total_gb = 256
regions = ["nyc", "sfo"]
```

### Custom Resource Templates

Define templates for quick provisioning:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[templates.web-server]
cpu = 2
memory_gb = 4
disk_gb = 80
image = "debian-11"
networks = ["app-network"]

[templates.db-server]
cpu = 4
memory_gb = 16
disk_gb = 200
volumes = [
  { name = "data", size_gb = 100, type = "ssd" },
  { name = "backup", size_gb = 200, type = "hdd" },
]
```

Usage:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
bbctl instances create web1 --template web-server
```

### API Configuration

Configure the API server component:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[api]
enabled = true
listen = "127.0.0.1"
port = 8080
auth_token = "YOUR_API_TOKEN"
cors_origins = ["http://localhost:3000"]
```

### SSH Key Management

Configure SSH keys for instance access:

<<<<<<< Updated upstream
```toml
=======
<<<<<<< HEAD
``` toml
=======
```toml
>>>>>>> chore/bisect
>>>>>>> Stashed changes
[ssh]
default_key = "~/.ssh/id_ed25519"
additional_keys = ["~/.ssh/id_rsa", "~/.ssh/custom_key"]
```

## Troubleshooting

### Common Configuration Issues

1. **Connection Problems**: Check host, port, and credentials
2. **Permission Errors**: Verify API key permissions and SSH key access
3. **File Format Errors**: Validate TOML syntax in configuration files

### Debugging Configuration

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Show current configuration
bbctl config show

# Show redacted credentials
bbctl config show --section credentials --redact

# Validate configuration
bbctl config validate
```

### Configuration Reset

If you need to reset your configuration:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
# Reset specific section
bbctl config reset --section credentials

# Reset all configuration
bbctl config reset --all
```

## Best Practices

1. **Organize by Environment**: Use naming conventions like `prod-`, `staging-` prefixes
2. **Document Custom Settings**: Add comments to configuration files
3. **Version Control**: Consider storing non-sensitive configuration in version control
4. **Regular Backups**: Back up your configuration directory regularly
5. **Security**: Never expose credentials in scripts or version control

## Further Reading

- [User Guide] - Comprehensive usage instructions
- [Command Reference] - Detailed command documentation
- [API Documentation] - API schema and integration details

[User Guide]: user-guide.md
[Command Reference]: command-reference.md
[API Documentation]: api-readme.md
