use crate::block::Block;

pub fn mine(block: &mut Block, difficulty: usize){
    let target = "0".repeat(difficulty);
    loop {
        block.rehash();
        if block.hash.starts_with(&target) {
            break;
        }
        block.nonce += 1;
    }
}
