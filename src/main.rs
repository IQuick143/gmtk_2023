use bevy::prelude::*;

// Modules
mod common;
mod logic;
mod debug;
mod ui;

pub mod prelude {
	pub use crate::logic::GameBoard;
	pub use crate::common::*;
}

fn main() {
	let default_plugins = DefaultPlugins;

	let default_plugins = default_plugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "gmtk_2023".to_string(),
			..Default::default()
		}), ..Default::default()
	});

	#[cfg(debug_assertions)]
	let default_plugins = default_plugins.set(AssetPlugin {
		watch_for_changes: true,
		..Default::default()
	});

	let mut app = App::new();

	app
	.add_state::<common::GameState>()
	.add_plugins(default_plugins)
	.add_plugin(logic::LogicPlugin)
	.add_plugin(ui::UIPlugin)
	;

	{
		#[cfg(debug_assertions)]
		app.add_plugin(debug::DebugPlugin);
	}

	app.run();
}
