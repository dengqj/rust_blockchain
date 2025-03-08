use bincode::serialize;
use sha2::{Sha256, Digest};
use crate::block::*;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    //创建新区块链
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        blockchain.blocks.push(Block::new_genesis_block(String::from("Genesis Block")));
        blockchain
    }
    //当前区块链上添加新区块
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let new_hash = block.calculate_hash();
        let get_hash = block.get_hash(); 
        let prev_hash = block.get_prev_hash();
        let last_hash = self.blocks.last().unwrap().get_hash();

        if get_hash == new_hash && prev_hash == last_hash {
            self.blocks.push(block);  
        }

        
        Ok(())
        
    }
    //打印区块链
    pub fn display(&self) {
        for block in &self.blocks {
            println!("Block Index: {}", block.get_index());
            println!("Block Timestamp: {}", block.get_timestamp_string());
            println!("Block Data: {}", block.get_data());
            println!("Block Prev Hash: {}", block.get_prev_hash());
            println!("Block Hash: {}", block.get_hash());
            println!("--------------------");
        }
    }    
}
