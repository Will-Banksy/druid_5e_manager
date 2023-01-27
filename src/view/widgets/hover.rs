use druid::{Widget, WidgetPod, Data, LifeCycle};

pub struct Hover<T> {
	child: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T> Hover<T> {
	pub fn new(child: impl Widget<T> + 'static) -> Self {
		Self {
			child: WidgetPod::new(child).boxed()
		}
	}
}

impl<T> Widget<T> for Hover<T> where T: Data {
	fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
		self.child.event(ctx, event, data, env);
	}

	fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &T, env: &druid::Env) {
		if let LifeCycle::HotChanged(_) = event {
			ctx.request_paint();
		}
		self.child.lifecycle(ctx, event, data, env);
	}

	fn update(&mut self, ctx: &mut druid::UpdateCtx, _: &T, data: &T, env: &druid::Env) {
		self.child.update(ctx, data, env);
	}

	fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> druid::Size {
		self.child.layout(ctx, bc, data, env)
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
		self.child.paint(ctx, data, env);
	}
}