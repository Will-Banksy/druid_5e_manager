use druid::{widget::Axis, Widget, KeyOrValue, Color, Size, RenderContext};

pub enum CrossAxisSize {
	Fraction(KeyOrValue<f64>),
	Absolute(KeyOrValue<f64>)
}

pub struct Separator {
	direction: Axis,
	size: KeyOrValue<f64>,
	colour: KeyOrValue<Color>,
	cross_axis_size: CrossAxisSize
}

impl Separator {
	pub fn horizontal() -> Self {
		Self {
			direction: Axis::Horizontal,
			size: 2.0.into(),
			colour: druid::theme::BORDER_LIGHT.into(),
			cross_axis_size: CrossAxisSize::Fraction(0.75.into())
		}
	}

	pub fn vertical() -> Self {
		Self {
			direction: Axis::Vertical,
			..Self::horizontal()
		}
	}

	pub fn set_size(&mut self, size: impl Into<KeyOrValue<f64>>) {
		self.size = size.into();
	}

	pub fn set_colour(&mut self, colour: impl Into<KeyOrValue<Color>>) {
		self.colour = colour.into();
	}

	pub fn set_cross_axis_size(&mut self, cross_axis_size: CrossAxisSize) {
		self.cross_axis_size = cross_axis_size;
	}

	pub fn with_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
		self.set_size(size);
		self
	}

	pub fn with_colour(mut self, colour: impl Into<KeyOrValue<Color>>) -> Self {
		self.set_colour(colour);
		self
	}

	pub fn with_cross_axis_size(mut self, cross_axis_size: CrossAxisSize) -> Self {
		self.set_cross_axis_size(cross_axis_size);
		self
	}
}

impl<T> Widget<T> for Separator {
	fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut T, _env: &druid::Env) {
		// do nothing
	}

	fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &T, _env: &druid::Env) {
		// do nothing
	}

	fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &druid::Env) {
		// do nothing
	}

	fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &T, env: &druid::Env) -> druid::Size {
		match self.direction {
			Axis::Horizontal => {
				let height = match &self.cross_axis_size {
					CrossAxisSize::Fraction(fu) => {
						bc.max().height * fu.resolve(env)
					},
					CrossAxisSize::Absolute(au) => {
						au.resolve(env)
					}
				};
				Size::new(self.size.resolve(env), height)
			},
			Axis::Vertical => {
				let width = match &self.cross_axis_size {
					CrossAxisSize::Fraction(fu) => {
						bc.max().width * fu.resolve(env)
					},
					CrossAxisSize::Absolute(au) => {
						au.resolve(env)
					}
				};
				Size::new(width, self.size.resolve(env))
			},
		}
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &T, env: &druid::Env) {
		let size = ctx.size().to_rect();
		ctx.fill(size, &self.colour.resolve(env));
	}
}