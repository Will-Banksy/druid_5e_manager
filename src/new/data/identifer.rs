use druid::Data;

pub const ID_5E_STAT_ABILITY_SCORE_STRENGTH: Identifier = Identifier::new("5e.ability_score.strength");
pub const ID_5E_STAT_ABILITY_SCORE_DEXTERITY: Identifier = Identifier::new("5e.ability_score.dexterity");
pub const ID_5E_STAT_ABILITY_SCORE_CONSTITUTION: Identifier = Identifier::new("5e.ability_score.constitution");
pub const ID_5E_STAT_ABILITY_SCORE_INTELLIGENCE: Identifier = Identifier::new("5e.ability_score.intelligence");
pub const ID_5E_STAT_ABILITY_SCORE_WISDOM: Identifier = Identifier::new("5e.ability_score.wisdom");
pub const ID_5E_STAT_ABILITY_SCORE_CHARISMA: Identifier = Identifier::new("5e.ability_score.charisma");

#[derive(PartialEq, Clone, Data, Hash, Eq)]
pub struct Identifier {
	id: &'static str
}

impl Identifier {
	pub const fn new(id: &'static str) -> Self { // TODO: Compute a hash of the string and store that instead to speed up computations
		Identifier { id }
	}

	pub fn lens(&self) -> druid::lens::Index<&Identifier> {
		druid::lens::Index::new(self)
	}
}

impl From<&'static str> for Identifier {
    fn from(value: &'static str) -> Self {
        Identifier { id: value }
    }
}
