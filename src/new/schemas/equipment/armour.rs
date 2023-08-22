use super::EquipmentBasicInfo;

pub struct Armour {
	basics: EquipmentBasicInfo,
	ac_base: u32,
	/// A list of stats that give AC bonuses when wearing this armour (usually just dexterity)
	ac_bonuses: im::Vector<String>, // TODO: A struct much like druid's Selector and consts for assigning and keeping track of IDs
	/// The maximum bonus AC that can be bestowed from the ac_bonuses, or None
	ac_bonus_max: Option<u32>,
	stealth_disadvantage: bool
}