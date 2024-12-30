use thiserror::Error;

/// Errors that can occur in the Tenzro Ledger system
#[derive(Error, Debug)]
pub enum LedgerError {
    /// Errors related to transaction operations
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    /// Errors related to cryptographic operations
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    /// Errors related to hardware security operations
    #[error("Hardware error: {0}")]
    HardwareError(String),
}

/// Result type for Tenzro Ledger operations
pub type Result<T> = std::result::Result<T, LedgerError>;