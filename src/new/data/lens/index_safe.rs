use std::ops;

use druid::Lens;

/// `Lens` for indexing containers
#[derive(Debug, Copy, Clone)]
pub struct IndexSafe<I> {
    index: I,
}

impl<I> IndexSafe<I> {
    /// Construct a lens that accesses a particular index
    ///
    /// See also `LensExt::index`.
    pub fn new(index: I) -> Self {
        Self { index }
    }
}

impl<T, I> Lens<T, T::Output> for IndexSafe<I>
where
    T: ?Sized + ops::Index<I> + ops::IndexMut<I>,
    I: Clone,
{
    fn with<V, F: FnOnce(&T::Output) -> V>(&self, data: &T, f: F) -> V {
        f(&data[self.index.clone()])
    }
    fn with_mut<V, F: FnOnce(&mut T::Output) -> V>(&self, data: &mut T, f: F) -> V {
        f(&mut data[self.index.clone()])
    }
}

// TODO: Create a trait that is implemented by collections I want to implement it that will basically be the same as the ops::Index<I> trait
// but will return Option<T> so invalid indicies do not cause a panic