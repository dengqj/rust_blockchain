use bincode::serialize;
use sha2::{Sha256, Digest};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use hex;
use std::time::SystemTime;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize}; 

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Block {
    index: u32,
    #[serde(with = "chrono::serde::ts_milliseconds")] 
    timestamp: DateTime<Utc>,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    //创建新区块
    pub fn new_block(index: u32, data: String, prev_hash: String, nonce: u32) -> Result<Block> {
        let hash = String::new();
        let timestamp: DateTime<Utc> = Utc::now(); 
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
    //创建创世区块
    pub fn new_genesis_block(data: String) -> Block {
        Block::new_block(0, data, String::new(),0).unwrap()
    }
    //验证区块
    fn validate_block(&mut self) -> Result<()> {
        while !self.validate()? {
            self.nonce += 1;
        }
        let hash = self.calculate_hash();
        self.hash = hash;
        Ok(())
    }
    //验证工作量证明
    fn validate(&self) -> Result<bool> {
        let hash = self.calculate_hash();
        Ok(&hash[0..4] == "0000")
    }
    //哈希数据处理
    fn hash_data(&self) -> Result<Vec<u8>> {
        let values = (self.index, self.timestamp.timestamp_millis(), &self.data, &self.prev_hash, self.nonce);
        let input = serialize(&values)?;
        Ok(input)
    }
    //哈希计算
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = &self.hash_data().unwrap()[..];
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }
    //提取当前区块哈希
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    //提取前一个区块哈希
    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }
    //提取当前区块索引
    pub fn get_index(&self) -> u64 {
        self.index as u64
    }
    //提取当前区块数据
    pub fn get_data(&self) -> String {
        self.data.clone()
    }
    //提取当前区块创建时间
    pub fn get_timestamp_string(&self) -> String {
        self.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
    

}
