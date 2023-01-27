use druid::{WidgetPod, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, Data, widget::Axis, Size, Selector, BoxConstraints};

const ACCORDION_TOGGLE: Selector<()> = Selector::new("druid_5e_manager.notifications.accordion-toggle");

pub struct Accordion<T> {
	title_bar: WidgetPod<T, Box<dyn Widget<T>>>,
	content: Option<WidgetPod<T, Box<dyn Widget<T>>>>,
	expanded: bool,
	direction: Axis
}

struct TitleBarContainer<T> {
	title_bar: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T> Accordion<T> where T: Data {
	pub fn horizontal(title_bar: impl Widget<T> + 'static) -> Self {
		Self {
			title_bar: WidgetPod::new(TitleBarContainer::new(title_bar)).boxed(),
			content: None,
			expanded: false,
			direction: Axis::Horizontal
		}
	}

	pub fn vertical(title_bar: impl Widget<T> + 'static) -> Self {
		Self {
			direction: Axis::Vertical,
			..Self::horizontal(title_bar)
		}
	}

	pub fn set_content(&mut self, content: impl Widget<T> + 'static) {
		self.content = Some(WidgetPod::new(content).boxed());
	}

	pub fn with_content(mut self, content: impl Widget<T> + 'static) -> Self {
		self.set_content(content);
		self
	}
}

impl<T> Widget<T> for Accordion<T> where T: Clone + Data {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		self.title_bar.event(ctx, event, data, env);
		if let Some(w) = &mut self.content {
			if self.expanded || event.should_propagate_to_hidden() {
				w.event(ctx, event, data, env);
			}
		}

		if let Event::Notification(notif) = event {
			if notif.is(ACCORDION_TOGGLE) {
				self.expanded = !self.expanded;
				ctx.children_changed();
				println!("Children changed..?")
			}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		self.title_bar.lifecycle(ctx, event, data, env);
		if let Some(w) = &mut self.content {
			if self.expanded || event.should_propagate_to_hidden() {
				w.lifecycle(ctx, event, data, env);
			}
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		self.title_bar.update(ctx, data, env);
		if let Some(w) = &mut self.content {
			w.update(ctx, data, env);
		}
	}

	fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &Env) -> druid::Size {
		if let Some(content) = &mut self.content {
			if self.expanded {
				let title_size = self.title_bar.layout(ctx, &bc.loosen(), data, env);
				self.title_bar.set_origin(ctx, data, env, (0.0, 0.0).into());

				let content_bc = {
					if self.direction == Axis::Horizontal {
						bc.shrink((title_size.width, 0.0)).loosen()
						// BoxConstraints::new(bc.min(), (bc.max().width - title_size.width, bc.max().height).into())
					} else {
						bc.shrink((0.0, title_size.height)).loosen()
						// BoxConstraints::new(bc.min(), (bc.max().width, bc.max().height - title_size.height).into())
					}
				};
				let content_size = content.layout(ctx, &content_bc, data, env);
				let content_origin = {
					if self.direction == Axis::Horizontal {
						(title_size.width, 0.0).into()
					} else {
						(0.0, title_size.height).into()
					}
				};
				content.set_origin(ctx, data, env, content_origin);
				return Size::new(content_origin.x + content_size.width, content_origin.y + content_size.height);
			} else {
				let title_size = self.title_bar.layout(ctx, &bc.loosen(), data, env);
				self.title_bar.set_origin(ctx, data, env, (0.0, 0.0).into());

				content.layout(ctx, &BoxConstraints::new(Size::ZERO, Size::ZERO), data, env);
				let content_origin = {
					if self.direction == Axis::Horizontal {
						(title_size.width, 0.0).into()
					} else {
						(0.0, title_size.height).into()
					}
				};
				content.set_origin(ctx, data, env, content_origin);
				return title_size;
			}
		}
		let size = self.title_bar.layout(ctx, &bc.loosen(), data, env);
		self.title_bar.set_origin(ctx, data, env, (0.0, 0.0).into());
		size
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
		self.title_bar.paint(ctx, data, env);
		if let Some(w) = &mut self.content {
			if self.expanded {
				w.paint(ctx, data, env);
			}
		}
	}
}

impl<T> TitleBarContainer<T> {
	fn new(title_bar: impl Widget<T> + 'static) -> Self {
		Self {
			title_bar: WidgetPod::new(title_bar).boxed()
		}
	}
}

// impl<T> Deref for TitleBarContainer<T> {
// 	type Target = WidgetPod<T, Box<dyn Widget<T>>>;

// 	fn deref(&self) -> &Self::Target {
// 		&self.title_bar
// 	}
// }

// impl<T> DerefMut for TitleBarContainer<T> {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.title_bar
// 	}
// }

impl<T> Widget<T> for TitleBarContainer<T> where T: Clone + Data {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.title_bar.event(ctx, event, data, env);

		if !ctx.is_handled() {
			if let Event::MouseUp(_) = event {
				ctx.submit_notification(ACCORDION_TOGGLE);
			}
		}
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.title_bar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, data: &T, env: &Env) {
        self.title_bar.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &Env) -> Size {
        self.title_bar.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        self.title_bar.paint(ctx, data, env);
    }
}