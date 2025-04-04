#!/bin/bash
# Main orchestration script for VyOS lab deployment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Setting up VyOS test lab environment for bbctl..."

# Create bbctl test configuration
mkdir -p ~/.bbctl
cat > ~/.bbctl/test-config.toml << EOF
# bbctl test configuration for VyOS lab

[providers]
[providers.vyos-pe1]
provider_type = "VyOS"
name = "vyos-pe1"
host = "localhost"
params = { network_type = "l3vpn-evpn", ssh_port = "21022", api_port = "21443" }

[providers.vyos-pe2]
provider_type = "VyOS"
name = "vyos-pe2"
host = "localhost"
params = { network_type = "l3vpn-evpn", ssh_port = "22022", api_port = "22443" }

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
password = "vyos"
api_key = "bbctl-test-api"
ssh_port = 21022
api_port = 21443

[credentials.vyos-pe2]
username = "vyos"
password = "vyos"
api_key = "bbctl-test-api"
ssh_port = 22022
api_port = 22443
EOF

echo "Created bbctl test configuration at ~/.bbctl/test-config.toml"

# Run the setup scripts
echo "====== Step 1: Setting up base infrastructure ======"
${SCRIPT_DIR}/scripts/setup-base.sh

echo "====== Step 2: Deploying VyOS Router 1 ======"
${SCRIPT_DIR}/scripts/setup-vyos-container.sh 1

echo "====== Step 3: Deploying VyOS Router 2 ======"
${SCRIPT_DIR}/scripts/setup-vyos-container.sh 2

echo "====== Step 4: Configuring L3VPN/EVPN on Router 1 ======"
${SCRIPT_DIR}/scripts/configure-l3vpn-evpn.sh 1

echo "====== Step 5: Configuring L3VPN/EVPN on Router 2 ======"
${SCRIPT_DIR}/scripts/configure-l3vpn-evpn.sh 2

echo "====== Step 6: Configuring WireGuard between routers ======"
echo "Setting up WireGuard on Router 1 to peer with Router 2..."
${SCRIPT_DIR}/scripts/configure-wireguard.sh 1 2

echo "Setting up WireGuard on Router 2 to peer with Router 1..."
${SCRIPT_DIR}/scripts/configure-wireguard.sh 2 1

echo "====== VyOS Test Lab Setup Complete ======"
echo ""
echo "Lab Summary:"
echo "------------"
echo "Router 1 (PE1):"
echo "  - Management: ssh -p 21022 vyos@localhost (password: vyos)"
echo "  - API: https://localhost:21443/api/ (key: bbctl-test-api)"
echo "  - WireGuard: 172.27.100.1 (secure management)"
echo ""
echo "Router 2 (PE2):"
echo "  - Management: ssh -p 22022 vyos@localhost (password: vyos)"
echo "  - API: https://localhost:22443/api/ (key: bbctl-test-api)"
echo "  - WireGuard: 172.27.100.2 (secure management)"
echo ""
echo "Tenant Networks:"
echo "  - Blue (Tenant 1):"
echo "    - PE1: 10.1.1.0/24"
echo "    - PE2: 10.1.2.0/24"
echo "  - Red (Tenant 2):"
echo "    - PE1: 10.2.1.0/24"
echo "    - PE2: 10.2.2.0/24"
echo ""
echo "Test bbctl with:"
echo "  bbctl test-vyos --host localhost --port 21022 --username vyos --api-key bbctl-test-api"
echo ""