use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
	#[default]
	InGame,
	GameOver
}
