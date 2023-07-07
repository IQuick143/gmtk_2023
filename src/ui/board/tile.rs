use bevy::prelude::*;
use super::super::TILE_SIZE;
use super::super::colours::TILE_LIGHT;
use super::super::colours::TILE_DARK;

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
			color: if (x + y) % 2 == 0 {TILE_LIGHT} else {TILE_DARK},
			custom_size: Some(Vec2::ONE * TILE_SIZE), anchor: bevy::sprite::Anchor::Center,
			..default()
		},
		transform: Transform::from_translation(offset),
		//texture: todo!(),
		..default()
	}, Tile {x,y}
	)).id()
}
