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
