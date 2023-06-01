use std::{path::{PathBuf}, fs::{File, self}, io::Write, sync::Arc};

use druid::{AppDelegate, DelegateCtx, Target, Command, Env, Handled, commands, FileDialogOptions, Selector, widget::ListIter, platform_menus::win::file::new};
use druid_widget_nursery::navigator::ViewController;

use crate::{data::{character_state::{CharacterState, AbilityScoreType}, AppData, transitive_app_state::NavState}, rules};

pub const UPDATE_WIDGET_TREE: Selector<()> = Selector::new("druid_5e_manager.command.update-widget-tree");
pub const SET_PROFICIENCY_BONUS: Selector<u16> = Selector::new("druid_5e_manager.command.set-proficiency-bonus");
pub const RECALC_OVERALL_LEVEL: Selector<()> = Selector::new("druid_5e_manager.command.recalc-overall-level");
pub const DELETE_LEVEL: Selector<u128> = Selector::new("druid_5e_manager.command.delete-level");
pub const SET_ABILITY_SCORE: Selector<(AbilityScoreType, u8)> = Selector::new("druid_5e_manager.command.set-ability-score");
pub const DELETE_SENSE: Selector<u128> = Selector::new("druid_5e_manager.command.delete-sense");
pub const DELETE_CONDITION: Selector<u128> = Selector::new("druid_5e_manager.command.delete-condition");
pub const UPDATE_FROM_CONDITIONS: Selector<()> = Selector::new("druid_5e_manager.command.update-from-conditions");
pub const NAV_SWITCH_TO_SOURCES: Selector<()> = Selector::new("druid_5e_manager.command.navigator.switch-to-sources");
pub const NAV_SWITCH_TO_CHARACTER: Selector<()> = Selector::new("druid_5e_manager.command.navigator.switch-to-character");

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

	// TODO: Move saving file to a function. DRY
}

// TODO: On app close save file?
impl AppDelegate<AppData> for Delegate {
	fn command(&mut self, ctx: &mut DelegateCtx, target: Target, cmd: &Command, data: &mut AppData, _env: &Env) -> Handled {
		if cmd.is(commands::SAVE_FILE_AS) {
			if let Some(path) = cmd.get(commands::SAVE_FILE_AS) {
				self.has_path = true;
				self.save_url = path.path.clone();
				println!("Saved file as: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.character.serialize()).unwrap();
			}
			Handled::Yes
		} else if cmd.is(commands::SAVE_FILE) {
			if self.has_path {
				println!("Saved file to: {}", self.save_url.display());
				let mut file = File::create(self.save_url.as_path()).unwrap(); // TODO: Error handling
				writeln!(&mut file, "{}", data.character.serialize()).unwrap();
			} else {
				ctx.submit_command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()).to(target));
			}

			Handled::Yes
		} else if cmd.is(commands::OPEN_FILE) {
			if let Some(path) = cmd.get(commands::OPEN_FILE) {
				self.has_path = true;
				self.save_url = path.path.clone();
				let serialised = fs::read_to_string(path.path.as_path()).unwrap();
				data.character = CharacterState::deserialize(&serialised);
			}

			Handled::Yes
		} else if cmd.is(SET_PROFICIENCY_BONUS) {
			if let Some(p) = cmd.get(SET_PROFICIENCY_BONUS) {
				data.character.proficiency_bonus = *p;
				data.character.ability_scores.for_each_mut(|a_s, _| a_s.proficiency_bonus = *p);
				data.character.skills.for_each_mut(|sk, _| sk.proficiency_bonus = *p);
			}

			Handled::Yes
		} else if cmd.is(RECALC_OVERALL_LEVEL) {
			let level_sum = data.character.levels.iter().fold(0, |val, level_struct| val as u16 + level_struct.level as u16);
			data.character.level = level_sum;

			ctx.submit_command(SET_PROFICIENCY_BONUS.with(rules::proficiency_bonus_for(data.character.level)));

			Handled::Yes
		} else if cmd.is(DELETE_LEVEL) {
			if let Some(uuid) = cmd.get(DELETE_LEVEL) {
				if let Some(i) = data.character.levels.iter().enumerate().find_map(|(i, l)| if l.uuid == *uuid { Some(i) } else { None }) {
					data.character.levels.remove(i);
				}
			}

			ctx.submit_command(RECALC_OVERALL_LEVEL);

			Handled::Yes
		} else if cmd.is(UPDATE_WIDGET_TREE) { // TODO: If it is even possible
			println!("This feature is unimplemented because I am not sure whether it is possible and I don't really remember why I wanted it in the first place");

			#[allow(unused)]
			Handled::Yes
		} else if cmd.is(SET_ABILITY_SCORE) {
			if let Some((as_type, score)) = cmd.get(SET_ABILITY_SCORE) {
				data.character.ability_scores.for_each_mut(|a_s, _| if a_s.score_type == *as_type { a_s.score = *score });
				data.character.skills.for_each_mut(|sk, _| if sk.score_type == *as_type { sk.score = *score });
			}

			Handled::Yes
	 	} else if cmd.is(DELETE_SENSE) {
			if let Some(uuid) = cmd.get(DELETE_SENSE) {
				if let Some(i) = data.character.senses.iter().enumerate().find_map(|(i, s)| if s.uuid == *uuid { Some(i) } else { None }) {
					data.character.senses.remove(i);
				}
			}

			Handled::Yes
	 	} else if cmd.is(DELETE_CONDITION) {
			if let Some(uuid) = cmd.get(DELETE_CONDITION) {
				if let Some(i) = data.character.conditions.iter().enumerate().find_map(|(i, s)| if s.uuid == *uuid { Some(i) } else { None }) {
					data.character.conditions.remove(i);
				}
			}

			Handled::Yes
	 	} else if cmd.is(UPDATE_FROM_CONDITIONS) {
			// TODO: Actually implement this properly, calculating increases to a base value which will be set by the race, or the race is an increase to a base value of 0
			data.character.speed = data.character.conditions.iter().fold(0, |acc, condition| acc + condition.speed_increase);

			Handled::Yes
	 	} else if cmd.is(NAV_SWITCH_TO_CHARACTER) {
			let new_nav_dest = Arc::make_mut(&mut data.uistate.nav_state);
			// new_nav_dest.pop();
			new_nav_dest.push(NavState::NavDestCharacter);
			data.uistate.nav_state = Arc::new(new_nav_dest.clone());

			println!("Switched to character");

			Handled::Yes
	 	} else if cmd.is(NAV_SWITCH_TO_SOURCES) {
			let new_nav_dest = Arc::make_mut(&mut data.uistate.nav_state);
			// new_nav_dest.pop();
			new_nav_dest.push(NavState::NavDestSourceManager);
			data.uistate.nav_state = Arc::new(new_nav_dest.clone());

			println!("Switched to sources");

			Handled::Yes
		} else {
			// println!("Unhandled command: {:?}", cmd);
			Handled::No
		}
	}
}