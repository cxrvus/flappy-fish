use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod structs;
use structs::*;

mod sys;
use sys::*;


fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb_u8(16, 28, 48)))
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window { title: "FLAPPY FISH".into(), resizable: false, ..default() }),
				..default()
			})
		)
		.add_plugins(RapierPhysicsPlugin::<NoUserData,>::pixels_per_meter(100.))
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_state::<GameState>()
		.insert_resource(RapierConfiguration::default())
		.init_resource::<PipeTimer>()
		.add_systems(Startup, setup)
		.add_systems(Startup, spawn_pipes)
		.add_systems(OnEnter(GameState::InGame), (
			reset_background,
			spawn_player,
			spawn_ground,
		))
		.add_systems(Update, 
			(
				jump,
				collision_check
			).run_if(in_state(GameState::InGame)))
		.run();
}
