use druid::{Data, Lens};

use self::{armour::Armour, feat::Feat};

pub mod armour;
pub mod item;
pub mod feat;

#[derive(Clone, Data, PartialEq)]
pub enum SourceCategory {
	Core,
	Supplements,
	Homebrew
}

#[derive(Data, Clone, Lens)]
pub struct InternalSource {
	pub name: String,
	pub category: SourceCategory,
	pub armours: im::Vector<Armour>,
	pub feats: im::Vector<Feat>
}