pub mod formatter;
pub mod data;
pub mod view;
pub mod delegate;
pub mod rules;
pub mod assets;
pub mod env;

use data::{CharacterState};
use druid::{PlatformError, AppLauncher, WindowDesc};
use env::config_env_defaults;
use view::{build_ui, build_app_menu};

// TODO: Move all this shit to github issues lmao it's just clutter here and maybe with issues, the few random people stumbling across my repository will be able to give some thoughts wishful thinking though that probably is

// TODO: [DESIGN] Remove duplicate data and use other methods to have a variable-length list of widgets that can still access certain data from containing data
//     Or think of ways in which it can otherwise be done... Perhaps by putting the CharacterState data in the Env although that's an ugly solution

// TODO: [UI/UX] More widgets required
//     At some point, add a number selection widget (Spinner) so I don't have to just use valuetextboxes
//     Also add a widget that contains another widget that enforces a minimum size

// TODO: [PLATFORMS] Consider platform support/differences and synchronising data
//     Compile it to webassembly to use on web. I also kinda wanna get it working on mobile some day...
//         I don't think druid has support for mobile outside of web but PWA maybe?
//         Hm actually how would file management work on web... differently I imagine? Also people don't want to work with files on mobile
//         Web and mobile might take a bit more thought and work than simply porting to those platforms
//         A splash screen with a default directory to save characters that displays those characters might work well... if web can do that
//         As part of web, it'd be helpful for characters to sync between installations with cloud storage... This might be something for far in the future
//             Or maybe use Google Drive or something

// TODO: [FEATURES]
//     Popup to confirm exit with unsaved data
//     Also add a command bar (Ctrl+P/Ctrl+Shift+P (or maybe Ctrl+K/Ctrl+Shift+K cause on Web Ctrl+P is print) to open, Esc to close) to do actions such as "Take Damage", "Add Item", "Deduct Money", "Learn Spell", etc.

// TODO: [DESIGN] Urgent-ish - I need to decide how things that increase stats are gonna work
//     E.g. Say we add a feat that increases Str by 1, what should happen?
//     Does the act of adding the feat directly increase Str by 1, and removing it decrease Str by 1?
//     Should every time Str is displayed, it calculates any increases to Str by looping over feats and examining them and displaying Str + increases without directly increasing Str?
//     Perhaps take an approach much like calculating overall level: Every time a feat or anything else is added/removed, recalculate stats
//         This would also require a slight changeup of the stats themselves - Instead of being a flat score they are now a modifier to some default score but assuming a default of 0 doesn't change much
//     Maybe it doesn't directly increase Str, but instead adds itself to and removes itself from a list of modifiers for Str?
//     The last one might be best since I do want to keep track of what things contribute to what stats but I also like the robustness of the second-to-last

// TODO: [DESIGN][UI/UX] Consider how to store Hit Dice, and more broadly how to store things like descriptions of items that are like "does 1d4 + Str piercing damage"
//     Do we want to insert actual values for stats in there when displaying? Cause that may change what we do here
//     I think it'd be good ui/ux to do so (along with the expression/stat name ofc)

// TODO: ADD README

fn main() -> Result<(), PlatformError> {
	let state: CharacterState = CharacterState::new();

	AppLauncher::with_window(WindowDesc::new(build_ui())
		.title("D&D Character Manager")
		.window_size((1200.0, 820.0))
		.menu(build_app_menu)
	)
		.delegate(delegate::Delegate::new())
		.configure_env(|env, _| config_env_defaults(env))
		.launch(state)?;

	Ok(())
}
