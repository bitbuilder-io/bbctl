# VyOS Test Lab for bbctl

This directory contains scripts for setting up a Docker-based VyOS test lab environment to test the bbctl CLI. The lab implements a secure, multi-tenant network with WireGuard, VXLAN, OSPF, and L3VPN technologies.

## Lab Architecture

The lab consists of:

- Two VyOS Provider Edge (PE) routers running in Docker containers
- WireGuard VPN for secure management plane
- L3VPN with BGP EVPN for tenant isolation
- VXLAN for tenant traffic encapsulation
- API endpoints for bbctl to manage infrastructure

## Prerequisites

- Docker installed on the host system
- Sudo access for network configuration
- VyOS Docker images (`vyos/vyos:latest`)

## Quick Start

1. Set up the lab environment:

```bash
./setup-lab.sh
```

2. Test the connection with bbctl:

```bash
bbctl test-vyos --host localhost --port 21022 --username vyos --api-key bbctl-test-api
```

3. Use bbctl to manage the infrastructure:

```bash
# List instances across all providers
bbctl instances list

# Create a new network
bbctl networks create tenant-net --cidr 10.100.0.0/24
```

4. When finished, clean up the lab environment:

```bash
./cleanup-lab.sh
```

## Lab Details

### VyOS Routers

Two VyOS routers are deployed with the following configuration:

- **Router 1 (PE1)**:
  - Management IP: 172.27.0.10
  - SSH Port: 21022
  - API Port: 21443
  - WireGuard IP: 172.27.100.1

- **Router 2 (PE2)**:
  - Management IP: 172.27.0.20
  - SSH Port: 22022
  - API Port: 22443
  - WireGuard IP: 172.27.100.2

### Tenant Networks

Two tenants are configured with isolated network segments:

- **Blue Tenant**:
  - VRF ID: 2000
  - VNI: 2000
  - Networks:
    - PE1: 10.1.1.0/24
    - PE2: 10.1.2.0/24

- **Red Tenant**:
  - VRF ID: 3000
  - VNI: 3000
  - Networks:
    - PE1: 10.2.1.0/24
    - PE2: 10.2.2.0/24

## Directory Structure

- `scripts/` - Contains the individual component scripts
  - `setup-base.sh` - Sets up base network infrastructure
  - `setup-vyos-container.sh` - Deploys a VyOS container
  - `configure-l3vpn-evpn.sh` - Configures L3VPN with EVPN
  - `configure-wireguard.sh` - Sets up WireGuard for management plane
- `config/` - Contains configuration files generated during setup
- `images/` - Directory for image files
- `setup-lab.sh` - Main orchestration script
- `cleanup-lab.sh` - Script to tear down the lab environment

## Testing and Debugging

To troubleshoot issues with the lab environment, you can:

1. Access the VyOS router directly:

```bash
ssh -p 21022 vyos@localhost
```

2. Check container status:

```bash
docker ps
docker logs vyos-test-1
```

3. Verify VyOS configuration:

```bash
ssh -p 21022 vyos@localhost "show configuration"
ssh -p 21022 vyos@localhost "show interfaces"
ssh -p 21022 vyos@localhost "show ip route"
```

4. Test WireGuard connectivity:

```bash
ssh -p 21022 vyos@localhost "ping 172.27.100.2"
```

5. Test L3VPN isolation:

```bash
# These should work (same tenant)
ssh -p 21022 vyos@localhost "ping 10.1.2.1 vrf blue"
ssh -p 22022 vyos@localhost "ping 10.1.1.1 vrf blue"

# These should fail (different tenants)
ssh -p 21022 vyos@localhost "ping 10.2.1.1 vrf blue"
ssh -p 22022 vyos@localhost "ping 10.2.2.1 vrf blue"
```

## Reference Documentation

For more details on the technologies used in this lab, refer to:

- [VyOS L3VPN/EVPN Documentation](https://docs.vyos.io/en/latest/configexamples/autotest/L3VPN_EVPN/L3VPN_EVPN.html)
- [VyOS WireGuard Documentation](https://docs.vyos.io/en/latest/configexamples/autotest/Wireguard/Wireguard.html)
- [VyOS VRF Documentation](https://docs.vyos.io/en/latest/configuration/vrf/index.html)
