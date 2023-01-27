use druid::{Data, Lens};

/// Dynamic Value: Basically, a calculated value
/// Discards changes to `dyn_val` through lenses
/// NOTE: Currently unused
pub struct DynVal<T> where T: Clone + Data {
	pub base_val: T,
	pub dyn_val: T,
}

#[allow(non_camel_case_types)]
pub mod dyn_val_lenses {
	pub struct base_val;
	pub struct dyn_val;
}

impl<T> Lens<DynVal<T>, T> for dyn_val_lenses::base_val where T: Clone + Data {
    fn with<V, F: FnOnce(&T) -> V>(&self, data: &DynVal<T>, f: F) -> V {
        f(&data.base_val)
    }

    fn with_mut<V, F: FnOnce(&mut T) -> V>(&self, data: &mut DynVal<T>, f: F) -> V {
        f(&mut data.base_val)
    }
}

impl<T> Lens<DynVal<T>, T> for dyn_val_lenses::dyn_val where T: Clone + Data {
    fn with<V, F: FnOnce(&T) -> V>(&self, data: &DynVal<T>, f: F) -> V {
        f(&data.dyn_val)
    }

    fn with_mut<V, F: FnOnce(&mut T) -> V>(&self, data: &mut DynVal<T>, f: F) -> V {
		let val = data.dyn_val.clone();
        let ret = f(&mut data.dyn_val);
		data.dyn_val = val;
		ret
    }
}

#[allow(non_upper_case_globals)]
impl<T> DynVal<T> where T: Clone + Data {
	pub const base_val: dyn_val_lenses::base_val = dyn_val_lenses::base_val;
	pub const dyn_val: dyn_val_lenses::dyn_val = dyn_val_lenses::dyn_val;
}
