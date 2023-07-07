use bevy::prelude::*;
use super::super::TILE_SIZE;

#[derive(Component, Clone, Copy)]
pub struct Tile {
	pub x: u32,
	pub y: u32,
}

pub fn spawn_tile(
	commands: &mut Commands,
	x: u32, y: u32,
	offset: Vec3,
) -> Entity {
	commands.spawn((SpriteBundle {
		sprite: Sprite {
		//	color: (), 
			custom_size: Some(Vec2::ONE * 0.95 * TILE_SIZE ), anchor: bevy::sprite::Anchor::Center,
			..default()
		},
		transform: Transform::from_translation(offset),
		//texture: todo!(),
		..default()
	}, Tile {x,y}
	)).id()
}
