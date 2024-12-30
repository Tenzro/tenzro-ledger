//! Tenzro Ledger: A quantum-resistant distributed ledger system
//! 
//! This crate provides a simple, efficient implementation of a distributed ledger
//! with quantum-resistant cryptography and hardware security integration capabilities.

pub mod chain;
pub mod error;
pub mod types;

// Re-export main types for convenience
pub use chain::Chain;
pub use error::{Result, LedgerError};
pub use types::{Transaction, HardwareAttestation};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");