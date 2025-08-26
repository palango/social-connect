# ODIS Rust Implementation

This directory contains the Rust implementation of the ODIS (Oblivious Decentralized Identifier Service) components.

## Structure

```
rust/
├── Cargo.toml          # Workspace configuration
├── rustfmt.toml        # Code formatting rules
├── bin/                # Binary applications
│   └── signer/         # ODIS Signer implementation
└── target/             # Build artifacts (ignored by git)
```

## Development

### Prerequisites

- Rust 1.75+ (edition 2021)
- PostgreSQL/MySQL (for production) or SQLite (for development)### Building

```bash
# Build all workspace members
cargo build

# Build specific binary
cargo build --bin signer

# Build with optimizations
cargo build --release
```

### Running

```bash
# Run the signer
cargo run --bin signer

# Run with debug logging
RUST_LOG=debug cargo run --bin signer
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific binary
cargo test --bin signer

# Run with coverage
cargo tarpaulin --out html
```

### Development Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

## Migration Status

This is part of the migration from TypeScript to Rust. See:

- [Migration Plan](../SIGNER_RUST_MIGRATION_PLAN.md)
- [Progress Checklist](../RUST_MIGRATION_CHECKLIST.md)

### Completed

- ✅ **Phase 1.1**: Project Setup - Basic Rust workspace with dependencies

### In Progress

- 🔄 **Phase 1.2**: Configuration System
- ⏳ **Phase 1.3**: Logging Infrastructure

## License

Apache 2.0 - See [LICENSE](../LICENSE) file.
