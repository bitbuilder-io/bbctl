# bbctl Development Guidelines

## Build & Run Commands

```bash
# Build
cargo build

# Run
cargo run

# Run with specific command
cargo run -- [command] [subcommand] [args]

# Build optimized release version
cargo build --release

# Run specific test
cargo test test_name -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy
```

## CLI Examples

```bash
# List instances
cargo run -- instances list

# Create instance
cargo run -- instances create my-instance --provider vyos --region nyc --cpu 2 --memory 4 --disk 80

# Create volume
cargo run -- volumes create my-volume --size 10 --region nyc

# Create network
cargo run -- networks create my-network --cidr 192.168.1.0/24

# Build optimized release version
cargo build --release

# Check for compilation errors without building
cargo check

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run specific test with output
cargo test test_name -- --nocapture
```

## Code Style Guidelines

-   **Formatting**: Use `cargo fmt` to format code according to Rust standard style

-   **Linting**: Run `cargo clippy` for static analysis

-   **Naming**:

-   Use snake_case for variables, functions, and modules

-   Use PascalCase for structs, enums, and traits

-   **Error Handling**: Use `AppResult<T>` for functions that can fail

-   **State Management**: Follow the App/AppMode pattern for managing application state

-   **UI Components**: Use Ratatui components (List, Table, Paragraph) with consistent styling

-   **Provider APIs**: VyOS and Proxmox providers should implement common traits

## Project Structure

-   **src/app.rs**: Core application state and data models
-   **src/event.rs**: Event handling for TUI (keyboard, mouse, resize)
-   **src/handler.rs**: Keyboard event processing
-   **src/tui.rs**: Terminal setup and management
-   **src/ui.rs**: UI rendering and layout components
-   **src/main.rs**: CLI command processing using Clap

Future work includes integrating with actual VyOS and Proxmox APIs and adding E2E encryption for public cloud integration.

-   **Imports**: Group imports by crate, with std first, then external, then internal
-   **Document**: Use three slashes (`///`) for public API documentation
-   **Async**: Use tokio runtime with futures for async operations

## Project Structure

The app is organized following a typical TUI pattern with app state, event handling, and UI rendering modules. Follow existing patterns when adding new functionality.
