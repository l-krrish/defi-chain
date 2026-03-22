#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stake {
	pub owner: String,
	pub amount: u64,
	pub lock_block: u64,
	pub lock_duration: u64,
}

#[derive(Debug, Default, Clone)]
pub struct StakingPool {
	pub stakes: Vec<Stake>,
	pub reward_rate: u64,
}

impl StakingPool {
	pub fn new(reward_rate: u64) -> Self {
		Self {
			stakes: Vec::new(),
			reward_rate,
		}
	}

	pub fn lock_tokens(
		&mut self,
		owner: String,
		amount: u64,
		current_block: u64,
		duration: u64,
	) {
		self.stakes.push(Stake {
			owner,
			amount,
			lock_block: current_block,
			lock_duration: duration,
		});
	}

	pub fn compute_yield(&self, owner: &str, current_block: u64) -> u64 {
		let stake = match self.stakes.iter().find(|stake| stake.owner == owner) {
			Some(stake) => stake,
			None => return 0,
		};

		let blocks_staked = current_block.saturating_sub(stake.lock_block);
		stake.amount.saturating_mul(self.reward_rate).saturating_mul(blocks_staked)
	}
}
