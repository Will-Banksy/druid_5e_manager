use druid::Lens;

use super::{AppData, internal::{InternalSource, SourceContentCollection, SourceContentItem}};

pub fn into_sources() -> impl Lens<AppData, (usize, im::Vector<InternalSource>)> {
	druid::lens::Map::new(
		|data: &AppData| (data.uistate.selected_source, data.sources.clone()),
		|data: &mut AppData, inner: (usize, im::Vector<InternalSource>)| {
			data.sources = inner.1;
			data.uistate.selected_source = inner.0;

			// Make sure the selected source array is one that exists in the new source
			let avail_content_len = data.sources[data.uistate.selected_source].content.len();
			if data.uistate.selected_source_array >= avail_content_len && avail_content_len != 0 {
				data.uistate.selected_source_array = avail_content_len - 1;
			}

			// Make sure the selected source content item is one that exists in the new source
			if let Some(avail_content_items) = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array) {
				if data.uistate.selected_source_array_item >= avail_content_items.content.len() && avail_content_items.content.len() != 0 {
					data.uistate.selected_source_array_item = avail_content_items.content.len() - 1;
				}
			}
		}
	)
}

pub fn into_selected_source_content() -> impl Lens<AppData, (usize, im::Vector<SourceContentCollection>)> {
	druid::lens::Map::new(
		|data: &AppData| (data.uistate.selected_source_array, data.sources[data.uistate.selected_source].content.clone()),
		|data: &mut AppData, inner: (usize, im::Vector<SourceContentCollection>)| {
			data.sources[data.uistate.selected_source].content = inner.1;
			data.uistate.selected_source_array = inner.0;

			// Make sure the selected source content item is one that exists in the new source collection
			if let Some(avail_content_items) = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array) {
				if data.uistate.selected_source_array_item >= avail_content_items.content.len() && avail_content_items.content.len() != 0 {
					data.uistate.selected_source_array_item = avail_content_items.content.len() - 1;
				}
			}
		}
	)
}

pub fn into_selected_source_content_items() -> impl Lens<AppData, Option<(usize, im::Vector<SourceContentItem>)>> {
	druid::lens::Map::new(
		|data: &AppData| {
			if let Some(content_collection) = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array) {
				Some((data.uistate.selected_source_array_item, content_collection.content.clone()))
			} else {
				None
			}
		},
		|data: &mut AppData, inner: Option<(usize, im::Vector<SourceContentItem>)>| {
			if let Some((inner_i, inner_data)) = inner {
				if let Some(content_collection) = data.sources[data.uistate.selected_source].content.get_mut(data.uistate.selected_source_array) {
					content_collection.content = inner_data;
				}
				data.uistate.selected_source_array_item = inner_i;
			}
		}
	)
}

pub fn into_selected_source_content_item() -> impl Lens<AppData, Option<SourceContentItem>> {
	druid::lens::Map::new(
		|data: &AppData| {
			if let Some(content_collection) = data.sources[data.uistate.selected_source].content.get(data.uistate.selected_source_array) {
				if let Some(item) = content_collection.content.get(data.uistate.selected_source_array_item) {
					Some(item.clone())
				} else {
					None
				}
			} else {
				None
			}
		},
		|data: &mut AppData, inner: Option<SourceContentItem>| {
			if let Some(content_collection) = data.sources[data.uistate.selected_source].content.get_mut(data.uistate.selected_source_array) {
				if let Some(item) = content_collection.content.get_mut(data.uistate.selected_source_array_item) {
					if let Some(inner_item) = inner {
						*item = inner_item
					}
				}
			}
		}
	)
}