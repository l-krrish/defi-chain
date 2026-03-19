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

   pub fn pop_batch(&mut self, n: usize) -> Vec<PendingTx> {
    let mut batch = Vec::new();
    while batch.len() < n {
        if let Some((&fee, _)) = self.inner.iter().next_back() {
            let txs = self.inner.get_mut(&fee).unwrap();
            batch.push(txs.remove(0));
            if txs.is_empty() {
                self.inner.remove(&fee);
            }
        } else {
            break;
        }
    }
    batch
}

pub fn prune_expired(&mut self, now: i64){
    for txs in self.inner.values_mut() {
        txs.retain(|tx| now - tx.submitted_at < self.ttl_secs);
    }
    self.inner.retain(|_, v| !v.is_empty());
}

}
