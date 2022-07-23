use std::{path::{PathBuf}, fs::{File, self}, io::Write};

use druid::{AppDelegate, DelegateCtx, Target, Command, Env, Handled, commands, FileDialogOptions, Selector};

use crate::data::{CharacterState, shared_data::SharedDataItem};

pub const UPDATE_WIDGET_TREE: Selector<()> = Selector::new("druid_play.update-widget-tree");
pub const SET_PROFICIENCY_BONUS: Selector<u8> = Selector::new("druid_play.set-proficiency-bonus");

pub struct Delegate {
	has_path: bool,
	save_url: PathBuf
}

impl Delegate {
	pub fn new() -> Self {
		Delegate {
			has_path: false,
			save_url: PathBuf::new()
		}
	}
}

// TODO: On app close save file?
impl AppDelegate<CharacterState> for Delegate {
	fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut CharacterState, _env: &Env) -> Handled {
		if cmd.is(commands::SAVE_FILE_AS) {
			if let Some(path) = cmd.get(commands::SAVE_FILE_AS) {
				self.has_path = true;
				self.save_url = path.path.clone();
				println!("Saved file as: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.serialise()).unwrap();
			}
			Handled::Yes
		} else if cmd.is(commands::SAVE_FILE) {
			if self.has_path {
				println!("Saved file to: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.serialise()).unwrap();
			} else {
				// println!("Submitted command");
				_ctx.submit_command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()).to(_target));
			}

			Handled::Yes
		} else if cmd.is(commands::OPEN_FILE) {
			if let Some(path) = cmd.get(commands::OPEN_FILE) {
				self.has_path = true;
				self.save_url = path.path.clone();
				let serialised = fs::read_to_string(path.path.as_path()).unwrap();
				*data = CharacterState::deserialise(&serialised);
			}

			Handled::Yes
		} else if cmd.is(SET_PROFICIENCY_BONUS) {
			if let Some(p) = cmd.get(SET_PROFICIENCY_BONUS) {
				data.proficiency_bonus.set(SharedDataItem::U8(*p));
			}

			#[allow(unused)]
			Handled::Yes
		} else if cmd.is(UPDATE_WIDGET_TREE) {
			todo!(); // TODO

			#[allow(unused)]
			Handled::Yes
	 	} else {
			println!("Unhandled command: {:?}", cmd);
			Handled::No
		}
	}
}