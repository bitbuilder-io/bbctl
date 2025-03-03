#!/bin/bash
set -e

# Directory where our lab is located
LAB_DIR=/home/bodnar/vyos-lab

# Create virtual networks
# - net1: 10.0.1.0/24 (router1 <-> router3)
# - net2: 10.0.2.0/24 (router1 <-> router2)
# - net3: 10.0.3.0/24 (router2 <-> router3)
# - net4: 192.168.100.0/24 (router3 -> internet)

echo "Creating virtual networks..."

# Function to create a network namespace, bridge, and veth pairs for a network
create_network() {
    local net_name=$1
    local net_addr=$2
    
    echo "Setting up network: $net_name ($net_addr)"
    
    # Create the network namespace
    sudo ip netns add $net_name 2>/dev/null || true
    
    # Create the bridge in the namespace
    sudo ip netns exec $net_name ip link add name br0 type bridge
    sudo ip netns exec $net_name ip link set br0 up
    sudo ip netns exec $net_name ip addr add $net_addr dev br0
}

# Create networks
create_network "net1" "10.0.1.254/24"
create_network "net2" "10.0.2.254/24"
create_network "net3" "10.0.3.254/24"
create_network "net4" "192.168.100.254/24"

# Give net4 namespace access to the host's internet connection
# This simulates internet access for our lab
HOST_IF=$(ip route | grep default | cut -d ' ' -f 5)

# Create veth pair between host and net4
sudo ip link add veth-host type veth peer name veth-net4
sudo ip link set veth-net4 netns net4
sudo ip link set veth-host up

# Configure the veth interfaces
sudo ip netns exec net4 ip link set veth-net4 up
sudo ip netns exec net4 ip link set veth-net4 master br0

# Enable routing on the host
echo 1 | sudo tee /proc/sys/net/ipv4/ip_forward
sudo iptables -t nat -A POSTROUTING -s 192.168.100.0/24 -o $HOST_IF -j MASQUERADE

# Create a script to connect containers to networks
cat > $LAB_DIR/connect-container.sh << 'EOF'
#!/bin/bash
set -e

if [ $# -lt 3 ]; then
    echo "Usage: $0 <container> <network> <interface_name>"
    exit 1
fi

CONTAINER=$1
NETWORK=$2
INTERFACE=$3

# Create veth pair
ip link add veth-$CONTAINER-$NETWORK type veth peer name $INTERFACE

# Move one end to the container
ip link set $INTERFACE netns $(machinectl show $CONTAINER -p Leader | cut -d= -f2)

# Move the other end to the network namespace and attach to bridge
ip link set veth-$CONTAINER-$NETWORK netns $NETWORK
ip netns exec $NETWORK ip link set veth-$CONTAINER-$NETWORK master br0
ip netns exec $NETWORK ip link set veth-$CONTAINER-$NETWORK up

# Configure the container to set up its interface
nsenter -t $(machinectl show $CONTAINER -p Leader | cut -d= -f2) -n ip link set $INTERFACE up

echo "Connected $CONTAINER to $NETWORK via $INTERFACE"
EOF

chmod +x $LAB_DIR/connect-container.sh
echo "Network setup complete!"