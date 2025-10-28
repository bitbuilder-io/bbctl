# BitBuilder Cloud CLI Command Reference

This document provides a comprehensive reference for all commands available in the BitBuilder Cloud CLI (bbctl).

## Global Options

The following options can be used with any command:

| Option                | Description                              |
|-----------------------|------------------------------------------|
| `--help`, `-h`        | Show help information                    |
| `--version`, `-V`     | Show version information                 |
| `--log-level=<level>` | Set log level (debug, info, warn, error) |
| `--json`              | Output results in JSON format            |
| `--quiet`, `-q`       | Suppress output except errors            |

## Core Commands

### init

Initialize a new BitBuilder Cloud project.

**Usage:**

```
bbctl init [OPTIONS]
```

**Options:** - `--name=<name>` - Name for the new project

**Example:**

```
bbctl init --name my-cloud-project
```

### deploy

Deploy an application to BitBuilder Cloud.

**Usage:**

```
bbctl deploy [OPTIONS]
```

**Options:** - `--config=<path>` - Path to deployment configuration file

**Example:**

```
bbctl deploy --config ./deploy.toml
```

## Provider Management

### providers list

List all configured infrastructure providers.

**Usage:**

```
bbctl providers list [OPTIONS]
```

**Example:**

```
bbctl providers list
```

### providers add

Add a new infrastructure provider.

**Usage:**

```
bbctl providers add <name> [OPTIONS]
```

**Options:** - `--type=<type>` - Provider type (vyos, proxmox) \[required\] - `--host=<host>` - Hostname or IP address \[required\] - `--username=<username>` - Username for authentication - `--api-key=<key>` - API key for VyOS providers - `--token-id=<id>` - Token ID for Proxmox providers - `--token-secret=<secret>` - Token secret for Proxmox providers - `--password=<password>` - Password for Proxmox providers - `--realm=<realm>` - Authentication realm for Proxmox providers - `--port=<port>` - Port number for connection - `--verify-ssl` - Verify SSL certificates (Proxmox)

**Examples:**

```
bbctl providers add vyos-router --type vyos --host 192.168.1.1 --username vyos --api-key abcdef123456
bbctl providers add proxmox-host --type proxmox --host 192.168.1.2 --token-id user@pam!token --token-secret abcdef123456
```

### providers remove

Remove a provider from configuration.

**Usage:**

```
bbctl providers remove <name>
```

**Example:**

```
bbctl providers remove vyos-router
```

### providers test

Test connectivity to a provider.

**Usage:**

```
bbctl providers test <name> [OPTIONS]
```

**Options:** - `--verbose`, `-v` - Show detailed connection information

**Example:**

```
bbctl providers test vyos-router --verbose
```

### providers update

Update provider configuration.

**Usage:**

```
bbctl providers update <name> [OPTIONS]
```

**Options:** Same as `providers add` command

**Example:**

```
bbctl providers update vyos-router --api-key new-api-key
```

## Instance Management

### instances list

List all instances (virtual machines).

**Usage:**

```
bbctl instances list [OPTIONS]
```

**Options:** - `--provider=<provider>` - Filter by provider - `--region=<region>` - Filter by region - `--status=<status>` - Filter by status (running, stopped, etc.)

**Example:**

```
bbctl instances list --provider vyos-router --status running
```

### instances create

Create a new instance.

**Usage:**

```
bbctl instances create <name> [OPTIONS]
```

**Options:** - `--provider=<provider>` - Provider to use \[required\] - `--region=<region>` - Region to deploy in \[required\] - `--cpu=<cores>` - Number of CPU cores - `--memory=<gb>` - Memory in GB - `--disk=<gb>` - Disk size in GB - `--network=<id>` - Network ID to connect to - `--image=<image>` - OS image to use - `--ssh-key=<path>` - Path to SSH public key to add

**Example:**

```
bbctl instances create web-server --provider vyos-router --region nyc --cpu 2 --memory 4 --disk 80
```

### instances delete

Delete an instance.

**Usage:**

```
bbctl instances delete <id>
```

**Example:**

```
bbctl instances delete i-01234567
```

### instances start

Start an instance.

**Usage:**

```
bbctl instances start <id>
```

**Example:**

```
bbctl instances start i-01234567
```

### instances stop

Stop an instance.

**Usage:**

```
bbctl instances stop <id>
```

**Example:**

```
bbctl instances stop i-01234567
```

### instances show

Show details about an instance.

**Usage:**

```
bbctl instances show <id>
```

**Example:**

```
bbctl instances show i-01234567
```

## Volume Management

### volumes list

List all volumes.

**Usage:**

```
bbctl volumes list [OPTIONS]
```

**Options:** - `--provider=<provider>` - Filter by provider - `--region=<region>` - Filter by region - `--status=<status>` - Filter by status

**Example:**

```
bbctl volumes list --region nyc
```

### volumes create

Create a new storage volume.

**Usage:**

```
bbctl volumes create <name> [OPTIONS]
```

**Options:** - `--size=<gb>` - Volume size in GB \[required\] - `--region=<region>` - Region to create in - `--type=<type>` - Volume type (standard, ssd, nvme, hdd) - `--provider=<provider>` - Provider to use

**Example:**

```
bbctl volumes create db-data --size 100 --region nyc --type ssd
```

### volumes delete

Delete a volume.

**Usage:**

```
bbctl volumes delete <id>
```

**Example:**

```
bbctl volumes delete vol-01234567
```

### volumes attach

Attach a volume to an instance.

**Usage:**

```
bbctl volumes attach <id> [OPTIONS]
```

**Options:** - `--instance=<id>` - Instance ID to attach to \[required\] - `--device=<device>` - Device name for the attachment

**Example:**

```
bbctl volumes attach vol-01234567 --instance i-01234567 --device /dev/sdb
```

### volumes detach

Detach a volume from an instance.

**Usage:**

```
bbctl volumes detach <id>
```

**Example:**

```
bbctl volumes detach vol-01234567
```

### volumes show

Show details about a volume.

**Usage:**

```
bbctl volumes show <id>
```

**Example:**

```
bbctl volumes show vol-01234567
```

## Network Management

### networks list

List all networks.

**Usage:**

```
bbctl networks list [OPTIONS]
```

**Options:** - `--provider=<provider>` - Filter by provider - `--region=<region>` - Filter by region

**Example:**

```
bbctl networks list --provider proxmox-host
```

### networks create

Create a new network.

**Usage:**

```
bbctl networks create <name> [OPTIONS]
```

**Options:** - `--cidr=<cidr>` - CIDR block (e.g. 192.168.1.0/24) \[required\] - `--type=<type>` - Network type (bridged, routed, isolated, vxlan, vpn) - `--provider=<provider>` - Provider to use - `--region=<region>` - Region to create in - `--gateway=<ip>` - Gateway IP address - `--dns=<ip>` - DNS server IP address (can be specified multiple times) - `--wireguard` - Enable WireGuard encryption

**Example:**

```
bbctl networks create app-network --cidr 192.168.1.0/24 --type routed --gateway 192.168.1.1 --dns 1.1.1.1
```

### networks delete

Delete a network.

**Usage:**

```
bbctl networks delete <id>
```

**Example:**

```
bbctl networks delete net-01234567
```

### networks connect

Connect an instance to a network.

**Usage:**

```
bbctl networks connect <id> [OPTIONS]
```

**Options:** - `--instance=<id>` - Instance ID to connect \[required\] - `--ip=<ip>` - IP address to assign to the instance

**Example:**

```
bbctl networks connect net-01234567 --instance i-01234567 --ip 192.168.1.10
```

### networks disconnect

Disconnect an instance from a network.

**Usage:**

```
bbctl networks disconnect <id> [OPTIONS]
```

**Options:** - `--instance=<id>` - Instance ID to disconnect \[required\]

**Example:**

```
bbctl networks disconnect net-01234567 --instance i-01234567
```

### networks show

Show details about a network.

**Usage:**

```
bbctl networks show <id>
```

**Example:**

```
bbctl networks show net-01234567
```

## Configuration Management

### config show

Show current configuration.

**Usage:**

```
bbctl config show [OPTIONS]
```

**Options:** - `--section=<section>` - Show only specified section - `--redact` - Redact sensitive information

**Example:**

```
bbctl config show --section providers --redact
```

### config reset

Reset configuration to defaults.

**Usage:**

```
bbctl config reset [OPTIONS]
```

**Options:** - `--section=<section>` - Reset only specified section - `--confirm` - Skip confirmation prompt

**Example:**

```
bbctl config reset --section credentials --confirm
```

### config set

Set a configuration value.

**Usage:**

```
bbctl config set <key> <value>
```

**Example:**

```
bbctl config set default_provider vyos-router
```

## Testing and Development

### test-vyos

Test connectivity to a VyOS router.

**Usage:**

```
bbctl test-vyos [OPTIONS]
```

**Options:** - `--host=<host>` - VyOS host to connect to - `--port=<port>` - SSH port (default: 22) - `--username=<username>` - Username (default: vyos) - `--password=<password>` - Password (optional) - `--key-path=<path>` - Path to SSH key (optional) - `--api-key=<key>` - API key for HTTP API (optional)

**Example:**

```
bbctl test-vyos --host 192.168.1.1 --port 22 --username vyos --api-key abcdef123456
```

## Terminal UI (TUI) Mode

Running `bbctl` without any commands will launch the interactive Terminal UI mode.

**Usage:**

```
bbctl
```

### TUI Key Bindings

| Key        | Action               |
|------------|----------------------|
| 1-5        | Switch tabs          |
| Tab        | Next tab             |
| Shift+Tab  | Previous tab         |
| j/k or ↑/↓ | Navigate items       |
| a          | Add new item         |
| d          | Delete selected item |
| e          | Edit selected item   |
| r          | Refresh data         |
| q or ESC   | Quit                 |
| ?          | Show help            |

## Environment Variables

The following environment variables can be used to override configuration:

| Variable                 | Description                          |
|--------------------------|--------------------------------------|
| `BBCTL_LOG_LEVEL`        | Log level (debug, info, warn, error) |
| `BBCTL_CONFIG_DIR`       | Custom configuration directory       |
| `BBCTL_DEFAULT_PROVIDER` | Default provider                     |
| `BBCTL_DEFAULT_REGION`   | Default region                       |
