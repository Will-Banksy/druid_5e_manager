use std::ops::{Add, Mul};
use num_traits::{Zero, One};

use paste::paste;

/// Describes a modifier to a stat, such as move speed. 3 ways of modifying the stat are available: adding to
/// `base`, `flat` and `multiplier`. The final score is calculated from the initial score with: `(initial + base) * multiplier + flat`.
/// Each value is actually stored as an array of tuples of the value and a string describing the source of the modifier
pub struct StatModifier<T> where T: Clone + Copy + Add<Output = T> + Mul<Output = T> + Zero + One {
	base: im::Vector<(T, String)>,
	flat: im::Vector<(T, String)>,
	multiplier: im::Vector<(T, String)>,
}

macro_rules! stat_add_fn {
	($mod_type:ident) => {
		paste! {
			fn [<add_ $mod_type>](&mut self, source: impl Into<String>, value: T) {
				self.$mod_type.push_back((value, source.into()));
			}

			fn [<with_added_ $mod_type>](mut self, source: impl Into<String>, value: T) -> Self {
				self.[<add_ $mod_type>](source, value);
				self
			}
		}
	};
}

macro_rules! stat_rem_fn {
	($mod_type:ident) => {
		paste! {
			fn [<remove_ $mod_type>](&mut self, source: impl Into<String>) {
				let source = source.into();
				self.$mod_type.retain(|(_, src)| *src != source);
			}

			fn [<with_removed_ $mod_type>](mut self, source: impl Into<String>) -> Self {
				self.[<remove_ $mod_type>](source);
				self
			}
		}
	};
}

macro_rules! stat_has_fn {
	($mod_type:ident) => {
		paste! {
			fn [<has_ $mod_type>](&self, source: impl Into<String>) -> bool {
				let source = source.into();
				self.$mod_type.iter().any(|b| b.1 == source)
			}
		}
	};
}

impl<T> StatModifier<T> where T: Clone + Copy + Add<Output = T> + Mul<Output = T> + Zero + One {
	fn new() -> Self {
		Self {
			base: im::Vector::new(),
			flat: im::Vector::new(),
			multiplier: im::Vector::new(),
		}
	}

	/// Uses this `StatModifier` to modify the passed-in stat
	fn modify(&self, initial: T) -> T {
		let base_mod = self.base.iter().fold(T::zero(), |acc, b| acc + b.0);
		let mult_mod = self.multiplier.iter().fold(T::one(), |acc, m| acc * m.0);
		let flat_mod = self.flat.iter().fold(T::zero(), |acc, f| acc + f.0);

		(initial + base_mod) * mult_mod + flat_mod
	}

	stat_add_fn!(base);
	stat_add_fn!(flat);
	stat_add_fn!(multiplier);

	stat_rem_fn!(base);
	stat_rem_fn!(flat);
	stat_rem_fn!(multiplier);

	stat_has_fn!(base);
	stat_has_fn!(flat);
	stat_has_fn!(multiplier);
}