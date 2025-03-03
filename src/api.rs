use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use validator::Validate;

use crate::app::AppResult;

// API Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        provision_instance,
    ),
    components(
        schemas(
            HealthCheckResponse,
            ProvisionRequest,
            ProvisionResponse,
            InstanceConfig,
            NetworkConfig,
            StorageConfig,
        )
    ),
    tags(
        (name = "bitbuilder", description = "BitBuilder Cloud API")
    )
)]
struct ApiDoc;

// API Server State
#[derive(Clone)]
pub struct ApiState {
    pub vyos_host: String,
    pub vyos_port: u16,
    pub vyos_username: String,
    pub api_version: String,
}

// Schema Definitions
#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, JsonSchema)]
pub struct InstanceConfig {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub cpu: u8,
    pub memory_gb: u8,
    pub disk_gb: u8,
    pub provider: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, JsonSchema)]
pub struct NetworkConfig {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(regex = r"^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$")]
    pub cidr: String,
    pub wireguard_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, JsonSchema)]
pub struct StorageConfig {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub size_gb: u8,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, JsonSchema)]
pub struct ProvisionRequest {
    pub instance: InstanceConfig,
    pub network: Option<NetworkConfig>,
    pub storage: Option<Vec<StorageConfig>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct ProvisionResponse {
    pub instance_id: String,
    pub network_id: Option<String>,
    pub storage_ids: Option<Vec<String>>,
    pub wireguard_config: Option<String>,
}

// API Routes
#[utoipa::path(
    get,
    path = "/health",
    tag = "bitbuilder",
    responses(
        (status = 200, description = "API health status", body = HealthCheckResponse)
    )
)]
async fn health_check(State(state): State<Arc<ApiState>>) -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: state.api_version.clone(),
    })
}

#[utoipa::path(
    post,
    path = "/provision",
    tag = "bitbuilder",
    request_body = ProvisionRequest,
    responses(
        (status = 201, description = "Instance provisioned successfully", body = ProvisionResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    )
)]
async fn provision_instance(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<ProvisionRequest>,
) -> Result<(StatusCode, Json<ProvisionResponse>), (StatusCode, String)> {
    // Validate the request
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    // TODO: Implement actual VyOS provisioning via SSH or API
    // For now, just return a mock response
    let wireguard_config = if payload.network.as_ref().map_or(false, |n| n.wireguard_enabled) {
        Some(format!(
            "[Interface]\nPrivateKey = {}\nAddress = 10.10.0.2/24\n\n[Peer]\nPublicKey = {}\nAllowedIPs = 10.10.0.0/24\nEndpoint = {}:51820\nPersistentKeepalive = 25\n",
            "PRIVATE_KEY_PLACEHOLDER",
            "PUBLIC_KEY_PLACEHOLDER",
            state.vyos_host
        ))
    } else {
        None
    };

    let response = ProvisionResponse {
        instance_id: format!("i-{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
        network_id: payload.network.map(|_| format!("net-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())),
        storage_ids: payload.storage.map(|s| s.iter().map(|_| format!("vol-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())).collect()),
        wireguard_config,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// Temporary VyOS SSH implementation until API is ready
async fn provision_via_ssh(
    payload: &ProvisionRequest,
    state: &ApiState,
) -> anyhow::Result<()> {
    // In a real implementation, you would use an SSH library or make API calls
    // For now, we'll use the system's ssh command via tokio::process::Command
    
    let instance_name = &payload.instance.name;
    
    // Example command to create a new VM
    let create_output = tokio::process::Command::new("ssh")
        .arg("-p")
        .arg(state.vyos_port.to_string())
        .arg(format!("{}@{}", state.vyos_username, state.vyos_host))
        .arg(format!("set interfaces dummy dum0 description '{}'", instance_name))
        .output()
        .await
        .context("Failed to execute SSH command")?;
    
    if !create_output.status.success() {
        let error = String::from_utf8_lossy(&create_output.stderr);
        anyhow::bail!("Failed to provision VM: {}", error);
    }
    
    // Commit the changes
    let commit_output = tokio::process::Command::new("ssh")
        .arg("-p")
        .arg(state.vyos_port.to_string())
        .arg(format!("{}@{}", state.vyos_username, state.vyos_host))
        .arg("commit")
        .output()
        .await
        .context("Failed to commit changes")?;
    
    if !commit_output.status.success() {
        let error = String::from_utf8_lossy(&commit_output.stderr);
        anyhow::bail!("Failed to commit changes: {}", error);
    }
    
    Ok(())
}

// Setup and create the API router
pub fn create_api_router(state: ApiState) -> Router {
    let shared_state = Arc::new(state);
    
    // Create the API documentation
    let openapi = ApiDoc::openapi();
    
    Router::new()
        .route("/health", get(health_check))
        .route("/provision", post(provision_instance))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state)
}

// Function to start the API server
pub async fn start_api_server(
    host: &str,
    port: u16,
    vyos_host: &str,
    vyos_port: u16,
    vyos_username: &str,
) -> AppResult<()> {
    let api_state = ApiState {
        vyos_host: vyos_host.to_string(),
        vyos_port,
        vyos_username: vyos_username.to_string(),
        api_version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    let app = create_api_router(api_state);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .context("Failed to bind to port")?;
    
    println!("API server listening on http://{}:{}", host, port);
    println!("API documentation available at http://{}:{}/docs", host, port);
    
    axum::serve(listener, app)
        .await
        .context("Failed to start API server")?;
    
    Ok(())
}