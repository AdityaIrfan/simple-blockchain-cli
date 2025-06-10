use clap::{Parser, Subcommand};
use crate::storage::{load_chain, save_chain};

mod block;
mod blockchain;
mod storage;

/// Simple Blockchain CLI
#[derive(Parser)]
#[command(author, version, about)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a block to the blockchain
    Add {data: String},
    /// Print all blocks in the chain
    Print,
    /// Validate the chain
    Validate,
}

// Shared blockchain state

fn main() {
    let cli: CLI = CLI::parse();
    let mut blockchain = load_chain();

    match &cli.command {
        Command::Add { data } => {
            blockchain.add_block(data.clone());
            save_chain(&blockchain).expect("Failed to save blockchain");
            println!("✅ Block added!");
        }
        Command::Print => {
            blockchain.print_chain();
        }
        Command::Validate => {
            if blockchain.is_valid() {
                println!("✅ Blockchain is valid");
            } else {
                println!("❌ Blockchain is NOT valid");
            }
        }
    }
}
