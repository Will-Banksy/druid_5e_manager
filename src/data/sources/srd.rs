mod source_data {
    use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct SourceArmour {
		pub name: String,
		pub category: String,
		pub rarity: Option<String>,
		pub base_ac: u8,
		#[serde(default)]
		pub plus_str_mod: bool,
		#[serde(default)]
		pub plus_dex_mod: bool,
		#[serde(default)]
		pub plus_con_mod: bool,
		#[serde(default)]
		pub plus_int_mod: bool,
		#[serde(default)]
		pub plus_wis_mod: bool,
		#[serde(default)]
		pub plus_cha_mod: bool,
		/// Max that can be gained from plus_mod
		pub plus_max: Option<u8>,
		#[serde(default)]
		pub plus_flat_mod: u8,
		pub cost: Option<String>,
		pub weight: Option<String>,
		pub stealth_disadvantage: bool
	}

	#[derive(Deserialize)]
	pub struct SourceFeat {
		pub name: String,
		pub prerequisite: String,
		pub desc: String
	}
}

use serde_json::from_str;

use crate::data::{internal::{InternalSource, armour::{Armour, ArmourCategory}, item::{Rarity, Money}, feat::Feat, SourceCategory, SourceContentItem, SourceContentCollection, SourceContentType}, sources::srd::source_data::SourceArmour, character_state::AbilityScoreType};

use self::source_data::SourceFeat;

use super::{Source, source_utils};

macro_rules! include_srd_file {
	($filename:literal) => {
		include_str!(concat!("../../../assets/sources/wotc_srd/", $filename))
	};
}

pub struct SrdSource;

impl Source for SrdSource {
    fn get_as_internal() -> InternalSource {
		// https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/

		let armours = from_str::<Vec<SourceArmour>>(include_srd_file!("armor.json")).unwrap_or(Vec::new()).into_iter().map(|src_armour| {
			SourceContentItem::ArmourItem(to_internal_armour(src_armour))
		}).collect();

		let feats = from_str::<Vec<SourceFeat>>(include_srd_file!("feats.json")).unwrap_or(Vec::new()).into_iter().map(|src_feat| {
			SourceContentItem::FeatItem(to_internal_feat(src_feat))
		}).collect();

		InternalSource {
			name: "System Reference Document".into(),
			category: SourceCategory::Core,
			content: im::vector![
				SourceContentCollection::new(SourceContentType::ArmourType, armours),
				SourceContentCollection::new(SourceContentType::FeatType, feats),
			]
		}
    }
}

fn to_internal_feat(src_feat: SourceFeat) -> Feat {
	Feat {
		name: src_feat.name,
		prerequisite: src_feat.prerequisite,
		description: src_feat.desc
	}
}

fn to_internal_armour(src_armour: SourceArmour) -> Armour {
	let name = src_armour.name;
	let category = {
		let cat_str = src_armour.category;
		source_utils::str_to_armour_category(&cat_str)
	};
	let rarity = {
		if let Some(rar_str) = src_armour.rarity {
			source_utils::str_to_rarity(&rar_str)
		} else {
			Rarity::Standard
		}
	};
	let plus_mod = {
		let mut mods = Vec::new();

		if src_armour.plus_str_mod {
			mods.push(AbilityScoreType::Strength)
		}
		if src_armour.plus_dex_mod {
			mods.push(AbilityScoreType::Dexterity)
		}
		if src_armour.plus_con_mod {
			mods.push(AbilityScoreType::Constitution)
		}
		if src_armour.plus_int_mod {
			mods.push(AbilityScoreType::Intelligence)
		}
		if src_armour.plus_wis_mod {
			mods.push(AbilityScoreType::Wisdom)
		}
		if src_armour.plus_cha_mod {
			mods.push(AbilityScoreType::Charisma)
		}

		mods.into_iter().collect()
	};

	Armour {
		name,
		category,
		rarity,
		base_ac: src_armour.base_ac,
		plus_mod,
		plus_mod_max: src_armour.plus_max,
		plus_flat_mod: src_armour.plus_flat_mod,
		cost: src_armour.cost.and_then(|cost| Some(Money::parse(&cost))),
		weight: 0, // TODO
		stealth_disadvantage: src_armour.stealth_disadvantage,
	}
}