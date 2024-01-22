use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::structs::*;


mod player {
	pub const WIDTH: f32 = 256.; //PX
	pub const HEIGHT: f32 = 64.; //PX
	pub const SCALE: f32 = 0.5;
	pub const WEIGHT: f32 = 10.;
	pub const GRAVITY: f32 = 20.;
	pub const FORCE: f32 = 10000.;
}

mod pipes {
	pub const GAP: f32 = 128.; //PX
	pub const INTERVAL: f32 = 2.; //SECONDS
	pub const SPEED: f32 = 5.; //PX PER FRAME
}

mod env {
	pub const GROUND_HEIGHT: f32 = 50.; //PX
}


pub fn setup
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	resolution: Res<Resoulution>
) {
	commands.spawn(Camera2dBundle::default());

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/underwater.png"),
		sprite: Sprite {
			custom_size: Some(resolution.vec2),
			..default()
		},
		..default()
	})
	.insert(Background);
}


pub fn reset_background
(
	mut bg_transform: Query<&mut Transform, With<Background>>
) {
	let mut bg_transform = bg_transform.single_mut();
	bg_transform.translation = Vec3::default();
}


pub fn spawn_player
(
	mut commands: Commands,
	window: Query<&Window, With<bevy::window::PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let window = window.single();
	let xpos = window.width() / -2. + player::WIDTH / 2.;

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/fish.png"),
		transform: Transform {
			translation: Vec3::new(xpos, 0.0, 1.0),
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
	.insert(Collider::cuboid(player::WIDTH / 2., player::HEIGHT / 2.))
	.insert(ColliderMassProperties::Mass(player::WEIGHT))
	;
}


pub fn spawn_ground
(
	mut commands: Commands,
	window: Query<&Window, With<bevy::window::PrimaryWindow>>,
)
{
	let window = window.single();
	let ground_size = Vec2::new(window.width(), env::GROUND_HEIGHT);
	let ground_ypos = window.height() / -2. + env::GROUND_HEIGHT / 2.;
	commands.spawn(SpriteBundle {
		sprite: Sprite { color: Color::MIDNIGHT_BLUE, custom_size: Some(ground_size), ..default() },
		transform: Transform::from_translation(Vec3::new(0., ground_ypos, 0.1)),
		..default()
	})
	.insert(Collider::cuboid(ground_size.x / 2., ground_size.y / 2.))
	;
}


// todo: spawn_pipe
// todo: move pipes
// todo: despawn pipes


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
