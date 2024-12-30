use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a single transaction in the ledger
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier for the transaction
    pub id: Uuid,
    /// Timestamp when the transaction was created
    pub timestamp: DateTime<Utc>,
    /// Actual transaction data
    pub data: Vec<u8>,
    /// Quantum-resistant signature
    pub signature: Vec<u8>,
    /// Reference to the previous transaction
    pub previous_transaction: Option<Uuid>,
    /// Hardware security attestation data
    pub hardware_attestation: Option<HardwareAttestation>,
}

/// Hardware security attestation information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareAttestation {
    /// Timestamp of the attestation
    pub timestamp: DateTime<Utc>,
    /// Unique identifier for the hardware security module
    pub device_id: String,
    /// Raw attestation data from the hardware
    pub attestation_data: Vec<u8>,
}