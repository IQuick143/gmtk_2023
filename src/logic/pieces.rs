use super::Player;


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct GamePiece {
	pub player: Player,
	pub piece_type: PieceType,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
	Pawn
}
