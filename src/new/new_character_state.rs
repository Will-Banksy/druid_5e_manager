//! Contains a sorta draft for a potential new design for storing character state

use druid::{Data, Lens};
use druid_widget_nursery::prism::Prism;

use super::stat_modifier::StatModifier;

/// This struct stores the data for a D&D character, including a list of stats, equipment, etc.
/// TODO: finish
#[derive(Clone, Data, Lens)]
pub struct NewCharacterState {
	pub name: String,
	pub stats: im::HashMap<String, Stat>,
	pub equipment: im::Vector<Equipment>
}

#[derive(Clone, Data, Prism)]
pub enum Equipment {
	Armour(Armour),
	Item(Item)
}

#[derive(Clone, Data)]
pub struct Armour {
	pub ac_bonus: i32
}

#[derive(Clone, Data)]
pub struct Item {
	pub damage: i32
}

#[derive(Clone, PartialEq, Data)]
pub enum Stat {
	Int(StatWithModifiers<i32>),
	UInt(StatWithModifiers<u32>),
	Decimal(StatWithModifiers<f32>)
}

#[derive(Clone, PartialEq, Data)]
pub struct StatWithModifiers<T> {
	base_stat: T,
	modifiers: StatModifier<f32> // NOTE: Cause I need decimal numbers for multipliers... is there a better way though
}

/// Implements StatWithModifiers<T>::modified where T is an integral type that can be cast to and from f32 with `as`
macro_rules! impl_modified {
	($num_type:ty) => {
		impl StatWithModifiers<$num_type> {
			pub fn modified(&self) -> $num_type {
				self.modifiers.modify(self.base_stat as f32) as $num_type
			}
		}
	};
}

impl_modified!(i32);
impl_modified!(u32);
impl_modified!(f32);

// NOTE: Revisit at some point, cause this would be nice to get working
// impl<T> StatWithModifiers<T> where T: AsPrimitive<f32> + FromPrimitive {
// 	pub fn modified(&self) -> T {
// 		self.modifiers.modify(self.base_stat.as_())
// 	}
// }