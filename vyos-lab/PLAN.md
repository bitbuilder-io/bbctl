# VyOS Multi-Tenant Lab Architecture Plan

## Overview
This document outlines a comprehensive plan for creating a fully virtualized VyOS lab environment on Linux using systemd-nspawn containers. The lab is designed to simulate multi-tenant isolation with secure overlay networking, similar to what would be deployed in a real cloud environment.

## Lab Architecture Components

### 1. Infrastructure Layer
- **Host System**: Linux with systemd-nspawn containers
- **Container Management**: Systemd-based virtualization
- **Storage**: Layered filesystem for efficient instance deployment
- **Networking**: Network namespaces with veth pairs

### 2. Management Network
- **WireGuard Overlay**:
  - Secure encrypted communication between all VyOS instances
  - Out-of-band management plane
  - Centralized control for administration and monitoring
  - Persistent across network changes and disruptions

### 3. Tenant Isolation Mechanisms
- **L3VPN with EVPN Control Plane**:
  - BGP-EVPN for distributed control
  - VXLAN encapsulation for tenant traffic
  - VRF isolation for complete tenant separation
  - Separate VNI per tenant for strict isolation

### 4. Data Plane Design
- **VXLAN Transport**:
  - Overlay networking with VXLAN encapsulation
  - Scale to multiple tenants without redesigning physical network
  - Support for VM and container workloads
- **Distributed Routing**:
  - BGP-based routing for scale
  - Support for asymmetric routing paths

## Implementation Plan

### Phase 1: Base Infrastructure Setup
1. Create the directory structure for lab components
2. Build base VyOS container image with required packages
3. Configure host network namespaces for simulation
4. Create deployment scripts for container lifecycle management

### Phase 2: Provisioning System
1. Implement cloud-init NoCloud provider for VyOS instances
   - Generate meta-data and user-data files
   - Mount as seed ISO or directly via filesystem
2. Create configuration templates for different node roles
   - Hub/edge routers
   - Tenant gateway routers
   - Management routers
3. Implement first-boot configuration logic

### Phase 3: Management Network Implementation
1. Configure WireGuard overlay network
   - Generate keypairs for all nodes
   - Configure hub-and-spoke topology initially
   - Enable route distribution for reachability
2. Implement PKI infrastructure for security
   - Certificate generation and distribution
   - Secure API access for automation

### Phase 4: Tenant Network Implementation
1. Configure BGP-EVPN infrastructure
   - Set up BGP for control plane
   - Configure route reflectors if needed
   - Implement EVPN address families
2. Configure VXLAN for data plane
   - Create VXLAN interfaces with appropriate VNIs
   - Bridge interfaces to tenant networks
3. Implement VRF isolation
   - Create separate routing tables per tenant
   - Configure forwarding rules and security policies

### Phase 5: Automation and API Integration
1. Enable and secure VyOS HTTP API
   - Configure API access credentials
   - Set up TLS for secure communication
2. Create automation scripts for common tasks
   - Tenant onboarding
   - Network configuration
   - Monitoring and observability

### Phase 6: Testing and Validation
1. Test tenant isolation
   - Verify separation between tenant networks
   - Validate security boundaries
2. Performance testing
   - Measure throughput with iperf/similar tools
   - Evaluate resource usage on host system
3. Failure scenario testing
   - Node failure/recovery
   - Network partition scenarios
   - High availability testing

## Component Details

### Host Network Configuration
- Network namespaces for isolation
- Bridge interfaces for container connectivity
- veth pairs for connecting containers to bridges

### Container Configuration
- Systemd-nspawn containers with appropriate capabilities
- Mount points for persistent storage
- CPU and memory limits for predictable performance

### VyOS Configuration Examples

#### WireGuard Management Network
```
set interfaces wireguard wg0 address '10.255.0.1/24'
set interfaces wireguard wg0 port '51820'
set interfaces wireguard wg0 private-key 'PRIVATEKEY'
set interfaces wireguard wg0 peer NODEID public-key 'PUBLICKEY'
set interfaces wireguard wg0 peer NODEID allowed-ips '10.255.0.2/32'
set interfaces wireguard wg0 peer NODEID endpoint '198.51.100.2:51820'
```

#### BGP-EVPN Configuration
```
set protocols bgp system-as '65000'
set protocols bgp address-family l2vpn-evpn advertise-all-vni
set protocols bgp neighbor 10.255.0.2 remote-as '65000'
set protocols bgp neighbor 10.255.0.2 update-source 'wg0'
set protocols bgp neighbor 10.255.0.2 address-family l2vpn-evpn
```

#### VXLAN and VRF Configuration
```
set interfaces vxlan vxlan100 vni '100'
set interfaces vxlan vxlan100 source-address '10.255.0.1'
set interfaces vxlan vxlan100 port '4789'

set interfaces bridge br100 member interface 'vxlan100'
set interfaces bridge br100 address '10.100.0.1/24'

set vrf name tenant1 table '1000'
set vrf name tenant1 interfaces 'br100'
```

## Required Software and Tools
1. **System Tools**:
   - Linux with systemd (recent version)
   - debootstrap or similar for container creation
   - bridge-utils and iproute2

2. **Virtualization**:
   - systemd-nspawn
   - optional: libvirt/KVM for VM-based nodes

3. **Networking**:
   - WireGuard tools
   - iptables/nftables
   - ethtool and debugging utilities

4. **Automation**:
   - Ansible or similar for orchestration
   - jq for JSON processing
   - curl for API interaction
   
## Security Considerations
- Isolation between tenants using VRFs
- Encryption of management traffic with WireGuard
- API access control and authentication
- Resource limits to prevent DoS scenarios
- Monitoring and logging for security events

## Future Enhancements
1. Integration with external authentication systems
2. Support for dynamic routing protocols within tenant VRFs
3. Integrated monitoring and alerting
4. Backup and restore mechanisms
5. Scaling to larger deployments with multiple hosts

## Conclusion
This lab architecture provides a comprehensive environment for testing multi-tenant network isolation using VyOS and modern networking concepts. It combines the security of WireGuard with the flexibility and scalability of EVPN to create isolated tenant environments that closely mirror cloud deployment scenarios.