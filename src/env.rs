use druid::{Env, Key, Color, Insets};

pub const THEME_COL_ABILITY_STRENGTH: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-strength");
pub const THEME_COL_ABILITY_DEXTERITY: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-dexterity");
pub const THEME_COL_ABILITY_CONSTITUTION: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-constitution");
pub const THEME_COL_ABILITY_INTELLIGENCE: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-intelligence");
pub const THEME_COL_ABILITY_WISDOM: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-wisdom");
pub const THEME_COL_ABILITY_CHARISMA: Key<Color> = Key::new("druid_5e_manager.env.theme-col-ability-charisma");

pub const THEME_SIZE_TITLE: Key<f64> = Key::new("druid_5e_manager.env.theme-size-title");
pub const THEME_SIZE_H1: Key<f64> = Key::new("druid_5e_manager.env.theme-size-h1");

pub const THEME_INSETS: Key<Insets> = Key::new("druid_5e_manager.env.theme-insets");

pub fn config_env_defaults(env: &mut Env) {
	env.set(THEME_COL_ABILITY_STRENGTH, Color::Rgba32(0x9e2835ff));// 0x421c1cff
	env.set(THEME_COL_ABILITY_DEXTERITY, Color::Rgba32(0x265c42ff));// 0x2a5639ff
	env.set(THEME_COL_ABILITY_CONSTITUTION, Color::Rgba32(0xbe4a2fff));// 0x4e351bff
	env.set(THEME_COL_ABILITY_INTELLIGENCE, Color::Rgba32(0x124e89ff));// 0x264063ff
	env.set(THEME_COL_ABILITY_WISDOM, Color::Rgba32(0x5a6988ff));// 0x4c5644ff
	env.set(THEME_COL_ABILITY_CHARISMA, Color::Rgba32(0x68386cff));// 0x5a2139ff

	env.set(THEME_SIZE_TITLE, 28.0);
	env.set(THEME_SIZE_H1, 20.0);

	env.set(THEME_INSETS, Insets::uniform_xy(8.0, 6.0));

	env_defaults_override_style(env);
	env_defaults_override_colours(env);
}

fn env_defaults_override_style(env: &mut Env) {
	env.set(druid::theme::BUTTON_BORDER_RADIUS, env.get(druid::theme::TEXTBOX_BORDER_RADIUS));
}

fn env_defaults_override_colours(env: &mut Env) { // TODO: Perfect this, e.g. colour buttons correctly (same as checkboxes)
	// Colours
	// Light back: 0x181818ff
	// Dark back: 0x0e0e0eff
	// Vdark back: 0x000000ff
	// Border light: 0x2f2f2fff

	env.set(druid::theme::BACKGROUND_DARK, Color::Rgba32(0x000000ff));
	env.set(druid::theme::BACKGROUND_LIGHT, Color::Rgba32(0x181818ff));
	env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::Rgba32(0x0e0e0eff));
	env.set(druid::theme::BORDER_DARK, Color::Rgba32(0x2f2f2fff));
}