# ODIS Signer Rust Port - Migration Plan

## Executive Summary

This document outlines a comprehensive plan to port the ODIS Signer service from TypeScript/Node.js to Rust. The migration will be conducted in small, testable increments to ensure reliability and minimize risk.

## Migration Goals

### Primary Objectives
1. **Performance**: Leverage Rust's zero-cost abstractions and memory safety for improved performance
2. **Security**: Benefit from Rust's memory safety guarantees for cryptographic operations
3. **Resource Efficiency**: Reduce memory footprint and CPU usage
4. **Maintainability**: Improve code reliability and reduce runtime errors
5. **Compatibility**: Maintain full API compatibility with existing clients

### Success Criteria
- [ ] Full feature parity with TypeScript implementation
- [ ] All existing tests pass with Rust implementation
- [ ] Performance improvements (latency and throughput)
- [ ] Reduced memory usage
- [ ] Seamless deployment with zero downtime migration

## Current Architecture Analysis

### Core Components to Port
1. **HTTP Server** (Express.js → Axum/Warp)
2. **BLS Cryptography** (blind-threshold-bls WASM → Native Rust)
3. **Database Layer** (Knex.js → SQLx/Diesel)
4. **Key Management** (Azure/GCP/AWS SDKs → Rust equivalents)
5. **Configuration** (dotenv + manual parsing → serde + config crates)
6. **Logging & Metrics** (Bunyan + prom-client → tracing + metrics)
7. **Authentication** (Custom middleware → Tower middleware)

### Dependencies Analysis
```typescript
// Current key dependencies to replace:
"express": "^4.17.1"                    → axum
"blind-threshold-bls": "1.0.0-beta"     → celo-threshold-bls-rs
"knex": "^2.1.0"                        → seaorm
"@azure/keyvault-secrets": "^4.x"       → azure_key_vault crate
"@google-cloud/secret-manager": "3.0.0" → google-cloud-storage-rs
"aws-sdk": "^3.x"                       → aws-sdk-secretsmanager
"bunyan": "1.8.15"                      → tracing
"prom-client": "12.0.0"                 → metrics + prometheus
```

## Migration Strategy: Incremental Replacement

### Phase 1: Foundation & Setup (Weeks 1-2)
**Goal**: Establish Rust project structure and basic infrastructure

#### Step 1.1: Project Setup
- [ ] Create new Rust workspace in `rust/signer/`
- [ ] Set up Cargo.toml with initial dependencies
- [ ] Configure CI/CD for Rust builds
- [ ] Set up development tooling (rustfmt, clippy, etc.)

**Deliverable**: Basic Rust project that compiles and runs "Hello World"

#### Step 1.2: Configuration System
- [ ] Port configuration parsing from TypeScript to Rust
- [ ] Use `serde` and `config` crates for environment variable handling
- [ ] Create `SignerConfig` struct matching TypeScript interface
- [ ] Write unit tests for configuration parsing

**Dependencies**:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
config = "0.13"
envy = "0.4"  # For environment variable parsing
```

**Test**: Configuration loaded correctly from environment variables

#### Step 1.3: Logging Infrastructure
- [ ] Set up structured logging with `tracing`
- [ ] Configure log levels and output formats
- [ ] Add request correlation IDs
- [ ] Test log output compatibility with existing monitoring

**Dependencies**:
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
tracing-bunyan-formatter = "0.3"
```

**Test**: Log format matches existing Bunyan output structure

### Phase 2: HTTP Server & Basic Endpoints (Weeks 3-4)
**Goal**: Create HTTP server with status and metrics endpoints

#### Step 2.1: HTTP Server Setup
- [ ] Implement HTTP server using Axum framework
- [ ] Add CORS middleware
- [ ] Add request logging middleware
- [ ] Add timeout and error handling middleware

**Dependencies**:
```toml
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.4", features = ["cors", "trace", "timeout"] }
hyper = "1.0"
```

**Test**: Server starts and responds to basic HTTP requests

#### Step 2.2: Status Endpoint
- [ ] Implement `GET /status` endpoint
- [ ] Return service version and health information
- [ ] Match existing JSON response format

**Test**: Status endpoint returns expected JSON format

#### Step 2.3: Metrics Endpoint  
- [ ] Set up Prometheus metrics collection
- [ ] Implement `GET /metrics` endpoint
- [ ] Add basic HTTP request metrics (latency, count, errors)

**Dependencies**:
```toml
metrics = "0.21"
metrics-exporter-prometheus = "0.12"
```

**Test**: Metrics endpoint returns Prometheus format, basic metrics collected

### Phase 3: Database Layer (Weeks 5-6)
**Goal**: Implement database connectivity and migrations

#### Step 3.1: Database Connection
- [ ] Implement database connection using SeaORM
- [ ] Support PostgreSQL, MySQL, and SQLite
- [ ] Add connection pooling
- [ ] Handle SSL configuration

**Dependencies**:
```toml
sea-orm = { version = "0.12", features = ["sqlx-postgres", "sqlx-mysql", "sqlx-sqlite", "runtime-tokio-rustls", "macros"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

**Test**: Successfully connect to all supported database types

#### Step 3.2: Database Models
- [ ] Define Rust structs for database entities
- [ ] Implement serialization/deserialization
- [ ] Create query builders for common operations

**Test**: Database models serialize/deserialize correctly

#### Step 3.3: Migration System
- [ ] Port existing Knex migrations to SeaORM format
- [ ] Implement migration runner
- [ ] Add migration rollback capability

**Test**: Migrations run successfully, schema matches TypeScript version

### Phase 4: Key Management (Weeks 7-8)
**Goal**: Implement secure key storage and retrieval

#### Step 4.1: Key Provider Interface
- [ ] Define `KeyProvider` trait matching TypeScript interface
- [ ] Implement mock key provider for testing
- [ ] Add key caching mechanism

**Test**: Mock key provider works correctly

#### Step 4.2: Azure Key Vault Integration
- [ ] Implement Azure Key Vault provider
- [ ] Handle authentication (service principal + managed identity)
- [ ] Add retry logic and error handling

**Dependencies**:
```toml
azure_security_keyvault = "0.17"
azure_identity = "0.17"
azure_core = "0.17"
```

**Test**: Successfully retrieve keys from Azure Key Vault

#### Step 4.3: Google Secret Manager Integration
- [ ] Implement Google Secret Manager provider
- [ ] Handle service account authentication
- [ ] Add retry logic and error handling

**Dependencies**:
```toml
google-cloud-storage = "0.15"
google-cloud-auth = "0.13"
```

**Test**: Successfully retrieve keys from Google Secret Manager

#### Step 4.4: AWS Secrets Manager Integration
- [ ] Implement AWS Secrets Manager provider
- [ ] Handle IAM authentication
- [ ] Add retry logic and error handling

**Dependencies**:
```toml
aws-sdk-secretsmanager = "1.0"
aws-config = "1.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

**Test**: Successfully retrieve keys from AWS Secrets Manager

### Phase 5: BLS Cryptography (Weeks 9-11)
**Goal**: Implement or integrate BLS threshold signatures

#### Step 5.1: Evaluate BLS Library Options
- [ ] Integrate celo-threshold-bls-rs library
- [ ] Set up Rust dependencies for threshold signatures
- [ ] Verify compatibility with existing TypeScript implementation

**Dependencies**:
```toml
celo-threshold-bls-rs = { git = "https://github.com/celo-org/celo-threshold-bls-rs" }
```

#### Step 5.2: BLS Signature Implementation
- [ ] Implement or integrate BLS signature operations
- [ ] Support threshold signature schemes
- [ ] Add blinding/unblinding operations
- [ ] Ensure constant-time operations

**Test**: BLS operations produce same results as TypeScript version

#### Step 5.3: Cryptography Client
- [ ] Port `computeBlindedSignature` function
- [ ] Add signature verification
- [ ] Implement proper error handling for crypto operations

**Test**: Signature generation matches TypeScript implementation

### Phase 6: Business Logic - PNP API (Weeks 12-13)
**Goal**: Implement Phone Number Privacy API endpoints

#### Step 6.1: Account Services
- [ ] Port account service interface
- [ ] Implement ContractKit integration for on-chain verification
- [ ] Add account caching mechanism

**Dependencies**:
```toml
# For Ethereum/Celo integration
ethers = "2.0"
```

#### Step 6.2: Request Tracking Service
- [ ] Port PNP request service
- [ ] Implement database operations for quota tracking
- [ ] Add request deduplication

**Test**: Request tracking works correctly, quota enforced

#### Step 6.3: PNP Endpoints
- [ ] Implement `POST /pnp/sign` endpoint
- [ ] Implement `POST /pnp/quota` endpoint
- [ ] Add request validation and authentication
- [ ] Port rate limiting logic

**Test**: PNP endpoints return same results as TypeScript version

### Phase 7: Business Logic - Domain API (Weeks 14-15)
**Goal**: Implement Domain API endpoints

#### Step 7.1: Domain Quota Service
- [ ] Port domain quota management
- [ ] Implement custom rate limiting schemes
- [ ] Add domain state tracking

#### Step 7.2: Domain Endpoints
- [ ] Implement `POST /domain/sign` endpoint
- [ ] Implement `POST /domain/quota` endpoint  
- [ ] Implement `POST /domain/disable` endpoint
- [ ] Port domain-specific validation logic

**Test**: Domain endpoints return same results as TypeScript version

### Phase 8: Integration & Testing (Weeks 16-17)
**Goal**: Comprehensive testing and integration

#### Step 8.1: Unit Test Migration
- [ ] Port all unit tests from TypeScript to Rust
- [ ] Add additional edge case tests
- [ ] Ensure 100% test coverage for critical paths

#### Step 8.2: Integration Testing
- [ ] Set up integration test suite
- [ ] Test against real cloud keystores (dev environments)
- [ ] Test database operations across all supported DBs
- [ ] Test BLS signature compatibility

#### Step 8.3: End-to-End Testing
- [ ] Port e2e tests to work with Rust implementation
- [ ] Test against staging environments
- [ ] Verify compatibility with existing combiner service

### Phase 9: Performance & Optimization (Weeks 18-19)
**Goal**: Optimize performance and prepare for production

#### Step 9.1: Performance Benchmarking
- [ ] Create benchmarks comparing TypeScript vs Rust
- [ ] Measure latency, throughput, and memory usage
- [ ] Identify and optimize bottlenecks

#### Step 9.2: Production Readiness
- [ ] Add comprehensive error handling
- [ ] Implement graceful shutdown
- [ ] Add health checks and readiness probes
- [ ] Optimize Docker image size

#### Step 9.3: Security Review
- [ ] Conduct security audit of crypto operations
- [ ] Review key handling and memory management
- [ ] Validate input sanitization and validation

### Phase 10: Deployment & Migration (Weeks 20-21)
**Goal**: Deploy to production with zero downtime

#### Step 10.1: Deployment Infrastructure
- [ ] Create Docker images for Rust signer
- [ ] Update Kubernetes manifests
- [ ] Set up blue-green deployment process

#### Step 10.2: Gradual Migration
- [ ] Deploy Rust signer alongside TypeScript version
- [ ] Implement traffic splitting
- [ ] Monitor performance and error rates
- [ ] Gradually increase traffic to Rust version

#### Step 10.3: Full Migration
- [ ] Complete traffic migration to Rust
- [ ] Remove TypeScript signer infrastructure
- [ ] Update documentation and runbooks

## Risk Mitigation

### Technical Risks
1. **BLS Library Compatibility**: Ensure Rust BLS implementation produces identical signatures
   - *Mitigation*: Extensive cross-testing with TypeScript version
   
2. **Performance Regression**: Rust version performs worse than TypeScript
   - *Mitigation*: Continuous benchmarking and optimization
   
3. **Key Provider Integration**: Cloud SDK compatibility issues
   - *Mitigation*: Early testing with all cloud providers
   
4. **Database Migration**: Schema or query compatibility issues
   - *Mitigation*: Comprehensive migration testing

### Operational Risks
1. **Deployment Complexity**: Blue-green deployment issues
   - *Mitigation*: Extensive staging environment testing
   
2. **Monitoring Gap**: Missing observability during migration
   - *Mitigation*: Ensure metric and log compatibility
   
3. **Rollback Complexity**: Difficulty reverting to TypeScript
   - *Mitigation*: Maintain parallel deployment capability

## Success Metrics

### Performance Targets
- **Latency**: 50% reduction in P95 response time
- **Throughput**: 2x increase in requests per second per instance
- **Memory Usage**: 70% reduction in memory footprint
- **CPU Usage**: 30% reduction in CPU utilization

### Quality Targets
- **Test Coverage**: Maintain 90%+ test coverage
- **Error Rate**: No increase in error rates
- **Availability**: Maintain 99.9% uptime during migration
- **Security**: Pass security audit with no critical findings

## Resource Requirements

### Development Team
- **Rust Developer**: 1 senior Rust developer (full-time)
- **Backend Developer**: 1 TypeScript/Node.js expert (part-time for compatibility)
- **DevOps Engineer**: 1 DevOps engineer (part-time for deployment)
- **QA Engineer**: 1 QA engineer (part-time for testing)

### Infrastructure
- **Development Environment**: Staging clusters for testing
- **CI/CD**: Enhanced build pipelines for Rust
- **Monitoring**: Extended observability for migration monitoring

## Timeline Summary

| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| 1. Foundation | 2 weeks | Project setup, config, logging |
| 2. HTTP Server | 2 weeks | Basic endpoints, middleware |
| 3. Database | 2 weeks | DB connectivity, migrations |
| 4. Key Management | 2 weeks | Cloud keystore integration |
| 5. BLS Crypto | 3 weeks | Threshold signatures |
| 6. PNP API | 2 weeks | Phone number privacy endpoints |
| 7. Domain API | 2 weeks | Domain-specific endpoints |
| 8. Integration Testing | 2 weeks | Comprehensive test suite |
| 9. Performance | 2 weeks | Optimization and benchmarking |
| 10. Deployment | 2 weeks | Production migration |
| **Total** | **21 weeks** | **Complete Rust signer** |

## Dependencies & Prerequisites

### External Dependencies
- [ ] celo-threshold-bls-rs library integration and testing
- [ ] Cloud provider SDK availability and stability
- [ ] Database migration tooling for SeaORM
- [ ] Container registry and deployment pipeline updates

### Internal Dependencies
- [ ] Staging environment availability
- [ ] Combiner service compatibility testing
- [ ] Security team review and approval
- [ ] Operations team training on Rust debugging

## File Structure for Rust Implementation

```
rust/signer/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs                     # Application entry point
│   ├── lib.rs                      # Library root
│   ├── config/
│   │   ├── mod.rs
│   │   └── signer_config.rs        # Configuration structs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── app.rs                  # Axum app setup
│   │   ├── middleware/             # HTTP middleware
│   │   └── handlers/               # HTTP handlers
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connection.rs           # DB connection management
│   │   ├── migrations/             # SeaORM migrations
│   │   └── models/                 # Database models
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── bls.rs                  # BLS signature operations
│   │   └── threshold.rs            # Threshold crypto
│   ├── key_management/
│   │   ├── mod.rs
│   │   ├── provider.rs             # Key provider trait
│   │   ├── azure.rs                # Azure Key Vault
│   │   ├── google.rs               # Google Secret Manager
│   │   ├── aws.rs                  # AWS Secrets Manager
│   │   └── mock.rs                 # Mock provider
│   ├── apis/
│   │   ├── mod.rs
│   │   ├── pnp/                    # PNP API implementation
│   │   │   ├── mod.rs
│   │   │   ├── handlers.rs
│   │   │   └── services.rs
│   │   └── domain/                 # Domain API implementation
│   │       ├── mod.rs
│   │       ├── handlers.rs
│   │       └── services.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── account.rs              # Account management
│   │   ├── quota.rs                # Quota management
│   │   └── request.rs              # Request tracking
│   ├── metrics/
│   │   ├── mod.rs
│   │   └── prometheus.rs           # Metrics collection
│   ├── tracing/
│   │   ├── mod.rs
│   │   └── setup.rs                # Tracing setup
│   └── error/
│       ├── mod.rs
│       └── odis_error.rs           # Error definitions
├── tests/
│   ├── integration/                # Integration tests
│   ├── e2e/                        # End-to-end tests
│   └── common/                     # Test utilities
├── benches/                        # Performance benchmarks
├── migrations/                     # Database migrations
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
└── scripts/
    ├── setup.sh
    └── benchmark.sh
```

## Next Steps

1. **Stakeholder Review**: Present this plan to engineering leadership and security team
2. **Resource Allocation**: Confirm team assignments and timeline
3. **Risk Assessment**: Review and approve risk mitigation strategies
4. **Proof of Concept**: Begin Phase 1 with a 2-week proof of concept
5. **Go/No-Go Decision**: Evaluate PoC results before full migration commitment

This plan provides a structured, low-risk approach to migrating the ODIS Signer to Rust while maintaining full compatibility and improving performance. Each phase has clear deliverables and tests to ensure quality and progress.
