use self::{armour::Armour, feat::Feat};

pub mod armour;
pub mod item;
pub mod feat;

pub struct InternalSource {
	armours: Vec<Armour>,
	feats: Vec<Feat>
}