#!/bin/bash
# Configure L3VPN with EVPN for a VyOS router

set -e

ROUTER_ID=$1
AS_NUMBER=65000
ROUTER_LOOPBACK="172.29.255.${ROUTER_ID}"
CONTAINER_NAME="vyos-test-${ROUTER_ID}"

# Validate router ID
if [ -z "$ROUTER_ID" ]; then
    echo "Error: Router ID is required"
    echo "Usage: $0 <router_id>"
    exit 1
fi

echo "Configuring L3VPN/EVPN for VyOS router ${CONTAINER_NAME}..."

# Check if container is running
if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Error: Container ${CONTAINER_NAME} is not running"
    exit 1
fi

# Create configuration commands
CONFIG_COMMANDS=$(cat <<EOF
# Configure loopback (router ID)
set interfaces dummy dum0 address ${ROUTER_LOOPBACK}/32

# Configure BGP system
set protocols bgp system-as ${AS_NUMBER}
set protocols bgp parameters router-id ${ROUTER_LOOPBACK}

# Configure EVPN BGP peers for all PE routers
set protocols bgp neighbor 172.29.255.1 remote-as ${AS_NUMBER}
set protocols bgp neighbor 172.29.255.1 update-source dum0
set protocols bgp neighbor 172.29.255.1 address-family l2vpn-evpn activate

set protocols bgp neighbor 172.29.255.2 remote-as ${AS_NUMBER}
set protocols bgp neighbor 172.29.255.2 update-source dum0
set protocols bgp neighbor 172.29.255.2 address-family l2vpn-evpn activate

# Enable EVPN
set protocols bgp address-family l2vpn-evpn advertise-all-vni

# Configure tenant VRFs with route targets
# Tenant 1 (Blue)
set vrf name blue table 2000
set vrf name blue protocols bgp address-family ipv4-unicast route-target vpn export '${AS_NUMBER}:2000'
set vrf name blue protocols bgp address-family ipv4-unicast route-target vpn import '${AS_NUMBER}:2000'

# Tenant 2 (Red)
set vrf name red table 3000
set vrf name red protocols bgp address-family ipv4-unicast route-target vpn export '${AS_NUMBER}:3000'
set vrf name red protocols bgp address-family ipv4-unicast route-target vpn import '${AS_NUMBER}:3000'

# Configure VXLAN interfaces for each tenant
# VXLAN for Tenant 1 (Blue)
set interfaces vxlan vxlan2000 vni 2000
set interfaces vxlan vxlan2000 source-address ${ROUTER_LOOPBACK}
set interfaces vxlan vxlan2000 mtu 9000
set interfaces vxlan vxlan2000 vrf blue

# VXLAN for Tenant 2 (Red)
set interfaces vxlan vxlan3000 vni 3000
set interfaces vxlan vxlan3000 source-address ${ROUTER_LOOPBACK}
set interfaces vxlan vxlan3000 mtu 9000
set interfaces vxlan vxlan3000 vrf red

# Configure tenant networks
# Tenant 1 (Blue) networks
set interfaces bridge br2000 address 10.1.${ROUTER_ID}.1/24
set interfaces bridge br2000 description 'Blue Tenant Bridge'
set interfaces bridge br2000 member interface vxlan2000
set interfaces bridge br2000 vrf blue

# Tenant 2 (Red) networks
set interfaces bridge br3000 address 10.2.${ROUTER_ID}.1/24
set interfaces bridge br3000 description 'Red Tenant Bridge'
set interfaces bridge br3000 member interface vxlan3000
set interfaces bridge br3000 vrf red
EOF
)

# Apply configuration to the VyOS container
echo "Applying L3VPN/EVPN configuration..."

# Start a configuration session
docker exec -i ${CONTAINER_NAME} /opt/vyatta/sbin/vyatta-cfg-cmd-wrapper begin

# Apply the configuration commands line by line
echo "$CONFIG_COMMANDS" | while read -r line; do
    # Skip empty lines and comments
    if [[ -n "$line" ]] && [[ ! "$line" =~ ^# ]]; then
        echo "Applying: $line"
        docker exec -i ${CONTAINER_NAME} /opt/vyatta/sbin/vyatta-cfg-cmd-wrapper "$line"
    fi
done

# Commit and save the configuration
docker exec -i ${CONTAINER_NAME} /opt/vyatta/sbin/vyatta-cfg-cmd-wrapper commit
docker exec -i ${CONTAINER_NAME} /opt/vyatta/sbin/vyatta-cfg-cmd-wrapper save

echo "L3VPN/EVPN configuration applied successfully!"