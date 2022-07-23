use std::sync::Arc;

use serde::{Serialize, Deserialize};

use super::{AbilityScoreType, SkillType, CharacterState, AbilityScore, Skill, MutArc};

pub fn serialise(state: &CharacterState) -> Result<String, ron::Error> {
	let sstate: SCharacterState = state.clone().into();
	ron::to_string(&sstate)
}

pub fn deserialise(serialised: &str) -> Result<CharacterState, ron::Error> {
	let sstate: SCharacterState = ron::from_str(serialised)?;
	Ok(sstate.into())
}

#[derive(Clone, Serialize, Deserialize)]
struct SCharacterState {
	name: String,
	level: u8,
	proficiency_bonus: i8,
	saving_throws: im::Vector<SAbilityScore>,
	skills: im::Vector<SSkill>
}

#[derive(Clone, Serialize, Deserialize)]
struct SAbilityScore {
	score_type: AbilityScoreType,
	score: u8,
	saving_proficiency: bool,
	saving_advantage: bool
}

#[derive(Clone, Serialize, Deserialize)]
struct SSkill {
	assoc_score_type: AbilityScoreType,
	skill_type: SkillType,
	proficiency: bool,
	expertise: bool,
	advantage: bool
}

impl From<CharacterState> for SCharacterState {
    fn from(state: CharacterState) -> Self {
		SCharacterState {
			name: state.name,
			level: state.level,
			proficiency_bonus: *state.proficiency_bonus,
			saving_throws: {
				state.ability_scores.iter().map(|abscore| {
					SAbilityScore { score_type: abscore.score_type, score: *abscore.score, saving_proficiency: abscore.saving_proficiency, saving_advantage: abscore.saving_advantage }
				}).collect()
			},
			skills: {
				state.skills.iter().map(|skill| {
					SSkill { assoc_score_type: skill.score_type, skill_type: skill.skill_type, proficiency: skill.proficiency, expertise: skill.expertise, advantage: skill.advantage }
				}).collect()
			}
		}
    }
}

impl From<SCharacterState> for CharacterState {
    fn from(sstate: SCharacterState) -> Self {
		let prof = MutArc::new(sstate.proficiency_bonus);

		let ability_scores: im::Vector<AbilityScore> = sstate.saving_throws.iter().map(|sabscore| {
			AbilityScore {
				proficiency_bonus: prof.clone(),
				score_type: sabscore.score_type,
				score: MutArc::new(sabscore.score),
				saving_proficiency: sabscore.saving_proficiency,
				saving_advantage: sabscore.saving_proficiency,
			}
		}).collect();

		let skills = sstate.skills.iter().map(|skill| {
			Skill {
				proficiency_bonus: prof.clone(),
				score_type: skill.assoc_score_type,
				score: MutArc::clone(&ability_scores.iter().find(|sabscore| sabscore.score_type == skill.assoc_score_type).expect("Serialised skill associated with non-serialised ability score").score),
				skill_type: skill.skill_type,
				proficiency: skill.proficiency,
				expertise: skill.expertise,
				advantage: skill.advantage,
			}
		}).collect();

        CharacterState {
			name: sstate.name,
			level: sstate.level,
			proficiency_bonus: prof.clone(),
			ability_scores,
			skills
		}
    }
}