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
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
		.add_plugins(RapierDebugRenderPlugin::default())
		.init_resource::<RapierConfiguration>()
		.init_resource::<PipeTimer>()
		.init_resource::<Score>()
		.add_state::<GameState>()
		.add_systems(Startup, setup)
		.add_systems(OnEnter(GameState::InGame), (
			new_game,
			spawn_player,
		))
		.add_systems(Update, 
			(
				jump,
				collision_check,
				spawn_pipes,
				move_pipes,
				despawn_pipes,
				update_score_display
			).run_if(in_state(GameState::InGame)))
		.add_systems(OnEnter(GameState::GameOver), game_over)
		.add_systems(Update, play_again.run_if(in_state(GameState::GameOver)))
		.run();
}
