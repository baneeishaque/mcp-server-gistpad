# Contributing to mcp-server-gistpad

Thank you for considering contributing to this project!

## Getting Started

1. Fork the repository.
2. Clone your fork: `git clone https://github.com/baneeishaque/mcp-server-gistpad.git`
3. Create a feature branch: `git checkout -b feature/my-feature`

## Development Setup

Rust stable + wasm32-wasip2: rustup target add wasm32-wasip2; build with cargo build
--target wasm32-wasip2 --release; test in Zed via "zed: install dev extension" on this repository.

## Code Style

rustfmt defaults (cargo fmt); clippy clean (cargo clippy --target wasm32-wasip2); Conventional Commits.

## Pull Request Process

1. Ensure your code follows the project's style guidelines.
2. Update documentation if you change functionality.
3. Add or update tests as needed.
4. Make sure all tests pass before submitting.
5. Submit a pull request with a clear description of changes.

## Reporting Issues

- Use the issue tracker to report bugs or request features.
- Check existing issues before creating a new one.
- Provide as much context as possible (OS, version, steps to reproduce).

## Code of Conduct

Please note that this project follows a Code of Conduct. By participating,
you agree to uphold its standards.

Thank you for contributing!
