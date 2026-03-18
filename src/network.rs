use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::debug;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};

const WIREGUARD_PORT: u16 = 51820;

/// A WireGuard peer configuration entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardPeer {
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u16,
}

/// A WireGuard interface configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardConfig {
    pub private_key: String,
    pub address: String,
    pub port: u16,
    pub peers: Vec<WireguardPeer>,
}

/// A generated WireGuard key pair (base64-encoded).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardKeypair {
    pub private_key: String,
    pub public_key: String,
}

/// Generate a WireGuard X25519 key pair.
///
/// Returns a `WireguardKeypair` where both keys are standard base64-encoded strings
/// compatible with the `wg` CLI tool and VyOS configuration.
pub fn generate_wireguard_keypair() -> Result<WireguardKeypair> {
    let mut rng = rand::thread_rng();
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    let secret = StaticSecret::from(secret_bytes);
    let public = PublicKey::from(&secret);

    let private_key = BASE64.encode(secret.as_bytes());
    let public_key = BASE64.encode(public.as_bytes());

    debug!("Generated WireGuard keypair (public key: {})", public_key);
    Ok(WireguardKeypair {
        private_key,
        public_key,
    })
}

/// Derive the public key from a base64-encoded WireGuard private key.
pub fn derive_public_key(private_key_b64: &str) -> Result<String> {
    let private_bytes = BASE64
        .decode(private_key_b64)
        .context("Failed to base64-decode private key")?;
    if private_bytes.len() != 32 {
        return Err(anyhow!(
            "Invalid private key length: expected 32 bytes, got {}",
            private_bytes.len()
        ));
    }
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&private_bytes);
    let secret = StaticSecret::from(key_array);
    let public = PublicKey::from(&secret);
    Ok(BASE64.encode(public.as_bytes()))
}

/// Generate a WireGuard client configuration file string.
pub fn generate_client_config(
    client_private_key: &str,
    client_ip: &str,
    server_public_key: &str,
    server_endpoint: &str,
    allowed_ips: &[&str],
    persistent_keepalive: u16,
) -> String {
    format!(
        "[Interface]\n\
        PrivateKey = {private_key}\n\
        Address = {address}\n\
        DNS = 1.1.1.1\n\
        \n\
        [Peer]\n\
        PublicKey = {pub_key}\n\
        AllowedIPs = {allowed}\n\
        Endpoint = {endpoint}\n\
        PersistentKeepalive = {keepalive}\n",
        private_key = client_private_key,
        address = client_ip,
        pub_key = server_public_key,
        allowed = allowed_ips.join(", "),
        endpoint = server_endpoint,
        keepalive = persistent_keepalive,
    )
}

/// Parse a WireGuard configuration file into a [`WireguardConfig`].
pub async fn parse_wireguard_config(config_path: &str) -> Result<WireguardConfig> {
    let content = tokio::fs::read_to_string(config_path)
        .await
        .context("Failed to read WireGuard config file")?;

    let mut private_key = String::new();
    let mut address = String::new();
    let mut port = WIREGUARD_PORT;
    let mut peers: Vec<WireguardPeer> = Vec::new();

    let mut current_section: Option<String> = None;
    let mut current_peer: Option<WireguardPeer> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            // Flush a completed peer when we hit a blank line
            if line.is_empty() {
                if current_section.as_deref() == Some("Peer") {
                    if let Some(peer) = current_peer.take() {
                        if !peer.public_key.is_empty() && !peer.endpoint.is_empty() {
                            peers.push(peer);
                        }
                    }
                }
            }
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            // Flush peer on new section header
            if let Some(peer) = current_peer.take() {
                if !peer.public_key.is_empty() && !peer.endpoint.is_empty() {
                    peers.push(peer);
                }
            }
            let section = line[1..line.len() - 1].to_string();
            if section == "Peer" {
                current_peer = Some(WireguardPeer {
                    public_key: String::new(),
                    endpoint: String::new(),
                    allowed_ips: Vec::new(),
                    persistent_keepalive: 0,
                });
            }
            current_section = Some(section);
            continue;
        }

        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim();
            let value = line[idx + 1..].trim().to_string();

            match current_section.as_deref() {
                Some("Interface") => match key {
                    "PrivateKey" => private_key = value,
                    "Address" => {
                        address = value.split('/').next().unwrap_or(&value).to_string()
                    }
                    "ListenPort" => {
                        if let Ok(p) = value.parse::<u16>() {
                            port = p;
                        }
                    }
                    _ => {}
                },
                Some("Peer") => {
                    if let Some(peer) = &mut current_peer {
                        match key {
                            "PublicKey" => peer.public_key = value,
                            "Endpoint" => peer.endpoint = value,
                            "AllowedIPs" => {
                                peer.allowed_ips = value
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .collect();
                            }
                            "PersistentKeepalive" => {
                                if let Ok(k) = value.parse::<u16>() {
                                    peer.persistent_keepalive = k;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Flush the last peer
    if let Some(peer) = current_peer {
        if !peer.public_key.is_empty() && !peer.endpoint.is_empty() {
            peers.push(peer);
        }
    }

    if private_key.is_empty() {
        return Err(anyhow!("PrivateKey is required in WireGuard config"));
    }
    if address.is_empty() {
        return Err(anyhow!("Address is required in WireGuard config"));
    }

    Ok(WireguardConfig {
        private_key,
        address,
        port,
        peers,
    })
}
