//! ODIS Signer - Rust Implementation
//!
//! This crate provides a Rust implementation of the ODIS (Oblivious Decentralized Identifier Service) Signer.
//! The signer participates in threshold cryptography protocols to provide privacy-preserving
//! identifier obfuscation services.

pub mod config;
pub mod error;

// Re-export commonly used types
pub use error::{OdisError, Result};

/// Application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
