use bevy::prelude::*;
use crate::prelude::*;

use super::camera::MainCamera;

// Position of the mouse in tile-coordinates
#[derive(Resource, Default)]
pub struct TileMouse(pub IVec2);

// Position of the mouse in world-coordinates
#[derive(Resource, Default)]
pub struct WorldMouse(pub Vec2);

pub struct MousePlugin;

impl Plugin for MousePlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<TileMouse>()
		.init_resource::<WorldMouse>()
		.add_systems((
			track_mouse,
			calculate_tile_hover.run_if(in_state(GameState::InGame))
		).chain())
		;
	}
}

fn track_mouse(
	window_query: Query<&Window>,
	camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
	mut world_pos: ResMut<WorldMouse>,
) {
	let (camera, transform) = if let Ok(camera) = camera.get_single() {
		camera
	} else {
		warn!("There should be 1 main camera with an orthographic projection!");
		return;
	};


	let window = window_query.get_single().expect("There should be a single window");
	if let Some(viewport_position) = window.cursor_position() {
		if let Some(pos) = camera.viewport_to_world_2d(transform, viewport_position) {
			world_pos.0 = pos;
		}
	}
}

fn calculate_tile_hover(
	board: Res<GameBoard>,
	world_pos: Res<WorldMouse>,
	mut hover_pos: ResMut<TileMouse>,
) {
	let left_most_pos = Vec2::new(board.size.x as f32 - 1.0, board.size.y as f32 - 1.0) * -0.5;
	hover_pos.0 = (world_pos.0 - left_most_pos).round().as_ivec2();
}
