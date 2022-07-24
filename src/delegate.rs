use std::{path::{PathBuf}, fs::{File, self}, io::Write};

use druid::{AppDelegate, DelegateCtx, Target, Command, Env, Handled, commands, FileDialogOptions, Selector};

use crate::{data::{CharacterState, shared_data::SharedDataItem}, dnd_rules};

pub const UPDATE_WIDGET_TREE: Selector<()> = Selector::new("druid_play.update-widget-tree");
pub const SET_PROFICIENCY_BONUS: Selector<u16> = Selector::new("druid_play.set-proficiency-bonus");
pub const RECALC_OVERALL_LEVEL: Selector<()> = Selector::new("druid_play.recalc-overall-level");
pub const DELETE_LEVEL: Selector<u128> = Selector::new("druid_play.delete-level");

pub struct Delegate {
	// TODO: Just replace with Option<PathBuf>
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
	fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut CharacterState, _env: &Env) -> Handled {
		if cmd.is(commands::SAVE_FILE_AS) {
			if let Some(path) = cmd.get(commands::SAVE_FILE_AS) {
				self.has_path = true;
				self.save_url = path.path.clone();
				println!("Saved file as: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.serialize()).unwrap();
			}
			Handled::Yes
		} else if cmd.is(commands::SAVE_FILE) {
			if self.has_path {
				println!("Saved file to: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.serialize()).unwrap();
			} else {
				// println!("Submitted command");
				ctx.submit_command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()).to(_target));
			}

			Handled::Yes
		} else if cmd.is(commands::OPEN_FILE) {
			if let Some(path) = cmd.get(commands::OPEN_FILE) {
				self.has_path = true;
				self.save_url = path.path.clone();
				let serialised = fs::read_to_string(path.path.as_path()).unwrap();
				*data = CharacterState::deserialize(&serialised);
			}

			Handled::Yes
		} else if cmd.is(SET_PROFICIENCY_BONUS) {
			if let Some(p) = cmd.get(SET_PROFICIENCY_BONUS) {
				data.proficiency_bonus.set(SharedDataItem::U16(*p));
			}

			Handled::Yes
		} else if cmd.is(RECALC_OVERALL_LEVEL) {
			let level_sum = data.levels.iter().fold(0, |val, level_struct| val as u16 + level_struct.level as u16);
			data.level = level_sum;

			ctx.submit_command(SET_PROFICIENCY_BONUS.with(dnd_rules::proficiency_bonus_for(data.level)));

			Handled::Yes
		} else if cmd.is(DELETE_LEVEL) {
			if let Some(uuid) = cmd.get(DELETE_LEVEL) {
				if let Some(i) = data.levels.iter().enumerate().find_map(|(i, l)| if l.uuid == *uuid { Some(i) } else { None }) {
					data.levels.remove(i);
				}
			}

			ctx.submit_command(RECALC_OVERALL_LEVEL);

			Handled::Yes
		} else if cmd.is(UPDATE_WIDGET_TREE) {
			todo!(); // TODO: If it is even possible

			#[allow(unused)]
			Handled::Yes
	 	} else {
			println!("Unhandled command: {:?}", cmd);
			Handled::No
		}
	}
}