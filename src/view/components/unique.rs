use druid::{Widget, widget::{Flex, Label, TextBox, Painter, Checkbox, Button}, WidgetExt, TextAlignment, Env, PaintCtx, RenderContext, EventCtx};

use crate::{data::character_state::{AbilityScore, Skill, AbilityScoreType, Level}, rules::modifier, env, view::{controllers, color_for, widgets::{accordion::Accordion, hover::Hover}, formatter::NumberFormatter}, delegate};

use super::common::small_input_label;

pub fn ability_score() -> impl Widget<AbilityScore> {
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
							format!("{}", modifier)
						} else {
							format!("+{}", modifier)
						}
					}).with_text_size(env::THEME_SIZE_H1).center()
				)
				.with_spacer(4.0)
				.with_child(
					Flex::row()
						.with_child(
							small_input_label(Label::new("BASE: "))
						)
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
		)
		.padding((6.0, 8.0))
		.center()
		.background(Painter::new(|ctx: &mut PaintCtx, data: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = color_for(data.score_type, env);
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
}

pub fn saving_throw() -> impl Widget<AbilityScore> {
	Hover::new(
		Accordion::vertical(
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
				.with_flex_spacer(1.0)
				.with_child(
					small_input_label(Label::new(|data: &AbilityScore, _: &Env| {
						if data.saving_advantage {
							"ADVANTAGE"
						} else {
							""
						}
					}))
				)
				.with_default_spacer()
				.with_child(
					small_input_label(Label::new(|data: &AbilityScore, _: &Env| {
						if data.saving_proficiency {
							"PROFICIENCY"
						} else {
							""
						}
					}))
				)
				.padding((4.0, 2.0))
		).with_content(
			Flex::row()
				.with_flex_spacer(1.0)
				.with_child(
					Checkbox::new("Proficiency").lens(AbilityScore::saving_proficiency)
				)
				.with_default_spacer()
				.with_child(
					Checkbox::new("Advantage").lens(AbilityScore::saving_advantage) // TODO: Need some sort of tri-state checkbox for Adv | Neut | Disadv and maybe double advantage/disadvantage
				)
				.padding((8.0, 4.0))
		)
		.expand_width()
		.background(Painter::new(|ctx: &mut PaintCtx, _: &AbilityScore, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = if ctx.is_hot() {
				env.get(druid::theme::BACKGROUND_LIGHT)
			} else {
				env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
			};
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
		.controller(controllers::DataUpdateAlertController::new(druid::Selector::<()>::new("data update"), |s, _| s.into()))
	)
}

pub fn skill() -> impl Widget<Skill> {
	Hover::new(
		Accordion::vertical(
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
					}).with_font(druid::theme::UI_FONT_ITALIC)
					.lens(Skill::score_type)
				)
				.with_flex_spacer(1.0)
				.with_child(
					small_input_label(Label::new(|data: &Skill, _: &Env| {
						if data.advantage {
							"ADVANTAGE"
						} else {
							""
						}
					}))
				)
				.with_default_spacer()
				.with_child(
					small_input_label(Label::new(|data: &Skill, _: &Env| {
						if data.proficiency {
							if data.expertise {
								"EXPERTISE"
							} else {
								"PROFICIENCY"
							}
						} else {
							""
						}
					}))
				)
				.padding((4.0, 2.0))
		).with_content(
			Flex::row()
				.with_flex_spacer(1.0)
				.with_child(
					Checkbox::new("Proficiency").lens(Skill::proficiency)
				)
				.with_child(
					Checkbox::new("Expertise").lens(Skill::expertise).disabled_if(|data: &Skill, _env: &_| !data.proficiency) // TODO: Some sort of tri-state checkbox?
				)
				.with_child(
					Checkbox::new("Advantage").lens(Skill::advantage) // TODO: Need some sort of tri-state checkbox for Adv | Neut | Disadv
				)
				.padding((8.0, 4.0))
		)
		.expand_width()
		.background(Painter::new(|ctx: &mut PaintCtx, _: &Skill, env: &Env| {
			let bounds = ctx.size().to_rect();
			let colour = if ctx.is_hot() {
				env.get(druid::theme::BACKGROUND_LIGHT)
			} else {
				env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
			};
			ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
		}))
		.controller(controllers::DataUpdateAlertController::new(druid::Selector::<()>::new("data update"), |s, _| s.into()))
	)
}

pub fn level() -> impl Widget<Level> {
	Flex::row()
		.with_child(
			TextBox::new()
				.with_placeholder("Class")
				.lens(Level::name)
		)
		.with_default_spacer()
		.with_child(
			small_input_label(Label::new("LEVEL: "))
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
}