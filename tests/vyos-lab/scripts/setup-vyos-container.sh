#!/bin/bash
# Setup VyOS container for testing

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROUTER_ID=$1
IMAGE_DIR="${SCRIPT_DIR}/../images"
CONFIG_DIR="${SCRIPT_DIR}/../config"

# Validate router ID
if [ -z "$ROUTER_ID" ]; then
    echo "Error: Router ID is required"
    echo "Usage: $0 <router_id>"
    exit 1
fi

ROUTER_NAME="vyos-pe${ROUTER_ID}"
ROUTER_MGMT_IP="172.27.0.${ROUTER_ID}0"
ROUTER_BACKBONE_IP="172.16.0.${ROUTER_ID}"
CONTAINER_NAME="vyos-test-${ROUTER_ID}"

echo "Setting up VyOS container ${CONTAINER_NAME}..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed. Please install Docker first."
    exit 1
fi

# Create VyOS config directory if it doesn't exist
mkdir -p "${CONFIG_DIR}/${CONTAINER_NAME}"

# Generate VyOS configuration
cat > "${CONFIG_DIR}/${CONTAINER_NAME}/config.boot" << EOF
interfaces {
    ethernet eth0 {
        address ${ROUTER_MGMT_IP}/16
        description "Management"
    }
    ethernet eth1 {
        address ${ROUTER_BACKBONE_IP}/16
        description "Backbone"
    }
    loopback lo {
    }
}
protocols {
    ospf {
        area 0 {
            network ${ROUTER_BACKBONE_IP}/16
        }
    }
}
service {
    ssh {
        port 22
    }
    https {
        listen-address 0.0.0.0
        listen-port 443
    }
    api {
        keys {
            id bbctl {
                key "bbctl-test-api"
            }
        }
    }
}
system {
    host-name ${ROUTER_NAME}
    login {
        user vyos {
            authentication {
                encrypted-password "\$6\$QxPS.uk6mfo\$9QBSo8u1FkH16gMyAVhus6fU3LOzvLR9Z9.82m3tiHFAxTtIkhaZSWssSgzt4v4dGAL8rhVQxTg0oAG9/q11h/"
                plaintext-password ""
            }
        }
    }
    syslog {
        global {
            facility all {
                level notice
            }
            facility protocols {
                level debug
            }
        }
    }
}
EOF

# Check if container already exists
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Container ${CONTAINER_NAME} already exists."
    
    # Check if it's running
    if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "Container is already running."
    else
        echo "Starting existing container..."
        docker start ${CONTAINER_NAME}
    fi
else
    echo "Creating and starting new VyOS container..."
    
    # Use the VyOS Docker image
    docker run -d \
        --name ${CONTAINER_NAME} \
        --hostname ${ROUTER_NAME} \
        --privileged \
        -v "${CONFIG_DIR}/${CONTAINER_NAME}/config.boot:/opt/vyatta/etc/config/config.boot" \
        --network bridge \
        -p "2${ROUTER_ID}022:22" \
        -p "2${ROUTER_ID}443:443" \
        vyos/vyos:latest
    
    # Sleep to ensure container is fully started
    sleep 5
    
    # Configure additional interfaces
    docker network create backbone-${ROUTER_ID} --subnet=172.16.${ROUTER_ID}.0/24 || echo "Network already exists"
    docker network connect backbone-${ROUTER_ID} ${CONTAINER_NAME}
fi

# Get container IP
CONTAINER_IP=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' ${CONTAINER_NAME})

echo ""
echo "VyOS container ${CONTAINER_NAME} is running!"
echo "--------------------------------"
echo "Container name: ${CONTAINER_NAME}"
echo "Container IP: ${CONTAINER_IP}"
echo "SSH access: ssh -p 2${ROUTER_ID}022 vyos@localhost (password: vyos)"
echo "API access: https://localhost:2${ROUTER_ID}443/api/ (key: bbctl-test-api)"
echo ""