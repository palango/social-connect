use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("Starting ODIS Signer (Rust implementation)");

    // Get version from environment or default
    let version = env::var("VERSION").unwrap_or_else(|_| "0.1.0".to_string());

    tracing::info!("ODIS Signer version: {}", version);
    tracing::info!("Hello from Rust! 🦀");

    // TODO: Initialize configuration system (Phase 1.2)
    // TODO: Set up HTTP server (Phase 2.1)
    // TODO: Connect to database (Phase 3.1)
    // TODO: Initialize key management (Phase 4.1)
    // TODO: Set up cryptography (Phase 5.1)

    Ok(())
}
