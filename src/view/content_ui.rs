use druid::{Widget, widget::{Flex, Label, CrossAxisAlignment, LineBreaking, Painter}, WidgetExt, Env, EventCtx, PaintCtx, RenderContext};
use im::Vector;

use crate::{data::{AppData, internal::{InternalSource, SourceCategory, armour::Armour}, transitive_app_state::SelectedSourceArray}, env};
use crate::utils::string_casing_ext::StringCasingExt;

use super::{components::common::labelled_flex_section, widgets::{advanced_list::{AdvancedList, AdvListData, ADV_LIST_SELECT}, hover::Hover}};

pub fn build_content_ui() -> impl Widget<AppData> {
	Flex::column()
		.with_child(
			Label::new("Content Manager").with_text_size(env::THEME_SIZE_TITLE).align_left()
		)
		.with_default_spacer()
		.with_flex_child(
			Flex::row()
				.with_child(
					labelled_flex_section(Label::new("SOURCES"), Flex::column()
						// .with_child(
						// 	ListFilter::new(
						// 		List::new(|| {
						// 			Flex::row()
						// 				.with_child(
						// 					Label::new(|src: &InternalSource, _env: &Env| {
						// 						src.name.clone()
						// 					})
						// 					.with_line_break_mode(LineBreaking::WordWrap) // TODO: Doesn't seem to work???
						// 				)
						// 				// .debug_paint_layout()
						// 		}),
						// 	|item: &InternalSource, filter: &SourceCategory| {
						// 		if item.category == *filter {
						// 			true
						// 		} else {
						// 			false
						// 		}
						// 	})
						// 	.lens(druid::lens::Map::new(
						// 		|data: &AppData| (data.sources.clone(), SourceCategory::Core),
						// 		|data: &mut AppData, inner: (Vector<InternalSource>, SourceCategory)| data.sources = inner.0
						// 	))
						// )
						.with_child(
							AdvancedList::vertical(|| {
								Hover::new(
									Flex::row()
										.with_flex_child(
											Label::new(|src: &AdvListData<InternalSource>, _env: &Env| {
												src.data.name.clone()
											})
											.with_line_break_mode(LineBreaking::WordWrap), // TODO: Doesn't seem to work???
											1.0
										)
										.padding((4.0, 2.0))
										.on_click(|ctx: &mut EventCtx, data: &mut AdvListData<InternalSource>, _env: &Env| {
											ctx.submit_notification(ADV_LIST_SELECT.with(data.this_idx))
										})
										.background(Painter::new(|ctx: &mut PaintCtx, data: &AdvListData<InternalSource>, env: &Env| {
											let bounds = ctx.size().to_rect();
											let colour = if ctx.is_hot() || data.selected_idx == data.this_idx {
												env.get(druid::theme::BACKGROUND_LIGHT)
											} else {
												env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
											};
											ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
										}))
									)
										// .debug_paint_layout()
							}).lens(druid::lens::Map::new(
								|data: &AppData| (data.transitive_app_state.selected_source, data.sources.clone()),
								|data: &mut AppData, inner: (usize, Vector<InternalSource>)| {
									data.sources = inner.1;
									data.transitive_app_state.selected_source = inner.0;
								}
							))
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).fix_width(240.0)
				)
				.with_default_spacer()
				.with_child(
					labelled_flex_section(Label::new("SOURCE CONTENT"), Flex::column()
						.with_child(
							Label::new(|data: &AppData, _: &_| {
								format!("Content for {}", data.sources[data.transitive_app_state.selected_source].name)
							}).with_line_break_mode(LineBreaking::WordWrap)
						)
						.with_default_spacer()
						.with_child(
							Flex::row()
								.with_flex_child(
									Label::new(SelectedSourceArray::ArmourArray.get_string().to_ascii_capitalised()),
									1.0
								)
								.padding((4.0, 2.0))
								.on_click(|_: &mut EventCtx, data: &mut AppData, _: &_| {
									data.transitive_app_state.selected_source_array = SelectedSourceArray::ArmourArray
								})
								.background(Painter::new(|ctx: &mut PaintCtx, data: &AppData, env: &Env| {
									let bounds = ctx.size().to_rect();
									let colour = if ctx.is_hot() || data.transitive_app_state.selected_source_array == SelectedSourceArray::ArmourArray {
										env.get(druid::theme::BACKGROUND_LIGHT)
									} else {
										env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
									};
									ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
								}))
						)
						.with_child(
							Flex::row()
								.with_flex_child(
									Label::new(SelectedSourceArray::FeatsArray.get_string().to_ascii_capitalised()),
									1.0
								)
								.padding((4.0, 2.0))
								.on_click(|_: &mut EventCtx, data: &mut AppData, _: &_| {
									data.transitive_app_state.selected_source_array = SelectedSourceArray::FeatsArray
								})
								.background(Painter::new(|ctx: &mut PaintCtx, data: &AppData, env: &Env| {
									let bounds = ctx.size().to_rect();
									let colour = if ctx.is_hot() || data.transitive_app_state.selected_source_array == SelectedSourceArray::FeatsArray {
										env.get(druid::theme::BACKGROUND_LIGHT)
									} else {
										env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
									};
									ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
								}))

						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).fix_width(240.0)
				)
				.cross_axis_alignment(CrossAxisAlignment::Fill)
				.expand_height(),
			1.0
		)
		.cross_axis_alignment(CrossAxisAlignment::Start)
		.expand()
		.padding(6.0)
		// .debug_paint_layout()
}