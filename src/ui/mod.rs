use bevy::prelude::*;

// Height in world-units that is seen by the camera
pub const CAMERA_HEIGHT: f32 = 10.0;
// Size of one tile 
pub const TILE_SIZE: f32 = 1.0;

mod mouse;
mod camera;
mod board;

mod colours {
    use bevy::prelude::*;

	pub const TILE_LIGHT: Color = Color::BEIGE;
	pub const TILE_DARK: Color = Color::DARK_GRAY;
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugin(camera::CameraPlugin)
		.add_plugin(board::BoardPlugin)
		;
	}
}
