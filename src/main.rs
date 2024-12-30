use tenzro_ledger::{Chain, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive mode
    Interactive,
    /// Add a new transaction
    Add {
        /// The data to add to the transaction
        data: String,
    },
    /// List all transactions
    List,
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    let cli = Cli::parse();
    let mut chain = Chain::new("Main Chain".to_string());

    match cli.command {
        Some(Commands::Interactive) => interactive_mode(&mut chain),
        Some(Commands::Add { data }) => {
            let tx_id = chain.add_transaction(data.as_bytes().to_vec())?;
            println!("Transaction added successfully. ID: {}", tx_id);
            Ok(())
        }
        Some(Commands::List) => {
            list_transactions(&chain);
            Ok(())
        }
        None => interactive_mode(&mut chain),
    }
}

fn interactive_mode(chain: &mut Chain) -> Result<()> {
    println!("Tenzro Ledger v{}", tenzro_ledger::VERSION);
    println!("Enter 'help' for available commands");

    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        match command {
            "help" => print_help(),
            "quit" | "exit" => break,
            _ if command.starts_with("add ") => {
                let data = command[4..].as_bytes().to_vec();
                match chain.add_transaction(data) {
                    Ok(tx_id) => println!("Transaction added successfully. ID: {}", tx_id),
                    Err(e) => println!("Error adding transaction: {}", e),
                }
            }
            "list" => list_transactions(chain),
            _ => println!("Unknown command. Type 'help' for available commands"),
        }
    }

    Ok(())
}

fn list_transactions(chain: &Chain) {
    let transactions = chain.get_all_transactions();
    if transactions.is_empty() {
        println!("No transactions found");
        return;
    }

    println!("Transactions:");
    for tx in transactions {
        println!("ID: {}", tx.id);
        println!("  Timestamp: {}", tx.timestamp);
        println!("  Data: {}", String::from_utf8_lossy(&tx.data));
        if let Some(prev) = tx.previous_transaction {
            println!("  Previous Transaction: {}", prev);
        }
        println!();
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  add <data>  - Add a new transaction with the specified data");
    println!("  list        - List all transactions");
    println!("  help        - Show this help message");
    println!("  quit/exit   - Exit the program");
}