pub struct Position{
    owner: String,
    collateral: u64,
    borrowed: u64,

}

pub struct LendingPool{
    positions: Vec<Position>,
    collateral_reserve: u64,
    borrow_reserve: u64,
}

impl LendingPool{
    pub fn new(collateral_reserve: u64, borrow_reserve: u64) -> Self{
        LendingPool{
            positions: Vec::new(),
            collateral_reserve,
            borrow_reserve,
        }
    }

    pub fn deposit(&mut self, owner: String, collateral: u64){
        self.positions.push(Position { owner, collateral, borrowed: 0 });
        self.collateral_reserve += collateral;
    }

    pub fn borrow(&mut self, owner: &str, amount: u64) -> bool {
    if let Some(position) = self.positions.iter_mut().find(|p| p.owner == owner) {
        let max_borrow = position.collateral * 75 / 100;
        if position.borrowed + amount <= max_borrow {
            position.borrowed += amount;
            self.borrow_reserve -= amount;
            true
        } else {
            false
        }
    } else {
        false
    }
}

    pub fn health_factor(&self, owner: &str) -> Option<u64>{
        if let Some(position) = self.positions.iter().find(|p| p.owner == owner) {
            if position.borrowed == 0 {
                None
            } else {
                let health_factor = position.collateral * 75 / 100 / position.borrowed;
                Some(health_factor)
            }
        } else {
            None
        }
    }

    pub fn liquidate(&mut self, owner: &str) -> bool {
    let hf = self.health_factor(owner);
    if let Some(health_factor) = hf {
        if health_factor >= 1 {
            return false;
        }
        if let Some(position) = self.positions.iter_mut().find(|p| p.owner == owner) {
            position.borrowed = 0;
            position.collateral = 0;
            return true;
        }
    }
    false
}
}