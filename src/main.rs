pub mod formatter;
pub mod data;
pub mod view;
pub mod delegate;
pub mod dnd_rules;
pub mod assets;
pub mod env;

use data::{CharacterState};
use druid::{PlatformError, AppLauncher, WindowDesc};
use env::config_env_defaults;
use view::{build_ui, build_app_menu};

// TODO: At some point, add a number selection widget (Spinner) so I don't have to just use valuetextboxes

fn main() -> Result<(), PlatformError> { // TODO: Popup to confirm exit with unsaved data
	let state: CharacterState = CharacterState::new();

    AppLauncher::with_window(WindowDesc::new(build_ui())
		.title("D&D Character Manager")
		.window_size((1200.0, 800.0))
		.menu(build_app_menu)
	)
		.delegate(delegate::Delegate::new())
		.configure_env(|env, _| config_env_defaults(env))
		.launch(state)?;

    Ok(())
}
