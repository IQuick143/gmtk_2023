use bevy::prelude::*;
use crate::logic::GamePiece;
use crate::logic::PieceType;
use crate::logic::Player;
use crate::prelude::*;

use super::TILE_SIZE;
use super::colours::TILE_LIGHT;
use super::colours::TILE_DARK;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(spawn_board.in_schedule(OnEnter(GameState::InGame)))
		.add_system(update_board.run_if(in_state(GameState::InGame)))
		;
	}
}

#[derive(Component, Clone, Copy)]
pub struct Tile {
	pub x: u32,
	pub y: u32,
	pub piece: Option<Entity>,
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
	pub x: u32,
	pub y: u32,
	pub displayed_piece: GamePiece,
	pub tile: Entity,
}

fn spawn_board(
	mut commands: Commands,
	board: Res<GameBoard>,
) {
	for x in 0..board.size.x {
		for y in 0..board.size.y {
			spawn_tile(&mut commands, x, y, board_pos_to_world(x, y, board.size).extend(0.0));
		}
	}
}

fn update_board(
	mut commands: Commands,
	mut tiles: Query<(Entity, &mut Tile)>,
	mut pieces: Query<(Entity, &mut Piece)>,
	board: Res<GameBoard>,
	asset_server: Res<AssetServer>,
) {
	for (tile_entity, mut tile) in tiles.iter_mut() {
		let piece = tile.piece.map(|entity| pieces.get_mut(entity).unwrap());
		let game_piece = board.get(tile.x, tile.y);

		if piece.is_some() && game_piece.is_none() {
			commands.entity(piece.unwrap().0).despawn_recursive();
			tile.piece = None;
			continue;
		}
		if piece.is_none() && game_piece.is_some() {
			let game_piece = *game_piece.unwrap();
			let is_black = game_piece.player == Player::Black;
			let piece_entity =
			commands.spawn((
				SpriteBundle {
			        sprite: Sprite {custom_size: Some(Vec2::ONE * TILE_SIZE), anchor: bevy::sprite::Anchor::Center, flip_y: is_black, color: get_player_colour(game_piece.player), ..default()},
					transform: Transform::from_translation(Vec2::ZERO.extend(1.0)),
					texture: asset_server.load(get_texture(game_piece.piece_type)),
					..default()
				},
				Piece {x: tile.x, y: tile.y, displayed_piece: game_piece, tile: tile_entity}
			)).set_parent(tile_entity)
			.id();
			tile.piece = Some(piece_entity);
			continue;
		}
		if piece.is_some() && game_piece.is_some() {
			let game_piece = *game_piece.unwrap();
			let (piece_entity, mut piece) = piece.unwrap();
			if piece.displayed_piece.piece_type != game_piece.piece_type {
				piece.displayed_piece.piece_type = game_piece.piece_type;
				// Override the texture
				commands.entity(piece_entity)
				.insert(asset_server.load::<Image, &str>(get_texture(game_piece.piece_type)));
			}
			if piece.displayed_piece.player != game_piece.player {
				piece.displayed_piece.player = game_piece.player;
				// Override the sprite
				commands.entity(piece_entity)
				.insert(Sprite {
					custom_size: Some(Vec2::ONE * TILE_SIZE), anchor: bevy::sprite::Anchor::Center,
					flip_y: game_piece.player == Player::Black, color: get_player_colour(game_piece.player), ..default()
				});
			}
		}
	}
}

pub fn spawn_tile(
	commands: &mut Commands,
	x: u32, y: u32,
	offset: Vec3,
) -> Entity {
	commands.spawn((SpriteBundle {
		sprite: Sprite {
			color: if (x + y) % 2 == 0 {TILE_LIGHT} else {TILE_DARK},
			custom_size: Some(Vec2::ONE * TILE_SIZE), anchor: bevy::sprite::Anchor::Center,
			..default()
		},
		transform: Transform::from_translation(offset),
		//texture: todo!(),
		..default()
	}, Tile {x,y, piece: None}
	)).id()
}

pub fn get_texture(kind: PieceType) -> &'static str {
	match kind {
		PieceType::Pawn => "images/pieces/pawn.png",
	}
}

pub fn get_player_colour(player: Player) -> Color {
	match player {
		Player::Black => Color::BLACK,
		Player::White => Color::WHITE,
	}
}

pub fn board_pos_to_world(x: u32, y: u32, board_size: UVec2) -> Vec2 {
	Vec2::new((x as f32) - 0.5 * (board_size.x - 1) as f32, (y as f32) - 0.5 * (board_size.y - 1) as f32) * TILE_SIZE
}
