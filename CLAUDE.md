# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

SocialConnect is an open source protocol that maps off-chain personal identifiers (such as phone numbers, twitter handles, etc.) to on-chain account addresses. The repository contains the **ODIS** (Oblivious Decentralized Identifier Service) infrastructure that provides privacy-preserving identifier obfuscation.

## Architecture

The repository uses a **monorepo structure with Yarn workspaces**:

### Core Applications
- **Signer** (`apps/signer/`): Individual threshold signing nodes that participate in the distributed signing protocol. Each signer maintains its own database and cryptographic keys.
- **Combiner** (`apps/combiner/`): Orchestrates multiple signers to produce combined threshold signatures. Acts as the coordination layer.
- **Monitor** (`apps/monitor/`): Monitoring and observability service for the ODIS network.

### Key Architecture Patterns
- **Threshold Cryptography**: Uses BLS threshold signatures where multiple signers must cooperate to produce valid signatures
- **Two-Service Types**: 
  - Phone Number Privacy (PNP) - for phone number identifiers
  - Domains - for domain-based identifiers
- **Database-per-Service**: Each signer runs its own database (PostgreSQL, MySQL, SQL Server, or SQLite for testing)
- **Key Management**: Supports Azure Key Vault, Google Secret Manager, AWS Secrets Manager, or mock providers
- **Express.js Servers**: Both signer and combiner run Express servers with middleware for authentication, rate limiting, and metrics

### Configuration
- **Environment-based**: Heavy use of environment variables for configuration
- **Development vs Production**: Special development configurations with mock signers and test keys
- **Multiple Networks**: Support for mainnet, alfajores (testnet), and staging deployments

## Common Development Commands

### Setup and Installation
```bash
yarn install              # Install all dependencies
yarn reset               # Clean and reinstall everything
yarn reset-modules       # Remove node_modules
```

### Building and Testing
```bash
yarn build               # Build all workspaces
yarn build:fast          # Build only changed workspaces since last commit
yarn test                # Run all tests
yarn test:fast           # Run tests only on changed workspaces
yarn lint                # Lint all workspaces
yarn prettify            # Format code with Prettier
```

### Individual App Commands
```bash
# Signer
cd apps/signer
yarn start               # Build and start signer with dotenv
yarn start:docker        # Start for Docker (no dotenv)
yarn test:integration    # Run integration tests
yarn test:e2e           # Run end-to-end tests
yarn db:migrate         # Run database migrations
yarn bls:keygen         # Generate BLS threshold keys

# Combiner  
cd apps/combiner
yarn start               # Build and start combiner
yarn test:e2e           # Run end-to-end tests
```

### Testing Against Live Networks
```bash
# Signer tests against specific environments
yarn test:e2e:staging:0     # Test staging signer 0
yarn test:e2e:alfajores:1   # Test alfajores signer 1
yarn test:e2e:mainnet:0     # Test mainnet signer 0

# Combiner tests
yarn test:e2e:staging       # Test staging combiner
yarn test:e2e:alfajores     # Test alfajores combiner
```

## Database Management

### Migrations
- Located in `apps/signer/src/common/database/migrations/`
- Use Knex.js migration system
- Run with `yarn db:migrate` in signer directory
- Create new migrations with `yarn db:migrate:make`

### Supported Databases
- PostgreSQL (production default)
- MySQL
- Microsoft SQL Server  
- SQLite (testing only)

## Key Management and Security

### Cryptographic Operations
- **BLS Threshold Signatures**: Core cryptographic primitive
- **POPRF (Partially Oblivious PRF)**: Used for identifier obfuscation
- **Key Rotation**: Support for multiple key versions

### Environment Variables
Critical configuration through environment variables:
- `DB_*`: Database connection settings
- `KEYSTORE_*`: Key provider configuration  
- `BLOCKCHAIN_PROVIDER`: Web3 provider URL
- `*_API_ENABLED`: Enable/disable specific API endpoints
- Service-specific timeouts, retry counts, and thresholds

## Development Tips

### Local Development
- Development mode uses mock signers running on localhost:3001-3003
- Test keys and polynomials are pre-configured for development
- Use `DEV_MODE=true` or `NODE_ENV=development`

### Testing Strategy
- **Unit tests**: Individual component testing
- **Integration tests**: Database and service integration
- **End-to-end tests**: Full protocol testing against live or mock services
- **Coverage**: Run with `yarn test:coverage`

### Docker Support
- Dockerfiles provided in `/dockerfiles/` directory
- Kubernetes deployment configs in `/kubernetes-deployment/`
- Special Docker start commands that skip dotenv loading