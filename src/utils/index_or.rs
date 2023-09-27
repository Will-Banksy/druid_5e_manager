use std::hash::Hash;

pub trait IndexOr<I> where I: ?Sized {
	type Output: ?Sized;

	fn index_or(&self, index: I) -> Option<&Self::Output>;
}

pub trait IndexOrMut<I>: IndexOr<I> {
	fn index_or_mut(&mut self, index: I) -> Option<&mut Self::Output>;
}

// This way of implementing the trait is unfortunately impossible due to it being impossible
// with trait bounds to determine whether an item exists within a generic collection
// impl<I, C> IndexOr<I> for C where C: ops::Index<I> + ops::IndexMut<I> {
// 	type Output = C::Output;

// 	fn index_or<T>(&self, index: I) -> Option<&Self::Output> {
// 	}
// }

impl<V> IndexOr<usize> for im::Vector<V> where V: Clone {
	type Output = V;

	fn index_or(&self, index: usize) -> Option<&Self::Output> {
		self.get(index)
	}
}

impl<V> IndexOrMut<usize> for im::Vector<V> where V: Clone {
	fn index_or_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
		self.get_mut(index)
	}
}

impl<I, V> IndexOr<&I> for im::HashMap<I, V> where I: Hash + Eq, V: Clone {
	type Output = V;

	fn index_or(&self, index: &I) -> Option<&Self::Output> {
		self.get(&index)
	}
}

impl<I, V> IndexOrMut<&I> for im::HashMap<I, V> where I: Hash + Eq + Clone, V: Clone {
	fn index_or_mut(&mut self, index: &I) -> Option<&mut Self::Output> {
		self.get_mut(&index)
	}
}

impl<I, V> IndexOr<I> for im::HashMap<I, V> where I: Hash + Eq, V: Clone {
	type Output = V;

	fn index_or(&self, index: I) -> Option<&Self::Output> {
		self.get(&index)
	}
}

impl<I, V> IndexOrMut<I> for im::HashMap<I, V> where I: Hash + Eq + Clone, V: Clone {
	fn index_or_mut(&mut self, index: I) -> Option<&mut Self::Output> {
		self.get_mut(&index)
	}
}