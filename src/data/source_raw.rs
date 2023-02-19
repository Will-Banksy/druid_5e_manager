pub mod feat_raw;
pub mod armour_raw;
pub mod item_raw;

use serde::{Serialize, Deserialize};

use self::armour_raw::ArmourRaw;

#[derive(Serialize, Deserialize)]
pub struct SourceRaw {
	armour: Vec<ArmourRaw>
}