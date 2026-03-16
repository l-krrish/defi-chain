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
            prev_hash,
            data,
            timestamp: 0,
            nonce: 0,
            hash: String::new()
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
}