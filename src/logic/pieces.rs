pub struct GamePiece {
	pub piece_type: PieceType,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
	Pawn
}
