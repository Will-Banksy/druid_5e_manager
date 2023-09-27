use druid::{Data, Widget, widget::{Flex, TextBox, List, Label}, lens, WidgetExt, LensExt};
use druid_widget_nursery::{enum_switcher::Switcher, prism::{OptionSome, OptionNone}};

use crate::view::formatter::NumberFormatter;

use super::{new_character_state::{NewCharacterState, Equipment, Armour, EquipmentArmour, EquipmentAdventuringGear, AdventuringGear}, data::{schemas::player_character::PlayerCharacter, identifer::{Identifier, ID_5E_STAT_ABILITY_SCORE_CHARISMA, ID_5E_STAT_ABILITY_SCORE_STRENGTH, ID_5E_STAT_ABILITY_SCORE_INTELLIGENCE, ID_5E_STAT_ABILITY_SCORE_CONSTITUTION, ID_5E_STAT_ABILITY_SCORE_DEXTERITY, ID_5E_STAT_ABILITY_SCORE_WISDOM}, attribute::{ValueAttribute, ValueAttributeFloat64, ValueAttributeInt64, ValueAttributeUInt64}}, widgets::index_or_wrap::{IndexOrWrap, IndexOrExt}};

/// Semantically, types implementing this trait can construct UI to view their data with the static `build_ui` method
///
/// Really this is nothing more than a way to associate a UI-building function with a specific type
pub trait BuildUi: Data {
	fn build_ui() -> Box<dyn Widget<Self>>;
}

/// Semantically, types implementing this trait can construct UI to view and mutate their data with the static `build_ui` method
///
/// Really this is nothing more than a way to associate a UI-building function with a specific type
pub trait BuildUiMut: Data {
	fn build_ui_mut() -> Box<dyn Widget<Self>>;
}

impl BuildUiMut for PlayerCharacter {
    fn build_ui_mut() -> Box<dyn Widget<Self>> {
		Flex::column()
			.with_child(
				TextBox::new()
					.with_text_size(crate::env::THEME_SIZE_TITLE)
					.with_placeholder("Name")
					.lens(PlayerCharacter::name)
			)
			.with_child(
				Flex::column()
					.with_child(ability_score("Strength", &ID_5E_STAT_ABILITY_SCORE_STRENGTH))
					.with_child(ability_score("Dexterity", &ID_5E_STAT_ABILITY_SCORE_DEXTERITY))
					.with_child(ability_score("Constitution", &ID_5E_STAT_ABILITY_SCORE_CONSTITUTION))
					.with_child(ability_score("Intelligence", &ID_5E_STAT_ABILITY_SCORE_INTELLIGENCE))
					.with_child(ability_score("Wisdom", &ID_5E_STAT_ABILITY_SCORE_WISDOM))
					.with_child(ability_score("Charisma", &ID_5E_STAT_ABILITY_SCORE_CHARISMA))
			)
			.boxed()
    }
}

impl BuildUiMut for ValueAttribute {
	fn build_ui_mut() -> Box<dyn Widget<Self>> {
		Flex::column()
			.with_child(
				Switcher::new()
					.with_variant(ValueAttributeFloat64, {
						TextBox::new().with_formatter(NumberFormatter::new())
					})
					.with_variant(ValueAttributeInt64, {
						TextBox::new().with_formatter(NumberFormatter::new())
					})
					.with_variant(ValueAttributeUInt64, {
						TextBox::new().with_formatter(NumberFormatter::new())
					})
			)
			.boxed()
	}
}

fn ability_score(name: &str, stat_id: &'static Identifier) -> impl Widget<PlayerCharacter> {
	Flex::row()
		.with_child(
			Label::new(name)
		)
		.with_default_spacer()
		.with_child(
			Switcher::new()
				.with_variant(OptionSome, ValueAttribute::build_ui_mut())
				.with_variant(OptionNone, Label::new(format!("Stat \"{}\" not found", stat_id)))
				.index_or(stat_id)
				.lens(PlayerCharacter::stats)
		)
}

// impl BuildUi for NewCharacterState {
// 	fn build_ui() -> Box<dyn Widget<Self>> {
// 		Box::new(Flex::column()
// 			.with_child(
// 				TextBox::new()
// 					.lens(NewCharacterState::name)
// 			)
// 			.with_child(
// 				List::new(|| Equipment::build_ui()).lens(NewCharacterState::equipment)
// 			)
// 		)
// 	}
// }

// impl BuildUi for Equipment {
// 	fn build_ui() -> Box<dyn Widget<Self>> {
// 		Box::new(
// 			Flex::column()
// 				.with_child(
// 					Switcher::new()
// 						.with_variant(EquipmentArmour, Armour::build_ui())
// 						.with_variant(EquipmentAdventuringGear, AdventuringGear::build_ui())
// 				)
// 		)
// 	}
// }

// impl BuildUi for Armour {
// 	fn build_ui() -> Box<dyn Widget<Self>> {
// 		Box::new(Label::new("Armour"))
// 	}
// }

// impl BuildUi for AdventuringGear {
// 	fn build_ui() -> Box<dyn Widget<Self>> {
// 		Box::new(Label::new("Item"))
// 	}
// }