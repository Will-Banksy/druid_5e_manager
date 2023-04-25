pub mod character_state;
pub mod internal;
pub mod sources;

use druid::{Lens, Data};

use self::character_state::CharacterState;

#[derive(Clone, Data, Lens)]
pub struct AppData {
	character: CharacterState
	// TODO: App config
}