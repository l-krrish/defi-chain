pub struct Pool{
    reserve_a: u64,
    reserve_b: u64,
    lp_supply: u64,
}

impl Pool{
    pub fn new(reserve_a: u64, reserve_b: u64) -> Self{
        Pool{
            reserve_a,
            reserve_b,
            lp_supply: 0,
        }
    }

    pub fn add_liquidity(&mut self, amount_a: u64, amount_b: u64) -> u64{

        let lp_minted = amount_a.min(amount_b);
        self.reserve_a += amount_a;
        self.reserve_b += amount_b;
        self.lp_supply += lp_minted;
        lp_minted
    }
}