use std::fmt::Display;

use druid::{Data, Lens};
use im;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use uuid::Uuid;

use crate::dnd_rules;

// Maybe someday I'll add the ability for homebrew ability scores and skills
#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct CharacterState {
	pub name: String,
	pub race: String,
	pub level: u16, // The combined level
	pub levels: im::Vector<Level>,
	pub proficiency_bonus: u16,
	pub ability_scores: im::Vector<AbilityScore>,
	pub skills: im::Vector<Skill>,
	pub hp: u32,
	pub hp_max: u32,
	pub temp_hp: u32
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
	pub proficiency_bonus: u16,
	pub score_type: AbilityScoreType,
	pub score: u8,
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
	pub proficiency_bonus: u16,
	pub score_type: AbilityScoreType,
	pub score: u8,
	pub skill_type: SkillType,
	pub proficiency: bool,
	pub expertise: bool,
	pub advantage: bool
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct Level {
	pub uuid: u128,
	pub name: String,
	pub level: u8
}

// #[derive(Default, Debug, Clone, Data, Lens, Serialize, Deserialize)]
// pub struct ValueOfMax<T> {
// 	pub val: T,
// 	pub max: T
// }

impl CharacterState {
	pub fn new() -> Self {
		let level = 1;
		let prof = dnd_rules::proficiency_bonus_for(level);

		let ability_scores = im::vector![
			AbilityScore::new(AbilityScoreType::Strength, prof),
			AbilityScore::new(AbilityScoreType::Dexterity, prof),
			AbilityScore::new(AbilityScoreType::Constitution, prof),
			AbilityScore::new(AbilityScoreType::Intelligence, prof),
			AbilityScore::new(AbilityScoreType::Wisdom, prof),
			AbilityScore::new(AbilityScoreType::Charisma, prof)
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

		let levels = im::vector![
			Level::new("".to_string(), 1)
		];

		CharacterState {
			name: "".into(),
			race: "".into(),
			level,
			levels,
			proficiency_bonus: prof,
			ability_scores,
			skills,
			hp: 0,
			hp_max: 0,
			temp_hp: 0
		}
	}

	pub fn serialize(&self) -> String {
		// ron::to_string(self).expect("Serialisation of CharacterState failed - This indicates a bug")
		serde_json::to_string(self).expect("Serialisation of CharacterState failed - This indicates a bug")
	}

	pub fn deserialize(serialized: &str) -> Self {
		// ron::from_str(serialized).expect("Deserialisation of RON to CharacterState failed - The file might've failed to save correctly, or been edited externally, or created with an incompatible version of this program")
		serde_json::from_str(serialized).expect("Deserialisation of RON to CharacterState failed - The file might've failed to save correctly, or been edited externally, or created with an incompatible version of this program")
	}
}

impl AbilityScore {
	pub fn new(score_type: AbilityScoreType, proficiency_bonus: u16) -> Self {
		AbilityScore { proficiency_bonus: proficiency_bonus, score_type, score: 8, saving_proficiency: false, saving_advantage: false }
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

impl Level {
	pub fn new(name: String, level: u8) -> Self {
		Level { uuid: Uuid::new_v4().as_u128(), name, level }
	}
}

// impl<T> ValueOfMax<T> {
// 	pub fn new(val: T, max: T) -> Self {
// 		Self {
// 			val,
// 			max
// 		}
// 	}
// }

// impl<T> Display for ValueOfMax<T> where T: Display {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}/{}", self.val, self.max)
// 	}
// }

// impl<T> Default for ValueOfMax<T> where T: Default {
// 	fn default() -> Self {
// 		Self { val: Default::default(), max: Default::default() }
// 	}
// }