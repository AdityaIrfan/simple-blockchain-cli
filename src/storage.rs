use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::blockchain::Blockchain;
use serde_json;
use crate::block::Block;

const FILE_PATH: &str = "blockchain.json";

pub fn load_chain() -> Blockchain {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Failed to read blockchain file");
        let chain:Vec<Block> = serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("⚠️ Failed to parse blockchain. Creating new one.");
            vec![Block::new(0, "Genesis Block".into(), "0".into())]
        });
        Blockchain{ chain }
    } else {
        Blockchain::new()
    }
}

pub fn save_chain(chain: &Blockchain) -> io::Result<()> {
    let data = serde_json::to_string_pretty(&chain.chain)?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}