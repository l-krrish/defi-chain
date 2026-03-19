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

    pub fn swap_a_for_b(&mut self, amount_a: u64) -> u64{
        let k = self.reserve_a * self.reserve_b;
        let amount_a_with_fee = amount_a * 997 / 1000;
        let new_reserve_a = self.reserve_a + amount_a_with_fee;
        let new_reserve_b = k / new_reserve_a;
        let output = self.reserve_b - new_reserve_b;
        self.reserve_a = new_reserve_a;
        self.reserve_b = new_reserve_b;
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut pool = Pool::new(100, 100_000);
        let output = pool.swap_a_for_b(10);
        println!("output: {}", output);
        assert!(output > 8_000 && output < 9_000);

    }
}