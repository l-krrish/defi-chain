use sled;

pub struct DB{
    tree: sled::Db,
}

impl DB{
    pub fn open(path: &str) -> Self{
        let tree = sled::open(path).expect("failed to open db");
        Self{tree}
    }

    pub fn save(&self, key: &str, value: &[u8]){
        self.tree.insert(key, value).expect("failed to save data");
    }

    pub fn load(&self, key: &str) -> Option<Vec<u8>>{
        self.tree.get(key).expect("failed to load data").map(|ivec| ivec.to_vec())
    }
}