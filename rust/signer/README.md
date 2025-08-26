# ODIS Signer - Rust Implementation

A Rust implementation of the ODIS (Oblivious Decentralized Identifier Service) Signer, providing privacy-preserving identifier obfuscation through threshold cryptography.

## Overview

This is a complete rewrite of the TypeScript ODIS Signer in Rust, designed to provide:

- **Better Performance**: Lower latency and higher throughput
- **Memory Safety**: Rust's guarantees eliminate entire classes of security vulnerabilities
- **Resource Efficiency**: Reduced memory footprint and CPU usage
- **Maintainability**: Strong type system and excellent tooling

## Architecture

The signer implements threshold BLS signatures to participate in the ODIS protocol:

- **Phone Number Privacy (PNP)**: Obfuscation of phone numbers
- **Domain Services**: Custom domain-based identifier obfuscation
- **Key Management**: Support for Azure Key Vault, Google Secret Manager, AWS Secrets Manager
- **Database**: PostgreSQL, MySQL, and SQLite support via SeaORM

## Development Status

🚧 **This is currently under active development as part of the migration from TypeScript.**

See the [Migration Plan](../../SIGNER_RUST_MIGRATION_PLAN.md) and [Progress Checklist](../../RUST_MIGRATION_CHECKLIST.md) for detailed status.

### Current Phase: Foundation & Setup
- ✅ Project structure created
- ✅ Basic configuration system
- ✅ Error handling framework
- 🔄 In progress...

## Building and Running

### Prerequisites

- Rust 1.70+ 
- PostgreSQL/MySQL (for production) or SQLite (for development)

### Development

```bash
# Build the project
cargo build

# Run with development defaults
cargo run

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Configuration

The signer is configured via environment variables:

```bash
# Server configuration
SERVER_PORT=3000
SERVER_HOST=0.0.0.0

# Database configuration
DATABASE_TYPE=postgres
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_NAME=odis_signer
DATABASE_USER=signer
DATABASE_PASSWORD=password

# Keystore configuration
KEYSTORE_PROVIDER=azure
AZURE_VAULT_URL=https://your-vault.vault.azure.net/

# API configuration
PNP_API_ENABLED=true
DOMAIN_API_ENABLED=true
```

## Testing

```bash
# Run unit tests
cargo test

# Run with coverage
cargo tarpaulin --out html

# Run integration tests (when available)
cargo test --test integration

# Run benchmarks
cargo bench
```

## API Compatibility

This Rust implementation maintains full API compatibility with the TypeScript version:

- All endpoints return identical JSON responses
- Same authentication mechanisms
- Compatible metrics and logging formats
- Identical cryptographic outputs

## Contributing

This implementation follows the same contribution guidelines as the main repository. See the main [README](../../README.md) for details.

## License

Apache 2.0 - See [LICENSE](../../LICENSE) file.
