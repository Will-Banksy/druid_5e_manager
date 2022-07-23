use std::{sync::Arc, ops::Deref};

use druid::{Lens, Data};

// INFO: So this doesn't help at all
// The problem I am having is in the lenses: When lensing to a MutArc or Arc if it's with_mut it reallocs the Arc... hang on wait MutArc doesn't have any realloc methods... is it even the lens? Something is replacing the old MutArc with a new MutArc with a different alloc
// I actually know little about why this doesn't work

// NOTE: Might wanna rename this to ArcMut as that fits closer with rust's typical type naming

#[derive(Clone, Debug)]
pub struct MutArc<T> {
	datacell: Arc<T>
}

impl<T> MutArc<T> {
	pub fn new(data: T) -> Self {
		Self {
			datacell: Arc::new(data)
		}
	}

	pub fn as_ptr(&self) -> *const T {
		Arc::as_ptr(&self.datacell)
	}
}

impl<T> AsRef<T> for MutArc<T> {
    fn as_ref(&self) -> &T {
		Arc::as_ref(&self.datacell)
    }
}

// impl<T> AsMut<T> for MutArc<T> {
// 	fn as_mut(&mut self) -> &mut T {
// 	}
// }

impl<T: Clone + Data + 'static> Data for MutArc<T> {
    fn same(&self, other: &Self) -> bool {
        self.as_ref().same(other.as_ref())
    }
}

impl<T> Deref for MutArc<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&*self.datacell
	}
}

impl<T> From<T> for MutArc<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}