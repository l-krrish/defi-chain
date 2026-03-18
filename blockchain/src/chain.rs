use crate::block::Block;
use crate::miner::mine;

pub struct Chain{
    pub blocks : Vec<Block>,
    difficulty : usize,
}

impl Chain{
    pub fn new(difficulty : usize) -> Self{
        let genesis = Block::genesis();
        mine(&mut genesis, difficulty);
        Chain{
            blocks : vec![genesis],
            difficulty,
        }
    }

    pub fn tip(&self)-> &Block{
            return self.blocks.last().expect("chain always has genesis");
    }
}