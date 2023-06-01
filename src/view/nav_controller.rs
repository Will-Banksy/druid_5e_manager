use std::sync::Arc;

use druid::{widget::Controller, Event, Env, Selector, Widget};
use druid_widget_nursery::navigator::{Navigator, View, ViewController};

use crate::data::{AppData, transitive_app_state::NavState};

const POP_VIEW: Selector<()> = Selector::new("druid_5e_manager.command.navigator.pop-view");

impl View for NavState {}

pub struct NavController;
impl Controller<AppData, Navigator<AppData, NavState>> for NavController {
	fn event(
		&mut self,
		child: &mut Navigator<AppData, NavState>,
		ctx: &mut druid::EventCtx,
		event: &Event,
		data: &mut AppData,
		env: &Env,
	) {
		match event {
			Event::Command(selector) if selector.is(POP_VIEW) => {
				data.pop_view();
			}
			_ => (),
		}
		child.event(ctx, event, data, env)
	}
}

impl ViewController<NavState> for AppData {
	fn add_view(&mut self, view: NavState) {
		let views: &mut Vec<NavState> = Arc::make_mut(&mut self.uistate.nav_state);
		views.push(view);
		let views = Arc::new(views.clone());
		self.uistate.nav_state = views;
	}

	fn pop_view(&mut self) {
		let views = Arc::make_mut(&mut self.uistate.nav_state);
		views.pop();
		let views = Arc::new(views.clone());
		self.uistate.nav_state = views;
	}

	fn current_view(&self) -> &NavState {
		self.uistate.nav_state.last().unwrap()
	}

	fn len(&self) -> usize {
		self.uistate.nav_state.len()
	}

	fn is_empty(&self) -> bool {
		self.uistate.nav_state.is_empty()
	}
}