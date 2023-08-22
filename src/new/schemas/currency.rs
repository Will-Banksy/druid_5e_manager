use druid::Data;

use std::fmt::Write;

#[derive(Clone, Data)]
pub struct Currency {
	cp: u64,
	sp: u64,
	ep: u64,
	gp: u64,
	pp: u64
}

impl Currency {
	pub fn new(cp: u64, sp: u64, ep: u64, gp: u64, pp: u64) -> Self {
		Self { cp, sp, ep, gp, pp }
	}

	/// Parses an amount of currency from an ASCII string, e.g. "100 gp"
	pub fn parse(currency_str: &str) -> Currency { // TODO: Probably should make this better... Maybe return some sort of null value if the currency_str doesn't conform to expectations
		let currency_chars: Vec<char> = currency_str.chars().collect();

		let mut currency_val: u64 = 0;
		let mut currency = Currency::new(0, 0, 0, 0, 0);

		let mut i: usize = 0;

		while i < currency_chars.len() {
			if currency_chars[i].is_ascii_digit() {
				let mut sb = String::new();
				while currency_chars[i].is_ascii_digit() {
					write!(sb, "{}", currency_chars[i]).unwrap();
					i += 1;
				}
				currency_val = sb.parse::<u64>().unwrap();
			}
			if currency_chars[i].is_ascii_alphabetic() {
				if currency_chars[i] == 'c' {
					currency.cp = currency_val;
				} else if currency_chars[i] == 's' {
					currency.sp = currency_val;
				} else if currency_chars[i] == 'e' {
					currency.ep = currency_val;
				} else if currency_chars[i] == 'g' {
					currency.gp = currency_val;
				} else if currency_chars[i] == 'p' {
					currency.pp = currency_val;
				}
				i += 1;
			}
			i += 1;
		}

		currency
	}
}