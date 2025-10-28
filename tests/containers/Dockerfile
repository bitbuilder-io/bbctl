FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    openssh-server \
    iputils-ping \
    curl \
    iproute2 \
    qemu-kvm \
    && rm -rf /var/lib/apt/lists/*

# Create a test user
RUN useradd -m -s /bin/bash testuser && \
    echo "testuser:password" | chpasswd && \
    mkdir -p /home/testuser/.ssh && \
    chmod 700 /home/testuser/.ssh && \
    chown -R testuser:testuser /home/testuser/.ssh

# Setup SSH
RUN mkdir -p /run/sshd && \
    sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config && \
    echo "root:password" | chpasswd

# Expose SSH port
EXPOSE 22

# Create a simple VM config file
RUN mkdir -p /etc/vyos/config && \
    echo "interfaces { \n  ethernet eth0 { \n    address 192.168.1.1/24 \n  } \n}" > /etc/vyos/config/config.boot

# Create a simple proxmox config
RUN mkdir -p /etc/pve && \
    echo "nodes: \n  node1: \n    ip: 192.168.1.1" > /etc/pve/nodes.conf

# Start SSH daemon
CMD ["/usr/sbin/sshd", "-D"]
