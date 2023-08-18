pub mod character_state;
pub mod internal;
pub mod sources;
pub mod app_config;
pub mod transitive_app_state;
pub mod lenses;
pub mod stat_modifier;

use std::sync::Arc;

use druid::{Lens, Data};
use im::vector;

use self::{character_state::CharacterState, internal::InternalSource, app_config::AppConfig, transitive_app_state::{TransitiveAppState}, sources::get_sources};

#[derive(Clone, Data, Lens)]
pub struct AppData {
	pub character: CharacterState,
	pub sources: im::Vector<InternalSource>,
	pub app_config: AppConfig,
	pub uistate: TransitiveAppState
}

impl AppData {
	pub fn new() -> AppData {
		AppData {
			character: CharacterState::new(),
			sources: vector![],
			app_config: AppConfig {},
			uistate: TransitiveAppState { nav_state: Arc::new(Vec::new()), selected_source: 0, selected_source_array: 0, selected_source_array_item: 0 }
		}
	}

	pub fn init_sources(&mut self) {
		self.sources = get_sources().into_iter().collect()
	}
}