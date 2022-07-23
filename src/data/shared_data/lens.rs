use druid::Lens;

use super::{SharedDataItem, SharedData, SHARED_MAP};

macro_rules! shared_data_variant_lens {
	($impling_type: ty, $variant: ident, $internal_type: ty) => {
		impl Lens<SharedData, $internal_type> for $impling_type {
			fn with<V, F: FnOnce(&$internal_type) -> V>(&self, data: &SharedData, f: F) -> V {
				if let SharedDataItem::$variant(n) = SHARED_MAP.read().unwrap().get(&data.uuid).unwrap() {
					f(n)
				} else {
					let impostor = Default::default();
					f(&impostor)
				}
			}

			fn with_mut<V, F: FnOnce(&mut $internal_type) -> V>(&self, data: &mut SharedData, f: F) -> V {
				if let SharedDataItem::$variant(n) = SHARED_MAP.write().unwrap().get_mut(&data.uuid).unwrap() {
					let r = f(n);
					let loc: $internal_type = { if let SharedDataItem::$variant(loc) = data.local_copy.read().unwrap().clone() { loc } else { panic!() } };
					if *n != loc {
						data.backup(SharedDataItem::$variant(n.clone()));
					}
					r
				} else {
					let mut impostor = Default::default();
					f(&mut impostor)
				}
			}
		}
	};
}


pub mod shared_data_item_lenses {
	pub struct U8Lens;
	pub struct U16Lens;
	pub struct U32Lens;
	pub struct U64Lens;
	pub struct U128Lens;
	pub struct I8Lens;
	pub struct I16Lens;
	pub struct I32Lens;
	pub struct I64Lens;
	pub struct I128Lens;
	pub struct StringLens;
}

shared_data_variant_lens!(shared_data_item_lenses::U8Lens, U8, u8);
shared_data_variant_lens!(shared_data_item_lenses::U16Lens, U16, u16);
shared_data_variant_lens!(shared_data_item_lenses::U32Lens, U32, u32);
shared_data_variant_lens!(shared_data_item_lenses::U64Lens, U64, u64);
shared_data_variant_lens!(shared_data_item_lenses::U128Lens, U128, u128);
shared_data_variant_lens!(shared_data_item_lenses::I8Lens, I8, i8);
shared_data_variant_lens!(shared_data_item_lenses::I16Lens, I16, i16);
shared_data_variant_lens!(shared_data_item_lenses::I32Lens, I32, i32);
shared_data_variant_lens!(shared_data_item_lenses::I64Lens, I64, i64);
shared_data_variant_lens!(shared_data_item_lenses::I128Lens, I128, i128);
shared_data_variant_lens!(shared_data_item_lenses::StringLens, String, String);

impl SharedData {
	pub const U8_LENS: shared_data_item_lenses::U8Lens = shared_data_item_lenses::U8Lens;
}