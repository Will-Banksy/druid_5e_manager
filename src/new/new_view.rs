use druid::{Data, Widget, widget::{Flex, TextBox, List, Label}, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;

use super::new_character_state::{NewCharacterState, Equipment, Armour, EquipmentArmour, EquipmentAdventuringGear, AdventuringGear};

/// Types implementing this trait can construct UI to view their data with the static `build_ui` method
///
/// Really this is nothing more than a way to associate a UI-building function with a specific type
pub trait BuildUi: Data {
	fn build_ui() -> Box<dyn Widget<Self>>;
}

impl BuildUi for NewCharacterState {
	fn build_ui() -> Box<dyn Widget<Self>> {
		Box::new(Flex::column()
			.with_child(
				TextBox::new()
					.lens(NewCharacterState::name)
			)
			.with_child(
				List::new(|| Equipment::build_ui()).lens(NewCharacterState::equipment)
			)
		)
	}
}

impl BuildUi for Equipment {
	fn build_ui() -> Box<dyn Widget<Self>> {
		Box::new(
			Flex::column()
				.with_child(
					Switcher::new()
						.with_variant(EquipmentArmour, Armour::build_ui())
						.with_variant(EquipmentAdventuringGear, AdventuringGear::build_ui())
				)
		)
	}
}

impl BuildUi for Armour {
	fn build_ui() -> Box<dyn Widget<Self>> {
		Box::new(Label::new("Armour"))
	}
}

impl BuildUi for AdventuringGear {
	fn build_ui() -> Box<dyn Widget<Self>> {
		Box::new(Label::new("Item"))
	}
}