// TODO: Create an Advanced List that acts like a List but with some features
//     1. Automatically adds a button (or additional empty item) to add elements to the List (Hm actually not sure on this one)
//     2. Supports filtering, optionally via stuff in env or data
//     3. Supports sorting, optionally via stuff in env or data
//
//     I need to think about what exactly I want here and what the use cases would be and also should it be a list or a table?
//         Maybe I can make an advanced table that uses an advanced list under the hood...

use std::cmp::Ordering;

use druid::{Widget, Data, widget::{Axis, ListIter}, WidgetPod, LifeCycle, BoxConstraints, Rect, Point, Size, Selector, Event};

pub const ADV_LIST_SELECT: Selector<usize> = Selector::new("druid_5e_manager.notifications.accordion-toggle");

/// An advanced version of druid's List widget that supports selection and filtering. Or, will do. Hopefully
pub struct AdvancedList<T> {
	constructor: Box<dyn Fn() -> Box<dyn Widget<T>>>,
	children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
	axis: Axis,
	old_bc: BoxConstraints
}

impl<T> AdvancedList<T> {
	pub fn vertical<W: Widget<T> + 'static>(constructor: impl Fn() -> W + 'static) -> Self {
		AdvancedList {
			constructor: Box::new(move || Box::new(constructor())),
			children: Vec::new(),
			axis: Axis::Vertical,
			old_bc: BoxConstraints::tight(Size::ZERO),
		}
	}

	pub fn horizontal<W: Widget<T> + 'static>(constructor: impl Fn() -> W + 'static) -> Self {
		AdvancedList {
			axis: Axis::Horizontal,
			..AdvancedList::vertical(constructor)
		}
	}

	pub fn update_children(&mut self, data: &impl ListIter<T>) -> bool {
		let len = self.children.len();
		match len.cmp(&data.data_len()) {
			Ordering::Greater => self.children.truncate(data.data_len()),
			Ordering::Less => data.for_each(|_, i| {
				if i >= len {
					let child = WidgetPod::new((self.constructor)());
					self.children.push(child);
				}
			}),
			Ordering::Equal => (),
		}
		len != data.data_len()
	}
}

impl<T, E> Widget<(usize, T)> for AdvancedList<AdvListData<E>> where E: Data, T: ListIter<E> {
	fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut (usize, T), env: &druid::Env) {
		let mut children = self.children.iter_mut();
		let sel_idx = data.0;
		data.1.for_each_mut(|child_data, i| {
			if let Some(child) = children.next() {
				child.event(ctx, event, &mut AdvListData::new(i, sel_idx, child_data.clone()), env)
			}
		});

		if let Event::Notification(notif) = event {
			if notif.is(ADV_LIST_SELECT) {
				let idx = notif.get(ADV_LIST_SELECT).unwrap();
				data.0 = *idx;
			}
		}
	}

	fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &(usize, T), env: &druid::Env) {
		let temp = {
			let mut v = im::Vector::new();
			data.1.for_each(|child_data, i| v.push_back(AdvListData::new(i, data.0, child_data.clone())));
			v
		};
		if let LifeCycle::WidgetAdded = event {
			if self.update_children(&temp) {
				ctx.children_changed()
			}
		}

		let mut children = self.children.iter_mut();
		data.1.for_each(|child_data, i| {
			if let Some(child) = children.next() {
				child.lifecycle(ctx, event, &AdvListData::new(i, data.0, child_data.clone()), env)
			}
		});
	}

	fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &(usize, T), data: &(usize, T), env: &druid::Env) {
		let mut children = self.children.iter_mut();
		data.1.for_each(|child_data, i| {
			if let Some(child) = children.next() {
				child.update(ctx, &AdvListData::new(i, data.0, child_data.clone()), env)
			}
		});

		let temp = {
			let mut v = im::Vector::new();
			data.1.for_each(|child_data, i| v.push_back(AdvListData::new(i, data.0, child_data.clone())));
			v
		};
		if self.update_children(&temp) {
			ctx.children_changed();
		}
	}

	fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &(usize, T), env: &druid::Env) -> druid::Size {
		let axis = self.axis;
        // let spacing = self.spacing.resolve(env);
        let mut minor = axis.minor(bc.min());
        let mut major_pos = 0.0;
        let mut paint_rect = Rect::ZERO;

        let bc_changed = self.old_bc != *bc;
        self.old_bc = *bc;

        let mut children = self.children.iter_mut();
        let child_bc = match axis {
			Axis::Horizontal => BoxConstraints::new(
				Size::new(0., bc.min().height),
				Size::new(f64::INFINITY, bc.max().height)
			),
			Axis::Vertical => BoxConstraints::new(
				Size::new(bc.min().width, 0.),
				Size::new(bc.max().width, f64::INFINITY)
			)
		};//axis.constraints(bc, 0., f64::INFINITY);
        data.1.for_each(|child_data, i| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };

            let child_size = if bc_changed || child.layout_requested() {
                child.layout(ctx, &child_bc, &AdvListData::new(i, data.0, child_data.clone()), env)
            } else {
                child.layout_rect().size()
            };

            let child_pos: Point = axis.pack(major_pos, 0.).into();
            child.set_origin(ctx, child_pos);
            paint_rect = paint_rect.union(child.paint_rect());
            minor = minor.max(axis.minor(child_size));
            major_pos += axis.major(child_size);// + spacing;
        });

        // correct overshoot at end.
        // major_pos -= spacing;

        let my_size = bc.constrain(Size::from(axis.pack(major_pos, minor)));
        let insets = paint_rect - my_size.to_rect();
        ctx.set_paint_insets(insets);
        // trace!("Computed layout: size={}, insets={:?}", my_size, insets);
        my_size
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &(usize, T), env: &druid::Env) {
		let mut children = self.children.iter_mut();
		data.1.for_each(|child_data, i| {
			if let Some(child) = children.next() {
				child.paint(ctx, &AdvListData::new(i, data.0, child_data.clone()), env)
			}
		});
	}
}

#[derive(Data, Clone)]
pub struct AdvListData<T> {
	pub this_idx: usize,
	pub selected_idx: usize,
	pub data: T
}

impl<T> AdvListData<T> {
	pub fn new(this_idx: usize, selected_idx: usize, data: T) -> Self {
		Self { this_idx, selected_idx, data }
	}
}