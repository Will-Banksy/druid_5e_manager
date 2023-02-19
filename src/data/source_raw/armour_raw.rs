use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmourRaw {
	name: String,
	category: String,
	rarity: Option<String>,
	base_ac: u8,
	#[serde(default)]
	plus_str_mod: bool,
	#[serde(default)]
	plus_dex_mod: bool,
	#[serde(default)]
	plus_con_mod: bool,
	#[serde(default)]
	plus_int_mod: bool,
	#[serde(default)]
	plus_wis_mod: bool,
	#[serde(default)]
	plus_cha_mod: bool,
	/// Max that can be gained from plus_mod
	plus_max: Option<u8>,
	#[serde(default)]
	plus_flat_mod: u8,
	cost: Option<String>,
	weight: Option<String>,
	stealth_disadvantage: bool
}

#[cfg(test)]
#[test]
fn test_armour_deserialization() {
	let armour_json = include_str!("../../../assets/sources/wotc_srd/armor.json");
	let armour: Vec<ArmourRaw> = serde_json::from_str(armour_json).unwrap();
	println!("ArmourRaw Array: {:?}", armour);
}