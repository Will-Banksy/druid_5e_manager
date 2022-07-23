// pub mod serialisation;
// pub mod arcmut;
pub mod shared_data;

// pub use arcmut::MutArc;

use std::{fmt::Display};

use druid::{Data, Lens};
use im;
use serde::{Serialize, Deserialize};
use strum_macros::Display;

use self::shared_data::{SharedData, SharedDataItem};

// Maybe someday I'll add the ability for homebrew ability scores and skills
#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct CharacterState {
	name: String,
	level: u8,
	proficiency_bonus: SharedData,
	ability_scores: im::Vector<AbilityScore>,
	skills: im::Vector<Skill>
}

#[derive(Debug, Clone, Copy, Data, PartialEq, Serialize, Deserialize)]
pub enum AbilityScoreType {
	Strength,
	Dexterity,
	Constitution,
	Intelligence,
	Wisdom,
	Charisma
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct AbilityScore {
	pub proficiency_bonus: SharedData,
	pub score_type: AbilityScoreType,
	pub score: SharedData,
	pub saving_proficiency: bool,
	pub saving_advantage: bool
}

#[derive(Debug, Display, Clone, Copy, Data, PartialEq, Serialize, Deserialize)]
pub enum SkillType {
	// Strength
	Athletics,

	// Dexterity
	Acrobatics,
	#[strum(serialize = "Sleight Of Hand")]
	SleightOfHand,
	Stealth,

	// Intelligence
	Arcana,
	History,
	Investigation,
	Nature,
	Religion,

	// Wisdom
	#[strum(serialize = "Animal Handling")]
	AnimalHandling,
	Insight,
	Medicine,
	Perception,
	Survival,

	// Charisma
	Deception,
	Intimidation,
	Performance,
	Persuasion
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct Skill {
	pub proficiency_bonus: SharedData,
	pub score_type: AbilityScoreType,
	pub score: SharedData,
	pub skill_type: SkillType,
	pub proficiency: bool,
	pub expertise: bool,
	pub advantage: bool
}

impl CharacterState {
	pub fn new() -> Self {
		let level = 1;
		let prof = SharedData::new(SharedDataItem::U8(2));

		let ability_scores = im::vector![
			AbilityScore::new(AbilityScoreType::Strength, prof.clone()),
			AbilityScore::new(AbilityScoreType::Dexterity, prof.clone()),
			AbilityScore::new(AbilityScoreType::Constitution, prof.clone()),
			AbilityScore::new(AbilityScoreType::Intelligence, prof.clone()),
			AbilityScore::new(AbilityScoreType::Wisdom, prof.clone()),
			AbilityScore::new(AbilityScoreType::Charisma, prof.clone())
		];

		let skills = im::vector![
			Skill::new(SkillType::Athletics, &ability_scores[0]),

			Skill::new(SkillType::Acrobatics, &ability_scores[1]),
			Skill::new(SkillType::SleightOfHand, &ability_scores[1]),
			Skill::new(SkillType::Stealth, &ability_scores[1]),

			Skill::new(SkillType::Arcana, &ability_scores[3]),
			Skill::new(SkillType::History, &ability_scores[3]),
			Skill::new(SkillType::Investigation, &ability_scores[3]),
			Skill::new(SkillType::Nature, &ability_scores[3]),
			Skill::new(SkillType::Religion, &ability_scores[3]),

			Skill::new(SkillType::AnimalHandling, &ability_scores[4]),
			Skill::new(SkillType::Insight, &ability_scores[4]),
			Skill::new(SkillType::Medicine, &ability_scores[4]),
			Skill::new(SkillType::Perception, &ability_scores[4]),
			Skill::new(SkillType::Survival, &ability_scores[4]),

			Skill::new(SkillType::Deception, &ability_scores[5]),
			Skill::new(SkillType::Intimidation, &ability_scores[5]),
			Skill::new(SkillType::Performance, &ability_scores[5]),
			Skill::new(SkillType::Persuasion, &ability_scores[5]),
		];

		CharacterState {
			name: "".into(),
			level,
			proficiency_bonus: prof.clone(),
			ability_scores,
			skills
		}
	}

	pub fn serialise(&self) -> String {
		ron::to_string(self).expect("Serialisation of CharacterState failed - This indicates a bug")
	}

	pub fn deserialise(serialized: &str) -> Self {
		ron::from_str(serialized).expect("Deserialisation of RON to CharacterState failed - The file might've failed to save correctly, or been edited externally, or created with an incompatible version of this program")
	}
}

impl AbilityScore {
	pub fn new(score_type: AbilityScoreType, proficiency_bonus: SharedData) -> Self {
		AbilityScore { proficiency_bonus: proficiency_bonus.clone(), score_type, score: SharedData::new(SharedDataItem::U8(8)), saving_proficiency: false, saving_advantage: false }
	}
}

impl Display for AbilityScore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}: {}", self.score_type, self.score)
	}
}

impl Skill {
	pub fn new(skill_type: SkillType, based_on: &AbilityScore) -> Self {
		Skill {
			proficiency_bonus: based_on.proficiency_bonus.clone(),
			score_type: based_on.score_type,
			score: based_on.score.clone(),
			skill_type,
			proficiency: false, expertise: false, advantage: false
		}
	}
}