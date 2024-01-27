use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::cfg::pipes;


pub const OBSTACLE_GROUPS: CollisionGroups = CollisionGroups::new(Group::GROUP_2, Group::GROUP_1);

#[derive(Bundle)]
pub struct ObstacleBundle {
	obstacle: Obstacle,
	groups: CollisionGroups,
}

impl Default for ObstacleBundle {
	fn default() -> Self {
		Self {
			obstacle: Obstacle,
			groups: OBSTACLE_GROUPS,
		}
	}
}


#[derive(Bundle)]
pub struct PipeBundle{
	sprite_bundle: SpriteBundle,
	collider: Collider,
	scroll: Scroll,
	obstacle: ObstacleBundle,
	pipe: Pipe
}

impl Default for PipeBundle {
    fn default() -> Self {
        Self {
			sprite_bundle: SpriteBundle::default(),
			collider: Collider::cuboid(pipes::WIDTH / 2., pipes::HEIGHT / 2.),
			scroll: Scroll(pipes::SPEED),
			obstacle: ObstacleBundle::default(),
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
pub struct Scroll(pub f32);

#[derive(Component)]
pub struct ScoreTrigger;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ScoreDisplay;


#[derive(Resource, Default)]
pub struct PipeTimer(pub Timer);

#[derive(Resource, Default)]
pub struct Score(pub u32);


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
	#[default]
	InGame,
	GameOver
}
