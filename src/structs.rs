use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use super::pipes;


#[derive(Bundle)]
pub struct PipeBundle{
	sprite_bundle: SpriteBundle,
	collider: Collider,
	pipe: Pipe
}

impl Default for PipeBundle {
    fn default() -> Self {
        Self {
			sprite_bundle: SpriteBundle::default(),
			collider: Collider::cuboid(pipes::WIDTH / 2., pipes::HEIGHT / 2.),
			pipe: Pipe
		}
    }
}

impl PipeBundle {
	pub fn with_sprite_bundle(sprite_bundle: SpriteBundle) -> Self {
		Self { sprite_bundle, ..default() }
	}
}


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
