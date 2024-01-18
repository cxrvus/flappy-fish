use bevy::{prelude::*, window::{WindowResolution, PrimaryWindow, WindowMode}};
use bevy_rapier2d::prelude::*;
use cfg::*;

mod cfg {
	pub const PLAYER_WIDTH: f32 = 512.;
	pub const PLAYER_HEIGHT: f32 = 64.;
	pub const PLAYER_SCALE: f32 = 0.5;
	pub const GROUND_HEIGHT: f32 = 50.;
	pub const JUMP_FORCE: f32 = 5000.;
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "FLAPPY FISH".into(),
					resolution: WindowResolution::new(1280.0, 720.0),
					resizable: false,
					mode: WindowMode::Fullscreen,
					..default()
				}),
				
				..default()
			})
		)
		.add_plugins(RapierPhysicsPlugin::<NoUserData,>::pixels_per_meter(10.))
		.add_state::<GameState>()
		.insert_resource(RapierConfiguration::default())
		.add_systems(Startup, (
			setup_camera,
		))
		.add_systems(OnEnter(GameState::InGame), (
			setup_background,
			spawn_player,
			spawn_ground,
		))
		.add_systems(Update, 
			(
				jump
			).run_if(in_state(GameState::InGame)))
		.run();
}

#[derive(Component)]
struct Player;


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum GameState {
	#[default]
	InGame,
	GameOver
}

fn setup_camera
(
	mut commands: Commands
) {
	commands.spawn(Camera2dBundle::default());
}

fn setup_background
(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/underwater.png"),
		..default()
	});
}


fn spawn_player
(
	mut commands: Commands,
	window: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let window = window.get_single().unwrap();
	let (_, w_width) = (window.height(), window.width());
	let xpos = w_width / -2.0 + PLAYER_WIDTH / 2.0;

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/fish.png"),
		transform: Transform {
			translation: Vec3::new(xpos, 0.0, 1.0),
			scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
			..default()
		},
		..default()
	})
	.insert(Player)
	.insert(RigidBody::Dynamic)
	.insert(ExternalImpulse::default())
	.insert(GravityScale(10.))
	.insert(Collider::cuboid(cfg::PLAYER_WIDTH / 2., cfg::PLAYER_HEIGHT / 2.))
	.insert(ColliderMassProperties::Mass(5.))
	.insert(LockedAxes::ROTATION_LOCKED)
	.insert(LockedAxes::TRANSLATION_LOCKED_X)
	;
}

fn spawn_ground
(
	mut commands: Commands,
	window: Query<&Window, With<PrimaryWindow>>,
)
{
	let window = window.single();
	let (winx, winy) = (window.width(), window.height());
	let ground_size = Vec2::new(winx, cfg::GROUND_HEIGHT);
	let ground_ypos = cfg::GROUND_HEIGHT / 2. + winy / -2.;
	commands.spawn(SpriteBundle {
		sprite: Sprite { color: Color::MIDNIGHT_BLUE, custom_size: Some(ground_size), ..default() },
		transform: Transform::from_translation(Vec3::new(0., ground_ypos, 0.1)),
		..default()
	})
	.insert(Collider::cuboid(ground_size.x / 2., ground_size.y / 2.))
	;
}

fn jump
(
	kb: Res<Input<KeyCode>>,
	mut impulse: Query<&mut ExternalImpulse, With<Player>>
) {
	if kb.just_pressed(KeyCode::Space) {
		if let Ok(mut impulse) = impulse.get_single_mut() {
			impulse.impulse = Vec2::new(0., cfg::JUMP_FORCE);
		};
	}
}
