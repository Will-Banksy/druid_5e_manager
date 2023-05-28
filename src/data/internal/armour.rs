use druid::Data;

use crate::data::character_state::AbilityScoreType;

use super::item::{Rarity, Money};

#[derive(Data, Clone)]
pub enum ArmourCategory {
	NoArmour,
	LightArmour,
	HeavyArmour,
	Shield,
	Spell,
	ClassFeature,
	Custom(String)
}

#[derive(Data, Clone)]
pub struct Armour {
	pub name: String,
	pub category: ArmourCategory,
	pub rarity: Rarity,
	pub base_ac: u8,
	/// A list of ability scores who's modifiers are added to AC
	pub plus_mod: im::Vector<AbilityScoreType>,
	/// Max that can be gained from plus_mod
	pub plus_mod_max: Option<u8>,
	pub plus_flat_mod: u8,
	pub cost: Option<Money>,
	/// (lbs)
	pub weight: u64,
	pub stealth_disadvantage: bool
}