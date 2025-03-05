use bincode::serialize;
use sha2::{Sha256, Digest};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use hex;
use std::time::SystemTime;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    pub fn new_block(index: u32, data: String, prev_hash: String, nonce: u32) -> Result<Block> {
        let hash = String::new();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            nonce,
        };
        
        block.validate_block()?;
        Ok(block)
    }

    pub fn new_genesis_block(data: String) -> Block {
        Block::new_block(0, data, String::new(),0).unwrap()
    }

    fn validate_block(&mut self) -> Result<()> {
        while !self.validate()? {
            self.nonce += 1;
        }
        let hash = self.calculate_hash();
        self.hash = hash;
        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let hash = self.calculate_hash();
        Ok(&hash[0..4] == "0000")
    }
    
    fn hash_data(&self) -> Result<Vec<u8>> {
        let values = (self.index, self.timestamp, &self.data, &self.prev_hash, self.nonce);
        let input = serialize(&values)?;
        Ok(input)
    }
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = &self.hash_data().unwrap()[..];
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

}
