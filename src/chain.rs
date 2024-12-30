use crate::{
    error::{Result, LedgerError},
    types::{Transaction, HardwareAttestation},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{DetachedSignature, PublicKey, SecretKey};
use log::{info, debug, warn};

/// Quantum-resistant key pair management
#[derive(Clone, Debug)]
struct QuantumKeys {
    public_key: Vec<u8>,
    #[allow(dead_code)]
    secret_key: Vec<u8>,
}

/// Main chain structure for the ledger
#[derive(Clone, Debug)]
pub struct Chain {
    /// Unique identifier for the chain
    pub id: Uuid,
    /// Human-readable name for the chain
    pub name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Transaction storage
    transactions: HashMap<Uuid, Transaction>,
    /// Quantum-resistant cryptographic keys
    quantum_keypair: Option<QuantumKeys>,
}

impl Chain {
    /// Creates a new chain with the given name
    pub fn new(name: String) -> Self {
        info!("Creating new chain: {}", name);
        
        // Generate quantum-resistant keypair
        let (pk, sk) = dilithium2::keypair();
        debug!("Generated quantum-resistant keypair");
        
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: Utc::now(),
            transactions: HashMap::new(),
            quantum_keypair: Some(QuantumKeys {
                public_key: pk.as_bytes().to_vec(),
                secret_key: sk.as_bytes().to_vec(),
            }),
        }
    }

    /// Adds a new transaction to the chain
    pub fn add_transaction(&mut self, data: Vec<u8>) -> Result<Uuid> {
        let quantum_keys = self.quantum_keypair.as_ref()
            .ok_or_else(|| LedgerError::CryptoError("No quantum keys available".to_string()))?;

        debug!("Creating new transaction with {} bytes of data", data.len());
        
        // Create and sign transaction
        let transaction = self.create_transaction(data, quantum_keys)?;
        let transaction_id = transaction.id;
        
        // Add to chain
        self.transactions.insert(transaction_id, transaction);
        info!("Added transaction: {}", transaction_id);
        
        Ok(transaction_id)
    }

    fn create_transaction(&self, data: Vec<u8>, keys: &QuantumKeys) -> Result<Transaction> {
        // Sign data using quantum-resistant algorithm
        let secret_key = dilithium2::SecretKey::from_bytes(&keys.secret_key)
            .map_err(|e| LedgerError::CryptoError(e.to_string()))?;
            
        let signature = dilithium2::detached_sign(&data, &secret_key);

        // Get previous transaction ID
        let previous_transaction = self.transactions.values()
            .max_by_key(|tx| tx.timestamp)
            .map(|tx| tx.id);

        // Generate hardware attestation
        let hardware_attestation = self.generate_hardware_attestation()?;

        Ok(Transaction {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            data,
            signature: signature.as_bytes().to_vec(),
            previous_transaction,
            hardware_attestation: Some(hardware_attestation),
        })
    }

    /// Verifies a transaction's signature and attestation
    pub fn verify_transaction(&self, transaction: &Transaction) -> Result<bool> {
        let quantum_keys = self.quantum_keypair.as_ref()
            .ok_or_else(|| LedgerError::CryptoError("No quantum keys available".to_string()))?;

        debug!("Verifying transaction: {}", transaction.id);

        // Verify quantum signature
        let public_key = dilithium2::PublicKey::from_bytes(&quantum_keys.public_key)
            .map_err(|e| LedgerError::CryptoError(e.to_string()))?;
            
        let signature = dilithium2::DetachedSignature::from_bytes(&transaction.signature)
            .map_err(|e| LedgerError::CryptoError(e.to_string()))?;

        let is_valid = dilithium2::verify_detached_signature(
            &signature,
            &transaction.data,
            &public_key
        ).is_ok();

        if !is_valid {
            warn!("Invalid signature for transaction: {}", transaction.id);
            return Ok(false);
        }

        // Verify hardware attestation if present
        if let Some(attestation) = &transaction.hardware_attestation {
            self.verify_hardware_attestation(attestation)?;
        }

        Ok(true)
    }

    /// Retrieves a transaction by ID
    pub fn get_transaction(&self, id: &Uuid) -> Option<&Transaction> {
        self.transactions.get(id)
    }

    /// Returns all transactions in the chain
    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    /// Gets transactions since a specific timestamp
    pub fn get_transactions_since(&self, since: DateTime<Utc>) -> Vec<&Transaction> {
        self.transactions.values()
            .filter(|tx| tx.timestamp >= since)
            .collect()
    }

    // Placeholder for hardware security integration
    fn generate_hardware_attestation(&self) -> Result<HardwareAttestation> {
        debug!("Generating hardware attestation (placeholder)");
        // In a real implementation, this would interact with secure hardware
        Ok(HardwareAttestation {
            timestamp: Utc::now(),
            device_id: "SIMULATED-TPM-01".to_string(),
            attestation_data: vec![0, 1, 2, 3], // Placeholder
        })
    }

    fn verify_hardware_attestation(&self, attestation: &HardwareAttestation) -> Result<()> {
        debug!("Verifying hardware attestation for device: {}", attestation.device_id);
        // In a real implementation, this would verify hardware signatures
        // and attestation data against the TPM/secure enclave
        if attestation.device_id.starts_with("SIMULATED-TPM-") {
            Ok(())
        } else {
            warn!("Invalid hardware attestation from device: {}", attestation.device_id);
            Err(LedgerError::HardwareError("Invalid hardware attestation".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_chain_creation() {
        let chain = Chain::new("Test Chain".to_string());
        assert_eq!(chain.name, "Test Chain");
        assert!(chain.quantum_keypair.is_some());
    }

    #[test]
    fn test_transaction_operations() -> Result<()> {
        let mut chain = Chain::new("Test Chain".to_string());

        // Add a transaction
        let tx_id = chain.add_transaction(b"test data".to_vec())?;
        
        // Verify the transaction exists
        let tx = chain.get_transaction(&tx_id).unwrap();
        assert_eq!(tx.data, b"test data".to_vec());

        // Verify the transaction signature
        assert!(chain.verify_transaction(tx)?);

        Ok(())
    }

    #[test]
    fn test_transaction_sequence() -> Result<()> {
        let mut chain = Chain::new("Test Chain".to_string());

        // Add multiple transactions
        let tx1_id = chain.add_transaction(b"first".to_vec())?;
        let tx2_id = chain.add_transaction(b"second".to_vec())?;

        // Verify sequence
        let tx2 = chain.get_transaction(&tx2_id).unwrap();
        assert_eq!(tx2.previous_transaction, Some(tx1_id));

        Ok(())
    }

    #[test]
    fn test_transaction_retrieval() -> Result<()> {
        let mut chain = Chain::new("Test Chain".to_string());
        
        // Add transactions with different timestamps
        let now = Utc::now();
        let tx1_id = chain.add_transaction(b"old".to_vec())?;
        let tx2_id = chain.add_transaction(b"new".to_vec())?;

        // Test get_all_transactions
        let all_txs = chain.get_all_transactions();
        assert_eq!(all_txs.len(), 2);

        // Test get_transactions_since
        let recent_txs = chain.get_transactions_since(now);
        assert_eq!(recent_txs.len(), 2);
        
        // Test get specific transaction
        let tx1 = chain.get_transaction(&tx1_id).unwrap();
        assert_eq!(tx1.data, b"old".to_vec());
        
        let tx2 = chain.get_transaction(&tx2_id).unwrap();
        assert_eq!(tx2.data, b"new".to_vec());

        Ok(())
    }

    #[test]
    fn test_hardware_attestation() -> Result<()> {
        let mut chain = Chain::new("Test Chain".to_string());
        
        // Add a transaction (which includes hardware attestation)
        let tx_id = chain.add_transaction(b"test data".to_vec())?;
        let tx = chain.get_transaction(&tx_id).unwrap();
        
        // Verify hardware attestation exists
        assert!(tx.hardware_attestation.is_some());
        
        // Verify attestation content
        if let Some(attestation) = &tx.hardware_attestation {
            assert!(attestation.device_id.starts_with("SIMULATED-TPM-"));
            assert!(!attestation.attestation_data.is_empty());
        }

        Ok(())
    }
}