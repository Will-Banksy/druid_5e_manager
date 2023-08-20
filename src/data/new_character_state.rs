//! Contains a sorta draft for a potential new design for storing character state

use std::ops::{Add, Sub, Mul};

use im::HashMap;

use super::stat_modifier::StatModifier;

pub struct NewCharacterState {
	pub name: String,
	// This is an issue... Can't store arbitrary data types in a hashmap... And I also can't use a trait for numeric types cause they're not object safe... (as in requiring like Add<Output = Self> is not object safe)
	// Gonna need an enum... Might require redesign
	pub stats: HashMap<String, (f32, StatModifier<f32>)>
}