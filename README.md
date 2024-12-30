# Tenzro Ledger

A quantum-resistant distributed ledger system with hardware-rooted security. This implementation focuses on core ledger functionality with post-quantum cryptography and hardware security integration.

## Features

- **Quantum-resistant cryptography** using Dilithium signatures
- **Hardware security** integration ready (TPM/Secure Enclave)
- **Simple, efficient** single-chain architecture, which can be extended to support multi-chain as needed
- **Production-ready** error handling and logging
- **Comprehensive** test coverage

## Installation

### From crates.io

Add this to your `Cargo.toml`:
```toml
[dependencies]
tenzro-ledger = "0.1.0"
```

### From source

```bash
git clone https://github.com/tenzro/tenzro-ledger
cd tenzro-ledger
cargo build --release
```

## Usage

### Command Line Interface

The ledger provides both interactive and command-line modes.

#### Interactive Mode
```bash
cargo run

# Or after installation:
tenzro-ledger interactive
```

Available commands in interactive mode:
```
add <data>  - Add a new transaction with the specified data
list        - List all transactions
help        - Show help message
quit/exit   - Exit the program
```

Example session:
```bash
> add Hello, Tenzro!
Transaction added successfully. ID: 123e4567-e89b-12d3-a456-426614174000

> add This is transaction #2
Transaction added successfully. ID: 987fcdeb-51d3-12d3-a456-426614174000

> list
Transactions:
ID: 123e4567-e89b-12d3-a456-426614174000
  Timestamp: 2024-12-30 10:30:00 UTC
  Data: Hello, Tenzro!

ID: 987fcdeb-51d3-12d3-a456-426614174000
  Timestamp: 2024-12-30 10:30:05 UTC
  Data: This is transaction #2
  Previous Transaction: 123e4567-e89b-12d3-a456-426614174000
```

#### Direct Commands
```bash
# Add a transaction
tenzro-ledger add "Hello, Tenzro!"

# List all transactions
tenzro-ledger list

# Show help
tenzro-ledger --help
```

### As a Library

```rust
use tenzro_ledger::Chain;

fn main() -> tenzro_ledger::Result<()> {
    // Create a new chain
    let mut chain = Chain::new("My Ledger".to_string());

    // Add transactions
    let tx_id = chain.add_transaction(b"Hello, Tenzro!".to_vec())?;
    println!("Added transaction: {}", tx_id);

    // Verify a transaction
    let tx = chain.get_transaction(&tx_id).unwrap();
    assert!(chain.verify_transaction(tx)?);

    // Get all transactions
    let transactions = chain.get_all_transactions();
    for tx in transactions {
        println!("Transaction {} - Data: {}", 
            tx.id, 
            String::from_utf8_lossy(&tx.data));
    }

    Ok(())
}
```

## Architecture

### Quantum Resistance

Tenzro Ledger uses the Dilithium digital signature algorithm from the `pqcrypto` family of crates. Dilithium is a lattice-based signature scheme that is considered resistant to attacks from both classical and quantum computers.

### Hardware Integration

The system includes placeholders for hardware security integration through the `HardwareAttestation` interface. This allows system integrators to implement their own hardware security module (HSM), Trusted Platform Module (TPM), or secure enclave interactions.

Example hardware integration points:
- Transaction signing
- Key storage
- Hardware-based attestation
- Secure transaction validation

### Transaction Structure

Each transaction includes:
- Unique identifier (UUID)
- Timestamp
- Data payload
- Quantum-resistant signature
- Link to previous transaction
- Optional hardware attestation

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/feature`)
3. Commit your changes (`git commit -am 'Add feature'`)
4. Push to the branch (`git push origin feature/feature`)
5. Open a Pull Request

### Security Notes

This software includes post-quantum cryptographic algorithms but has not undergone a formal security audit. Users should:
- Perform their own security assessment
- Implement appropriate hardware security measures
- Monitor for cryptographic advances and updates
- Consider the specific threat model of their application

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Uses the `pqcrypto` family of crates for post-quantum cryptography
- Inspired by modern distributed ledger architectures
- Designed with future quantum computing threats in mind