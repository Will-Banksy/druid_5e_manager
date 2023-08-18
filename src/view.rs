pub mod controllers;
pub mod widgets;
pub mod components;
pub mod formatter;
pub mod nav_controller;
pub mod character_ui;
pub mod content_ui;

use druid::{Widget, WidgetExt, Menu, MenuItem, Env, WindowId, FileDialogOptions, commands, SysMods};

use druid_widget_nursery::navigator::Navigator;

use crate::data::AppData;
use crate::data::transitive_app_state::NavState;
use crate::delegate;

use self::character_ui::build_character_ui;
use self::nav_controller::NavController;
use self::content_ui::build_content_ui;

pub fn build_app_menu(_window_id: Option<WindowId>, _state: &AppData, _env: &Env) -> Menu<AppData> {
	// TODO: Force immediate update/handling of commands...somehow?
	Menu::new("Application Menu").entry(
		Menu::new("File")
			.entry(
				MenuItem::new("Save")
					.command(druid::commands::SAVE_FILE)
					.hotkey(SysMods::Cmd, "s")
			)
			.entry(
				MenuItem::new("Save As")
					.command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::new()))
					.hotkey(SysMods::CmdShift, "s")
			)
			.separator()
			.entry(
				MenuItem::new("Open").command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()))
				.hotkey(SysMods::Cmd, "o")
			)
			// .separator()
			// .entry(
			// 	MenuItem::new("[Dev] Update Widget Tree")
			// 	.command(delegate::UPDATE_WIDGET_TREE)
			// )
	).entry(
		Menu::new("View")
			.entry(
				MenuItem::new("Switch to Sources")
					.command(delegate::NAV_SWITCH_TO_SOURCES)
			)
			.entry(
				MenuItem::new("Switch to Character")
					.command(delegate::NAV_SWITCH_TO_CHARACTER)
			)
	)
}

pub fn build_ui() -> impl Widget<AppData> {
	Navigator::new(NavState::NavDestCharacter, || {
		Box::new(build_character_ui().lens(AppData::character))
	})
	.with_view_builder(NavState::NavDestSourceManager, || {
		Box::new(build_content_ui())
	})
	.controller(NavController)
}