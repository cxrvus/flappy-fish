use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod structs;
use structs::*;

mod sys;
use sys::*;


fn main() {
	App::new()
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window { title: "FLAPPY FISH".into(), ..default() }),
				..default()
			})
		)
		.add_plugins(RapierPhysicsPlugin::<NoUserData,>::pixels_per_meter(100.))
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
