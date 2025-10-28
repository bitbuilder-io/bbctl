# VyOS Test Lab Setup for bbctl

This document outlines the setup for a dynamically provisioned systemd-vmspawn multi-tenant test environment for bbctl that uses VyOS, WireGuard, VXLAN, OSPF, and L3VPN technologies.

## Lab Architecture

The lab will consist of:

1. **Management Plane**: Secure WireGuard overlay network for router management
2. **Service Provider Network**: OSPF-based core network with BGP EVPN for tenant isolation
3. **Tenant Networks**: L3VPN with VXLAN encapsulation for tenant traffic
4. **Integration Points**: API endpoints for bbctl to manage and automate infrastructure

```
┌────────────────────────────────────────────────────────────────────┐
│                          Host System                               │
│                                                                    │
│  ┌─────────────────────┐  ┌─────────────────────┐                  │
│  │   systemd-vmspawn   │  │   systemd-vmspawn   │                  │
│  │                     │  │                     │                  │
│  │  ┌───────────────┐  │  │  ┌───────────────┐  │                  │
│  │  │  VyOS Router  │  │  │  │  VyOS Router  │  │  ...             │
│  │  │  (PE1)        │◄─┼──┼──►  (PE2)        │  │                  │
│  │  └──────┬────────┘  │  │  └──────┬────────┘  │                  │
│  │         │           │  │         │           │                  │
│  │  ┌──────┴────────┐  │  │  ┌──────┴────────┐  │                  │
│  │  │  Tenant VMs   │  │  │  │  Tenant VMs   │  │                  │
│  │  └───────────────┘  │  │  └───────────────┘  │                  │
│  └─────────────────────┘  └─────────────────────┘                  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │                     Management Bridge                      │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │                       Data Bridge                          │    │
│  └────────────────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────────────┘
```

## Implementation Components

### 1. Base Infrastructure

- **Host Setup**:
  - Arch Linux (as specified in your vyos-network-plan.md)
  - systemd-vmspawn for container deployment
  - Linux bridge setup for network connectivity

- **Network Configuration**:
  - Management network (172.27.0.0/16)
  - Backbone network (172.16.0.0/16)
  - Public IP space simulation (5.254.54.0/26)
  - Tenant space (100.64.0.0/16)

### 2. VyOS Images

We'll create two types of VyOS images:

1. **Base VyOS Image**: Minimal image with core functionality
2. **Provider Edge Router Image**: Pre-configured with L3VPN, EVPN, and WireGuard

### 3. Test Environment Provisioning Scripts

#### Base System Setup Script

```bash
#!/bin/bash
# Setup script for VyOS lab base infrastructure

# Create network bridges
ip link add br-mgmt type bridge
ip link add br-data type bridge
ip link set br-mgmt up
ip link set br-data up

# Assign management IP
ip addr add 172.27.0.1/16 dev br-mgmt

# Setup NAT for outbound connectivity
iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
```

#### VyOS Image Builder Script

```bash
#!/bin/bash
# Build VyOS base image for systemd-vmspawn

# Create VyOS image using mkosi
cat > mkosi.default << EOF
[Distribution]
Distribution=vyos
Release=current

[Output]
Format=disk
Output=vyos-base.img
Size=2G
EOF

# Run mkosi to build the image
mkosi
```

#### Provider Edge Router Deployment Script

```bash
#!/bin/bash
# Deploy a VyOS Provider Edge router using systemd-vmspawn

ROUTER_ID=$1
ROUTER_NAME="vyos-pe${ROUTER_ID}"
ROUTER_MGMT_IP="172.27.0.${ROUTER_ID}0"
ROUTER_BACKBONE_IP="172.16.0.${ROUTER_ID}"

# Create cloud-init configuration
cat > cloud-init.yaml << EOF
#cloud-config
vyos_config_commands:
  # Setup system basics
  - set system host-name ${ROUTER_NAME}
  
  # Setup management interface
  - set interfaces ethernet eth0 address ${ROUTER_MGMT_IP}/16
  - set interfaces ethernet eth0 description 'Management'
  
  # Setup backbone interface
  - set interfaces ethernet eth1 address ${ROUTER_BACKBONE_IP}/16
  - set interfaces ethernet eth1 description 'Backbone'
  
  # Setup OSPF
  - set protocols ospf area 0 network ${ROUTER_BACKBONE_IP}/16
  
  # Enable HTTP API
  - set service https api keys id admin key 'bbctl-test-api'
  - set service https listen-address 0.0.0.0
EOF

# Create systemd-vmspawn service
cat > /etc/systemd/system/${ROUTER_NAME}.service << EOF
[Unit]
Description=VyOS PE${ROUTER_ID} Router
After=network.target

[Service]
Type=notify
ExecStart=/usr/bin/systemd-vmspawn --network-bridge=br-mgmt --network-bridge=br-data -i /var/lib/machines/vyos-base.img --cloud-init=cloud-init.yaml -n ${ROUTER_NAME}
ExecStop=/usr/bin/machinectl poweroff ${ROUTER_NAME}
KillMode=mixed
Restart=on-failure
TimeoutStartSec=300

[Install]
WantedBy=multi-user.target
EOF

# Start the service
systemctl enable --now ${ROUTER_NAME}.service
```

### 4. L3VPN/EVPN Configuration Script

```bash
#!/bin/bash
# Configure L3VPN with EVPN for a VyOS router

ROUTER_ID=$1
ROUTER_NAME="vyos-pe${ROUTER_ID}"
AS_NUMBER=65000
ROUTER_LOOPBACK="172.29.255.${ROUTER_ID}"

# Configure BGP, EVPN, and L3VPN
vyos_config_commands="
# Configure loopback
set interfaces dummy dum0 address ${ROUTER_LOOPBACK}/32

# Configure BGP
set protocols bgp system-as ${AS_NUMBER}
set protocols bgp parameters router-id ${ROUTER_LOOPBACK}

# Configure EVPN
set protocols bgp neighbor 172.29.255.1 remote-as ${AS_NUMBER}
set protocols bgp neighbor 172.29.255.1 update-source dum0
set protocols bgp neighbor 172.29.255.1 address-family l2vpn-evpn activate
set protocols bgp neighbor 172.29.255.2 remote-as ${AS_NUMBER}
set protocols bgp neighbor 172.29.255.2 update-source dum0
set protocols bgp neighbor 172.29.255.2 address-family l2vpn-evpn activate
set protocols bgp l2vpn-evpn advertise-all-vni

# Configure tenant VRFs
set vrf name tenant1 table 2000
set vrf name tenant1 protocols bgp address-family ipv4-unicast route-target vpn export '${AS_NUMBER}:2000'
set vrf name tenant1 protocols bgp address-family ipv4-unicast route-target vpn import '${AS_NUMBER}:2000'

set vrf name tenant2 table 3000
set vrf name tenant2 protocols bgp address-family ipv4-unicast route-target vpn export '${AS_NUMBER}:3000'
set vrf name tenant2 protocols bgp address-family ipv4-unicast route-target vpn import '${AS_NUMBER}:3000'

# Configure VXLAN interfaces
set interfaces vxlan vxlan2000 vni 2000
set interfaces vxlan vxlan2000 source-address ${ROUTER_LOOPBACK}
set interfaces vxlan vxlan2000 mtu 9000
set interfaces vxlan vxlan2000 vrf tenant1

set interfaces vxlan vxlan3000 vni 3000
set interfaces vxlan vxlan3000 source-address ${ROUTER_LOOPBACK}
set interfaces vxlan vxlan3000 mtu 9000
set interfaces vxlan vxlan3000 vrf tenant2
"

# Apply configuration to the router
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper begin
echo "$vyos_config_commands" | while read cmd; do
    if [[ -n "$cmd" && ! "$cmd" =~ ^# ]]; then
        machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper "$cmd"
    fi
done
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper commit
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper save
```

### 5. WireGuard Secure Management Plane

```bash
#!/bin/bash
# Configure WireGuard for secure management plane

ROUTER_ID=$1
ROUTER_NAME="vyos-pe${ROUTER_ID}"
WG_PRIVATE_KEY=$(machinectl shell ${ROUTER_NAME} generate pki wireguard | grep 'Private key:' | cut -d' ' -f3)
WG_PUBLIC_KEY=$(machinectl shell ${ROUTER_NAME} generate pki wireguard show-public | grep 'Public key:' | cut -d' ' -f3)
WG_ADDRESS="172.27.100.${ROUTER_ID}/24"
WG_PORT=$((51820 + ROUTER_ID))

# Configure WireGuard interface
vyos_config_commands="
# WireGuard interface for secure management
set interfaces wireguard wg0 address ${WG_ADDRESS}
set interfaces wireguard wg0 description 'Secure Management Plane'
set interfaces wireguard wg0 port ${WG_PORT}
set interfaces wireguard wg0 private-key ${WG_PRIVATE_KEY}
"

# Add peer configurations based on the router ID
if [ "$ROUTER_ID" -eq "1" ]; then
    # Router 1 peers with 2
    vyos_config_commands+="
set interfaces wireguard wg0 peer PE2 allowed-ips 172.27.100.2/32
set interfaces wireguard wg0 peer PE2 persistent-keepalive 25
# Replace with actual public key from PE2
set interfaces wireguard wg0 peer PE2 public-key REPLACE_WITH_PE2_PUBLIC_KEY
"
elif [ "$ROUTER_ID" -eq "2" ]; then
    # Router 2 peers with 1
    vyos_config_commands+="
set interfaces wireguard wg0 peer PE1 allowed-ips 172.27.100.1/32
set interfaces wireguard wg0 peer PE1 persistent-keepalive 25
# Replace with actual public key from PE1
set interfaces wireguard wg0 peer PE1 public-key REPLACE_WITH_PE1_PUBLIC_KEY
"
fi

# Apply configuration to the router
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper begin
echo "$vyos_config_commands" | while read cmd; do
    if [[ -n "$cmd" && ! "$cmd" =~ ^# ]]; then
        machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper "$cmd"
    fi
done
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper commit
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper save

# Output the public key for use in other routers
echo "WireGuard public key for ${ROUTER_NAME}: ${WG_PUBLIC_KEY}"
```

### 6. Tenant VM Deployment

```bash
#!/bin/bash
# Deploy a tenant VM

TENANT_ID=$1
VM_ID=$2
ROUTER_ID=$3
ROUTER_NAME="vyos-pe${ROUTER_ID}"
VM_NAME="tenant${TENANT_ID}-vm${VM_ID}"
VRF_ID=$((1000 + TENANT_ID * 1000))
VM_IP="10.${TENANT_ID}.${ROUTER_ID}.${VM_ID}"

# Create a simple VM image
qemu-img create -f qcow2 ${VM_NAME}.qcow2 5G

# Create systemd-vmspawn service for the VM
cat > /etc/systemd/system/${VM_NAME}.service << EOF
[Unit]
Description=Tenant ${TENANT_ID} VM ${VM_ID}
After=${ROUTER_NAME}.service

[Service]
Type=notify
ExecStart=/usr/bin/systemd-vmspawn --network-zone=${ROUTER_NAME} -i ${VM_NAME}.qcow2 -n ${VM_NAME}
ExecStop=/usr/bin/machinectl poweroff ${VM_NAME}
KillMode=mixed
Restart=on-failure
TimeoutStartSec=300

[Install]
WantedBy=multi-user.target
EOF

# Start the VM
systemctl enable --now ${VM_NAME}.service

# Configure tenant network on the router
vyos_config_commands="
# Add interface for tenant VM
set interfaces ethernet eth${VM_ID + 1} vrf tenant${TENANT_ID}
set interfaces ethernet eth${VM_ID + 1} address ${VM_IP}/24
set interfaces ethernet eth${VM_ID + 1} description 'Tenant ${TENANT_ID} VM ${VM_ID}'
"

# Apply configuration to the router
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper begin
echo "$vyos_config_commands" | while read cmd; do
    if [[ -n "$cmd" && ! "$cmd" =~ ^# ]]; then
        machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper "$cmd"
    fi
done
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper commit
machinectl shell ${ROUTER_NAME} /opt/vyatta/bin/vyatta-cfg-cmd-wrapper save
```

## Orchestration Script

Let's create a master orchestration script to deploy the entire testbed:

```bash
#!/bin/bash
# Master orchestration script for VyOS lab deployment

set -e

# Setup base infrastructure
echo "Setting up base infrastructure..."
./setup-base.sh

# Build VyOS image
echo "Building VyOS base image..."
./build-vyos-image.sh

# Deploy Provider Edge routers
echo "Deploying Provider Edge routers..."
./deploy-pe-router.sh 1
./deploy-pe-router.sh 2

# Store WireGuard public keys
PE1_PUBKEY=$(./setup-wireguard.sh 1 | grep "public key" | awk '{print $6}')
PE2_PUBKEY=$(./setup-wireguard.sh 2 | grep "public key" | awk '{print $6}')

# Update WireGuard peers with correct public keys
sed -i "s/REPLACE_WITH_PE2_PUBLIC_KEY/$PE2_PUBKEY/" wireguard-config.sh
sed -i "s/REPLACE_WITH_PE1_PUBLIC_KEY/$PE1_PUBKEY/" wireguard-config.sh

# Finalize WireGuard setup
./wireguard-config.sh 1
./wireguard-config.sh 2

# Configure L3VPN/EVPN
echo "Configuring L3VPN/EVPN..."
./setup-l3vpn-evpn.sh 1
./setup-l3vpn-evpn.sh 2

# Deploy tenant VMs
echo "Deploying tenant VMs..."
./deploy-tenant-vm.sh 1 1 1  # Tenant 1, VM 1, on Router 1
./deploy-tenant-vm.sh 1 2 2  # Tenant 1, VM 2, on Router 2
./deploy-tenant-vm.sh 2 1 1  # Tenant 2, VM 1, on Router 1
./deploy-tenant-vm.sh 2 2 2  # Tenant 2, VM 2, on Router 2

echo "Lab deployment complete!"
echo "Management IPs:"
echo "  PE1: 172.27.0.10"
echo "  PE2: 172.27.0.20"
echo "WireGuard Management IPs:"
echo "  PE1: 172.27.100.1"
echo "  PE2: 172.27.100.2"
echo "API access:"
echo "  PE1: https://172.27.0.10/api/ (key: bbctl-test-api)"
echo "  PE2: https://172.27.0.20/api/ (key: bbctl-test-api)"
```

## Integration with bbctl

Now, let's set up the bbctl CLI to work with our lab environment. We'll create integration scripts and update the CLI with appropriate command-line options.

### 1. bbctl Test Configuration

Create a configuration file for bbctl to access the test environment:

```toml
# bbctl test configuration for VyOS lab

[providers]
[providers.vyos-pe1]
provider_type = "VyOS"
name = "vyos-pe1"
host = "172.27.0.10"
params = { network_type = "l3vpn-evpn" }

[providers.vyos-pe2]
provider_type = "VyOS"
name = "vyos-pe2"
host = "172.27.0.20"
params = { network_type = "l3vpn-evpn" }

[regions]
[regions.region1]
id = "region1"
name = "Region 1"
provider = "VyOS"
location = "Local DC 1"
available = true

[regions.region2]
id = "region2"
name = "Region 2"
provider = "VyOS"
location = "Local DC 2"
available = true

[credentials]
[credentials.vyos-pe1]
username = "vyos"
api_key = "bbctl-test-api"
ssh_port = 22
api_port = 443

[credentials.vyos-pe2]
username = "vyos"
api_key = "bbctl-test-api"
ssh_port = 22
api_port = 443
```

### 2. Testing bbctl Commands

Sample commands to test bbctl with the lab environment:

```bash
# Test connection to VyOS routers
bbctl test-vyos --host 172.27.0.10 --port 22 --username vyos --api-key bbctl-test-api

# List instances
bbctl instances list

# Create a new instance on the lab
bbctl instances create test-vm --provider vyos-pe1 --region region1 --cpu 1 --memory 1 --disk 5

# Create a new network
bbctl networks create tenant-net --cidr 10.100.0.0/24

# Connect instance to network
bbctl networks connect tenant-net --instance $INSTANCE_ID
```

## Testing and Debugging

The following methods can be used to verify and troubleshoot the test environment:

1. **Verify OSPF adjacencies**:
   ```
   show ip ospf neighbor
   ```

2. **Verify BGP EVPN**:
   ```
   show bgp l2vpn evpn
   ```

3. **Verify L3VPN routes**:
   ```
   show ip route vrf all
   ```

4. **Verify WireGuard status**:
   ```
   show interfaces wireguard
   ```

5. **Test connectivity between tenants**:
   ```
   # From tenant1-vm1
   ping 10.1.2.1  # Should work
   ping 10.2.1.1  # Should fail due to VRF isolation
   ```

## Next Steps

1. Add support for Docker container deployment
2. Implement automated testing with the lab
3. Add CI/CD pipeline for continuous testing
4. Extend the lab with additional provider types (Proxmox)
5. Implement high availability scenarios
