use druid::{Widget, Data, widget::Label};

use crate::env;

pub fn small_input_label<T>(label: Label<T>) -> impl Widget<T> where T: Data {
	label
		.with_text_size(env::THEME_SIZE_SMALL_LABEL)
		.with_text_color(druid::theme::FOREGROUND_DARK)
}
