use paste::paste;

/// Describes a modifier to a stat, such as move speed. 3 ways of modifying the stat are available: adding to
/// `base`, `flat` and `multiplier`. The final score is calculated from the initial score with: `(initial + base) * multiplier + flat`.
/// Each value is actually stored as an array of tuples of the value and a string describing the source of the modifier
struct StatModifier<T> {
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
				if let Some(i) = self.$mod_type.iter().position(|(_, src)| *src == source) {
					self.$mod_type.remove(i);
				}
			}

			fn [<with_removed_ $mod_type>](mut self, source: impl Into<String>) -> Self {
				self.[<remove_ $mod_type>](source);
				self
			}
		}
	};
}

impl<T> StatModifier<T> where T: Clone {
	fn new() -> Self {
		Self {
			base: im::Vector::new(),
			flat: im::Vector::new(),
			multiplier: im::Vector::new()
		}
	}

	stat_add_fn!(base);
	stat_add_fn!(flat);
	stat_add_fn!(multiplier);

	stat_rem_fn!(base);
	stat_rem_fn!(flat);
	stat_rem_fn!(multiplier);
}