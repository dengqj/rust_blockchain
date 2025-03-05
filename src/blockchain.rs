use bincode::serialize;
use sha2::{Sha256, Digest};
use crate::block::*;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        blockchain.blocks.push(Block::new_block(0, String::from("Genesis Block"), String::from("0"),0).unwrap());
        blockchain
    }

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
}