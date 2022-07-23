pub mod formatter;
pub mod data;
pub mod view;
pub mod delegate;

use data::{CharacterState};
use druid::{PlatformError, AppLauncher, WindowDesc};
use view::{build_ui, build_app_menu};

fn main() -> Result<(), PlatformError> {
	let state: CharacterState = CharacterState::new();

    AppLauncher::with_window(WindowDesc::new(build_ui())
		.title("DnD Character Manager")
		.menu(build_app_menu)
	)
		.delegate(delegate::Delegate::new())
		.launch(state)?;

    Ok(())
}
