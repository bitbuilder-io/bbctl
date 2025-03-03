#!/bin/bash
# Cleanup script to teardown the VyOS lab environment

set -e

echo "Cleaning up VyOS test lab environment..."

# Stop and remove containers
echo "Stopping and removing VyOS containers..."
docker stop vyos-test-1 vyos-test-2 2>/dev/null || true
docker rm vyos-test-1 vyos-test-2 2>/dev/null || true

# Remove Docker networks
echo "Removing Docker networks..."
docker network rm backbone-1 backbone-2 2>/dev/null || true

# Remove bridge interfaces
echo "Removing bridge interfaces..."
sudo ip link set br-mgmt down 2>/dev/null || true
sudo ip link set br-data down 2>/dev/null || true
sudo ip link del br-mgmt 2>/dev/null || true
sudo ip link del br-data 2>/dev/null || true

# Remove NAT rules
echo "Removing NAT rules..."
sudo iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE 2>/dev/null || true

echo "Cleanup complete!"