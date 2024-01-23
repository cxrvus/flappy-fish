use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Background;


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
	#[default]
	InGame,
	GameOver
}
