use std::marker::PhantomData;

use druid::{Widget, Data};

use crate::utils::index_or::{IndexOr, IndexOrMut};

use super::super::data::lens::index_or as lens;

// NOTE: Could probably clean up this a bit, especially all the trait bounds everywhere

pub trait IndexOrExt<I, T>: Widget<Option<T::Output>> + Sized + 'static
where
	I: Clone,
	T: Data + IndexOr<I> + IndexOrMut<I>,
	<T as IndexOr<I>>::Output: Sized + Data
{
	fn index_or(self, index: I) -> IndexOrWrap<I, T, Self> {
		IndexOrWrap::new(self, lens::IndexOr::new(index))
	}
}

impl<I, T, W> IndexOrExt<I, T> for W
where
	I: Clone,
	T: Data + IndexOr<I> + IndexOrMut<I>,
	W: Widget<Option<T::Output>> + 'static,
	<T as IndexOr<I>>::Output: Sized + Data
{}

pub struct IndexOrWrap<I, T, W>
where
	I: Clone,
	T: Data + IndexOr<I> + IndexOrMut<I>,
	W: Widget<Option<T::Output>>,
	<T as IndexOr<I>>::Output: Sized + Data
{
	child: W,
	lens: lens::IndexOr<I>,
	t_marker: PhantomData<T>,
}

impl<I, T, W> IndexOrWrap<I, T, W>
where
	I: Clone,
	T: Data + IndexOr<I> + IndexOrMut<I>,
	W: Widget<Option<T::Output>>,
	<T as IndexOr<I>>::Output: Sized + Data
{
	pub fn new(child: W, lens: lens::IndexOr<I>) -> Self {
		Self {
			child,
			lens,
			t_marker: PhantomData,
		}
	}
}

impl<I, T, W> Widget<T> for IndexOrWrap<I, T, W>
where
	I: Clone,
	T: Data + IndexOr<I> + IndexOrMut<I>,
	W: Widget<Option<T::Output>>,
	<T as IndexOr<I>>::Output: Sized + Data
{
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
        let child = &mut self.child;
        self.lens
            .with_mut(data, |data| /*if let Some(data) = data*/ { child.event(ctx, event, &mut data.cloned(), env) })
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &T, env: &druid::Env) {
        let child = &mut self.child;
        self.lens
            .with(data, |data| /*if let Some(data) = data*/ { child.lifecycle(ctx, event, &mut data.cloned(), env) })
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        let child = &mut self.child;
        let lens = &self.lens;
        lens.with(old_data, |old_data| {
            lens.with(data, |data| {
				if let Some(s_old_data) = old_data {
					if let Some(s_data) = data {
						if ctx.has_requested_update() || !s_old_data.same(s_data) || ctx.env_changed() {
							child.update(ctx, &mut old_data.cloned(), &mut data.cloned(), env);
						}
					}
				}
            })
        })
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> druid::Size {
        let child = &mut self.child;
        self.lens
            .with(data, |data| /*match data { Some(data) =>*/ child.layout(ctx, bc, &data.cloned(), env)/*, _ => druid::Size { width: 0.0, height: 0.0 } }*/)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let child = &mut self.child;
        self.lens.with(data, |data| /*if let Some(data) = data {*/ child.paint(ctx, &data.cloned(), env) /*}*/);
    }
}