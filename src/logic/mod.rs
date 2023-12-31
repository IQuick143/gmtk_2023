use bevy::prelude::*;

mod pieces;

pub use pieces::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Player {
	Black, White
}

#[derive(Resource)]
pub struct GameBoard {
	pub size: UVec2,
	pieces: Vec<Option<GamePiece>>
}

impl GameBoard {
	pub fn new(x: u32, y: u32) -> Self {
		GameBoard {
			size: UVec2::new(x, y),
			pieces: Vec::new(),
		}
	}

	pub fn get(&self, x: u32, y: u32) -> Option<&GamePiece> {
		self.pieces[(x + self.size.x * y) as usize].as_ref()
	}

	pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut GamePiece> {
		self.pieces[(x + self.size.x * y) as usize].as_mut()
	}
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(GameBoard::new(8,8))
		.add_system(update_board)
		;
	}
}

fn update_board(
	mut board: ResMut<GameBoard>,
	time: Res<Time>,
) {
	board.pieces = (0..board.size.x*board.size.y).map(
		|n| if (n + ((time.elapsed_seconds() * 50.0) as u32)) % 5 == 0 {Some(GamePiece {
			player: if n > 31 {Player::Black} else {Player::White},
			piece_type: PieceType::Pawn,
	})} else {None}).collect()
}
