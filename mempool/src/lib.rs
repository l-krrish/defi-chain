use std::collections::BTreeMap;

pub struct PendingTx{
    id: String,
    fee : u64,
    data : String,
    submitted_at: i64,
}

pub struct Mempool{
    inner: BTreeMap<u64, Vec<PendingTx>>,
    ttl_secs: i64
}

impl Mempool{
    pub fn new(ttl_secs: i64) -> Self{
        Mempool{
            inner: BTreeMap::new(),
            ttl_secs,
        }
    }

    pub fn insert(&mut self, tx: PendingTx){
        let fee=tx.fee;
        self.inner.entry(fee).or_default().push(tx);
    }
}
