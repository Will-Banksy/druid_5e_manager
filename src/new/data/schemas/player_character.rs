use druid::{Lens, Data};

use crate::new::data::{identifer::*, attribute::ValueAttribute};

// use super::{stat::Stat, equipment::Equipment};

#[derive(Clone, Data, Lens)]
pub struct PlayerCharacter {
	pub name: String,
	pub stats: im::HashMap<Identifier, ValueAttribute>,
	// pub equipment: im::Vector<Equipment>
}

impl PlayerCharacter {
	pub fn new_5e() -> Self {
		PlayerCharacter {
			name: String::new(),
			stats: im::hashmap! {
				ID_5E_STAT_ABILITY_SCORE_STRENGTH => 8u64.into(),
				ID_5E_STAT_ABILITY_SCORE_DEXTERITY => 8u64.into(),
				ID_5E_STAT_ABILITY_SCORE_CONSTITUTION => 8u64.into(),
				ID_5E_STAT_ABILITY_SCORE_INTELLIGENCE => 8u64.into(),
				// ID_5E_STAT_ABILITY_SCORE_WISDOM => 8u64.into(),
				ID_5E_STAT_ABILITY_SCORE_CHARISMA => 8u64.into(),
			}
		}
	}
}

// PlayerCharacter design
// HashMap of stats, which are simply an enum of basic numeric values indexed by Identifier
// HashMap of attributes
