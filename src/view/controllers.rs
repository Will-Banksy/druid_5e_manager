use std::marker::PhantomData;

use druid::{widget::Controller, Widget, Data, UpdateCtx, Env, Selector, Command};

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

pub struct DataUpdateAlertController<S, F, T> {
	selector: Selector<S>,
	select_to_command: F,
	_phantom: PhantomData<T>
}

impl<S, F, T: Data> DataUpdateAlertController<S, F, T> where F: Fn(Selector<S>, T) -> Command {
	pub fn new(s: Selector<S>, f: F) -> Self {
		Self { selector: s, select_to_command: f, _phantom: PhantomData }
	}
}

impl<S, F, T: Data, W: Widget<T>> Controller<T, W> for DataUpdateAlertController<S, F, T> where F: Fn(Selector<S>, T) -> Command {
	fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
		if !old_data.same(&data) {
			ctx.submit_command((self.select_to_command)(self.selector, data.clone()))
		}
		child.update(ctx, old_data, data, env)
	}
}
