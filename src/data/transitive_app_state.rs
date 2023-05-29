use std::sync::Arc;

use druid::Data;

#[derive(Data, Clone)]
pub struct TransitiveAppState {
	pub nav_state: Arc<Vec<NavState>>,
	pub selected_source: usize,
	pub selected_source_array: SelectedSourceArray
}

#[derive(Data, Clone, PartialEq, Debug, Hash, Eq)]
pub enum NavState {
	NavDestSourceManager,
	NavDestCharacter,
}

#[derive(Data, Clone, PartialEq)]
pub enum SelectedSourceArray {
	ArmourArray,
	FeatsArray
}

impl SelectedSourceArray {
	pub fn get_string(&self) -> String {
		match self {
			SelectedSourceArray::ArmourArray => "ARMOUR".into(),
			SelectedSourceArray::FeatsArray => "FEATS".into(),
		}
	}
}