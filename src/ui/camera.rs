use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(setup_camera.in_schedule(OnEnter(GameState::InGame)))
		;
	}
}

fn setup_camera(
	mut commands: Commands
) {
	commands
	.spawn((Camera2dBundle {
//		camera: todo!(),
		projection: OrthographicProjection {
			scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(super::CAMERA_HEIGHT),
			..default()
		},
		transform: Transform::from_translation(Vec3::new(0.0,0.0,100.0)),
//		camera_2d: todo!(),
		..Default::default()
	}, MainCamera));
}
