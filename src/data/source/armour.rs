use druid::{Data, Lens};

use crate::data::character_state::AbilityScoreType;

use super::item::{Rarity, Money};

pub enum ArmourCategory {
	NoArmour,
	LightArmour,
	HeavyArmour,
	Shield,
	Spell,
	ClassFeature
}

pub struct Armour {
	name: String,
	category: ArmourCategory,
	rarity: Rarity,
	base_ac: u8,
	/// A list of ability scores who's modifiers are added to AC
	plus_mod: im::Vector<AbilityScoreType>,
	/// Max that can be gained from plus_mod
	plus_mod_max: u8,
	plus_flat_mod: u8,
	cost: Money,
	/// (lbs)
	weight: u64,
	stealth_disadvantage: bool
}