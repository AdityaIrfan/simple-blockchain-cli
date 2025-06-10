use serde::Deserialize;
use crate::block::Block;

#[derive(Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".into(), "0".into());
        Blockchain{
            chain: vec![genesis]
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i-1];

            if current.previous_hash != previous.hash {
                return false
            }

            let recalculated_hash = Block::calculate_hash(
                current.index,
                &current.timestamp,
                &current.data,
                &current.previous_hash
            );

            return current.hash == recalculated_hash
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}\n", block);
        }
    }
}