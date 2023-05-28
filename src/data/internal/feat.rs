use druid::Data;

#[derive(Data, Clone)]
pub struct Feat {
	pub name: String,
	pub prerequisite: String,
	pub description: String
}