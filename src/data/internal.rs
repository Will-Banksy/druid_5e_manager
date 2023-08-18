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

#[derive(Data, Clone)]
pub enum SourceContentItem {
	ArmourItem(Armour),
	FeatItem(Feat)
}

impl SourceContentItem {
	pub fn display_name(&self) -> String {
		match self {
			SourceContentItem::ArmourItem(armour) => armour.name.clone(),
			SourceContentItem::FeatItem(feat) => feat.name.clone(),
		}
	}
}

#[derive(Data, Clone, PartialEq)]
pub enum SourceContentType {
	ArmourType,
	FeatType
}

impl SourceContentType {
	pub fn get_type_string(&self) -> String {
		match self {
			SourceContentType::ArmourType => "Armour".into(),
			SourceContentType::FeatType => "Feats".into(),
		}
	}

	pub fn get_singular_string(&self) -> String {
		match self {
			SourceContentType::ArmourType => "Armour".into(),
			SourceContentType::FeatType => "Feat".into(),
		}
	}
}

#[derive(Data, Clone, Lens)]
pub struct SourceContentCollection {
	pub content_type: SourceContentType,
	pub content: im::Vector<SourceContentItem>
}

impl SourceContentCollection {
	pub fn empty(content_type: SourceContentType) -> Self {
		SourceContentCollection { content_type, content: im::Vector::new() }
	}

	pub fn new(content_type: SourceContentType, content: im::Vector<SourceContentItem>) -> Self {
		SourceContentCollection { content_type, content }
	}
}

#[derive(Data, Clone, Lens)]
pub struct InternalSource {
	pub name: String,
	pub category: SourceCategory,
	pub content: im::Vector<SourceContentCollection>,
}