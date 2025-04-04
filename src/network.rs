use anyhow::{anyhow, Context, Result};
use boringtun::noise::{Tunn, TunnResult};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
    sync::mpsc,
    time::sleep,
};

const WIREGUARD_PORT: u16 = 51820;
const MAX_PACKET_SIZE: usize = 1500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardPeer {
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardConfig {
    pub private_key: String,
    pub address: String,
    pub port: u16,
    pub peers: Vec<WireguardPeer>,
}

#[derive(Debug, Clone)]
pub struct WireguardTunnel {
    config: WireguardConfig,
    socket: UdpSocket,
    tunnel: Tunn,
    peer_map: HashMap<String, SocketAddr>,
}

impl WireguardTunnel {
    pub async fn new(config: WireguardConfig) -> Result<Self> {
        // Parse private key
        let private_key = base64::decode(&config.private_key)
            .context("Failed to decode private key")?;
        if private_key.len() != 32 {
            return Err(anyhow!("Invalid private key length"));
        }
        
        let mut private_key_bytes = [0u8; 32];
        private_key_bytes.copy_from_slice(&private_key);
        
        // Create tunnel
        let tunnel = Tunn::new(
            private_key_bytes,
            None,
            None,
            None,
            0,
            None
        ).context("Failed to create WireGuard tunnel")?;
        
        // Bind to UDP socket
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", config.port.unwrap_or(WIREGUARD_PORT)))
            .await
            .context("Failed to bind WireGuard socket")?;
        
        // Create peer map
        let mut peer_map = HashMap::new();
        for peer in &config.peers {
            let endpoint: SocketAddr = peer.endpoint.parse()
                .context("Failed to parse peer endpoint")?;
            
            // Parse peer public key
            let public_key = base64::decode(&peer.public_key)
                .context("Failed to decode peer public key")?;
            if public_key.len() != 32 {
                return Err(anyhow!("Invalid peer public key length"));
            }
            
            peer_map.insert(peer.public_key.clone(), endpoint);
        }
        
        Ok(Self {
            config,
            socket,
            tunnel,
            peer_map,
        })
    }
    
    // Start the WireGuard tunnel
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting WireGuard tunnel...");
        
        // Create channels for communication
        let (inbound_tx, mut inbound_rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1000);
        let (outbound_tx, mut outbound_rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1000);
        
        // Clone socket for the receiver task
        let recv_socket = self.socket.clone();
        
        // Spawn receiver task
        tokio::spawn(async move {
            let mut buf = vec![0u8; MAX_PACKET_SIZE];
            loop {
                match recv_socket.recv_from(&mut buf).await {
                    Ok((n, addr)) => {
                        let packet = buf[..n].to_vec();
                        if let Err(e) = inbound_tx.send((packet, addr)).await {
                            warn!("Failed to send packet to processor: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("Error receiving from socket: {}", e);
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        });
        
        // Clone socket for the sender task
        let send_socket = self.socket.clone();
        
        // Spawn sender task
        tokio::spawn(async move {
            while let Some((packet, addr)) = outbound_rx.recv().await {
                if let Err(e) = send_socket.send_to(&packet, addr).await {
                    warn!("Error sending to {}: {}", addr, e);
                }
            }
        });
        
        // Main processing loop
        loop {
            tokio::select! {
                Some((packet, addr)) = inbound_rx.recv() => {
                    debug!("Received {} bytes from {}", packet.len(), addr);
                    
                    // Process the packet through WireGuard
                    match self.tunnel.decapsulate(None, &packet) {
                        TunnResult::WriteToNetwork(packet) => {
                            debug!("Sending {} bytes to {}", packet.len(), addr);
                            if let Err(e) = outbound_tx.send((packet.to_vec(), addr)).await {
                                warn!("Failed to send packet: {}", e);
                            }
                        }
                        TunnResult::WriteToTunnelV4(packet, _) => {
                            debug!("Received IPv4 packet from tunnel: {} bytes", packet.len());
                            // Here you would forward the packet to the TUN device
                            // or your application logic
                        }
                        TunnResult::WriteToTunnelV6(packet, _) => {
                            debug!("Received IPv6 packet from tunnel: {} bytes", packet.len());
                            // Here you would forward the packet to the TUN device
                            // or your application logic
                        }
                        TunnResult::HandshakeComplete => {
                            info!("WireGuard handshake completed with {}", addr);
                        }
                        TunnResult::Done => {
                            // Nothing to do
                        }
                    }
                }
                else => {
                    // All channels closed
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    // Generate WireGuard configuration
    pub fn generate_client_config(&self, client_private_key: &str, client_ip: &str) -> Result<String> {
        // Find server public key (first peer)
        let server_peer = self.config.peers.first()
            .ok_or_else(|| anyhow!("No server peer found"))?;
            
        // Generate client config
        let config = format!(
            "[Interface]\n\
            PrivateKey = {}\n\
            Address = {}\n\
            DNS = 1.1.1.1\n\
            \n\
            [Peer]\n\
            PublicKey = {}\n\
            AllowedIPs = {}\n\
            Endpoint = {}\n\
            PersistentKeepalive = {}\n",
            client_private_key,
            client_ip,
            server_peer.public_key,
            server_peer.allowed_ips.join(", "),
            server_peer.endpoint,
            server_peer.persistent_keepalive
        );
        
        Ok(config)
    }
}

// Utility function to generate WireGuard keypair
pub fn generate_wireguard_keypair() -> Result<(String, String)> {
    // Generate a random private key
    let mut private_key = [0u8; 32];
    getrandom::getrandom(&mut private_key)
        .context("Failed to generate random private key")?;
    
    // Derive public key from private key
    let public_key = x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(private_key));
    
    // Encode keys to base64
    let private_key_base64 = base64::encode(private_key);
    let public_key_base64 = base64::encode(public_key.as_bytes());
    
    Ok((private_key_base64, public_key_base64))
}

// Helper function to parse WireGuard configuration from file
pub async fn parse_wireguard_config(config_path: &str) -> Result<WireguardConfig> {
    let content = tokio::fs::read_to_string(config_path)
        .await
        .context("Failed to read WireGuard config file")?;
    
    let mut private_key = String::new();
    let mut address = String::new();
    let mut port = WIREGUARD_PORT;
    let mut peers = Vec::new();
    
    let mut current_section = None;
    let mut current_peer = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line.starts_with('[') && line.ends_with(']') {
            let section = line[1..line.len()-1].to_string();
            if section == "Interface" {
                current_section = Some("Interface".to_string());
            } else if section == "Peer" {
                current_section = Some("Peer".to_string());
                current_peer = Some(WireguardPeer {
                    public_key: String::new(),
                    endpoint: String::new(),
                    allowed_ips: Vec::new(),
                    persistent_keepalive: 0,
                });
            }
            continue;
        }
        
        if let Some(section) = &current_section {
            if let Some(idx) = line.find('=') {
                let key = line[..idx].trim().to_string();
                let value = line[idx+1..].trim().to_string();
                
                if section == "Interface" {
                    match key.as_str() {
                        "PrivateKey" => private_key = value,
                        "Address" => address = value.split('/').next().unwrap_or(&value).to_string(),
                        "ListenPort" => {
                            if let Ok(p) = value.parse::<u16>() {
                                port = p;
                            }
                        }
                        _ => {}
                    }
                } else if section == "Peer" {
                    if let Some(peer) = &mut current_peer {
                        match key.as_str() {
                            "PublicKey" => peer.public_key = value,
                            "Endpoint" => peer.endpoint = value,
                            "AllowedIPs" => {
                                peer.allowed_ips = value.split(',')
                                    .map(|s| s.trim().to_string())
                                    .collect();
                            }
                            "PersistentKeepalive" => {
                                if let Ok(keep) = value.parse::<u16>() {
                                    peer.persistent_keepalive = keep;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        // If we have a complete peer, add it to the list
        if current_section == Some("Peer".to_string()) && line.is_empty() {
            if let Some(peer) = current_peer.take() {
                if !peer.public_key.is_empty() && !peer.endpoint.is_empty() {
                    peers.push(peer);
                }
            }
            current_section = Some("Interface".to_string());
        }
    }
    
    // Add the last peer if there is one
    if let Some(peer) = current_peer {
        if !peer.public_key.is_empty() && !peer.endpoint.is_empty() {
            peers.push(peer);
        }
    }
    
    if private_key.is_empty() {
        return Err(anyhow!("PrivateKey is required"));
    }
    
    if address.is_empty() {
        return Err(anyhow!("Address is required"));
    }
    
    Ok(WireguardConfig {
        private_key,
        address,
        port,
        peers,
    })
}