mod block;
mod blockchain;
use crate::block::Block;
use crate::blockchain::Blockchain;


fn main() {

    let mut blockchain = Blockchain::new();
    blockchain.add_block(Block::new_block(1, String::from("Block 1"), blockchain.blocks.last().unwrap().get_hash(),0).unwrap()).unwrap();
    blockchain.display();


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    #[test]
    fn test_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(Block::new_block(1, String::from("Block 1"), blockchain.blocks.last().unwrap().get_hash(),0).unwrap()).unwrap();
        blockchain.add_block(Block::new_block(2, String::from("Block 2"), blockchain.blocks.last().unwrap().get_hash(),0).unwrap()).unwrap();
        assert_eq!(blockchain.blocks.len(), 3);
        dbg!(&blockchain);
    }
    
}