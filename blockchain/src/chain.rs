use crate::block::Block;
use crate::miner::mine;
use crate::miner;

pub struct Chain{
    pub blocks : Vec<Block>,
    difficulty : usize,
}

impl Chain{
    pub fn new(difficulty : usize) -> Self{
        let mut genesis = Block::genesis();
        mine(&mut genesis, difficulty);
        Chain{
            blocks : vec![genesis],
            difficulty,
        }
    }

    pub fn tip(&self)-> &Block{
            return self.blocks.last().expect("chain always has genesis");
    }

    pub fn add_block(&mut self, data: String) -> &Block {
    let prev_hash = self.tip().hash.clone();
    let index = self.blocks.len() as u64;
    let mut block = Block::new(index, prev_hash, data);
    miner::mine(&mut block, self.difficulty);
    self.blocks.push(block);
    self.tip()
}
}