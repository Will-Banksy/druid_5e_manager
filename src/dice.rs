#[derive(PartialEq, Debug)]
pub struct DiceExpr {
	terms: im::Vector<DiceTerm>,
	expr_str: String
}

#[derive(PartialEq, Debug, Clone)]
enum DiceTerm {
	Roll { num_rolls: u32, dice_sides: u32 },
	Variable(String),
	Constant(i32)
}

enum ParseSymbol { // NOTE Is this necessary
	Number(u32),
	Text(String),
}

#[cfg(test)]
#[test]
fn test_dice_expr_parse() {
	const EXPR: &'static str = "a d102";
	let expected: DiceExpr = DiceExpr {
		terms: im::vector![
			DiceTerm::Roll { num_rolls: 1, dice_sides: 102 }
		],
		expr_str: EXPR.to_string()
	};

	let parsed = DiceExpr::parse(EXPR).unwrap();
	assert_eq!(expected, parsed);
}

impl DiceExpr {
	pub fn parse(expr: &str) -> Result<Self, &'static str> { // TODO: Think of the best way to do this (Probably a simple lexer and then Shunting Yard?)
		let exprch: Vec<char> = expr.chars().collect();
		let mut curr_term: Vec<char> = Vec::with_capacity(exprch.len());
		let mut dice_terms: Vec<DiceTerm> = Vec::new();
		for mut i in 0..exprch.len() {
			if exprch[i].is_ascii_digit() {
				let num_len = exprch[i..].iter().position(|elem| !elem.is_ascii_digit());

			}

			if exprch[i] == 'd' && i != exprch.len() - 1 && !exprch[i + 1].is_alphabetic() {
				let n: u32 = { // The number of rolls
					if curr_term.is_empty() {
						1
					} else {
						match curr_term.iter().collect::<String>().parse() {
							Ok(n) => {
								curr_term.clear();
								n
							},
							Err(_) => return Err("The number prefixing 'd' was unparseable in dice expression")
						}
					}
				};

				i += 1;
				while i < exprch.len() && exprch[i].is_ascii_digit() { // TODO!
					curr_term.push(exprch[i]);

					i += 1;
				}

				let sides = {
					if curr_term.is_empty() {
						1
					} else {
						match curr_term.iter().collect::<String>().parse() {
							Ok(n) => {
								curr_term.clear();
								n
							},
							Err(_) => return Err("The number that is a suffix of 'd' was unparseable in dice expression")
						}
					}
				};

				dice_terms.push(DiceTerm::Roll { num_rolls: n, dice_sides: sides });

			} else if exprch[i].is_alphabetic() {
				// Detect variables

			} else {
				curr_term.push(exprch[i]);
			}
		}

		Ok(DiceExpr {
			terms: dice_terms.into_iter().collect(),
			expr_str: expr.to_string()
		})
	}
}