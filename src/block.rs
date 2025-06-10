use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash
        }
    }

    pub fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, previous_hash, timestamp, data);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}