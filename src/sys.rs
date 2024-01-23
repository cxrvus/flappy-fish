use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use super::structs::*;


mod player {
	pub const WIDTH: f32 = 512.; //PX
	pub const HEIGHT: f32 = 256.; //PX
	pub const COL_WIDTH: f32 = WIDTH / 3.; //PX
	pub const COL_HEIGHT: f32 = HEIGHT / 6.; //PX
	pub const SCALE: f32 = 0.5;
	pub const ZPOS: f32 = 2.; //PX
	pub const WEIGHT: f32 = 10.;
	pub const GRAVITY: f32 = 20.;
	pub const FORCE: f32 = 8000.;
}

pub mod pipes {
	pub const HEIGHT: f32 = 600.; //PX
	pub const WIDTH: f32 = 196.; //PX
	pub const GAP: f32 = 250.; //PX between upper & lower
	pub const INTERVAL: f32 = 1.7; //SECONDS between spawns
	pub const SPEED: f32 = 5.; //PX per frame
	pub const MAX_Y_OFFSET: f32 = 150.; //PX - max above/below screen center
	pub const ZPOS: f32 = 1.; //PX
}

mod env {
	pub const W_HEIGHT: f32 = 720.; //PX
	pub const W_WIDTH: f32 = 1280.; //PX
}


pub fn setup
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut timer: ResMut<PipeTimer>
) {
	timer.0 = Timer::new(Duration::from_secs_f32(pipes::INTERVAL), TimerMode::Repeating);

	commands.spawn(Camera2dBundle::default());

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/underwater.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(env::W_WIDTH, env::W_HEIGHT)),
			..default()
		},
		..default()
	})
	.insert(Background);
}


pub fn new_game
(
	mut commands: Commands,
	mut bg_transform: Query<&mut Transform, With<Background>>,
	pipes: Query<Entity, With<Pipe>>,
	mut timer: ResMut<PipeTimer>
) {
	// unpause pipe timer
	timer.0.unpause();

	// despawn pipes
	pipes.for_each(|pipe| commands.entity(pipe).despawn());

	// reset background
	let mut bg_transform = bg_transform.single_mut();
	bg_transform.translation = Vec3::default();

	// spawn lower bound
	// todo: spawn upper bound
	commands
		.spawn(TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., -env::W_HEIGHT / 2., 0.))))
		.insert(Collider::cuboid(env::W_WIDTH / 2., 1.))
	;
}


pub fn spawn_player
(
	mut commands: Commands,
	player: Query<Entity, With<Player>>,
	asset_server: Res<AssetServer>,
) {
	if let Ok(player) = player.get_single() { commands.entity(player).despawn() }

	let xpos = (-env::W_WIDTH + player::WIDTH) / 2. ;

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/fish.png"),
		transform: Transform {
			translation: Vec3::new(xpos, 0.0, player::ZPOS),
			scale: Vec3::new(player::SCALE, player::SCALE, 1.0),
			..default()
		},
		..default()
	})
	.insert(Player)
	.insert(RigidBody::Dynamic)
	.insert(ExternalImpulse::default())
	.insert(LockedAxes::ROTATION_LOCKED_Z)
	.insert(LockedAxes::TRANSLATION_LOCKED_X)
	.insert(GravityScale(player::GRAVITY))
	.insert(Collider::cuboid(player::COL_WIDTH, player::COL_HEIGHT))
	.insert(ColliderMassProperties::Mass(player::WEIGHT))
	.insert(ActiveEvents::COLLISION_EVENTS)
	;
}


pub fn jump
(
	kb: Res<Input<KeyCode>>,
	mut impulse: Query<&mut ExternalImpulse, With<Player>>
) {
	if kb.just_pressed(KeyCode::Space) {
		if let Ok(mut impulse) = impulse.get_single_mut() {
			impulse.impulse = Vec2::new(0., player::FORCE);
		};
	}
}


pub fn collision_check
(
	mut collision_events: EventReader<CollisionEvent>,
	mut state: ResMut<NextState<GameState>>
) {
	if let Some(_) = collision_events.read().next() {
		state.set(GameState::GameOver);
	}
}




pub fn spawn_pipes
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	mut timer: ResMut<PipeTimer>
) {
	if !timer.0.just_finished() {
		timer.0.tick(time.delta());
		return;
	}
	else {
		timer.0.reset();
	}

	let texture = asset_server.load("sprites/pipe.png");
	let mut rng = thread_rng();
	let offset = rng.gen_range(-pipes::MAX_Y_OFFSET..=pipes::MAX_Y_OFFSET);
	let gap = (pipes::GAP + pipes::HEIGHT) / 2.;
	let ypos_lower = offset - gap;
	let ypos_upper = offset + gap;
	let xpos = env::W_WIDTH / 2. + pipes::WIDTH;

	commands.spawn(PipeBundle::with_sprite_bundle(SpriteBundle {
		texture: texture.clone(),
		transform: Transform::from_translation(Vec3::new(xpos, ypos_lower, pipes::ZPOS)),
		..default()
	}));

	commands.spawn(PipeBundle::with_sprite_bundle(SpriteBundle {
		texture,
		transform: Transform::from_translation(Vec3::new(xpos, ypos_upper, pipes::ZPOS)),
		sprite: Sprite { flip_y: true, ..default() },
		..default()
	}));
}


pub fn move_pipes
(
	mut pipe_transforms: Query<&mut Transform, With<Pipe>>
) {
	for mut transform in &mut pipe_transforms {
		transform.translation.x -= pipes::SPEED;
	}
}


pub fn despawn_pipes
(
	mut commands: Commands,
	pipes: Query<(&Transform, Entity), With<Pipe>>
) {
	for (transform, entity) in &pipes {
		if transform.translation.x < - (env::W_WIDTH / 2. + pipes::WIDTH) {
			commands.entity(entity).despawn();
		}
	}
}


pub fn game_over
(
	mut player_sprite: Query<&mut Sprite, With<Player>>,
	mut timer: ResMut<PipeTimer>
) {
	let mut player_sprite = player_sprite.single_mut();
	player_sprite.flip_y = true;
	timer.0.pause();
	timer.0.reset();
}


pub fn play_again
(
	kb: Res<Input<KeyCode>>,
	mut next_state: ResMut<NextState<GameState>>
) {
	if kb.pressed(KeyCode::Escape) {
		next_state.set(GameState::InGame);
	}
}
