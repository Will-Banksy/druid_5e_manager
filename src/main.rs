pub mod formatter;
pub mod data;
pub mod view;
pub mod delegate;
pub mod dnd_rules;
pub mod assets;

use data::{CharacterState};
use druid::{PlatformError, AppLauncher, WindowDesc};
use view::{build_ui, build_app_menu};

// TODO: At some point, add a number selection widget so I don't have to just use valuetextboxes

// TODO: Evaluate whether the pain of trying to get SharedData working flawlessly is worth it
//       and whether I should just have a fuckload of controllers firing off a fuckload of commands whenever something
//       that should be shared is changed so it gets changed throughout the whole program

fn main() -> Result<(), PlatformError> {
	let state: CharacterState = CharacterState::new();

    AppLauncher::with_window(WindowDesc::new(build_ui())
		.title("D&D Character Manager")
		.window_size((1200.0, 800.0))
		.menu(build_app_menu)
	)
		.delegate(delegate::Delegate::new())
		.launch(state)?;

    Ok(())
}
