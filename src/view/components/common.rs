use druid::{Widget, Data, widget::{Label, Flex, Painter}, WidgetExt, KeyOrValue, PaintCtx, Color, Env, RenderContext};

use crate::env;

pub fn painter_background<T>(col: impl Into<KeyOrValue<Color>>) -> Painter<T> {
	let col = col.into();
	Painter::new(move |ctx: &mut PaintCtx, _data: &_, env: &Env| {
		let bounds = ctx.size().to_rect();
		let colour: Color = if ctx.is_hot() && false { Color::Rgba32(0x777700ff) } else {
			match &col {
				KeyOrValue::Concrete(c) => c.clone(),
				KeyOrValue::Key(k) => env.get(k)
			}
		};
		ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
	})
}

pub fn small_input_label<T>(label: Label<T>) -> impl Widget<T> where T: Data {
	label
		.with_text_size(env::THEME_SIZE_SMALL_LABEL)
		.with_text_color(druid::theme::FOREGROUND_DARK)
}

pub fn labelled_section<T>(label: impl Into<Label<T>>, content: impl Widget<T> + 'static) -> impl Widget<T> where T: Data {
	Flex::column()
		.with_child(label.into())
		.with_default_spacer()
		.with_child(content)
		.padding(env::THEME_INSETS)
		.background(painter_background(druid::theme::BACKGROUND_DARK))
}

pub fn labelled_flex_section<T>(label: impl Into<Label<T>>, content: impl Widget<T> + 'static) -> impl Widget<T> where T: Data {
	Flex::column()
		.with_child(label.into())
		.with_default_spacer()
		.with_flex_child(content, 1.0)
		.padding(env::THEME_INSETS)
		.background(painter_background(druid::theme::BACKGROUND_DARK))
}