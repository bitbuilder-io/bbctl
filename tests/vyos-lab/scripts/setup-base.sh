#!/bin/bash
# Setup script for VyOS lab base infrastructure

set -e

echo "Setting up VyOS lab base infrastructure..."

# Create network bridges
echo "Creating network bridges..."
sudo ip link add br-mgmt type bridge || echo "Management bridge already exists"
sudo ip link add br-data type bridge || echo "Data bridge already exists"
sudo ip link set br-mgmt up
sudo ip link set br-data up

# Assign management IP
echo "Assigning management IP..."
sudo ip addr add 172.27.0.1/16 dev br-mgmt || echo "IP already assigned"

# Setup NAT for outbound connectivity
echo "Setting up NAT..."
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE || echo "NAT rule already exists"

# Enable IP forwarding
echo "Enabling IP forwarding..."
sudo sysctl -w net.ipv4.ip_forward=1

# Create necessary directories
echo "Creating directories..."
mkdir -p ../images
mkdir -p ../config

echo "Base infrastructure setup complete!"