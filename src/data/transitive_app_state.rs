use std::sync::Arc;

use druid::Data;

#[derive(Data, Clone)]
pub struct TransitiveAppState {
	pub nav_state: Arc<Vec<NavState>>,
	pub selected_source: usize,
	pub selected_source_array: usize,
	pub selected_source_array_item: usize
}

#[derive(Data, Clone, PartialEq, Debug, Hash, Eq)]
pub enum NavState {
	NavDestSourceManager,
	NavDestCharacter,
}