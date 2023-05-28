use druid::Data;

use self::{armour::Armour, feat::Feat};

pub mod armour;
pub mod item;
pub mod feat;

#[derive(Data, Clone)]
pub struct InternalSource {
	armours: im::Vector<Armour>,
	feats: im::Vector<Feat>
}