pub mod character_state;
pub mod source_raw;

use druid::{Lens, Data};

use self::character_state::CharacterState;

#[derive(Clone, Data, Lens)]
pub struct AppData {
	character: CharacterState
	// TODO: App config
}