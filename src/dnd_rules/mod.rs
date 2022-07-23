pub fn proficiency_bonus_for(level: u8) -> u8 {
	(level as f32 / 4.0).ceil() as u8 + 1
}

pub fn modifier(score: u8, proficiency: bool, expertise: bool, proficiency_bonus: u8) -> i16 {
	((score as f32 - 10.0) / 2.0).floor() as i16 + if proficiency { if expertise { proficiency_bonus + proficiency_bonus } else { proficiency_bonus } } else { 0 } as i16
}