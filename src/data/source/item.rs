use druid::{Data, Lens};

#[derive(Clone, Data, PartialEq)]
pub enum Rarity {
	Standard,
	Common,
	Uncommon,
	Rare,
	VeryRare,
	Legendary
}

#[derive(Clone, Data, Lens)]
pub struct Money {
	cp: u64,
	sp: u64,
	ep: u64,
	gp: u64,
	pp: u64
}

impl Money {
	pub fn new(cp: u64, sp: u64, ep: u64, gp: u64, pp: u64) -> Self {
		Self { cp, sp, ep, gp, pp }
	}
}