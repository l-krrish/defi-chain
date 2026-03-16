#[derive(Debug, Clone)]

pub struct Block{
    pub index : u64,
    pub timestamp : i64,
    pub prev_hash : String,
    pub nonce : u64,
    pub data : String,
    pub hash : String,
}