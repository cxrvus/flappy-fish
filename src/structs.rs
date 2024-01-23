use bevy::prelude::*;


#[derive(Component)]
pub struct Background;


#[derive(Component)]
pub struct Pipe;


#[derive(Component)]
pub struct Player;


#[derive(Resource, Default)]
pub struct PipeTimer(pub Timer);


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
	#[default]
	InGame,
	GameOver
}
