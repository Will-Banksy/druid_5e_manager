use druid::{Widget, widget::{Flex, Label, CrossAxisAlignment, LineBreaking, Painter, Maybe}, WidgetExt, Env, EventCtx, PaintCtx, RenderContext};

use crate::{data::{AppData, internal::{InternalSource, SourceContentCollection, SourceContentItem}, lenses}, env};

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
											.with_line_break_mode(LineBreaking::WordWrap),
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
							}).lens(lenses::into_sources())
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).fix_width(240.0)
				)
				.with_default_spacer()
				.with_child(
					labelled_flex_section(Label::new("SOURCE CONTENT"), Flex::column()
						.with_child(
							Label::new(|data: &AppData, _: &_| {
								format!("Content for \"{}\"", data.sources[data.uistate.selected_source].name)
							}).with_line_break_mode(LineBreaking::WordWrap).with_font(druid::theme::UI_FONT_ITALIC)
						)
						.with_default_spacer()
						.with_child(
							AdvancedList::vertical(|| {
								Hover::new(
									Flex::row()
										.with_flex_child(
											Label::new(|src: &AdvListData<SourceContentCollection>, _env: &Env| {
												src.data.content_type.get_type_string()
											})
											.with_line_break_mode(LineBreaking::WordWrap),
											1.0
										)
										.padding((4.0, 2.0))
										.on_click(|ctx: &mut EventCtx, data: &mut AdvListData<SourceContentCollection>, _env: &Env| {
											ctx.submit_notification(ADV_LIST_SELECT.with(data.this_idx))
										})
										.background(Painter::new(|ctx: &mut PaintCtx, data: &AdvListData<SourceContentCollection>, env: &Env| {
											let bounds = ctx.size().to_rect();
											let colour = if ctx.is_hot() || data.selected_idx == data.this_idx {
												env.get(druid::theme::BACKGROUND_LIGHT)
											} else {
												env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
											};
											ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
										}))
									)
							}).lens(lenses::into_selected_source_content())
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).fix_width(240.0)
				)
				.with_default_spacer()
				.with_child(
					labelled_flex_section(Label::new(|data: &AppData, _: &_| {
						let source_collection = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array);
						if let Some(source_collection) = source_collection {
							source_collection.content_type.get_type_string().to_ascii_uppercase()
						} else {
							"".into()
						}
					}), Flex::column()
						.with_child(
							Maybe::new(|| {
								AdvancedList::vertical(|| {
									Hover::new(
										Flex::row()
											.with_flex_child(
												Label::new(|src: &AdvListData<SourceContentItem>, _env: &Env| {
													match &src.data {
														SourceContentItem::ArmourItem(armour) => armour.name.clone(),
														SourceContentItem::FeatItem(feat) => feat.name.clone()
													}
												})
												.with_line_break_mode(LineBreaking::WordWrap),
												1.0
											)
											.padding((4.0, 2.0))
											.on_click(|ctx: &mut EventCtx, data: &mut AdvListData<SourceContentItem>, _env: &Env| {
												ctx.submit_notification(ADV_LIST_SELECT.with(data.this_idx))
											})
											.background(Painter::new(|ctx: &mut PaintCtx, data: &AdvListData<SourceContentItem>, env: &Env| {
												let bounds = ctx.size().to_rect();
												let colour = if ctx.is_hot() || data.selected_idx == data.this_idx {
													env.get(druid::theme::BACKGROUND_LIGHT)
												} else {
													env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
												};
												ctx.fill(bounds.to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS)), &colour);
											}))
										)
								})
							}, || {
								Flex::column()
							}).lens(lenses::into_selected_source_content_items())
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).fix_width(240.0)
				)
				.with_default_spacer()
				.with_flex_child(
					labelled_flex_section(Label::new(|data: &AppData, _: &_| {
						let source_collection = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array);
						if let Some(source_collection) = source_collection {
							if let Some(item) = source_collection.content.get(data.uistate.selected_source_array_item) {
								format!("{}: {}", source_collection.content_type.get_singular_string(), item.display_name()).to_ascii_uppercase()
							} else {
								"".into()
							}
						} else {
							"".into()
						}
					}), Flex::column()
						.with_child(
							Maybe::new(|| {
								Flex::column()
									.with_child(Label::new("ContentItemView is not yet implemented"))
							}, || {
								Flex::column()
							}).lens(lenses::into_selected_source_content_item())
						)
						.cross_axis_alignment(CrossAxisAlignment::Fill)
					).expand(),
					1.0
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