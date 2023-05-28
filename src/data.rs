pub mod character_state;
pub mod internal;
pub mod sources;
pub mod app_config;
pub mod transitive_app_state;

use druid::{Lens, Data};

use self::{character_state::CharacterState, internal::InternalSource, app_config::AppConfig, transitive_app_state::TransitiveAppState};

#[derive(Clone, Data, Lens)]
pub struct AppData {
	character: CharacterState,
	sources: im::Vector<InternalSource>,
	app_config: AppConfig,
	transitive_app_state: TransitiveAppState
}