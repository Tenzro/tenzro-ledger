//! # Tenzro Ledger
//! 
//! A quantum-resistant distributed ledger with hardware-rooted security, zero fees, and instant finality.
//! 
//! ## Features
//! 
//! * Quantum-resistant cryptography using Dilithium signatures
//! * Hardware security integration ready (TPM/Secure Enclave)
//! * Zero network fees and instant finality
//! * No validators or consensus mechanisms required
//! * Unlimited scalability through hardware-based validation
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use tenzro_ledger::Chain;
//! 
//! // Create a new chain
//! let mut chain = Chain::new("My Ledger".to_string());
//! 
//! // Add a transaction
//! let tx_id = chain.add_transaction(b"Hello, quantum world!".to_vec())?;
//! 
//! // Verify the transaction
//! let tx = chain.get_transaction(&tx_id).unwrap();
//! assert!(chain.verify_transaction(tx)?);
//! # Ok::<(), tenzro_ledger::error::LedgerError>(())
//! ```
//! 
//! ## Architecture
//! 
//! Tenzro Ledger uses post-quantum cryptographic algorithms from the `pqcrypto` family of crates,
//! specifically Dilithium for digital signatures. The system is designed to be easily integrated
//! with hardware security modules through a flexible attestation interface.
//! 
//! Instead of relying on network validators and consensus mechanisms, Tenzro Ledger uses
//! hardware-based security features for transaction validation, enabling instant finality
//! without fees or network latency.

pub mod chain;
pub mod error;
pub mod types;

pub use chain::Chain;
pub use error::{Result, LedgerError};
pub use types::{Transaction, HardwareAttestation};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");