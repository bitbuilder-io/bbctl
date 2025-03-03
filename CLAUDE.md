# bbctl Development Guidelines

## Build & Run Commands
```bash
# Build
cargo build

# Run
cargo run

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
- **Formatting**: Use `cargo fmt` to format code according to Rust standard style
- **Linting**: Run `cargo clippy` for static analysis
- **Naming**: 
  - Use snake_case for variables, functions, and modules
  - Use PascalCase for structs, enums, and traits
- **Error Handling**: Use `AppResult<T>` for functions that can fail
- **Imports**: Group imports by crate, with std first, then external, then internal
- **Document**: Use three slashes (`///`) for public API documentation
- **Async**: Use tokio runtime with futures for async operations

## Project Structure
The app is organized following a typical TUI pattern with app state, event handling, and UI rendering modules. Follow existing patterns when adding new functionality.