pub mod controllers;
pub mod widgets;

use druid::{Widget, WidgetExt, TextAlignment, Menu, MenuItem, Env, WindowId, FileDialogOptions, commands, PaintCtx, Color, RenderContext, SysMods, theme, EventCtx, KeyOrValue};
use druid::widget::{Label, Flex, TextBox, List, Painter, CrossAxisAlignment, Checkbox, Button, MainAxisAlignment};

use crate::{delegate, env};
use crate::rules::modifier;
use crate::formatter::NumberFormatter;
use crate::data::{CharacterState, AbilityScore, AbilityScoreType, Skill, Level};

use self::widgets::ratio_split::RatioSplit;
use self::widgets::separator::{Separator, CrossAxisSize};

fn painter_background<T>(col: impl Into<KeyOrValue<Color>>) -> Painter<T> {
	let col = col.into();
	Painter::new(move |ctx: &mut PaintCtx, _data: &_, env: &Env| {
		let bounds = ctx.size().to_rect();
		let colour: Color = match &col {
			KeyOrValue::Concrete(c) => c.clone(),
			KeyOrValue::Key(k) => env.get(k)
		};
		ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
	})
}

fn color_for(score_type: AbilityScoreType, env: &Env) -> Color {
	match score_type {
		AbilityScoreType::Strength => env.get(env::THEME_COL_ABILITY_STRENGTH),
		AbilityScoreType::Dexterity => env.get(env::THEME_COL_ABILITY_DEXTERITY),
		AbilityScoreType::Constitution => env.get(env::THEME_COL_ABILITY_CONSTITUTION),
		AbilityScoreType::Intelligence => env.get(env::THEME_COL_ABILITY_INTELLIGENCE),
		AbilityScoreType::Wisdom => env.get(env::THEME_COL_ABILITY_WISDOM),
		AbilityScoreType::Charisma => env.get(env::THEME_COL_ABILITY_CHARISMA)
	}
	// env.get(theme::BACKGROUND_DARK)
}

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
						.with_text_size(env::THEME_SIZE_TITLE)
						.lens(CharacterState::name).expand_width(), 0.75
				)
				.with_default_spacer()
				.with_flex_child(
					TextBox::new()
						.with_placeholder("Race")
						.lens(CharacterState::race).expand_width(), 0.25
				)
				.with_default_spacer()
				.with_child(
					Label::new(|data: &u16, _env: &_| {
						format!("Level: {}", data)
					}).lens(CharacterState::level)
				)
				.with_default_spacer()
				.with_child(
					Label::new(|data: &u16, _env: &_| {
						format!("Proficiency Bonus: {}", data)
					}).lens(CharacterState::proficiency_bonus)
				)
		)
		.with_default_spacer()
		.with_child(
			Flex::row()
				.with_child(
					Label::new("LEVELS\n& CLASSES")
				)
				.with_default_spacer()
				.with_child(
					List::new(|| {
						Flex::row() // TODO: This is kinda ugly imo having so many ".with_default_spacer()"s can I neaten it up at all?
							.with_default_spacer()
							.with_child(
								Separator::horizontal()
									.with_size(1.0)
									.with_cross_axis_size(CrossAxisSize::Absolute(32.0.into()))
									.with_colour(druid::theme::BORDER_DARK)
							)
							.with_default_spacer()
							.with_default_spacer()
							.with_child(
								level()
							)
					}).horizontal()
						.with_spacing(druid::theme::WIDGET_PADDING_HORIZONTAL)
						.lens(CharacterState::levels)
				)
				.with_default_spacer()
				.with_child(
					Button::new("+")
						.on_click(|_ctx: &mut EventCtx, data: &mut CharacterState, _env: &_| {
							data.levels.push_back(Level::new("".into(), 1));
							_ctx.submit_command(delegate::RECALC_OVERALL_LEVEL);
						})
				)
				.align_left() // This makes the levels bar take up the whole window width
				.scroll() // TODO: When scrollbar is showing it overlays the widgets a bit - Can I stop this without allocating spare space below the levels that would ruin the lovely clean look of them?
				.padding(env::THEME_INSETS)
				.background(painter_background(druid::theme::BACKGROUND_DARK))
		)
		.with_default_spacer()
		.with_child(
			Flex::row()
				.with_child( // COLUMN 1
					Flex::column()
						.with_child(
							Label::new("ABILITY SCORES")
						)
						.with_default_spacer()
						.with_child(
							List::new(|| {
								ability_score()
							})
								.with_spacing(druid::theme::WIDGET_PADDING_VERTICAL)
								.lens(CharacterState::ability_scores)
								.expand_width()
						)
						.padding(env::THEME_INSETS)
						.background(painter_background(druid::theme::BACKGROUND_DARK))
						.fix_width(128.0)
				)
				.with_default_spacer()
				.with_child( // COLUMN 2
					Flex::column()
						.with_child( // SAVING THROWS
							Flex::column()
								.with_child(
									Label::new("SAVING THROWS")
								)
								.with_default_spacer()
								.with_child(
									List::new(|| {
										saving_throw()
									})
									.lens(CharacterState::ability_scores)
								)
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
						)
						.with_default_spacer()
						.with_child( // SKILLS
							Flex::column()
								.with_child(
									Label::new("SKILLS")
								)
								.with_default_spacer()
								.with_child(
									List::new(|| {
										skill()
									})
									.lens(CharacterState::skills)
								)
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
						.fix_width(456.0)
				)
				.with_default_spacer()
				.with_child( // COLUMN 3
					Flex::column()
						.with_child( // HEALTH POINTS
							Flex::column()
								.with_child(
									Label::new("HEALTH POINTS")
								)
								.with_default_spacer()
								.with_child(
									Flex::row()
										.with_child(
											TextBox::new()
												.with_text_alignment(TextAlignment::Center)
												.with_text_size(env::THEME_SIZE_H1)
												.with_formatter(NumberFormatter::new())
												.fix_width(46.0)
												.lens(CharacterState::hp)
										)
										.with_child(
											Label::new("/")
										)
										.with_child(
											TextBox::new()
												.with_text_alignment(TextAlignment::Center)
												.with_formatter(NumberFormatter::new())
												.fix_width(36.0)
												.lens(CharacterState::hp_max)
										)
										// .with_default_spacer()
										.with_child(
											Label::new("Temp: ")
										)
										.with_child(
											TextBox::new()
												.with_text_alignment(TextAlignment::Center)
												.with_formatter(NumberFormatter::new())
												.fix_width(36.0)
												.lens(CharacterState::temp_hp)
										)
								)
								.with_default_spacer()
								.with_child(
									Flex::row()
										.with_child(Label::new("Hit Dice")) // TODO: Hit dice
								)
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
						)
						.with_default_spacer()
						.with_child( // ARMOUR CLASS
							Flex::column()
								.with_child(
									Label::new("ARMOUR CLASS")
										.center()
								)
								.with_default_spacer()
								.with_child(
									Flex::row()
										.with_flex_child(
											Flex::column()
												.with_child(
													TextBox::new()
														.with_placeholder("Armour")
														.lens(CharacterState::equip_armour)
												)
												.with_default_spacer()
												.with_child(
													TextBox::new()
														.with_placeholder("Shield")
														.lens(CharacterState::equip_shield)
												)
												.cross_axis_alignment(CrossAxisAlignment::Fill)
												.expand_width(),
											1.0
										)
										.with_default_spacer()
										.with_child(
											Flex::column()
												.with_child(
													TextBox::new()
														.with_text_alignment(TextAlignment::Center)
														.with_text_size(env::THEME_SIZE_H1)
														.with_formatter(NumberFormatter::new())
														.fix_width(46.0)
														.lens(CharacterState::ac)
												)
												.with_default_spacer()
												.with_child(Label::new("AC"))
										)
								)
								.with_default_spacer()
								.with_child(
									Checkbox::new("Stealth Disadvantage")
										.lens(CharacterState::stealth_disadvantage)
								)
								.cross_axis_alignment(CrossAxisAlignment::Fill)
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
						)
						.with_default_spacer()
						.with_child( // SPEEDS
							Flex::column()
								.with_child(Label::new("SPEEDS"))
								.with_default_spacer()
								.with_child(
									RatioSplit::row()
										.with_child(
											Flex::column()
												.with_child(Label::new("Walking"))
												.with_child(
													TextBox::new()
														.with_text_alignment(TextAlignment::Center)
														.with_formatter(NumberFormatter::new().with_unit("ft"))
														.lens(CharacterState::speed)
												)
												.with_default_spacer()
												.with_child(Label::new("Flying"))
												.with_child(
													TextBox::new()
														.with_text_alignment(TextAlignment::Center)
														.with_formatter(NumberFormatter::new().with_unit("ft"))
														.lens(CharacterState::speed_fly)
												)
												.cross_axis_alignment(CrossAxisAlignment::Start)
												.expand_width(),
											0.5
										)
										.with_child(
											Flex::column()
												.with_child(Label::new("Climbing"))
												.with_child(
													TextBox::new()
														.with_text_alignment(TextAlignment::Center)
														.with_formatter(NumberFormatter::new().with_unit("ft"))
														.lens(CharacterState::speed_climb)
												)
												.with_default_spacer()
												.with_child(Label::new("Swimming"))
												.with_child(
													TextBox::new()
														.with_text_alignment(TextAlignment::Center)
														.with_formatter(NumberFormatter::new().with_unit("ft"))
														.lens(CharacterState::speed_swim)
												)
												.cross_axis_alignment(CrossAxisAlignment::Start)
												.expand_width(),
											0.5
										)
								)
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
								// .debug_paint_layout()
						)
						.with_default_spacer()
						.with_child( // SENSES
							Flex::column()
								.with_child(Label::new("SENSES"))
								.with_default_spacer()
								// TODO: Some sort of List similar to how levels are done
								.padding(env::THEME_INSETS)
								.background(painter_background(druid::theme::BACKGROUND_DARK))
								// .debug_paint_layout()
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
						.fix_width(220.)
				)
				.cross_axis_alignment(CrossAxisAlignment::Start)
				.main_axis_alignment(MainAxisAlignment::Start)
		)
		.cross_axis_alignment(CrossAxisAlignment::Fill)
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
						let modifier = modifier(data.score, false, false, 0);
						if modifier < 0 {
							format!("{}", modifier.to_string())
						} else {
							format!("+{}", modifier)
						}
					}).with_text_size(env::THEME_SIZE_H1).center().fix_width(48.0)
				)
				.with_spacer(4.0)
				.with_child(
					TextBox::new()
						.with_text_alignment(TextAlignment::Center)
						.with_formatter(NumberFormatter::new())
						.lens(AbilityScore::score)
						.center()
						.fix_width(48.0)
						.controller(controllers::DataUpdateAlertController::new(delegate::SET_ABILITY_SCORE, |sel, data: AbilityScore| sel.with((data.score_type, data.score))))
				)
		)
		.padding((6.0, 8.0))
		.center()
		.fix_width(100.0)
		.background(Painter::new(|ctx: &mut PaintCtx, data: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = color_for(data.score_type, env);
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}

fn saving_throw() -> impl Widget<AbilityScore> {
	Flex::row()
		.with_child(
			Label::new(|data: &AbilityScore, _env: &_| {
				let modifier = modifier(data.score, data.saving_proficiency, false, data.proficiency_bonus);
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
			Checkbox::new("Prof.").lens(AbilityScore::saving_proficiency)
		)
		.with_child(
			Checkbox::new("Adv.").lens(AbilityScore::saving_advantage)
		)
		.padding((4.0, 2.0))
		.background(Painter::new(|ctx: &mut PaintCtx, data: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = color_for(data.score_type, env);
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}

fn skill() -> impl Widget<Skill> {
	Flex::row()
		.with_child(
			Label::new(|data: &Skill, _env: &_| {
				let modifier = modifier(data.score, data.proficiency, data.expertise, data.proficiency_bonus);
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
			Label::new(|data: &AbilityScoreType, _env: &_| {
				format!("({})", format!("{:?}", data)[..3].to_string())
			}).with_font(theme::UI_FONT_ITALIC)
			.lens(Skill::score_type)
		)
		.with_default_spacer()
		.with_flex_spacer(1.0)
		.with_child(
			Checkbox::new("Prof.").lens(Skill::proficiency)
		)
		.with_child(
			Checkbox::new("Ex.").lens(Skill::expertise).disabled_if(|data: &Skill, _env: &_| !data.proficiency)
		)
		.with_child(
			Checkbox::new("Adv.").lens(Skill::advantage)
		)
		.padding((4.0, 2.0))
		.background(Painter::new(|ctx: &mut PaintCtx, data: &Skill, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = color_for(data.score_type, env);
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
		// TODO REMOVE
		// .padding(env::THEME_INSETS)
		// .background(painter_background(druid::theme::BACKGROUND_DARK))
}