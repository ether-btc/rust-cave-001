# Contributing Guide

Thank you for your interest in contributing to rust-cave-001! This document will help you get started.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-cave-001.git
   cd rust-cave-001
   ```
3. Set up a Python virtual environment (recommended):
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pytest
   ```

## Building

```bash
# Development build
maturin develop

# Release build
maturin develop --release
```

## Running Tests

```bash
pytest tests/ -v
```

## Code Quality

- Run `cargo fmt` and `cargo clippy` before committing
- Ensure all tests pass before submitting a PR
- Add tests for new features

## Pull Request Process

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Make your changes and add tests
3. Run the full test suite
4. Commit with a clear message
5. Push and open a Pull Request

## Code of Conduct

Be respectful and constructive. We welcome contributors of all skill levels.

## Questions?

Open a GitHub Discussion or Issue. We are happy to help.
