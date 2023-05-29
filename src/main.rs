pub mod data;
pub mod view;
pub mod delegate;
pub mod rules;
pub mod env;
pub mod dice;
pub mod utils;

use data::AppData;
use druid::{PlatformError, AppLauncher, WindowDesc};
use env::config_env_defaults;
use view::{build_ui, build_app_menu};

// TODO: Move all this shit to github issues lmao it's just clutter here and maybe with issues, the few random people stumbling across my repository will be able to give some thoughts wishful thinking though that probably is

// TODO: [DESIGN] Remove duplicate data and use other methods to have a variable-length list of widgets that can still access certain data from containing data
//     Or think of ways in which it can otherwise be done... Perhaps by putting the CharacterState data in the Env although that's an ugly solution imo. But maybe it isn't actually
//         But it is only one-way

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
//             Or maybe use Google Drive or something. Maybe still for far future though
//                 https://crates.io/crates/google-drive
//                 https://developers.google.com/drive/api/guides/api-specific-auth

// TODO: [FEATURES]
//     Popup to confirm exit with unsaved data
//     Also add a command bar (Ctrl+P/Ctrl+Shift+P (or maybe Ctrl+K/Ctrl+Shift+K cause on Web Ctrl+P is print) to open, Esc to close) to do actions such as "Take Damage", "Add Item", "Deduct Money", "Learn Spell", etc.
//     Status bar with icon button for showing command bar

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
//     Idea: Display the dice equation below a textbox, and have the textbox display values (e.g. substituting Str + Prof for the value of the Strength ability score + proficiency bonus) but when editing the textbox it will display Str
//         Or if a textbox isn't a good idea, maybe have a button which switches representations (maybe in status bar, switches representations for entire application)
//         Or, y'know, just display like "1d4 + 4(Str + Prof)" but nahh seems dumb and not nice
//     Alternatively, just display the "1d4 + Str" but show the actual values on hover?

// TODO: [FEATURES] Add the ability to convert between units, and use different units

// TODO: [DESIGN][FEATURES]
//     Start designing a content management system, and sort of design the character manager around that
//     Like, the app opens on to a content management GUI, where you can manage and browse your content including standard stuff and homebrew stuff
//         You have the capabilities to like create new homebrew sources, and add items/creatures/spells/etc
//     It will also be necessary to be able to access this functionality from the character screen (Modal dialog?)
//     The format of things like characters and stuff will need to be considered. Are there any examples of how characters are stored somewhere?
//         Like, what fields the character struct has?
//         NOTE: Serde's skip attribute might be useful for this https://serde.rs/field-attrs.html#skip

// TODO: ADD README

fn main() -> Result<(), PlatformError> {
	let mut state: AppData = AppData::new();
	state.init_sources();

	AppLauncher::with_window(WindowDesc::new(build_ui())
		.title("D&D Character Manager")
		.window_size((1400.0, 820.0))
		.menu(build_app_menu)
	)
		.delegate(delegate::Delegate::new())
		.configure_env(|env, _| config_env_defaults(env))
		.launch(state)?;

	Ok(())
}
