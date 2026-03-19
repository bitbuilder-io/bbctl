# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                          # Dev build
cargo build --release                # Release build
cargo run                            # Launch TUI (no args)
cargo run -- instances list          # CLI mode with subcommand
cargo run -- test-vyos --host HOST --port 60022 --username vyos --api-key KEY
cargo test                           # All tests
cargo test test_name -- --nocapture  # Single test with output
cargo fmt                            # Format
cargo clippy                         # Lint
```

## What This Project Is

bbctl is a CLI/TUI tool for provisioning multi-tenant infrastructure on bare metal servers running VyOS v1.5 or Proxmox. It provides both a Clap-based CLI and an interactive Ratatui terminal dashboard.

## Architecture

Five-layer design with strict dependency direction (upper layers depend on lower):

```
CLI (main.rs) / TUI (app.rs, ui.rs, handler.rs, event.rs, tui.rs)
                            |
              Services (services/) — orchestration, client factory
                            |
              Models (models/) — domain entities with business logic
                            |
              Config (config/) — TOML persistence (~/.bbctl/)
                            |
              API (api/) — provider-specific HTTP/SSH clients
```

### Entry Point (main.rs)

`#[tokio::main]` dispatches two paths:
- **CLI mode**: `env::args().len() > 1` → `cli_handler()` processes Clap subcommands synchronously
- **TUI mode**: No args → `run_tui()` launches the async Ratatui event loop
- **Special case**: `test-vyos` subcommand is matched before `cli_handler()` because it needs the async runtime for SSH/HTTP calls

Clap subcommands: `init`, `deploy`, `instances {list|create|delete|start|stop|show}`, `volumes {list|create|delete|attach|detach|show}`, `networks {list|create|delete|connect|disconnect|show}`, `test-vyos`

### Provider System (api/)

`Provider` trait in `api/mod.rs` defines the interface (uses `anyhow::Result`):
```rust
pub trait Provider {
    fn connect(&self) -> Result<()>;
    fn check_connection(&self) -> Result<bool>;
    fn name(&self) -> &str;
}
```

Two implementations:
- **VyOSClient** (`api/vyos.rs`) — SSH via `tokio::process` + HTTP API with `reqwest`. Key methods: `execute_ssh_command()`, `api_call()`, `get_config()`, `set_config()`, `commit()`, `save()`, `get_system_info()`
- **ProxmoxClient** (`api/proxmox.rs`) — HTTP API with ticket/CSRF auth or API token. Key methods: `login()`, `api_call()`. Auth enum: `ProxmoxAuth::UserPass` or `ProxmoxAuth::ApiToken`

Both use `reqwest::Client` with `danger_accept_invalid_certs` for self-signed certs.

### Services Layer (services/)

`ProviderService` is the factory that bridges config → API clients:
- Loads `Providers` + `Credentials` from TOML
- `get_vyos_client()` / `get_proxmox_client()` instantiate typed clients from stored config + credentials
- `add_vyos_provider()` / `add_proxmox_provider_with_token()` register new providers

`InstanceService`, `VolumeService`, `NetworkService` each hold in-memory storage (`HashMap<Uuid, T>`) plus a `ProviderService` reference.

### Config System (config/)

Config dir: `~/.bbctl/` (constant `APP_DIR_NAME`). Three TOML files:
- **settings.toml** — global settings (default provider, region, log level)
- **providers.toml** — provider configs + regions (HashMap-based, keyed by name)
- **credentials.toml** — auth credentials, intentionally separated from provider configs

Helper functions: `get_config_dir()`, `get_config_file()`, `read_config_file()`, `write_config_file()`, `config_file_exists()`, `delete_config_file()`, `init_config()` (creates defaults on first run).

### TUI Architecture

Event loop: `App::new()` → `EventHandler` (spawns tokio task reading `crossterm::EventStream` with 250ms tick) → main loop calls `tui.draw()` then matches on `Event::{Tick, Key, Mouse, Resize}`.

`AppMode` enum (`Home | Instances | Volumes | Networks | Settings | Help`) drives which `render_*()` function runs. `selected_index` tracks list navigation. Keys: `1-5` jump modes, `j/k` navigate, `q/ESC` back/quit, `?` help.

Note: `app.rs` contains both TUI state (`App`, `AppMode`) and simple display structs (`Instance`, `Volume`, `Network`) used for hardcoded demo data. The richer domain types live in `models/`.

### Models (models/)

Domain types with business logic methods (separate from the simpler display structs in `app.rs`):
- **Instance** (`models/instance.rs`) — UUID id, `InstanceStatus` enum, `InstanceSize {cpu, memory_gb, disk_gb}`, `Vec<InstanceNetwork>`, tags HashMap
- **Volume** (`models/volume.rs`) — `VolumeStatus`/`VolumeType` enums, `attach()`/`detach()`/`extend()` methods with validation
- **Network** (`models/network.rs`) — `NetworkType` enum, `HashSet<Uuid>` for connected instances, `Vec<IpAllocation>` with `allocate_ip()`/`release_ip()`
- **Provider** (`models/provider.rs`) — `ProviderType` enum used by other models

## Error Handling

Two error types coexist:
- `AppResult<T> = Result<T, Box<dyn error::Error>>` in `app.rs` — used by TUI and main
- `anyhow::Result<T>` — used by config, API, and services layers (with `anyhow::Context` for wrapping)

## Key Dependencies

- **ratatui** + **crossterm** (event-stream) — TUI framework
- **clap** (derive) — CLI parsing
- **tokio** (full) — async runtime
- **reqwest** (json, rustls-tls) — HTTP client
- **serde** + **toml** — config serialization
- **uuid** (v4, serde) — resource identifiers
- **anyhow** — error handling in lower layers
- **chrono** (serde) — timestamps on domain models

## Code Style

- **Formatting**: `cargo fmt` (standard rustfmt)
- **Linting**: `cargo clippy`
- **Naming**: snake_case (vars/fns/modules), PascalCase (structs/enums/traits)
- **Imports**: Group by crate — std first, then external, then internal
- **Docs**: `///` for public API documentation

## Current State

- CLI handler uses hardcoded println output (not yet connected to real providers)
- TUI displays hardcoded demo data from `App::new()`
- Provider trait only covers connection; no CRUD traits for resources yet
- Services use in-memory `HashMap` storage, no persistence layer
- `models/` has rich domain types but they're not yet wired into `app.rs` display structs
