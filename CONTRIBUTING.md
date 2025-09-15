# Contributing to Polymarket Substreams

We welcome contributions to the Polymarket Substreams project! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- [Substreams CLI](https://docs.substreams.dev/how-to-guides/install-substreams-cli)
- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/Polymarket/polymarket-substreams.git
cd polymarket-substreams
```

2. Build the project:
```bash
substreams build
```

3. Test your changes:
```bash
substreams run substreams.yaml map_ctf_exchange_events --start-block 50000000 --stop-block 50000100
```

## Code Style

- Follow Rust conventions and best practices
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and single-purpose

## Adding New Features

### New Contract Support

To add support for a new Polymarket contract:

1. Add the contract address to `src/lib.rs`
2. Update `substreams.yaml` with a new module
3. Add corresponding protobuf messages in `proto/contract.proto`
4. Implement event extraction logic
5. Add tests for the new functionality

### New Event Types

To add support for new event types:

1. Add the event to the appropriate protobuf message
2. Implement the event extraction logic
3. Update the module handler
4. Add tests

## Testing

Before submitting a pull request:

1. Build the project: `substreams build`
2. Test with sample data
3. Verify all modules work correctly
4. Check for any compilation warnings

## Pull Request Process

1. Create a descriptive title
2. Provide a detailed description of changes
3. Reference any related issues
4. Ensure all tests pass
5. Update documentation if needed

## Reporting Issues

When reporting issues:

1. Use the issue template
2. Provide clear steps to reproduce
3. Include relevant logs or error messages
4. Specify the environment (OS, Substreams version, etc.)

## Questions?

- Join our [Discord](https://discord.gg/thegraph)
- Check the [Substreams documentation](https://docs.substreams.dev)
- Open an issue for questions

Thank you for contributing to Polymarket Substreams!
