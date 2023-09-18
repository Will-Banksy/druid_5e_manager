pub mod armour;
pub mod adventuring_gear;

use druid::Data;

use self::armour::Armour;

use super::currency::Currency;

pub enum Equipment { // TODO: Implement each
	Armour(Armour),
	AdventuringGear,
	Instrument,
	Vehicle,
	Container,
	Tool,
	Weapon
}

pub enum EquipSlot {
	OneHand,
	TwoHands,
	WholeBody,
	Head,
	Neck,
	Legs,
	Feet,
	Back
}

#[derive(Clone, PartialEq, Data)]
pub enum Rarity {
	Standard,
	Common,
	Uncommon,
	Rare,
	VeryRare,
	Legendary,
	Custom(String)
}

pub struct EquipmentBasicInfo {
	name: String,
	description: String,
	cost: Currency,
	weight: f32, // TODO: Weight struct? Structs for all units? idk
	rarity: Rarity,
	/// The slots that this piece of equipment can be equipped in, or empty if it can't be equipped/takes up no slots
	equip_slots: im::Vector<EquipSlot>,
}