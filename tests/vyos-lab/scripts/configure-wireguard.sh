#!/bin/bash
# Configure WireGuard for secure management plane

set -e

ROUTER_ID=$1
PEER_ROUTER_ID=$2
CONTAINER_NAME="vyos-test-${ROUTER_ID}"
PEER_CONTAINER_NAME="vyos-test-${PEER_ROUTER_ID}"
WG_ADDRESS="172.27.100.${ROUTER_ID}/24"
WG_PORT=$((51820 + ROUTER_ID))
PEER_WG_ADDRESS="172.27.100.${PEER_ROUTER_ID}/24"

# Validate arguments
if [ -z "$ROUTER_ID" ] || [ -z "$PEER_ROUTER_ID" ]; then
    echo "Error: Router ID and Peer Router ID are required"
    echo "Usage: $0 <router_id> <peer_router_id>"
    exit 1
fi

echo "Configuring WireGuard for VyOS router ${CONTAINER_NAME} to peer with ${PEER_CONTAINER_NAME}..."

# Check if containers are running
if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Error: Container ${CONTAINER_NAME} is not running"
    exit 1
fi

if ! docker ps --format '{{.Names}}' | grep -q "^${PEER_CONTAINER_NAME}$"; then
    echo "Error: Peer container ${PEER_CONTAINER_NAME} is not running"
    exit 1
fi

# Generate WireGuard keys for router
echo "Generating WireGuard keys for ${CONTAINER_NAME}..."
PRIVATE_KEY=$(docker exec -i ${CONTAINER_NAME} sh -c "vyos-gen-key wireguard" | grep -o '[A-Za-z0-9+/=]\{43\}')
PUBLIC_KEY=$(docker exec -i ${CONTAINER_NAME} sh -c "echo '${PRIVATE_KEY}' | wg pubkey")

# Get peer's public key
echo "Getting peer's public key from ${PEER_CONTAINER_NAME}..."
PEER_PRIVATE_KEY=$(docker exec -i ${PEER_CONTAINER_NAME} sh -c "vyos-gen-key wireguard" | grep -o '[A-Za-z0-9+/=]\{43\}')
PEER_PUBLIC_KEY=$(docker exec -i ${PEER_CONTAINER_NAME} sh -c "echo '${PEER_PRIVATE_KEY}' | wg pubkey")

echo "Router ${ROUTER_ID} public key: ${PUBLIC_KEY}"
echo "Peer Router ${PEER_ROUTER_ID} public key: ${PEER_PUBLIC_KEY}"

# Create WireGuard configuration commands
CONFIG_COMMANDS=$(cat <<EOF
# Configure WireGuard interface
set interfaces wireguard wg0 address ${WG_ADDRESS}
set interfaces wireguard wg0 description 'Secure Management Plane'
set interfaces wireguard wg0 port ${WG_PORT}
set interfaces wireguard wg0 private-key '${PRIVATE_KEY}'

# Configure peer
set interfaces wireguard wg0 peer PE${PEER_ROUTER_ID} allowed-ips '${PEER_WG_ADDRESS}'
set interfaces wireguard wg0 peer PE${PEER_ROUTER_ID} persistent-keepalive '25'
set interfaces wireguard wg0 peer PE${PEER_ROUTER_ID} public-key '${PEER_PUBLIC_KEY}'
EOF
)

# Apply configuration to the VyOS container
echo "Applying WireGuard configuration to ${CONTAINER_NAME}..."

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

echo "WireGuard configuration applied successfully to ${CONTAINER_NAME}!"