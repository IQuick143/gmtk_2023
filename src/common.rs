use bevy::prelude::*;

#[derive(Default, States, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameState {
	MainMenu,
	#[default]
	InGame,
	Paused
}
