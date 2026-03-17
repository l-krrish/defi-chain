use sha2::{Digest, Sha256};
use hex;

#[derive(Debug, Clone)]

pub struct Block{
    pub index : u64,
    pub timestamp : i64,
    pub prev_hash : String,
    pub nonce : u64,
    pub data : String,
    pub hash : String,
}

impl Block{
    pub fn new(index : u64, prev_hash : String, data: String) -> Self{
        Block{
            index,
            prev_hash : prev_hash.clone(),
            data : data.clone(),
            timestamp: 0,
            nonce: 0,
            hash: Block::compute_hash(index, 0, &prev_hash, 0,&data)
        }
    }

    pub fn genesis() -> Self{
            Block{
                index : 0,
                prev_hash: String::from("0"),
                data: String::from("Genesis Block"),
                timestamp: 0,
                nonce: 0,
                hash: String::from("0")
            }
    }

    pub fn compute_hash(index : u64, timestamp : i64, prev_hash : &str, nonce : u64, data : &str)-> String{
        let mut hasher = Sha256::new();
        hasher.update(index.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(prev_hash.as_bytes());
        hasher.update(nonce.to_string().as_bytes());
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn rehash(&mut self){
        self.hash=Block::compute_hash(self.index, self.timestamp, &self.prev_hash, self.nonce, &self.data);
    }
}