use druid::{Data, Lens};
use std::fmt::Write;

#[derive(Clone, Data, PartialEq)]
pub enum Rarity {
	Standard,
	Common,
	Uncommon,
	Rare,
	VeryRare,
	Legendary,
	Custom(String)
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

	/// Parses an amount of money from an ASCII string, e.g. "100 gp"
	pub fn parse(money_str: &str) -> Money { // TODO: Probably should make this better... Maybe return some sort of null value if the money_str doesn't conform to expectations
		let money_chars: Vec<char> = money_str.chars().collect();

		let mut money_val: u64 = 0;
		let mut money = Money::new(0, 0, 0, 0, 0);

		let mut i: usize = 0;

		while i < money_chars.len() {
			if money_chars[i].is_ascii_digit() {
				let mut sb = String::new();
				while money_chars[i].is_ascii_digit() {
					write!(sb, "{}", money_chars[i]).unwrap();
					i += 1;
				}
				money_val = sb.parse::<u64>().unwrap();
			}
			if money_chars[i].is_ascii_alphabetic() {
				if money_chars[i] == 'c' {
					money.cp = money_val;
				} else if money_chars[i] == 's' {
					money.sp = money_val;
				} else if money_chars[i] == 'e' {
					money.ep = money_val;
				} else if money_chars[i] == 'g' {
					money.gp = money_val;
				} else if money_chars[i] == 'p' {
					money.pp = money_val;
				}
				i += 1;
			}
		}

		money
	}
}