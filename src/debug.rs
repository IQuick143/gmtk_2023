use bevy::prelude::*;

use crate::{prelude::*, ui::mouse::{WorldMouse, TileMouse}};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			//.add_plugin(EditorPlugin::default())
			.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_system(click_debug)
		;
	}
}

fn click_debug(
	input: Res<Input<MouseButton>>,
	world_pos: Res<WorldMouse>,
	tile_pos: Res<TileMouse>,
) {
	if input.just_pressed(MouseButton::Left) {
		println!("Clicked at {} {}", world_pos.0, tile_pos.0);
	}
}
