pub mod controllers;

use druid::{Widget, WidgetExt, TextAlignment, Menu, MenuItem, Env, WindowId, FileDialogOptions, commands, PaintCtx, Color, RenderContext, SysMods, LensExt, theme, EventCtx};
use druid::widget::{Label, Flex, TextBox, List, Painter, CrossAxisAlignment, Checkbox, Button};

use crate::data::shared_data::SharedData;
use crate::delegate;
use crate::dnd_rules::modifier;
use crate::formatter::NumberFormatter;
use crate::data::{CharacterState, AbilityScore, AbilityScoreType, Skill, Level};

const H1_TEXT_SIZE: f64 = 20.0;
const STRENGTH_COLOUR: Color = Color::Rgba32(0x421c1cff);
const DEXTERITY_COLOUR: Color = Color::Rgba32(0x2a5639ff);
const CONSTITUTION_COLOUR: Color = Color::Rgba32(0x4e351bff);
const INTELLIGENCE_COLOUR: Color = Color::Rgba32(0x264063ff);
const WISDOM_COLOUR: Color = Color::Rgba32(0x4c5644ff);
const CHARISMA_COLOUR: Color = Color::Rgba32(0x5a2139ff);

pub fn build_app_menu(_window_id: Option<WindowId>, _state: &CharacterState, _env: &Env) -> Menu<CharacterState> {
	// TODO: Force immediate update/handling of commands...somehow?
	Menu::new("Application Menu").entry(
		Menu::new("File")
			.entry(
				MenuItem::new("Save")
					.command(druid::commands::SAVE_FILE)
					.hotkey(SysMods::Cmd, "s")
			)
			.entry(
				MenuItem::new("Save As")
					.command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()))
					.hotkey(SysMods::CmdShift, "s")
			)
			.separator()
			.entry(
				MenuItem::new("Open").command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()))
				.hotkey(SysMods::Cmd, "o")
			)
			.separator()
			.entry(
				MenuItem::new("[Dev] Update Widget Tree")
				.command(delegate::UPDATE_WIDGET_TREE)
			)
	)
}

pub fn build_ui() -> impl Widget<CharacterState> {
	Flex::column()
		.with_child(
			Flex::row()
				.with_flex_child(
					TextBox::new()
						.with_placeholder("Character Name")
						.with_text_size(H1_TEXT_SIZE)
						.lens(CharacterState::name).expand_width(), 1.0
				)
				.with_default_spacer()
				.with_child(
					Label::new(|data: &u16, _env: &_| {
						format!("Level: {}", data)
					}).lens(CharacterState::level)//.controller(controllers::CharacterLevelController)
				)
				// .with_child(
				// 	// TextBox::new()
				// 	// 	.with_formatter(NumberFormatter::new())
				// 	// 	.fix_width(48.0)
				// 	// 	.lens(CharacterState::level)
				// 	// 	.controller(controllers::CharacterLevelController)
				// )
				.with_default_spacer()
				.with_child(
					Label::new(|data: &CharacterState, _env: &_| {
						format!("Proficiency Bonus: {}", data.proficiency_bonus.item().unwrap_u16())
					})//.lens(CharacterState::proficiency_bonus.then(SharedData::U8_LENS))
				)
		)
		.with_default_spacer()
		.with_child(
			Flex::row()
				.with_child(
					List::new(|| {
						level()
					}).horizontal().with_spacing(10.0).lens(CharacterState::levels)
				)
				.with_default_spacer()
				.with_child(
					Button::new("+")
						.on_click(|_ctx: &mut EventCtx, data: &mut CharacterState, _env: &_| {
							data.levels.push_back(Level::new("".into(), 1));
							_ctx.submit_command(delegate::RECALC_OVERALL_LEVEL);
						})
				)
				.align_left()
		)
		.with_default_spacer()
		.with_child(
			Flex::row()
				.with_child(
					Flex::column()
						.with_child(
							Label::new("Ability Scores").with_text_size(H1_TEXT_SIZE)
						)
						.with_default_spacer()
						.with_child(
							List::new(|| {
								ability_score()
							})
							.with_spacing(10.0)
							.fix_width(100.0)
							.lens(CharacterState::ability_scores)
						)
				)
				.with_default_spacer()
				.with_flex_child(
					Flex::column()
						.with_child(
							Label::new("Saving Throws").with_text_size(H1_TEXT_SIZE)
						)
						.with_default_spacer()
						.with_child(
							List::new(|| {
								saving_throw()
							})
							.lens(CharacterState::ability_scores)
						)
						.with_default_spacer()
						.with_child(
							Label::new("Skills").with_text_size(H1_TEXT_SIZE)
						)
						.with_default_spacer()
						.with_child(
							List::new(|| {
								skill()
							})
							.lens(CharacterState::skills)
						)
						.cross_axis_alignment(CrossAxisAlignment::Start),
						1.0
				)
				.cross_axis_alignment(CrossAxisAlignment::Start)
				.fix_width(640.0)
				.align_left()
				// .debug_paint_layout()
		)
		.padding(6.0)
		.scroll()
		.vertical()
}

fn ability_score() -> impl Widget<AbilityScore> {
	Flex::row()
		.with_child(
			Flex::column()
				.with_child(
					Label::new(|data: &AbilityScore, _env: &_| {
						format!("{:?}", data.score_type)
					}).center()
				)
				.with_spacer(4.0)
				.with_child(
					Label::new(|data: &AbilityScore, _env: &_| {
						let modifier = modifier(data.score.item().unwrap_u8(), false, false, 0);
						if modifier < 0 {
							format!("{}", modifier.to_string())
						} else {
							format!("+{}", modifier)
						}
					}).with_text_size(H1_TEXT_SIZE).center().fix_width(48.0)
				)
				.with_spacer(4.0)
				.with_child(
					TextBox::new()
						.with_text_alignment(TextAlignment::Center)
						.with_formatter(NumberFormatter::new())
						.lens(AbilityScore::score.then(SharedData::U8_LENS))
						.center()
						.fix_width(48.0)
				)
		)
		.padding((6.0, 8.0))
		.center()
		.fix_width(100.0)
		.background(Painter::new(|ctx: &mut PaintCtx, data: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = match data.score_type {
				AbilityScoreType::Strength => STRENGTH_COLOUR,
				AbilityScoreType::Dexterity => DEXTERITY_COLOUR,
				AbilityScoreType::Constitution => CONSTITUTION_COLOUR,
				AbilityScoreType::Intelligence => INTELLIGENCE_COLOUR,
				AbilityScoreType::Wisdom => WISDOM_COLOUR,
				AbilityScoreType::Charisma => CHARISMA_COLOUR
			};
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}

fn saving_throw() -> impl Widget<AbilityScore> {
	Flex::row()
		.with_child(
			Label::new(|data: &AbilityScore, _env: &_| {
				let modifier = modifier(data.score.item().unwrap_u8(), data.saving_proficiency, false, data.proficiency_bonus.item().unwrap_u16());
				if modifier < 0 {
					format!("{}", modifier.to_string())
				} else {
					format!("+{}", modifier)
				}
			})
		)
		.with_default_spacer()
		.with_child(
			Label::new(|data: &AbilityScore, _env: &_| {
				format!("{:?}", data.score_type)
			})
		)
		.with_default_spacer()
		.with_flex_spacer(1.0)
		.with_child(
			Checkbox::new("Proficiency").lens(AbilityScore::saving_proficiency)
		)
		.with_child(
			Checkbox::new("Advantage").lens(AbilityScore::saving_advantage)
		)
		.padding((4.0, 2.0))
		.background(Painter::new(|ctx: &mut PaintCtx, data: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = match data.score_type {
				AbilityScoreType::Strength => STRENGTH_COLOUR,
				AbilityScoreType::Dexterity => DEXTERITY_COLOUR,
				AbilityScoreType::Constitution => CONSTITUTION_COLOUR,
				AbilityScoreType::Intelligence => INTELLIGENCE_COLOUR,
				AbilityScoreType::Wisdom => WISDOM_COLOUR,
				AbilityScoreType::Charisma => CHARISMA_COLOUR
			};
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
		// .debug_paint_layout()
}

fn skill() -> impl Widget<Skill> {
	Flex::row()
		.with_child(
			Label::new(|data: &Skill, _env: &_| {
				let modifier = modifier(data.score.item().unwrap_u8(), data.proficiency, data.expertise, data.proficiency_bonus.item().unwrap_u16());
				if modifier < 0 {
					format!("{}", modifier.to_string())
				} else {
					format!("+{}", modifier)
				}
			})
		)
		.with_default_spacer()
		.with_child(
			Label::new(|data: &Skill, _env: &_| {
				format!("{}", data.skill_type)
			})
		)
		.with_child(
			// Label::raw().lens(Constant::<RichText>({
			// 	let mut rtb = RichTextBuilder::new();
			// 	rtb.push("Int").style(FontStyle::Italic);
			// 	rtb.build()
			// }))
			Label::new(|data: &AbilityScoreType, _env: &_| {
				format!("({})", format!("{:?}", data)[..3].to_string())
			}).with_font(theme::UI_FONT_ITALIC)
			.lens(Skill::score_type)
		)
		.with_default_spacer()
		.with_flex_spacer(1.0)
		.with_child(
			Checkbox::new("Proficiency").lens(Skill::proficiency)
		)
		.with_child(
			Checkbox::new("Expertise").lens(Skill::expertise).disabled_if(|data: &Skill, _env: &_| !data.proficiency)
		)
		.with_child(
			Checkbox::new("Advantage").lens(Skill::advantage)
		)
		.padding((4.0, 2.0))
		.background(Painter::new(|ctx: &mut PaintCtx, data: &Skill, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = match data.score_type {
				AbilityScoreType::Strength => STRENGTH_COLOUR,
				AbilityScoreType::Dexterity => DEXTERITY_COLOUR,
				AbilityScoreType::Constitution => CONSTITUTION_COLOUR,
				AbilityScoreType::Intelligence => INTELLIGENCE_COLOUR,
				AbilityScoreType::Wisdom => WISDOM_COLOUR,
				AbilityScoreType::Charisma => CHARISMA_COLOUR
			};
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}

fn level() -> impl Widget<Level> {
	Flex::row()
		.with_child(
			TextBox::new()
				.with_placeholder("Class")
				.lens(Level::name)
		)
		.with_default_spacer()
		.with_child(
			Label::new("Level: ")
		)
		.with_child(
			TextBox::new()
				.with_formatter(NumberFormatter::new())
				.lens(Level::level)
				.fix_width(32.0)
				.controller(controllers::LevelsController)
		)
		.with_default_spacer()
		.with_child(
			Button::new("🗑")
				.on_click(|ctx: &mut EventCtx, data: &mut Level, _env: &_| {
					ctx.submit_command(delegate::DELETE_LEVEL.with(data.uuid))
				})
				// .background(Painter::new(|ctx: &mut PaintCtx, _data: &Level, env: &Env| {
				// 	let image = ctx.make_image(16, 16, &assets::ASSETIMAGE_IMG_BIN, druid::piet::ImageFormat::Rgb).unwrap();
				// 	ctx.draw_image(&image, (Point::new(0.0, 0.0), Point::new(16.0, 16.0)), InterpolationMode::Bilinear);
				// }))
		)
		.padding((8.0, 6.0))
		.background(Painter::new(|ctx: &mut PaintCtx, _data: &Level, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour: Color = env.get(theme::BACKGROUND_DARK);
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}