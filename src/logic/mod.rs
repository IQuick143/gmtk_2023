use bevy::prelude::*;

#[derive(Resource)]
pub struct GameBoard {
	pub size: UVec2
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(GameBoard {size: UVec2::new(8, 8)})
		;
	}
}
