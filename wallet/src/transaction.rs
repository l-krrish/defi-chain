pub struct Transaction{
    pub from : String,
    pub to : String,
    pub amount : u64,
    pub fee : u64,
    pub nonce : u64,
    pub signature : String,
}

impl Transaction{
    pub fn new(from : String, to : String, amount : u64, fee : u64, nonce : u64) -> Self{
        return Transaction{
             from,
             to,
             amount,
             fee,
             nonce,
             signature : String::new(),
        }
    }

    pub fn signing_bytes(&self) -> Vec<u8>{
        format!("{}{}{}{}{}", self.from, self.to, self.amount, self.fee, self.nonce).into_bytes()
    }

}
