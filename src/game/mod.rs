use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod structs;
use structs::*;

mod sys;
use sys::*;

mod cfg;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.))
			// .add_plugins(RapierDebugRenderPlugin::default())
			.init_resource::<RapierConfiguration>()
			.init_resource::<PipeTimer>()
			.init_resource::<Score>()
			.init_state::<GameState>()
			.add_systems(OnEnter(GameState::Setup), setup)
			.add_systems(OnEnter(GameState::InGame), (new_game, spawn_player))
			.add_systems(
				Update,
				(
					jump,
					handle_collisions,
					spawn_pipes,
					scroll_objects,
					despawn_pipes,
					update_score_display,
				)
					.run_if(in_state(GameState::InGame)),
			)
			.add_systems(OnEnter(GameState::GameOver), game_over)
			.add_systems(
				Update,
				(play_again, despawn_oob).run_if(in_state(GameState::GameOver)),
			);
	}
}
