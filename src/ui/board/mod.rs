use bevy::prelude::*;
use crate::prelude::*;

use self::tile::spawn_tile;

use super::TILE_SIZE;

mod tile;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(spawn_board.in_schedule(OnEnter(GameState::InGame)))
		;
	}
}

fn spawn_board(
	mut commands: Commands,
	board: Res<GameBoard>,
) {
	for x in 0..board.size.x {
		for y in 0..board.size.y {
			let offset = Vec3::new((x as f32) - 0.5 * (board.size.x - 1) as f32, (y as f32) - 0.5 * (board.size.y - 1) as f32, 0.0) * TILE_SIZE;
			spawn_tile(&mut commands, x, y, offset);
		}
	}
}
