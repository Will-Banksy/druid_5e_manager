use crate::data::internal::{item::Rarity, armour::ArmourCategory};

pub fn str_to_rarity(rar_str: &str) -> Rarity {
	if rar_str == "Standard" {
		Rarity::Standard
	} else if rar_str == "Common" {
		Rarity::Common
	} else if rar_str == "Uncommon" {
		Rarity::Uncommon
	} else if rar_str == "Rare" {
		Rarity::Rare
	} else if rar_str == "Very Rare" {
		Rarity::VeryRare
	} else if rar_str == "Legendary" {
		Rarity::Legendary
	} else {
		Rarity::Custom(rar_str.to_string())
	}
}

pub fn str_to_armour_category(cat_str: &str) -> ArmourCategory {
	if cat_str == "No Armor" {
		ArmourCategory::NoArmour
	} else if cat_str == "Light Armor" {
		ArmourCategory::LightArmour
	} else if cat_str == "Heavy Armor" {
		ArmourCategory::HeavyArmour
	} else if cat_str == "Shield" {
		ArmourCategory::Shield
	} else if cat_str == "Spell" {
		ArmourCategory::Spell
	} else if cat_str == "Class Feature" {
		ArmourCategory::ClassFeature
	} else {
		ArmourCategory::Custom(cat_str.to_string())
	}
}