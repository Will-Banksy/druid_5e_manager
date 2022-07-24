use druid::{widget::Controller, Widget, Data, UpdateCtx, Env};

use crate::{data::{CharacterState, Level}, delegate, dnd_rules};

pub struct CharacterLevelController;

impl<W: Widget<CharacterState>> Controller<CharacterState, W> for CharacterLevelController {
	fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &CharacterState, data: &CharacterState, env: &Env) {
		if !old_data.level.same(&data.level) {
			ctx.submit_command(delegate::SET_PROFICIENCY_BONUS.with(dnd_rules::proficiency_bonus_for(data.level)))
		}
		child.update(ctx, old_data, data, env)
	}
}

pub struct LevelsController;

impl<W: Widget<Level>> Controller<Level, W> for LevelsController {
	fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &Level, data: &Level, env: &Env) {
		if !old_data.level.same(&data.level) {
			ctx.submit_command(delegate::RECALC_OVERALL_LEVEL)
		}
		child.update(ctx, old_data, data, env)
	}
}
