---
name: test-scaffold
description: Generate test module for a Rust source file using project conventions (assert_cmd, mockito, tempfile)
disable-model-invocation: true
---

# Test Scaffold Generator

Generate idiomatic Rust tests for a given source file in bbctl.

## Arguments

- `$ARGUMENTS` — path to the source file to generate tests for (e.g., `src/config/mod.rs`)

## Instructions

1. Read the target source file
2. Identify all public functions, structs, and trait implementations
3. Generate a `#[cfg(test)]` module at the bottom of the file with tests for each public item
4. Use the project's dev-dependencies:
   - `tempfile` for tests that need filesystem operations (config files)
   - `mockito` for tests that make HTTP requests (API clients)
   - `assert_cmd` for CLI integration tests (main.rs)
5. Follow these patterns:
   - Test function naming: `test_<function_name>_<scenario>`
   - Use `#[test]` for sync tests, `#[tokio::test]` for async tests
   - Include both happy path and error cases
   - For config tests, use `tempfile::tempdir()` instead of writing to `~/.bbctl/`
6. Show the generated test module and ask for confirmation before writing
