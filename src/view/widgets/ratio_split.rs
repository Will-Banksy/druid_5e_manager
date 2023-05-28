use druid::{WidgetPod, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, widget::Axis, Data, KeyOrValue};

/// Flexibly sizes widgets where each widget is a percentage (or ratio) of the maximum available width or height (depending on direction)
pub struct RatioSplit<T> {
	widgets: Vec<ChildWidget<T>>,
	boundaries: Vec<f64>,
	direction: Axis,
	bound_sum: f64,
	gap_size: KeyOrValue<f64>,
	// TODO: Cross axis alignment
}

struct ChildWidget<T> {
	pub widget: WidgetPod<T, Box<dyn Widget<T>>>,
	// TODO: Cross axis alignment
}

impl<T> RatioSplit<T> {
	pub fn row() -> Self {
		Self {
			widgets: Vec::new(),
			boundaries: vec![0.0],
			direction: Axis::Horizontal,
			bound_sum: 0.0,
			gap_size: KeyOrValue::Key(druid::theme::WIDGET_PADDING_HORIZONTAL)
		}
	}

	pub fn column() -> Self {
		Self {
			direction: Axis::Vertical,
			gap_size: KeyOrValue::Key(druid::theme::WIDGET_PADDING_VERTICAL),
			..Self::row()
		}
	}

	pub fn set_start(&mut self, start: f64) {
		self.boundaries[0] = start;
	}

	pub fn add_child(&mut self, child: impl Widget<T> + 'static, size: f64) {
		self.widgets.push(ChildWidget::new(WidgetPod::new(child).boxed()));
		self.bound_sum += size;
		self.boundaries.push(self.bound_sum);
		if self.bound_sum > 1.0 { // If the sum of the boundaries has gone above 1.0, normalise it
			for bnd in self.boundaries.iter_mut() {
				*bnd /= self.bound_sum;
			}
			self.bound_sum = self.boundaries.iter().sum();
		}
	}

	pub fn set_gap_size(&mut self, gap_size: impl Into<KeyOrValue<f64>>) {
		self.gap_size = gap_size.into();
	}

	pub fn with_start(mut self, start: f64) -> Self {
		self.set_start(start);
		self
	}

	pub fn with_child(mut self, child: impl Widget<T> + 'static, size: f64) -> Self {
		self.add_child(child, size);
		self
	}

	pub fn with_gap_size(mut self, gap_size: impl Into<KeyOrValue<f64>>) -> Self {
		self.set_gap_size(gap_size);
		self
	}
}

impl<T> Widget<T> for RatioSplit<T> where T: Data {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		for child in &mut self.widgets {
			child.widget.event(ctx, event, data, env);
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		for child in &mut self.widgets {
			child.widget.lifecycle(ctx, event, data, env);
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		for child in &mut self.widgets {
			child.widget.update(ctx, data, env);
		}
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size { // Would be nice to generalise this implementation a bit so as to not have 2 implementations for both horizontal and vertical case
		let mut max_major = 0.0; // How far along the major axis we need in terms of space
		let mut max_minor = 0.0; // How far along the minor axis we need in terms of space
		let gap_size = self.gap_size.resolve(env);
		for i in 0..self.widgets.len() {
			let bound_start = self.boundaries[i];
			let bound_end = self.boundaries[i + 1]; // self.boundaries is guaranteed to have length 1 more than self.widgets
			let main_axis_rlength = bound_end - bound_start;
			match self.direction {
				Axis::Horizontal => {
					let width = (main_axis_rlength as f64 * bc.max().width) - gap_size / 2.0;
					let child_bc = BoxConstraints::new(Size::ZERO, Size::new(width, bc.max().height));
					let child_size = self.widgets[i].widget.layout(ctx, &child_bc, data, env);
					let origin = (
						max_major,
						0.0,
					);
					self.widgets[i].widget.set_origin(ctx, origin.into());
					max_major += child_size.width + gap_size;
					if max_minor < child_size.height {
						max_minor = child_size.height;
					}
				},
				Axis::Vertical => {
					let height = (main_axis_rlength as f64 * bc.max().height) - gap_size / 2.0;
					let child_bc = BoxConstraints::new(Size::ZERO, Size::new(bc.max().width, height));
					let child_size = self.widgets[i].widget.layout(ctx, &child_bc, data, env);
					let origin = (
						0.0,
						max_major,
					);
					self.widgets[i].widget.set_origin(ctx, origin.into());
					max_major += child_size.height + gap_size;
					if max_minor < child_size.width {
						max_minor = child_size.width;
					}
				},
			}
		}

		match self.direction {
			Axis::Horizontal => (max_major, max_minor),
			Axis::Vertical => (max_minor, max_major)
		}.into()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		for child in &mut self.widgets {
			child.widget.paint(ctx, data, env);
		}
	}
}

impl<T> ChildWidget<T> {
	fn new(widget: WidgetPod<T, Box<dyn Widget<T>>>) -> Self {
		Self {
			widget
		}
	}
}