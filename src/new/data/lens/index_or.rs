use crate::utils::index_or;

/// `Lens` for indexing containers. Returns `Some` if data could be retrieved at the provided index, `None` otherwise
#[derive(Debug, Copy, Clone)]
pub struct IndexOr<I> {
	index: I,
}

impl<I> IndexOr<I> {
	pub fn new(index: I) -> Self {
		Self { index }
	}

	pub fn with<F, T, R>(&self, data: &T, f: F) -> R where F: FnOnce(Option<&T::Output>) -> R, T: index_or::IndexOr<I>, I: Clone {
		f(data.index_or(self.index.clone()))
	}

	pub fn with_mut<F, T, R>(&self, data: &mut T, f: F) -> R where F: FnOnce(Option<&mut T::Output>) -> R, T: index_or::IndexOrMut<I>, I: Clone {
		f(data.index_or_mut(self.index.clone()))
	}
}

// TODO: Implement either Lens or Prism, so IndexOr can be used much like Index
// Might this help? https://stackoverflow.com/questions/71026147/reference-to-an-option-versus-option-of-a-reference

// macro_rules! impl_indexor_for {
// 	($collection: ty, $index_out: ty) => {
// 		impl<$collection, I> Lens<$collection, $index_out> for IndexOr<I> {
// 			fn with<V, F: FnOnce(&$index_out) -> V>(&self, data: &$collection, f: F) -> V {
// 				f(&data.get(self.index.clone()))
// 			}

// 			fn with<V, F: FnOnce(&mut $index_out) -> V>(&self, data: &mut $collection, f: F) -> V {
// 				f(&mut data.get(self.index.clone()))
// 			}
// 		}
// 	};
// }

// impl<T, I> Lens<T, T::Output> for IndexOr<I>
// where
// 	T: ?Sized + ops::Index<I> + ops::IndexMut<I>,
// 	I: Clone,
// {
// 	fn with<V, F: FnOnce(&T::Output) -> V>(&self, data: &T, f: F) -> V {
// 		f(&data[self.index.clone()])
// 	}
// 	fn with_mut<V, F: FnOnce(&mut T::Output) -> V>(&self, data: &mut T, f: F) -> V {
// 		f(&mut data[self.index.clone()])
// 	}
// }

// impl<T, I> Lens<T, Option<T::Output>> for IndexOr<I>
// where
// 	T: ?Sized + index_or::IndexOr<I> + index_or::IndexOrMut<I>,
// 	I: Clone,
// 	<T as index_or::IndexOr<I>>::Output: Sized
// {
// 	fn with<V, F: FnOnce(&Option<T::Output>) -> V>(&self, data: &T, f: F) -> V {
// 		// f(data.index_or(&self.index))
// 		todo!()
// 	}

// 	fn with_mut<V, F: FnOnce(&mut Option<T::Output>) -> V>(&self, data: &mut T, f: F) -> V {
// 		todo!()
// 	}
// }

// impl<'a, T, I> Prism<T, &'a T::Output> for IndexOr<I> where T: index_or::IndexOr<I> + index_or::IndexOrMut<I>, I: Clone {
// 	fn get(&self, data: &T) -> Option<&'a T::Output> {
// 		data.index_or(self.index.clone())
// 	}

// 	fn put(&self, data: &mut T, inner: &T::Output) {
// 		todo!()
// 	}
// }
