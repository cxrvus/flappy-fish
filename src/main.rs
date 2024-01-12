use bevy::{prelude::*, window::{WindowResolution, PrimaryWindow}};
use cfg::*;

mod cfg {
	pub const PLAYER_WIDTH: f32 = 512.0;
	pub const PLAYER_HEIGHT: f32 = 256.0;
	pub const PLAYER_SCALE: f32 = 0.5;
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "FLAPPY FISH".into(),
					resolution: WindowResolution::new(1280.0, 720.0),
					resizable: false,
					..default()
				}),
				..default()
			})
		)
		.add_systems(Startup, (
			setup_camera,
			setup_background,
			spawn_player
		))
		.run();
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
		// transform: Transform::from_translation(Vec3::new(xpos, 0.0, 1.0)),
		transform: Transform {
			translation: Vec3::new(xpos, 0.0, 1.0),
			scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
			..default()
		},
		..default()
	});
}
