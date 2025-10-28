# BitBuilder Cloud CLI Documentation

Welcome to the BitBuilder Cloud CLI (bbctl) documentation. This index provides an overview of all available documentation for working with and extending the bbctl project.

## Documentation Index

| Document              | Description                                             |
| --------------------- | ------------------------------------------------------- |
| [User Guide]          | Comprehensive guide for using bbctl                     |
| [Command Reference]   | Detailed reference of all bbctl commands                |
| [Configuration Guide] | Guide for configuring bbctl                             |
| [Deployment Guide]    | Guide for deploying applications with bbctl             |
| [Architecture Design] | Technical architecture and system design                |
| [API Documentation]   | API schema and OpenAPI integration details              |
| [Rust Integration]    | Guide for maintaining Rust and TypeScript compatibility |
| [VyOS Network Plan]   | Comprehensive networking architecture design            |
| [VyOS Test Lab Setup] | Instructions for setting up a test environment          |

[User Guide]: user-guide.md
[Command Reference]: command-reference.md
[Configuration Guide]: configuration-guide.md
[Deployment Guide]: deployment-guide.md
[Architecture Design]: ARCHITECTURE_DESIGN.md
[API Documentation]: api-readme.md
[Rust Integration]: rust-integration.md
[VyOS Network Plan]: vyos-network-plan.md
[VyOS Test Lab Setup]: vyos-test-lab-setup.md

## Getting Started

If you're new to the project, we recommend starting with:

1. The main [README] for an overview of capabilities
2. [User Guide] for a comprehensive introduction
3. [Command Reference] for detailed command usage
4. [Architecture Design] to understand the system
5. [VyOS Test Lab Setup] to create a test environment

[README]: ../README.md
[User Guide]: user-guide.md
[Command Reference]: command-reference.md
[Architecture Design]: ARCHITECTURE_DESIGN.md
[VyOS Test Lab Setup]: vyos-test-lab-setup.md

## API and Development

For developers looking to integrate or extend bbctl:

- [API Documentation] contains schema details and API reference
- [Rust Integration] provides guidance for maintaining compatibility between Rust and TypeScript
- [Configuration Guide] explains configuration options and customization
- [Deployment Guide] covers advanced deployment scenarios and CI/CD integration
- Run the [examples] to understand API usage

[API Documentation]: api-readme.md
[Rust Integration]: rust-integration.md
[Configuration Guide]: configuration-guide.md
[Deployment Guide]: deployment-guide.md
[examples]: ../examples/

## Network Architecture

The networking components are documented in:

- [VyOS Network Plan] for detailed network design
- [VyOS Test Lab Setup] for lab environment configuration

[VyOS Network Plan]: vyos-network-plan.md
[VyOS Test Lab Setup]: vyos-test-lab-setup.md

## Contributing

Contributions are welcome! Please follow the development guidelines in the [Architecture Design][1] document.

[1]: ARCHITECTURE_DESIGN.md#development-guidelines

When contributing code, ensure compatibility between the Rust backend and TypeScript schema as described in [Rust Integration].

[Rust Integration]: rust-integration.md

## Additional Resources

- [GitHub Repository]
- [Issue Tracker]
- View API documentation by running `bun run generate-openapi` and opening the HTML page
- Check the [Command Reference] for all available commands
- See the [User Guide] for tutorials and examples

[GitHub Repository]: https://github.com/bitbuilder-io/bbctl
[Issue Tracker]: https://github.com/bitbuilder-io/bbctl/issues
[Command Reference]: command-reference.md
[User Guide]: user-guide.md
