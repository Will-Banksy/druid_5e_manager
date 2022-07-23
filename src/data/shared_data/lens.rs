use druid::Lens;

use super::{SharedDataItem, SharedData, SHARED_MAP};

macro_rules! shared_data_variant_lens {
	($impling_type: ty, $variant: ident, $internal_type: ty) => {
		impl Lens<SharedData, $internal_type> for $impling_type {
			fn with<V, F: FnOnce(&$internal_type) -> V>(&self, data: &SharedData, f: F) -> V {
				if let SharedDataItem::$variant(n) = SHARED_MAP.read().unwrap().get(&data.uuid).unwrap() {
					f(n)
				} else {
					let impostor = 0;
					f(&impostor)
				}
			}

			fn with_mut<V, F: FnOnce(&mut $internal_type) -> V>(&self, data: &mut SharedData, f: F) -> V {
				if let SharedDataItem::$variant(n) = SHARED_MAP.write().unwrap().get_mut(&data.uuid).unwrap() {
					f(n)
				} else {
					let mut impostor = 0;
					f(&mut impostor)
				}
			}
		}
	};
}


pub mod shared_data_item_lenses {
	pub struct U8Lens;
	pub struct I8Lens;
}

shared_data_variant_lens!(shared_data_item_lenses::U8Lens, U8, u8);
shared_data_variant_lens!(shared_data_item_lenses::I8Lens, I8, i8);

impl SharedData {
	pub const U8_LENS: shared_data_item_lenses::U8Lens = shared_data_item_lenses::U8Lens;
}